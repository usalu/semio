# Filesystem Namespace Durability and Actor Compaction Audit

## Outcome

The newly landed filesystem lifecycle helpers have the right ordinary ordering
for a new WAL name: create the directory, create the leaf with `create_new`,
sync the file, then sync its parent.  They are **not yet a native durability
qualification**.  Two changes are required before treating the native boundary
as safe:

1. a recovery/header `Fsync` must repair an entry whose original `WalCreate`
   returned after a file mutation but before its parent barrier; and
2. all marker/state and namespace accesses must reject links/reparse points and
   malformed markers rather than treating them as storage authority.

Separately, the current live actor compactor has a direct cross-document data
loss bug: it can delete a globally shared payload referenced by another
document.  Payload deletion must be disabled from the per-document pass until
there is a storage-global reference authority.

This was source review only.  No source gate, native test, power-loss test, or
compaction test was run.

## P0: Actor Compaction Can Delete Another Document's Live CAS Blob

`PayloadStorage` is expressly content-addressed and **shared across every
document** ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4733)).
The actor path is real: `ArtifactEngine::compact_retained` gives its retained
`ArtifactWal` to `retained_compaction_with_wal`
([artifact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1607-1609)),
and the authority dispatches `ArtifactMessage::Compact` through that method
([artifact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:3756-3764)).
Thus this is not a test-only `Compactor` concern.

The liveness scan accepts only one `document` and calls
`replay_committed_document(storage, document, ...)`
([compact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1194-1228)).
It classifies a CAS hash as live only if it appears in an unselected segment
of that one document.  The live actor pass then deletes a candidate from the
global payload facet ([compact](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1315-1334)).

Concrete loss sequence:

1. Documents `A` and `B` use the same backend and have a valid committed
   `WalPayloadRef::CasRef(H)` for the same `H`.
2. `A` has a snapshot-covered sealed segment containing `H`; `B` retains a
   live active or unselected sealed segment containing `H`.
3. Compacting `A` makes `H` a candidate, sees no local `A` live reference, and
   calls global `payload.delete(H)`.
4. `PayloadStorage::get(H)` for `B` returns `NotFound`, despite `B`'s valid
   committed WAL reference.

The current `PayloadStorage` trait has no global-document enumeration or
transactional reference index.  The smallest safe correction is therefore to
retain payloads in `retained_compaction_under_lease` (leave
`payloads_deleted == 0`) while retaining the independently safe sealed-WAL
deletion.  Do not make a local-document scan look global.  A future
`PayloadRefIndex` must be storage-global and atomically updated with all
WAL/snapshot references before it can authorize `delete(H)`.

Required first law: seed two distinct documents in one `MemoryStorage` with
the same committed `CasRef`; make only `A`'s segment snapshot-covered; compact
`A`; require the selected `A` segment is deleted but `payload.get(H)` still
succeeds for `B`.  The existing committed fixture helper in
`compact/🦀️.rs` is the appropriate physical-chain seeding seam.

## Filesystem Review

### What the landed helpers get right

`durable_wal_create` uses `create_new`, file `sync_all`, and a parent directory
barrier ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6816-6827)).
`durable_wal_seal` creates or validates the zero-length marker, syncs it, then
syncs the WAL directory ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6829-6847)).
Deletion uses the safe name order: segment first, directory barrier, marker
second, directory barrier ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6849-6861)).
`replace_step` syncs the temporary before rename and then its parent
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7177-7204)).

`sync_directory` uses `FILE_FLAG_BACKUP_SEMANTICS` on Windows and propagates
both open and `sync_all` errors; it does not silently fall back to success
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6782-6801)).
This is the correct fail-closed shape.  It still needs actual Windows native
execution: no source-only claim can establish that the chosen access mode and
directory handle are accepted on the repository's supported Windows filesystems.

### P0: A Failed Create Followed by Reopen Does Not Repair the Parent Barrier

The lifecycle hook can fail after `SegmentCreated` or `SegmentFileSynced` but
before `SegmentParentSynced`.  The leaf can consequently remain visible on a
running machine without any acknowledged namespace barrier
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6821-6826)).

`ArtifactWal::open_acquired` does make an existing, active sub-header segment
usable: it calls `SegmentWriter::initialize_existing_empty`
([wal](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2373-2387)).
That initializer writes and Fsyncs the header, but its Fsync is `WalSync`; the
filesystem implementation syncs only the segment file
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7340-7346)),
not the document directory.  Therefore reopen **does not repair** a surviving
pre-parent-barrier entry.  If power loss instead drops the entry, list is empty
and the opener creates a fresh segment; that is safe for an uncommitted empty
segment but it also cannot repair the lost prior name.

The smallest safe, implementation-local correction is to make filesystem
`WalSync` with `Fsync`/`Quorum` run `sync_directory(wal_document_dir)` after a
successful segment-file sync.  This deliberately pays one directory flush per
durable WAL sync, but it repairs the exact existing-empty recovery path without
changing the shared `WalStorage` trait.  Do not claim a more selective fast
path until a retained namespace-operation phase survives `WalCreate` failure
and recovery.

Required law: inject after `SegmentCreated` and after `SegmentFileSynced`,
close that storage instance, reopen the same root, let `ArtifactWal::open`
initialize the segment, and assert the trace contains a directory barrier
*after* the recovery header Fsync.  The same law must cover a missing-entry
branch by reopening after deleting the unacknowledged file, proving a new
create performs file sync then parent barrier.  A same-process reopen without
the post-recovery barrier is insufficient.

### P0: Marker and Namespace Checks Follow Links and Accept Invalid State

The storage root is made from user-admitted path authority, but subsequent
operations use following APIs:

* `durable_directory` accepts `AlreadyExists && path.is_dir()`
  ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6803-6813));
* `durable_wal_seal` uses `metadata` for the segment and reopens an existing
  marker normally ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6829-6839));
* `WalState` calls `metadata(marker)` and treats **any** successful metadata as
  `Sealed`, while append/truncate use `marker.exists()`
  ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7323-7334,7359-7368)).

Thus a zero-byte symlink marker to a file outside the storage root passes the
idempotent seal check, and any directory/non-empty marker is reported sealed
by `WalState` even though a later `WalSeal` rejects it.  A link/reparse point
in `root/wal/<document>` also makes writes and directory flushing operate
outside the admitted root.  This is a storage-authority and integrity breach;
it is not safe to make a durable-storage claim on a hostile/shared filesystem
root with this path traversal behavior.

Before native qualification, centralize marker state in one private helper:

* `NotFound` means active;
* only a no-follow, zero-byte regular marker means sealed;
* every other file type, size, link/reparse point, or stat error is
  `DbError::Corrupt`/`DbError::Io`, never a state value.

Use that helper in seal, `WalState`, append, and truncate.  `FsStorage::open`
and each path-component traversal also need a repository-owned no-follow
directory-handle walk (Unix directory descriptors with no-follow component
opens; Windows `FILE_FLAG_OPEN_REPARSE_POINT` plus reparse inspection).  A
canonicalize-before-use check is not enough because the path can change before
the later open.  Until that adapter exists, document the filesystem root as a
trusted, exclusive directory rather than a containment boundary.

Required laws: regular empty marker is sealed; non-empty regular marker,
directory marker, dangling marker link, and external zero-byte link all fail
closed for `WalState`, append, truncate, and seal.  Include a `wal/<doc>`
link/reparse test that proves no outside target receives a write.

### P1: Directory Recursion Is Conservative Recovery, but Expensive

`durable_directory` recursively visits every ancestor and calls
`sync_directory` even for established directories
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6803-6813)).
Every WAL create and replacement can therefore issue O(path-depth) directory
flushes, including filesystem root.  It is conservative: after a prior failed
directory create, an existing component might still need its parent barrier.
It must not simply be changed to “sync only newly created directories,” because
that abandons recovery for that failed-create case.

The clean optimisation comes only with a retained namespace state machine:
record each component created by the exact operation; barrier its parent before
advancing; on a failure retain the same phase and retry only that barrier.  On
fresh operations, a root authority already admitted by `BackendOpen` can avoid
walking stable ancestors.  Until then the present cost is safer than a
stateless optimisation.  It should nevertheless be measured by a depth-8
native trace, because an Fsync per stable ancestor on every snapshot/index
replacement is material.

### P1: Replace Is an Uncertain Mutation, Not a Retry-Safe Operation

After `std::fs::rename` succeeds, `ReplacementRenamed` can inject an error
before the parent barrier ([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7197-7203)).
The task returns an ordinary I/O error, but the target can already hold the new
bytes.  The current law writes a *different* second snapshot after that error
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6949-6965));
it does not establish what a caller retrying the logical operation may assume.

Do not add a hidden automatic retry of this terminal `DbIoTask`: its input has
already been advanced and the temporary no longer exists.  Treat the error as
uncertain completion, or introduce an owned replacement phase before adding a
retry mechanism.  Add a direct law that starts with old target bytes, injects
after rename, observes `Err` while the target holds the new bytes, and verifies
that no acknowledgment was emitted before the directory barrier.  The normal
success path is correct once the barrier returns.

`std::fs::rename` is currently used directly.  Local Rust documentation maps
it to `MoveFileExW` with an internal `SetFileInformationByHandle` fallback on
Windows and warns that detailed behavior differs by Windows/filesystem version.
The implementation has no project-level success fallback, which is good, but
a Windows native replace-over-existing-target law is still required before a
portable atomic-replacement claim.

### P1: Delete Must Not Turn a Stat Failure Into Success

`durable_wal_delete` begins with `if !dir.exists() { return Ok(()) }`
([storage](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6849-6850)).
`Path::exists` suppresses access and other metadata errors.  A permission or
I/O failure can therefore be reported as a successful delete.  Replace it with
`symlink_metadata`: only `NotFound` is idempotent success; all other failures
must map through `io_err`, and a link must fail through the no-follow policy.

## Native Qualification Matrix

| Law | Required assertion |
| --- | --- |
| Create fault + recovery | error after leaf create/file-sync is not an acknowledgment; recovery header Fsync also barriers the parent, or a missing leaf is durably recreated. |
| Seal marker validity | empty regular marker only; malformed/link/reparse markers never yield `Sealed` or authorize append/truncate. |
| Delete fault | segment-first ordering never leaves a surviving old segment active; access/stat error is not `Ok(())`. |
| Replace uncertainty | error after rename has no acknowledgment; target content is explicitly observed as uncertain; normal replacement has temp sync → rename → parent barrier. |
| Windows filesystem | directory handle flag/open/flush and replace-over-existing-target run on a real supported Windows filesystem; no suppression fallback. |
| Cross-document CAS | compacting A cannot make B's committed shared hash unreadable. |

## Qualification Boundary

The current source has lifecycle fixtures, but no executed native result was
reviewed.  The ordinary close/reopen behavior is not a power-loss proof; it
cannot establish an entry whose parent directory was never flushed.  The actor
compactor must not advertise payload GC as safe until the cross-document law
and a storage-global liveness authority exist.
