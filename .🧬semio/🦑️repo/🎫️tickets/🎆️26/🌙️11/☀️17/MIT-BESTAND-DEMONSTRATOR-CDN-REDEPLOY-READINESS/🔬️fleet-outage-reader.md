# Fleet Outage History Reader

The running uploader recorded 603 consecutive `upload-deferred: down` events over 4,996 ms. Its shared `fl/down` history held 118,523 timestamps in 1,659,322 bytes. Each deferral checks both `usable()` and `state()`, and each check previously reread, split and converted the entire file.

The reader now inspects a 4,096-byte tail for a sufficient count of recent faults. If that tail cannot decide, it scans the full history once and remembers the threshold timestamp. An unchanged file reuses the timestamp after checking device, inode, length and nanosecond modification and change times. The threshold is reapplied to the current wall clock on every call, including clock rollback and the exact 15-second boundary. A read whose metadata changes during observation retries and checks the file directly if it remains in motion.

The test corpus covers long histories, sparse reports, future and out-of-order timestamps, partial and invalid records, and unchanged files evaluated at different times. Both the former line scanner and SQLite independently returned the expected result for every corpus row. Two clients with cached results cover append, atomic replacement, in-place reset with a restored modification time, removal and recreation. The client suite passed three tests and 45 assertions.

A separate local file with the same 118,523 records and 1,659,322 bytes returned the same outage verdict for 240 repeated calls. The former reader used 1,433.86 ms. The bounded reader used 2.04 ms, including its first tail read. The benchmark measured the evaluator in one process using a ticket scratch copy.

[Benchmark input](fleet-outage-benchmark.ts)
