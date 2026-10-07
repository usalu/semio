# Interrupted Native Preparation Queue Read

The neutral fixture and registered resource-leases suite cover two injected EINTR interruptions followed by exact directory output, cancellation after interruption, and unchanged EACCES refusal. The output is independently compared with fast-glob. Existing real Node, Bun and Python SQLite races and FIFO/crash/cancellation laws remain exercised.

The first request failed before the test because SEMIO_TEST_ARTIFACT_DIR was absent. The second waited behind a live Nx graph builder; after the host process census reset its process was missing, so it provides no test result. The third fresh registered request actually ran the suite successfully in 3.9 seconds, including the DEBUG exact-oracle diagnostic and the queue PASS diagnostic. Its name still says red-3; its actual result is GREEN. No meaningful before-fix RED is claimed.

Before writing the planned production helper, current-source inspection found that concurrent work had already added retryInterruptedFilesystemRead and wired queued readdir through it. That change was preserved. The helper yields on EINTR with the caller's AbortSignal and propagates every permanent error unchanged. The successful run verifies this current implementation.

Authored test inputs: framework/process/leases/fixtures/interrupted-read.json and the registered resource-leases TypeScript test. Verified current implementation: framework/process/leases TypeScript module. Existing project target and launch registration were reused.

Previous queued native/parity requests stopped when their actual processes disappeared after the host reset; partial bootstrap logs do not prove assertions ran. Fresh requests are necessary.
