# WAL Filesystem Directory-Entry Durability Packet

## Outcome

`FsStorage` currently has durable *file-content* barriers, but it has no durable
directory-entry barrier for WAL lifecycle state. Therefore it must not yet be
used as evidence that an acknowledged `WalCreate`, `WalSeal`, `WalDelete`, or
rotation survives a power loss. This is separate from the new WAL retained
recovery work: recovery cannot find a segment name or a `.sealed` name that
the filesystem has lost.

The smallest coherent repair stays private to the existing filesystem backend:
one platform-specific `sync_directory_entry_parent(&Path) -> Result<(),
DbError>` helper, invoked by the already-admitted filesystem `DbIoTask`s. Do
not add a caller-controlled filesystem API, and do not silently accept an
unsupported directory flush.

## Current Evidence

| Boundary | Current code | Missing durability / correctness condition |
| --- | --- | --- |
| storage root | `BackendOpen` calls `create_dir_all` | Newly created root and ancestors are not parent-synced. |
| WAL directory and segment | `WalCreate` calls `create_dir_all`, `exists`, then `File::create` | The directory chain and `.bin` entry are not persistent. The check-then-create race can let `File::create` truncate a concurrently created segment. |
| seal | `WalSeal` only `File::create(.sealed)` | The marker entry is not durable. A lost marker makes a previously sealed segment active after restart. |
| delete | `WalDelete` removes `.bin` and marker conditionally | Neither removal is durable; the two-name transition has no safe crash order. |
| rotation | `ArtifactWal::rotate` flushes, seals, then calls `SegmentWriter::begin` | A durable old seal followed by a failed/lost successor is intentionally recoverable as all-sealed, but only if seal metadata was actually persisted. |

Exact current locations:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6883-6931` performs `BackendOpen`, `WalCreate`, `WalSync`, and `WalSeal`; `:6959-6979` performs truncate/delete.
- The physical naming authority is `:6577-6584` and `:6628-6659`:
  `wal/<document>/segment-<index>.bin` and the adjacent `.sealed` marker.
- `WalStorage` promises an empty unsealed create, idempotent seal, and a full
  segment deletion at `:4583-4624`; the implementation must make a successful
  durable backend call satisfy those claims after restart.
- `FsStorage` dispatches each call through the registered typed backend at
  `:7256-7269`, `:7298-7345`; `execute` is the existing
  `submit_db_io_task(...).finish()` path at `:7192-7194`. This is the admitted
  `Lane::Io` route to retain.
- `SegmentWriter::begin` creates the leaf and then writes and fsyncs the
  segment header at `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2132-2156`.
  `ArtifactWal::rotate` flushes, seals, poisons the old writer, then begins
  the successor at `:2484-2500` (the presently checked source has the same
  control flow around `rotate`).

## Exact Contract and Call Order

Strengthen the private filesystem implementation of the existing trait; no
new `WalStorage` method is needed.

1. `BackendOpen` and `WalCreate` must ensure the directory chain one component
   at a time. For every component actually created (`root`, then `wal`, then
   document), flush its **parent** before proceeding. `create_dir_all` cannot
   establish this because it hides which entries were newly added. Treat an
   extant non-directory as an error. For a relative root, the parent barrier is
   `.` rather than an empty `Path`.
2. Create the segment with `OpenOptions::create_new(true).write(true)`, never
   `exists` plus `File::create`. Sync the new regular file, close it, then
   flush the document directory. Success means the empty active `.bin` name is
   restart-visible. The later header write still needs the existing
   `WalSync(Fsync)` file barrier.
3. For sealing, first verify the `.bin` exists. Create the zero-byte marker
   using `create_new`; on `AlreadyExists`, validate the existing marker is the
   expected regular zero-byte marker and continue the barrier (this is the
   trait's idempotence path). Sync the marker, close it, then flush the
   document directory. Do not report `Sealed` before that last flush.
4. Delete in safe order: remove the `.bin`, flush the document directory,
   remove `.sealed`, flush the directory again. A crash after the first
   successful barrier leaves a harmless stale marker with no segment, which
   current `WalState` already treats as `NotFound` (`storage/🦀️.rs:6942-6950`).
   The reverse order can expose a still-present old segment as *active* after
   restart, so it is unsafe. A failure after the first removal is an uncertain
   completion: return the I/O error, do not publish compaction completion, and
   let idempotent retry remove the remaining marker.
5. For `rename`-based publication, the generic sequence is: fsync temp file,
   rename, then flush the source parent and (if different) destination parent.
   None of the WAL lifecycle operations currently renames; this is needed by
   `replace_step` (`storage/🦀️.rs:6798-6821`) and is deliberately not a reason
   to implement seal as a temp-marker rename.

`ArtifactWal::rotate` then has the desired crash states:

```
flush active bytes -> durable old .sealed -> create+durably-name successor
 -> durable successor SegmentHeader -> accept a caller's next transaction
```

A crash after the old seal and before a successor header is the documented
all-sealed transient (`WalStorage` says it is valid at `storage/🦀️.rs:4580-4582`);
opener recovery must create/resume the successor, not append to the sealed
highest segment. A failure after `seal` is already fail-stop for the in-memory
writer because `rotate` poisons it before returning the seal result. Do not
try to roll the durable marker back.

`WalTruncate` is not a directory-entry transition. Its `set_len` at
`storage/🦀️.rs:6959-6970` remains safe only because every recovery caller
follows it with the existing `WalSync(Fsync)`; retain that ordering.

## Platform Adapter (No New Runtime Dependency)

The helper is internal to `fs_storage`, maps every platform error through the
existing `io_err`, and is called only from the existing `BackendOpen`,
`WalCreate`, `WalSeal`, `WalDelete`, and rename/replace tasks. It must never
return success by ignoring `EINVAL`, `ENOTSUP`, access, or sharing errors.

| Target | Required operation | Source-backed detail |
| --- | --- | --- |
| Linux/other supported non-Apple Unix | Open the parent directory and `fsync` it under the admitted task. `File::sync_all` maps to `fsync` there. | Local Rust source `library/std/src/sys/fs/unix.rs:1413-1424`. |
| macOS | Open the parent directory, then use a small repository-owned direct system `fsync(fd)` adapter on its raw fd, returning its error. Do **not** use `File::sync_all` for this directory adapter: Rust maps it to `fcntl(F_FULLFSYNC)` on Apple. | Rust's public `File::sync_all` calls `inner.fsync` (`library/std/src/fs.rs:748-781`) and the Apple branch uses `F_FULLFSYNC` (`sys/fs/unix.rs:1417-1423`). The direct Unix FFI is a system-library call, not a new crate dependency. Native macOS law must prove the directory call succeeds. |
| Windows | Open a directory handle with the public `std::os::windows::fs::OpenOptionsExt`: `FILE_FLAG_BACKUP_SEMANTICS` (`0x0200_0000`) is mandatory, then call `File::sync_all`, which Rust maps to `FlushFileBuffers`. Keep this behind the same private adapter and propagate a failed open/flush. | Rust's local directory implementation calls `CreateFileW` with `FILE_FLAG_BACKUP_SEMANTICS` (`sys/fs/windows/dir.rs:77-92`); public `access_mode`/`custom_flags` are available (`os/windows/fs.rs:139-165, 195-224`); `sync_all` maps to `FlushFileBuffers` (`sys/fs/windows.rs:400-407`). Exact desired-access compatibility must be proved by the Windows native law; do not replace a failed flush with `Ok(())`. |

The repository has examples of dependency-free OS FFI and Windows custom flags
in `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:694-720,723-745,914-927`.
Those are implementation style references only. Do **not** reuse
`sync_fence_parent` from `:929-937`: its Windows arm is an unconditional
success and therefore cannot prove WAL durability. Do not reuse the repo
normalizer's `fsyncDirectory`, either: it deliberately suppresses
`EINVAL`/`ENOTSUP`/`EISDIR` at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:8338-8345`.

Likewise `pack::write_atomic` has only temp-file `sync_all` plus `rename`
(`🧰️framework/🔨️modules/🎒️pack/🔌️io/🦀️.rs:123-140`), so it is useful only
after extracting/adding the parent barrier; it cannot be reused unchanged as
the durable lifecycle primitive.

## Exact Fault and Native-Law Seams

Current `FaultStorage` cannot test this: it explicitly injects only append,
sync, and catalog-CAS and delegates `create_segment`, `seal`, and
`delete_segment` unchanged (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️.rs:254-405`).

Add a private `FsDirectoryEntryOps`/`FsLifecycleFault` test seam owned by
`FsDbIoExecutor`, not by `WalStorage` callers. The real implementation makes
the OS calls above; the test implementation records and can return an error
**after** a delegated mutation. The required phase names are:

- `AfterDirectoryCreateBeforeParentFlush`
- `AfterSegmentCreateBeforeFileFlush`, `AfterSegmentFileFlushBeforeParentFlush`
- `AfterSealMarkerCreateBeforeFileFlush`, `AfterSealMarkerFlushBeforeParentFlush`
- `AfterRenameBeforeSourceParentFlush`, `AfterSourceParentFlushBeforeDestinationParentFlush`
- `AfterDeleteSegmentBeforeParentFlush`, `AfterDeleteSegmentParentFlushBeforeMarkerDelete`, and `AfterDeleteMarkerBeforeParentFlush`.

Each injected error is uncertain completion, not a rollback request. Reopen or
retry must establish the post-crash invariant. The first compact native corpus
should be:

1. create a segment at a new nested root; assert barrier ordering includes each
   newly created parent, file flush, then document-parent flush; concurrent
   same-index create yields `AlreadyExists` without truncating the first bytes;
2. inject after seal-marker creation; first rotate errors and admits no further
   append, a re-open/retry observes sealed rather than active, and only then
   opens the successor;
3. inject after `.bin` deletion and its parent barrier; list/state never
   resurrects the segment as active, then idempotent delete removes the stale
   marker;
4. inject after rename before parent flush using `replace_step`; acknowledged
   snapshot/catalog replacement is prohibited until its parent barrier;
5. run 1--3 under the platform adapter on native macOS, Linux, and Windows.
   The current general storage law (`storage/🦀️.rs:9072-9123`) and the active
   abort double-reopen law (`wal/🦀️.rs:2717-2764`) are useful lifecycle hosts,
   but neither currently injects directory metadata loss.

## Qualification Boundary

No build or runtime test was run for this audit. Existing ordinary reopen laws
only show process-retained data, not a power-loss directory-entry guarantee.
The repair is qualified only after the new native platform laws pass and an
unsupported directory-flush API fails closed rather than preserving the
current `FsStorage { durable: true, supports_fsync: true }` assertion at
`storage/🦀️.rs:7284-7287`.
