# Dependency post-publication cleanup

The 18:54:08 UTC m07 dependency child published epoch 3 for all six scopes and completed strict preservation retirement. At 19:46:28 it was in the ordinary generation collector, outside the bindings gate. At 19:54:11 the collector had already been active for 464 seconds, and at 19:56:25 it was still removing a tree at 594.125 seconds into that pass. The changing per-tree counts reflect separate tombstone walks. The last terminal timing still belonged to the previous 18:36:30 failure.

The post-publication collector now uses a two-second or 4,096-entry slice. It starts only after publication and strict preservation retirement. The ordinary selection, current pointer protection, candidate binding check and exclusive generation pins still apply. The slice checks its limits at the existing deletion progress boundaries. It leaves a partially removed generation under its owned tombstone name when the limit expires and releases the pin before reporting the deferral. The slice does not run the separate stale-scratch sweep because its recursive removal has no deletion progress boundary.

An owned later invocation can resume the tombstone. Explicit maintenance and exact pressure collection retain their complete work policies. The slice signal is private to cleanup. Caller cancellation still propagates, real removal failures remain failures in the collector diagnostic, and a pin-release failure takes precedence over the slice deferral.

The neutral fixture saw six removals before the change where the entry slice allowed three. It now exercises both the entry and clock limits, normal and pressure full passes, actual cancellation and removal errors, pin-release failure at expiry, and a subsequent pass that resumes the physical tombstone. The fixture confirms current and referenced directories stay intact. SQLite independently calculates the permitted removal count for each sequence. Four seeded and four focused concurrency tests passed with 428 assertions.

## Observed m07 completion

The original m07 child finished its unbounded normal collector after 719.168 seconds and reported 12 collected generations. Its dependency timing ended at 19:58:14.119 UTC with exit status 0. At 19:58:44 UTC the Ready supervisor entered workspace. This run still used the old inline collector. It measures the delay addressed by the bounded slice, while the neutral fixture verifies later owned resumption.
