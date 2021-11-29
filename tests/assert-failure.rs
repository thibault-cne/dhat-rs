#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn main() {
    let _profiler = dhat::ProfilerBuilder::new().testing().start();

    let _v1 = vec![1, 2, 3, 4];
    let _v2 = vec![5, 6, 7, 8];

    // Test with and without extra arguments.
    let stats = dhat::HeapStats::get();
    dhat::assert!(stats.curr_blocks == 2);

    dhat::assert_is_panic(
        || dhat::assert!(stats.curr_bytes == 31),
        "dhat: assertion failed: stats.curr_bytes == 31",
    );

    dhat::assert_is_panic(
        || dhat::assert!(stats.curr_bytes == 32, "extra {} {}", 1, "2"),
        "dhat: asserting after the profiler has asserted",
    );
}