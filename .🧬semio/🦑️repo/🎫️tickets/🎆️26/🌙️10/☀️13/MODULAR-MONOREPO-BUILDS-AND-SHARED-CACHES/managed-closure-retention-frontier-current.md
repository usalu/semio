# Managed Closure Retention Frontier

2026-10-16. Read-only Source audit and the separate bounded census in managed-closure-observation-current.md. No cache mutation, image, Native lease, deletion or runtime proof.

The existing managed Graph filesystem supplies a hard4GiB outer bound. A point-in-time single-directory census found18,910 source-closure records,383,061,664 logical bytes, largest1,115,889bytes. This observation has no active Native custody and cannot authorize retirement. It does establish that immutable input inventories accumulate beyond the currently selected graph's immediate set.

Current optional Graph retention recognizes explicit optional inference-cache leaves, not top-level build-input-closures/build-lock-closures. Therefore the hard filesystem cap by itself does not establish sustained usable iteration under repeated closure-key changes. Those mandatory records and their referencing inference publications/graph snapshots require a joint retirement transaction; deleting just old-looking hash files could make a valid warm inference result point at a missing inventory. Age/name alone is insufficient ownership, reachability or quiescence proof.

A future cut should first trace every actual reader's retained Graph image/namespace lifetime. Only genuine whole-image EX, complete physical reader exclusion and exact current owner can retire coupled inactive cache data. A reset/compaction must invalidate/recompute every dependent inference publication and graph snapshot, retain the Native journal/bootstrap/guard identities, and preserve error/undefined/cancellation plus accepted children until actual physical settlement. It must neither silently treat missing inventory as current successful scope nor use a new Original/clock for same-operation recovery. Per-key reader/reference custody is preferable for concurrent growth if actually complete; interface declarations or a fresh inode are not proof.

The selected-task raw metadata read also remains unbounded in the current narrow namespace fix. A separate finite descriptor policy should admit record bytes/files/aggregate and validate the full source contract; observed maximum1.1MiB is not itself a quota or safe hard bound. Preserve future growth rather than choosing a limit from this single observation.

This is an acceptance frontier, not an implemented eviction algorithm or permission to reset the managed image. See build-cache-acceptance-current.md for the active broader goal.

## Bounded Reader Successor

The earlier raw metadata read frontier is superseded by [Selected Task Bounded Metadata Reader](selected-task-bounded-record-current.md): actual descriptor/extent/ancestry/EOF admission, private finite schema policy, BOTH4tests/36assertions and BOTH owned strict exit0. Complete paid callback lifetime and coupled Graph retention remain separate; no Native or retirement action ran.
