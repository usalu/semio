# WAL Open Retained Rejection Design

Status: schema, owner-preserving Rust API migration, source oracle, and exact native registration completed on 2026-09-05. Native qualification remains pending.

## Implemented owner flow

- `ArtifactWalAcquiredRejected` returns the original `DbError` and exact `WalWriterPermit`. Its `retry_open` invokes `open_acquired` with that same permit; it never releases/reacquires or admits a generic mutation shortcut.
- `ArtifactWalOpenRejected` distinguishes a failure before acquisition from an acquired failure holding the exact `WalWriterRelease`. `retry_close` returns the original open cause only after terminal ACK; a close fault returns the same release plus its distinct cleanup error.
- `ArtifactWal::{create,open,open_with_control}` no longer await or stringify failed-open release ownership. `open_acquired` returns the exact caller permit and joins all current recovery failures after infallible segment-index retirement.
- `ArtifactEngineOpenRejected` distinguishes pre-WAL, WAL-open, and post-open retained-WAL rejection. Post-open replay rejection closes retained state first and returns the live WAL for explicit bounded close.
- The authority builder/ready channel carries `ArtifactEngineOpenRejected`. `DatabaseDocumentOpenRejected` carries it through `create_document`, `document`, and actor-mounted compaction entry.
- Cluster replication acquires the follower writer before inventory, uses `open_acquired`, and returns the exact permit on recovery rejection. A later close fault returns the live follower WAL.
- CLI repair, migration, document open/create, compaction, and replication explicitly drive typed rejections to terminal before reporting their ordinary cause.

The retained writer signal/backend cell remains the nonpanicking fail-closed owner if a rejection is abandoned. Explicit permit-to-release transfer is dormant: the first `WalWriterRelease` poll requests backend close, so a caller sees the retained rejection while the exact writer is still fenced. Dropping either a permit or an unpolled release requests nonblocking cleanup and leaves the physical guard in the backend table until terminal ACK. Abandonment does not claim terminal success. Normal production paths retain and drive the typed owner explicitly.

## Validated current frontier

- `ArtifactWal::release_failed_open` still awaits a writer release and flattens `WalWriterReleaseFailure` into a formatted `DbError`, losing the only explicit retry witness.
- `create` and every `open_acquired` recovery failure can reach that flattening helper.
- The audit's retained-index cleanup premise is stale: current `DbIoU64List::close_step` returns `bool`, and `open_acquired` completes the infallible index retirement before its final result/owner match. Page retirement remains inside the recovery result future, so its errors already join the normal recovery error path.
- `ArtifactEngine::{create_retained,open_retained}`, the authority builder/ready channel, both Database mount paths, CLI repair/migrate, standalone compaction, and cluster follower open currently consume plain `DbError` and therefore cannot preserve a release witness.
- A cancelled `ArtifactAuthority::spawn` waiter is an additional ownership edge: the generic oneshot stores the builder result, but dropping its receiver can later drop that stored value. A typed rejection alone is insufficient unless the authority or Database mounts undelivered construction owners.

## Frozen neutral contract

The strict fixture and schema are in `db/📝️wal/🧪️fixtures/🚪️open-rejection`. They distinguish:

- failure before acquisition: original cause, no writer owner, same-document acquisition remains available;
- failure with a caller-supplied writer: original cause plus the exact `WalWriterPermit`, and no nested close;
- failure after self-acquisition: original cause plus the exact `WalWriterRelease`;
- repeated close faults: original open cause and latest close cause remain distinct, the same release witness is returned, and a contender remains fenced;
- terminal close: only then may a same-document acquisition succeed;
- dropped rejection: nonpanicking transfer into the backend's fixed release-recovery cell, with no terminal-success claim;
- engine propagation: distinct before-WAL, WAL-open-rejected, and WAL-close-rejected states, retained through authority readiness and Database mounting.

The registered writer source oracle evaluates the exact owner transition and source propagation in addition to AJV validation:

```text
NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/framework-os-kernel:wal-writer-authority-check --skip-nx-cache
exit 0
wal-writer-authority-independent-oracle: AJV=6 exact-u64=1 cases=3 mutations=6 remote=5 writer-slots=32 retained-result=1 directory-barriers=4 wal-open-owner=1 backend-pool-use=5
```

## Exact native registration

The existing `wal-writer-authority-native-check` all-features exact group now includes:

- `db_wal::tests::artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry`
- `db_artifact::tests::artifact_engine_create_rejection_propagates_exact_wal_release_owner`
- `db_engine::tests::database_document_rejection_retains_authority_builder_wal_owner`

## Nonclaims

The source receipt is not a Rust compile or runtime result. Root receipt `yDnbws` built and reached `artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry`, where it exposed the former eager-release defect by allowing same-document reacquisition before retained cleanup. The dormant-transfer correction postdates that receipt and remains native-pending. The first implementation corpus uses injected WAL begin faults and exact conflict/terminal reacquisition; repeated physical guard-close faulting remains covered by the writer-controller laws in the same exact group. Cancellation of an abandoned `ArtifactAuthority::spawn` receiver still relies on the backend release cell rather than a caller-recoverable authority-mount handle and needs a later dedicated cancellation law.
