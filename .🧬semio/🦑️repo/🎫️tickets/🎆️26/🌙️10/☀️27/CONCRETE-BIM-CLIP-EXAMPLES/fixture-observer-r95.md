# Own-Process Fixture Observer

fixture-observer-r95.py is a ticket-only diagnostic helper for subsequent fixture runs. It reads only its owning process through native Windows counters, Linux procfs or the macOS resource interface. It records the current test/geometry frames and bounded reservation/part counts every ten seconds. It never inspects or controls a peer process.

The task-owned fixture-fits-cancel-r95 signal, a 5.5 GiB own resident-memory bound, or a 3,600 second diagnostic bound requests interruption of only the owning Python main thread. The test driver always restores its process-local model loader and settles the observer in finally. Existing current compatibility session3715 started before the observer was added and cannot gain the watcher retroactively.

The observer is bound into future fixture runner input hashes. It has not been executed yet because normal admission is queued under critical peer pressure. It must pass its first real native run before being relied on. No completed test or source publication claim is made by this helper.
