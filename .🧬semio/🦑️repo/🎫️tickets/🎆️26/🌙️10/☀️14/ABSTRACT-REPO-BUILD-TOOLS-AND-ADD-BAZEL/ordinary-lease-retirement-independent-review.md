# Ordinary Lease Database Retirement: Independent Design Review

This is a read-only review of the existing lease owner and High's proposed admission-gate design. No production change, SQLite connection, native image, lease, maintenance, deletion, process probe or runtime was performed. Root/High's retained observations of ordinary DB and system event usage are incoming evidence, not independently measured here.

The design is sound only if the directory catalog excludes every opener during the close-to-unlink interval. Per-resource SQLite EX alone does not exclude an already-open connection that is waiting to acquire its transaction: after retirement closes EX, that connection could later lock the unlinked old inode while a new opener creates a different database at the same path.

## Actual Existing Boundaries

- `acquireResourceLease` validates/materializes a database path at lines164–166, then awaits its SQLite open. Busy retry retains that same open handle through awaited progress and delay. The catalog SH must already exist before that path/open and survive every retry until either the actual BEGIN plus identity SELECT establishes resource custody or the failed DB is closed.
- A successful resource transaction currently remains alive until `release` rolls back and closes it. Once real custody exists, the opener may drop directory SH: retirement's nonwaiting per-resource EX then detects the active holder. Mere `BEGIN` without the subsequent identity SELECT is insufficient for a shared reader because SQLite may defer its actual read lock.
- `queueClaimCatalog` currently calls public `acquireResourceLease` at236. Wrapping every public acquisition in the same catalog recursively would not terminate. The catalog primitive must be private and ungated, limited to its fixed schema-owned resource, with no caller-controlled bypass flag.
- `readResourceQueueClaimIdentityV1` directly opens SQLite at241. It is also an opener and requires catalog protection. A public protected read and an internal already-held-EX read must share authentication logic without acquiring SH under its own catalog EX.
- Claim GC holds the root queue catalog EX then opens/probes databases under `root/claims`; queue admission and sweeping currently hold root catalog SH while acquiring claim DBs. These existing nested calls must be explicitly integrated, not left to recurse through a new ordinary gate.

## Lock Ordering and Exact Directory Scope

A gate per physical DB directory protects only that directory. Holding `root` catalog EX alone does not block an arbitrary public `acquireResourceLease({directory:root/claims,...})` that obtains `claims` catalog SH. Claim retirement must therefore also hold the exact claim-directory admission gate EX, acquired in a fixed parent-before-child order with no waiting on a resource DB, or every claim-directory opener must provably map to one canonical schema-owned parent gate. A caller-supplied catalog selector is not such a proof.

Keep the catalog database itself permanently outside retirement. Otherwise unlinking the catalog creates the same split-lock-domain race the catalog is meant to prevent. All catalog acquisitions must preserve the original incoming signal, control, timeout and awaited progress; ordinary maintenance cannot manufacture a replacement deadline. Automatic or nested maintenance must not wait indefinitely for catalog EX while retaining a resource that a pending SH opener is waiting for. A nonwaiting/busy-skip maintenance entry is the safe general case; explicit waiting requires an owner that has already drained its own conflicting resources.

## Safe Retirement Eligibility and Sequence

1. Under the exact directory gate EX, bound the filename inventory and select only canonical resource-hash SQLite names. Exclude the catalog, queue directories, metadata reserve, `.fseventsd` and all unknown files/directories.
2. Require current no-symlink ancestry, a regular single-link file, bounded extent, canonical hash/resource relation and actual immutable application/version/resource row. Foreign, empty, malformed, linked or unknown databases must be retained/refused. Retirement must not use normal initialization to turn an empty file into owned material.
3. Open/probe the same current candidate under a nonwaiting SQLite EX transaction. Busy means preserve the peer's database and continue/return. Do not use PID existence, guessed liveness, age or a copied marker as authority.
4. Authenticate the resource identity while EX is held, with matching physical inode and canonical named path. After closing that handle, revalidate that the named regular inode is the retained one before unlinking, while directory EX remains held. There must be no awaited unguarded gap allowing a new opener.
5. Preserve any unexplained SQLite sidecar. The normal owner requires rollback-journal mode; maintenance should not broaden itself into deleting WAL/SHM/journal files or unknown material merely because the main filename matches.
6. Release every probe and catalog independently, preserving an arbitrary original failure including `undefined` or a frozen Error. If dropping an admission gate fails before a newly acquired resource can be returned, do not leak the real resource transaction to which the caller never received custody.

The existing allocated metadata reserve is specifically authenticated by `queue-claim-catalog` custody. Reuse/recovery must retain that real owner contract; a new ordinary catalog cannot impersonate it. Catalog bootstrap and metadata writes must still be possible under the existing image quota/headroom or safely refuse. There is no permission to enlarge the 64 MiB image, remove system event data, or bypass SQLite/custody on a full image.

## Required Before Implementation Acceptance

Use a closed portable sequence corpus and independent SQLite oracle, then actual Bun/Node child transactions for at least these boundaries: opener paused after open/before BEGIN blocks retirement; acquired reader/writer survives nonwaiting probe; cancellation closes a waiting DB before gate release; retired idle DB can be recreated without an old-handle lock domain; named-inode substitution refuses deletion; foreign/link/sidecar material remains; claim GC and queued admission/sweeping do not recurse/deadlock; catalog is never retired; release/close failures retain arbitrary primary and attempt all cleanup. A modeled sequence is not physical race qualification.

The exact four inspected Source hashes and opened/named pre/post stamps are retained in `🗑️generated/ordinary-lease-retirement-independent-source.json`. No implementation verdict or reclamation capacity is inferred until the actual patch/laws and bounded physical behavior are reviewed.

## Broader Opener/Deletion Census During Implementation Preparation

A bounded production Source search for `semio_resource_lease` and `resourceLeasePathV1` found two additional actual owner boundaries. This is a Source census, not a live database/image observation.

- `PROCESS/🔒️leases/📦️namespace/📒️baseline/🟦️.ts` has a raw read-only SQLite opener in `readLogical`. Its actual caller is the baseline builder, which holds the root `queue-claim-catalog` EX and recursively uses the macOS physical directory seal during inventory. This is **not** a proven unguarded opener bug. Preserve those protections and avoid adding an SH acquisition beneath its own EX. A per-directory replacement must still account for nested claim directories. Later baseline use binds captured resource device/inode under an issued image lifetime and a metadata generation reader; canonical image EX before ordinary GC must exclude those active immutable-baseline lifetimes. The baseline owner explicitly labels canonical wiring as separate.
- `library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` `withRepositoryCargoIsolatedStoreV1` currently removes a store, calls `lease.release()`, then directly removes `resourceLeasePathV1(directory,nonce)`. That is an actual production deletion outside any new catalog gate. The random nonce reduces accidental sharing but is not authority to bypass an exact retirement boundary; preserve actual storage/control and migrate or otherwise prove exclusion for this owner too.

Both findings and the distinction between baseline's existing protection and Cargo's post-release unlink were sent to Root/High. No changes were made to either owner by this review. Additional Source hashes/stamps are retained in `🗑️generated/ordinary-lease-retirement-extra-openers-source.json`.

## Assigned Private Adversarial Test Slice

Root separately assigned Core a permanent/private fixture test slice once High provides the concrete closed API and schema; no implementation or test files have been edited for that slice yet. High retains runtime/schema/countermodel ownership. All eventual test databases, child coordination material and output belong under ticket Generated; no canonical cache/image/SDK/store is admitted by this release.

The proposed actual race test needs no production pause seam. A parent holds a real target EX transaction. A same-wire owned child attempts the target and reports readiness from its awaited `onWait` after SQLite reports BUSY, so it has an open handle without acquired transaction. A nonwaiting retirement then must refuse or busy-skip with the exact inode preserved. Once the parent releases, the child acquires a real reader/writer; a second retirement must preserve that live transaction using its per-DB EX probe. After the child has actually settled/released, idle retirement and recreation can prove that the new logical resource has one current inode/lock domain. This requires exact child ACK/wire checks, bounded requested phases, all accepted child work drained before directory cleanup, and no new Original or global monkeypatch.
