# Owner look cancellation

The take pass carried a caller-specific cancellation signal into owner inference. Its first wait on the farm look queue omitted the signal. A cancelled pass could therefore retain a wait in the per-owner in-flight map until the native farm replied. Normal inference can join an entry in that map, and the read joins the pass on exit.

A native fixture drives the actual guarded pool and worker through a singleton job. The worker signals that it started, then waits for a fixture-controlled gate. Before the fix, aborting the observer returned no cancellation result during the 500 ms observation window while the worker stayed gated. The queue now withdraws that observer using the original cancellation reason and retains the native job and its result. After the gate opens, a later observer uses the completed answer without a second dispatch. A subsequent request after successful use sends a new job. The neutral fixture and SQLite calculate dispatch counts of 1, 1 and 2.

The same isolated native worker established the existing batch limitation. An early owner marked itself complete, the next owner remained gated and the early answer stayed pending until the terminal batch reply. The observed read queued 4,808 owners in 4,671 jobs, so at most 274 owners were in multi-owner jobs. The cancellation repair uses the existing IPC and applies to singleton jobs. A still-needed inference may independently wait on the same queue-owned native job.

The look queue, isolation, take, borrowed-look and release suites passed 31 tests across seven files. No live read or farm process was modified.
