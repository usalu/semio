# Current Admission Pulse

Read-only repository status observations at 2026-11-21, sampled at epoch 1795283075. No workload, lease, cache or graph was changed.

`repo doctor --only=graph,capacity,storage --json --lang=en` reports graph generation 260, with implementation files newer. Its long-wait finding says PID 24613 has waited 23,308 seconds for the 14,336 MiB show upgrade; capacity headroom is 3,326 MiB above a 4,096 MiB floor. It recommends checking capacity and allowing grants to finish; its separate idle-grant finding says to ask the process owner, not that stopping it is safe.

`repo fleet status --json --lang=en` shows PID 24613 blocked at resource-directory-admission, its 2,747 MiB grant held, queue position 1, with no wait estimate. A separate graph-plan service PID 34231 and worker PID 34356 were running on shard 23 with older implementation digest c03081374080. This slice is not evidence of a new sealed whole graph: fleet reported generation null. Concurrent cargo and graph grants were active.

The same fleet snapshot reported 41,141,440,512 bytes host free, 18,988,101,632 bytes below the established 60,129,542,144-byte lab admission gate. This is a sample, not a reservation. Peer idle Rust workers do not establish an exact, closure-complete remote graph. `repo remote-cache status --json --lang=en` passed source admission but refused the unsupported `--json` option; it returned no independent remote graph result.

Existing `audit-remote-graph-closure-completeness.md` requires referenced closure completeness for a valid remote head; `audit-current-constrained-lab.md` records the physical and source-isolation requirements for the full lab. No new legitimate route was established by these status probes. The concrete next check is the existing queued whole read and physical capacity after other admitted work finishes, or an independently authenticated closure-complete exact remote head; neither warrants bypassing admission.


## Current Graph Query

The native current-policy whole-project query completed with exit 1 because the coordinator had no sealed whole-workspace generation. It started no graph rebuild. Before reaching that verdict, the command reported a stale root dependency receipt for the observed `BUN_INSTALL` environment, processed more than 178,816 parser-source listing operations, and reached 106,496 dependency-verification operations under both shared and exclusive custody. This establishes real prerequisite cost in a current-policy query. Whether the two installer environments differed semantically requires comparison with the pinned producer environment.
