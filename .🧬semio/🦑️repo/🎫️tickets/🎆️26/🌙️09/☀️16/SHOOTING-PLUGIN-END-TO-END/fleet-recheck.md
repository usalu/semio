# Scheduler Recheck

Read-only owned `repo fleet status --lang=en` and `repo leases status --lang=en` snapshot reporting fleet time 23:23:20. Its timezone was not established.

- Three capacity grants reserve 10,530 MB; measured usage is 8,452 MB. Thirteen requests wait, with 376 MB available above the 4,096 MB floor.
- The shooting native request needs at least 1,333 MB and has not been admitted since the value-cycle correction.
- Seventeen leases are held, with zero current lease waiters and zero reported deadlock cycles. Ten recorded order violations refer to earlier Cargo/configuration activity; they are not evidence of a current cycle.
- Active peer work reports graph facts and dependency-generation verification. A graph-custody owner reports a long silent interval, but this snapshot does not establish why or prove it is stuck.
- Storage is within its current budget. This team has no active serve process and has not stopped peer requests, reset caches, rewritten receipts, or lowered admission guards.

TypeScript verification and managed-browser preflight have completed. Native test execution and the subsequent browser acceptance remain pending. Keep the goal and ticket open.
