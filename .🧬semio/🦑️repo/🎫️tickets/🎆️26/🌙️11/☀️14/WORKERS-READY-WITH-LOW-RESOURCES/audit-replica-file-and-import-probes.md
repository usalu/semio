# Replica File and Import Admission Probes

At 19:05 UTC the Windows worker g1-m05 still had about 5,095 incoming entries. Its current replica had observed 31,521 sharing violations. Those are aggregate observations. They do not identify which entries were blocked or how many were observed more than once. The coordinator also had a real local file-phase sample while its ordinary journal head was briefly unchanged. That sample does not attribute the coordinator pause to one filesystem call.

The hot and cold file scanners previously opened the physical metadata of a queued incoming path before declining to observe it as a local publish. Both the successful stat branch and the missing-file branch already deferred to that same queue. The path now leaves the scanner before the metadata request. SQLite independently verifies that queued present and missing cases use zero probes, while a new, settled, changed or removed unqueued file uses one probe and yields the original verdict. The owned queue still performs its normal retry and file checks.

The recursive import closure also checked files already scheduled for examination, when a second source imported the same file before the first entry's turn in the queue. It now marks positively observed targets when they are queued. It does not cache misses, so a later reference can still find a newly available path. The exact output and hashing inputs remain the same on the fixture. The shared target took two real stat calls before the change and one after it, with the same imported paths under the independent Python closure oracle.

The focused source closure tests passed with 16 assertions and the file sweep regression passed with 12. The broader resilience and file-race suites each had one timing-sensitive failure while running together. Both cases passed in isolated reruns, along with the new queue case. No worker process or resource reservation changed.

The complete resilience suite passed 26 tests with 342 assertions in a serial rerun. The real native replica self-check also passed with digest b0cc3cfb71fb53cc3d8524013031bc4201285165ba2ed86f01f3873e2a7fa6fa, 34 modules, 23 schemas and three bilingual catalogues. At 19:13 UTC the coordinator and eight fresh workers attested that exact replica implementation.

The final replica race suite also passed 37 tests with 277 assertions in a serial rerun.
