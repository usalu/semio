# P2-D Global Artifact-CAS Delete Barrier Post-Implementation Audit

## Verdict

**REJECT — do not accept or claim the global deletion barrier implemented.** This is a source-only, read-only audit of the shared tree on 2026-09-03. No build, test, service, database, or runtime command was run.

The storage and directory portions are materially closer to the required topology-independent design than the previous audit: each directory backend now has a stable private coordinator and per-space epoch, each dedicated CAS backend has a physical epoch predicate, and the latest shared-tree revision now binds the coordinator at hub startup. The required two-service/process safety evidence still does not exist, so the end-to-end invariant is not established.

## Required invariant

For a durable `(directory coordinator, space)` pair, the directory epoch and CAS physical epoch must be monotonic and bound the following orders:

```text
reserve durable ownership -> CAS advance(E) -> stage/readback -> atomic reference publication
sweep durable lease D -> CAS advance(D) -> recheck under D -> CAS delete iff physical_epoch == D
```

If an expired holder resumes after a successor reservation, the successor's advance must either precede the old conditional deletion (which then fails) or follow it (which then stages the bytes before publishing). Thus **every successfully referenced CAS object is physically present**. The law must hold for independently selected directory/CAS stores, not merely within one process or one database transaction.

## Evidence that is now structurally present

- SQLite, PostgreSQL, and Neo4j directory control state has a private coordinator identity plus a per-space `fence_epoch`; reservation increments the epoch and returns a fenced reservation. See [SQLite directory](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs), [PostgreSQL directory](../../../../../../../../../🌎️hub/📇️directory/🐘️postgres/🦀️.rs), and [Neo4j directory](../../../../../../../../../🌎️hub/📇️directory/🌐️neo4j/🦀️.rs). The SQLite reservation barrier is at lines 421–429, PostgreSQL at 361–368, and Neo4j at 97–107.
- The three directory implementations acquire a lease, increment its epoch, and query current references and unexpired reservations before minting a private delete fence. SQLite is at lines 1208–1249, PostgreSQL at 1249–1275, and Neo4j at 1137–1167.
- `HubVerifiedCheckpointPublisher::reserve` rejects zero permits and calls `configure_coordinator` followed by `advance_physical_epoch` before it returns a reservation ([directory source](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1724)).
- The current hub source connects the directory first, obtains `artifact_cas_coordinator_id`, configures CAS, and constructs a `CheckpointPublicationOrchestrator` with the publisher. Its main and test state literals now retain that object ([bin.rs:2278](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2278)). This supersedes the incomplete-integration snapshot observed earlier in this audit.
- The latest binary revision also mounts `ArtifactCasMaintenanceSupervisor` after coordinator binding. It defaults execution to dry-run, uses a 30-second cancellable operation context, reports progress, and its live health now makes `/readyz` fail closed. This supersedes the earlier unmounted/stale-readiness snapshots observed during this audit.
- Memory shares the coordinator, epoch map, and objects under one mutex. SQLite, PostgreSQL, and Neo4j CAS variants persist `hub_artifact_cas_space_fence` / `ArtifactCasSpaceFence` and condition deletion on equality. The CAS port remains separate from generic payload storage ([chunk CAS](../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:206)).
- The filesystem implementation now has a real native advisory lock (`flock` on Unix, `LockFileEx` on Windows), domain-separated checksum framing, temp-file write, file sync, replacement, and parent-directory sync ([chunk CAS](../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:670)). This corrects the earlier in-place fence-write finding.
- Sweep requests are bounded to 4,096 objects, default to `execute: false`, checkpoint per object, report committed progress, and use an opaque authenticated, generation- and instance-bound continuation ([directory source](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1572)). The fixture/oracle covers the continuation’s bounded convergence, restart, mode, and generation invalidation; it is not a barrier-race oracle.

## Blocking findings

### P1 — mounted supervisor cannot converge past its 16-request tail

The newly mounted supervisor keeps its continuation only for `for _ in 0..16`, then drops it and begins the next 60-second run with `None` ([bin.rs:228](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:228)). At 4,096 objects per request, a sweep whose immutable candidate set exceeds 65,536 objects repeatedly begins from generation zero. Because the append-only ledger continues to return already-deleted candidates as `Missing`, the supervisor can repeatedly consume the same prefix and fail to converge to the tail. This violates the required bounded opaque-continuation convergence beyond 4,128 objects.

Persist or retain the authenticated continuation across supervisor cycles until it completes, and record a bounded checkpoint/error state that invalidates only on the already specified restart/generation/mode rules. The service-level continuation is correct; the mounted owner discards it.

### P2 — the sweep now has the specified recheck, but lease renewal and structured error cleanup are absent

The latest source now performs `lease/recheck -> CAS advance -> validate_artifact_cas_delete_fence -> conditional delete` at [directory source:1625](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1625). Validation checks the exact token/epoch/expiry, coordinator, ledger generation, references, and reservations in every directory backend. This corrects the earlier no-post-advance-recheck finding.

The directory trait exposes renewal, but the sweep never calls it. A failure/cancellation in `advance_physical_epoch` or validation returns before the release at line 1658; expiry supplies eventual recovery, but there is no scoped cleanup proof or stated bounded recovery behavior. The physical delete result itself is released before its error is propagated, which is the right local ordering. This is not the primary safety-race blocker, but it remains a cancellation/liveness gap.

### P1 — the only race test does not prove the required stale-holder law or an independent process law

The sole named two-service test is [directory source:3061](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:3061). It wraps one shared `MemoryArtifactChunkCasStorage` in an in-process `BlockingDeleteArtifactCas`; it does not open two filesystem handles/processes and cannot exercise the native lock/fence publication path.

More importantly, it attempts the successor reservation while the first five-second lease is still active (line 3092), expects conflict, releases the old sweep, and only then reserves. That verifies ordinary live-lease exclusion, not an expired old holder resuming after a successor advances its epoch. After the successor reservation at line 3095, the test directly stages at lines 3100–3102 without calling `advance_physical_epoch` for that successor permit. It therefore bypasses the decisive reserve/advance/stage sequence it is meant to prove.

There is no second independent service/process race for filesystem CAS, no oracle that pauses old delete across lease expiry then forces successor reserve/advance/stage/publish, and no such oracle for independently selected SQLite/PostgreSQL/Neo4j directory/CAS combinations. The generic `fenced_delete_law` only tests a same-epoch direct delete for memory, filesystem, and SQLite ([chunk CAS:1552](../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:1552)); PostgreSQL and Neo4j have no corresponding source test.

### P2 — filesystem fence path traversal is not fail-closed against leaf/path replacement

The filesystem root is validated as a non-symlink at open, but `advance_physical_epoch` creates the digest-named space directory and opens `fence.lock` / `fence-v1` without symlink-metadata checks or no-follow opens ([chunk CAS:820](../../../../../../../../../🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:820)). The design required rejecting symlinked space/fence paths. The checksum detects corrupt metadata; it does not prevent redirecting the lock or metadata path. This is a fail-closed/safety hardening gap for a locally writable CAS root.

### P2 — dry-run mutates durable coordination state

Although `execute: false` is the default, the sweep takes the directory lease and increments its `fence_epoch` for every candidate before deciding not to touch CAS ([directory source:1625](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1625)). It releases the lease but retains the epoch mutation. The current tests show only that bytes remain; they do not assert a read-only coordination outcome. Decide whether dry-run is allowed to fence a space; if not, split a bounded reachability preview from lease acquisition.

## Backend/topology assessment

| Requirement | Static assessment |
| --- | --- |
| Stable directory coordinator | Present in SQLite/PostgreSQL/Neo4j schemas and now configured by hub startup. |
| Monotonic per-space directory epoch | Present in all three directory implementations; persists outside projection rebuild paths by source inspection. |
| CAS physical equality delete fence | Present in memory, filesystem, SQLite, PostgreSQL, and Neo4j implementations. |
| Filesystem cross-process fence | Native locking and checksummed atomic metadata are present; leaf/path symlink rejection and process-race proof are absent. |
| Independently selected directory/CAS backends | Startup binding, live fail-closed supervisor health, and a mounted dry-run-default supervisor exist, but the supervisor loses its continuation tail and no combination/race oracle establishes it. |
| Memory parity | Structural mutex model is present; it is intentionally non-durable and only indirectly exercised. |
| SQLite/PostgreSQL/Neo4j parity | Source shapes are aligned; PostgreSQL/Neo4j have no audit-visible test oracle. |
| Retention/SpaceDeleted | They release references then defer physical reclamation to sweep. No direct deletion occurs in the event append path. |
| Generic `PayloadStorage` isolation / no public locator | Maintained by the dedicated CAS port and private control fields, by source inspection. |

## Required first correction packet

1. Keep the mounted supervisor, dynamic `/readyz` health gate, and coordinator startup binding. Retain its opaque continuation across ticks until completion (or persist an authenticated bounded checkpoint). Add a source test that forces more than 16 pages and proves the tail is reached rather than repeatedly sweeping the prefix.
2. Keep the new post-advance validation. Add scoped lease cleanup plus bounded renewal only within the request deadline, and treat lease loss as a non-delete outcome.
3. Add a neutral, language-agnostic barrier-race fixture plus an independent oracle for both physical orders. It must run two independently constructed directory services/CAS handles; pause delete after lease expiry; perform successor reserve -> epoch advance -> stage -> publish; then resume old delete and assert the published reference reads exact bytes. Cover memory and two filesystem handles/processes at minimum; use the same oracle with SQLite/PG/Neo feature-backed integration environments.
4. Add filesystem no-follow/symlink rejection for every directory and fence artifact used in lock/read/write paths. Add cancellation/error assertions that prove the lease releases or expires in the documented bounded manner.

Only after those changes compile and the required oracles are run successfully may a subsequent audit assess acceptance. This audit makes no runtime claim.
