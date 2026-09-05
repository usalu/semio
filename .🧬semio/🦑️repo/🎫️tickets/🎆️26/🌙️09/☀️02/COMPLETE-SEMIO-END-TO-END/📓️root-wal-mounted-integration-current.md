# Mounted WAL Integration And Metadata Durability

## Current Qualification Boundary

The writer capability migration is source-coherent across Memory, filesystem, SQLite, WAL, cluster replication, compaction, CLI and FaultStorage. Postgres and Neo4j deliberately refuse writer admission pending honest provider-specific fences. The source oracle passed in 68742 (AJV3) and 66132 (AJV4, directory metadata corpus). Earlier 22590 failed during Nx graph processing; it executed no tests.

Native run 11353 is building the 25-law group at `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-tldqJV/00`. No result is claimed before that run finishes. The prior 262-diagnostic run ZSVUzs caught the intentionally incomplete signature migration, not a qualified current implementation. Historical writer8, WorkerPool11 and sequential WAL23 receipts remain separate.

## Current Changes

Cluster replication now opens and owns the follower WAL before inventory/replay/planning, retains it across both successful and failed replication, and closes it before returning. Record retirement no longer depends on cancellation grants. Two new laws cover an occupied follower even for an apparent up-to-date result and follower release after leader replay failure; the three existing replication paths now prove reacquisition after completion.

Controller results distinguish an exact per-key `Faulted` result from an unexpected executor error. Per-key faults allow a healthy coalesced peer to finish; an outer error faults every currently requested writer and stops instead of hot-looping. New mounted laws cover coalesced release progress and an outer panic waking retained owners once. The generic controller-test helper’s result lifetime was shortened after the previous build diagnostic.

Filesystem segment creation uses atomic `create_new`, followed by file and parent-directory barriers. Seal creation validates an existing marker and flushes both it and its directory. Deletion durably removes the segment before removing its marker. Root/directory construction and same-parent atomic replacements also flush parent entries. A private fixed test-only phase recorder injects post-effect failures through the actual mounted backend; three registered native laws cover ordering, duplicate-create preservation, seal/delete retry and post-rename acknowledgment refusal.

The macOS directory helper calls the system `fsync` directly; other targets use their platform file handle flush with errors propagated. Windows opens directory handles with backup semantics and fails closed on unsupported access or flush. This source is not a Windows qualification, a simulated power-cut test, or proof of every filesystem.

## Remaining Frontiers

The actor-compaction implementation/report is `📓️sol-compaction-actor-owned-wal.md`. Its source oracle is green; native actor queue/cancellation behavior remains pending.

Constructor cleanup currently flattens a failed writer release. The execution agent is preparing typed retained open rejections, with a nonpanicking backend-owned fallback and explicit exact-owner retry; it will not change signatures during the current native boundary. Schema scaffolding is not implementation qualification.

Scheduler refusal can fault an already-pending other writer without independently waking it; direct public Drop/handback notification can inherit application locks. The read-only alternatives and bounded deferred-notification requirements are preserved in `📓️terra-writer-refusal-reentrancy-and-current-api-callers.md`. No solution is claimed yet.
