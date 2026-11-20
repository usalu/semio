# m02 watchdog and whole-graph tail audit

## m02

At 13:08:45 UTC, g1-m02 attested replica code `5455ef8a…` from its fallback snapshot. Its server cursor reached `157:228729`, equal to the server's head observed at 13:08:48 UTC, with zero incoming entries. It had moved from `156:260238` through `156:520484` during this audit. The snapshot's replica loop is synchronizing, and the earlier queue backlog cleared. Its existing readiness receipt still carries a successful bootstrap time of November 18 at 09:25:07 UTC.

The newer digest `70491258c6242104df9a35e8d723d84a2d9d039fbeec8c5f4cc4e151855002ab` remains refused on m02 after the November 19 06:51:04 UTC record. The durable message says the worktree daemon had no ready marker within 900 seconds. The supervisor records that as an early failure, selects the fallback snapshot and the daemon reports the refused digest as ineligible for automatic adoption. g1-server and g1-m06 have both attested that same newer code without fallback. The 13 round failures are counters in the replica attestation. The latest shared resident-service failure predates this refusal and names a graph slice exit, so it supplies no startup root cause. The shared receipts contain no startup phase, local boot log or stack for the rejected candidate. They do not establish an intrinsic code defect or a basis for clearing the refusal. No reset, forced candidate adoption, or source change was performed.

## Whole-graph read

The automatic read started before R233 and remains at 4,787 of 4,788 owners accounted for. Its sole remaining owner, `@semio-tech/framework-os-dev`, has zero emitted rows after approximately 24,000 seconds. The log names a farm child's stored-record verification, with one busy child. The busy child sampled on the coordinator consumed about 92% CPU. The three-second macOS sample showed the main thread executing unsymbolicated Bun/JavaScriptCore frames. It did not identify a remote endpoint or a blocked network stack.

The current farm extends its per-job timeout when child CPU time advances. The read-wide watchdog also includes child CPU in its progress signal while an owner is active. In this situation, CPU activity can keep verification alive without any published owner progress. The sampled production child provides no symbolized evidence for the exact computation. The farm uses local IPC for its verification reply. m06's selected-graph take path separately uses remote cache fetches, and the Linux handoff applies cancellation there. There is no direct evidence that m06's fetch issue accounts for this live parked verifier. Neither the old runner nor its children were interrupted, and no whole read was started.

The short stack sample was temporary and was removed after inspection.

## Readiness holder custody

The integration baseline reproduced three failures. The fixture wrote a single registry heartbeat for a supposedly renewing holder and kept reporting a dead PID for twelve seconds without reaching its reclaim path. The fixture now renews the actual registry row atomically, waits for holder and reader-record publication, and completes its simulated reclaim after observing the fixture PID leave.

The watchdog delayed the first custody lookup until a second progress poll because it required a positive elapsed wait. The new first-recognition regression measured a 2,051 ms delay before the fix and passed below its 1,800 ms bound afterward. The watchdog now checks the first recognized named wait while preserving the independent holder-check cadence, silence and expiry rules, and positive yield threshold. The foreground stage can still make only a background reader with matching machine record and process identity give way.

The readiness holder integrations and the process-tree suite passed all 13 tests with 93 assertions, including the independent Python custody oracle and Windows-shaped PID cases. The tests used ticket-owned scratch and fixture processes.
