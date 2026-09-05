# Mounted WAL Integration And Metadata Durability

## Current Qualification Boundary

The writer capability migration is source-coherent across Memory, filesystem, SQLite, WAL, cluster replication, compaction, CLI and FaultStorage. Postgres and Neo4j deliberately refuse writer admission pending honest provider-specific fences. The source oracle passed in 68742 (AJV3) and 66132 (AJV4, directory metadata corpus). Earlier 22590 failed during Nx graph processing; it executed no tests.

Native run 11353 exited 1 during the 25-law build at `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-tldqJV/00`. Its remaining errors were remote private-adapter UFCS calls, an omitted engine-test trait import, two fully-qualified raw artifact fixture mutations, and release-error diagnostic conversions. Root and the execution agent corrected those sources. Retry 47635 built and listed the expanded 26-law group at `🗑️generated/wal-writer-authority-exact/exact-cargo-laws-kWbc23/00`; its first 16 exact native assertions passed. The seventeenth SQLite subprocess law returned status 0, but its inherited child stdout duplicated the exact test name, so the strict runner correctly rejected the group before later laws. Both child invocations now capture output and include it only on failure; a new full run is required. The prior 262-diagnostic run ZSVUzs caught the intentionally incomplete signature migration. Historical writer8, WorkerPool11 and sequential WAL23 receipts remain separate.

## Current Changes

Sequential run12508 completed RED with two concrete native failures. Writer26 at `exact-cargo-laws-TaSKsE/00` passed its first25 (including SQLite captured children, all four filesystem metadata laws and four replication laws); final snapshot replication rejected reusing an outstanding read-result aggregate as a new write task. Compaction3 at `exact-cargo-laws-2V4Mai/00` passed the committed-effects and global-CAS-preservation laws, then the live actor law aborted during lifecycle cleanup. The execution lane owns that compaction repair.

Snapshot replication now boundedly copies the read result into a distinct input owner and terminally closes the source lease before admitting the follower write. A 36,000-byte neutral pattern spans three pages and the strengthened native law checks distinct operation IDs, exact payload/hash and explicit final read retirement. The registered source gate first failed as intended in8937, then passed in46507 after implementation (AJV5). This is source qualification only pending the native repeat. Drop/cancellation still follows the existing fixed lost-owner cleanup authority; no new cancellation guarantee is claimed by this wrapper repair.

Fresh committed-WAL run 65998 is GREEN: 23 exact native assertions at `🗑️generated/exact-cargo-laws-yoofrE/00`, executable SHA-256 `901ef2ebb68e886b974f88e66607fda0fc914523964ba10db62546c0f4705cfa`. This qualifies the mounted writer migration's committed replay, retained decoding, actor history, filesystem abort/reopen and committed compaction effects for that executable. It does not qualify the writer26 group or the new actor-owned compaction law. A sequential Nx run now queues writer26 and compaction3 against the warm cache with captured SQLite child output.

Cluster replication now opens and owns the follower WAL before inventory/replay/planning, retains it across both successful and failed replication, and closes it before returning. Record retirement no longer depends on cancellation grants. Two new laws cover an occupied follower even for an apparent up-to-date result and follower release after leader replay failure; the three existing replication paths now prove reacquisition after completion.

Controller results distinguish an exact per-key `Faulted` result from an unexpected executor error. Per-key faults allow a healthy coalesced peer to finish; an outer error faults every currently requested writer and stops instead of hot-looping. New mounted laws cover coalesced release progress and an outer panic waking retained owners once. The generic controller-test helper’s result lifetime was shortened after the previous build diagnostic.

Filesystem segment creation uses atomic `create_new`, followed by file and parent-directory barriers. Seal creation validates an existing marker and flushes both it and its directory. Deletion durably removes the segment before removing its marker. Root/directory construction and same-parent atomic replacements also flush parent entries. A private fixed test-only phase recorder injects post-effect failures through the actual mounted backend; three registered native laws cover ordering, duplicate-create preservation, seal/delete retry and post-rename acknowledgment refusal.

The macOS directory helper calls the system `fsync` directly; other targets use their platform file handle flush with errors propagated. Windows opens directory handles with backup semantics and fails closed on unsupported access or flush. This source is not a Windows qualification, a simulated power-cut test, or proof of every filesystem.

Audit follow-up adds a fourth filesystem law: failures after segment creation and file flush are followed by independent reopen; recovery's header Fsync must also barrier the surviving directory entry, or durably recreate a missing entry. Filesystem WAL Fsync now flushes the directory chain and the document directory after the regular file. Delete uses explicit metadata errors instead of `Path::exists`, so access failures cannot masquerade as successful deletion.

## Remaining Frontiers

The actor-compaction implementation/report is `📓️sol-compaction-actor-owned-wal.md`. Its source oracle is green; native actor queue/cancellation behavior remains pending.

The later audit found that payload storage is global while the old liveness scan was document-local. The execution agent removed document-scoped CAS deletion, added a shared/private two-document regression, and preserved sealed-WAL retention. Source 76244 is green; the three-law compaction native group remains queued. A storage-global reference authority is still required for actual payload reclamation.

Constructor cleanup currently flattens a failed writer release. The execution agent is preparing typed retained open rejections, with a nonpanicking backend-owned fallback and explicit exact-owner retry; it will not change signatures during the current native boundary. Schema scaffolding is not implementation qualification.

Scheduler refusal can fault an already-pending other writer without independently waking it; direct public Drop/handback notification can inherit application locks. The read-only alternatives and bounded deferred-notification requirements are preserved in `📓️terra-writer-refusal-reentrancy-and-current-api-callers.md`. No solution is claimed yet.
