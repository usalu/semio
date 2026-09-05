# WAL Open Retained Rejection Design

Status: schema and read-only design completed on 2026-09-05. No Rust constructor signature or writer-controller source was changed while the root-owned writer native snapshot was running.

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

The registered writer source oracle now evaluates the exact owner transition in addition to AJV validation:

```text
NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:wal-writer-authority-check --skip-nx-cache
session 96806, exit 0
wal-writer-authority-independent-oracle: AJV=5 exact-u64=1 cases=3 mutations=6 remote=5 writer-slots=32 retained-result=1 directory-barriers=4 wal-open-owner=1
```

## Coherent implementation plan

1. Replace `release_failed_open` with `ArtifactWalAcquiredRejected` and `ArtifactWalOpenRejected`. `open_acquired` returns the caller's exact permit untouched; self-acquiring entry points synchronously transform it into an unpolled release owner.
2. Give the public rejection an owning close/retry operation. Every fault returns the original cause, latest close cause, and same release. Moving parts out is explicit; no conversion to `DbError` consumes a retained owner.
3. Add a nonpanicking abandoned-release transfer in the fixed writer signal/controller authority. Drop requests fail-closed cleanup and leaves the actual guard backend-retained; it never reports success. Explicit close remains the normal path.
4. Propagate `ArtifactEngineOpenRejected` through the authority construction result. A post-WAL replay error that cannot terminally close returns the whole WAL owner. The authority/Database must mount an undelivered builder rejection so cancellation of the waiter cannot discard it.
5. Migrate Database create/open, CLI, cluster, standalone compaction, sync, and tests in one cross-file patch. Cluster uses `acquire_writer -> open_acquired` and retains the returned permit in its existing outer cleanup rather than closing and reacquiring.
6. Add the native fault law only after the signature migration is coherent. The mounted backend must fail the initial segment operation and at least two writer closes, proving conflict before each exact retry and reacquisition only after terminal cleanup; direct WAL and engine/authority propagation are both required.

## Nonclaims

The fixture receipt is a schema/source oracle, not a Rust compile or runtime result. No constructor owner preservation, abandoned-release recovery, or native fault behavior is claimed yet.
