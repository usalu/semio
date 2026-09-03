# P2-D Global Artifact-CAS Delete Barrier Design Audit

## Decision

Adopt one logical **fenced space barrier** with two persistent parts:

1. the selected directory backend owns a short, expiring, per-space lease and a monotonic `fence_epoch`; and
2. the selected dedicated CAS backend owns a persistent per-space physical fence epoch and conditionally deletes only when that epoch still equals the delete permit's epoch.

This is the smallest design that closes the rejected P2-D race for co-located and independently selected directory/CAS stores without a distributed transaction or a process-local mutex.  It is safe even when a process pauses after its directory lease expires: a new reservation advances the CAS physical epoch before it stages; a stale delete then loses the conditional comparison.  A lease token by itself is not a sufficient fence.

This is a read-only design audit.  No production/test source was changed and no build, test, or runtime command was run.

## Why the in-flight token-only change must not land alone

The current shared diff has expanded `ArtifactCasDeleteFence` with a token and an expiry (`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:119-156`).  The directory backends still mint fences without those arguments (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1173-1195`; equivalent PostgreSQL/Neo4j code), and the physical stores only inspect key/expiry before a raw delete (`🗂️chunk-cas/🦀️.rs:574-581`, `:785-802`, `:867-878`, `:934-945`).  Thus it is incomplete in the shared tree, and, even after its constructors are wired, it would not resolve the race: an old process can be paused immediately after the expiry check, a new process can reserve/stage, and the old process can resume the raw delete.

Do not replace the old local mutex with an expiry/token check.  Do not add a process-global static lock.  They have no cross-process fencing effect.

## The exact safety invariant

For each `(coordinator_id, space_id)` the directory has a strictly increasing `fence_epoch`, and the CAS has a strictly increasing `physical_epoch`.

```text
new reservation:
  directory lease epoch E
  -> durable reservation
  -> CAS advance_physical_epoch(E)
  -> stage/readback
  -> atomic publish/reference

sweep deletion:
  directory lease epoch D
  -> CAS advance_physical_epoch(D)
  -> directory recheck under lease D
  -> CAS delete only if physical_epoch == D
```

The following are mandatory.

- An authority cannot obtain a staging permit before its reservation is durable.
- An authority cannot publish until staging/readback succeeded under such a permit.
- A sweep cannot obtain a delete permit unless the directory proves that its lease is current and the object has no reference or live reservation.
- `advance_physical_epoch` and `delete_if_epoch` serialize in the CAS backend for the same space.  A larger epoch never decreases.
- Lease expiry may allow a successor to acquire the directory lease, but a stale holder can never delete after the successor has advanced the CAS epoch.
- Directory lease state and CAS physical-fence state are private control state.  They are not public events, locators, generic payloads, or client capabilities.

The decisive race has only two physical orders:

```text
A: delete_if_epoch(K, E)       B: advance_physical_epoch(E + 1), then stage K

A first:  K may be removed, then B's stage restores K before B can publish.
B first:  A's equality predicate fails, so K remains through publication.
```

If a reservation already existed when A rechecked, A receives no delete permit.  If a reservation expires before publication, publication fails and any later delete is an orphan cleanup.  These cases establish that no successful reference can name missing bytes without a distributed transaction.

## Schema-first private control model

The control plane is deliberately separate from both the append-only business ledger and rebuildable projections.  Lease acquisition/renewal is coordination churn, not a public domain event; writing it to `DirectoryEvent` would mutate stream/replay semantics and reveal operational timing.  The rows/nodes persist across a projection rebuild and survive `SpaceDeleted` so an ID is never allowed to reuse an earlier epoch.

### Directory-private schema

Add a durable coordinator identity and a per-space lease in every directory backend.

```text
ArtifactCasBarrierIdentityV1
  singleton: true
  coordinator_id: [u8; 32]              // generated once by the directory backend

ArtifactCasSpaceLeaseV1
  coordinator_id: [u8; 32]
  space_id: UTF-8, 1..=1024 bytes
  fence_epoch: u64, >= 1                // never reset or decremented
  holder_id: [u8; 16] | null            // server-generated opaque operation id
  expires_at_ms: u64 | null
  updated_at_ms: u64
```

SQLite: `hub_artifact_cas_barrier_identity` and `hub_artifact_cas_space_lease`, with `space_id PRIMARY KEY`, checked widths, and the null-pair invariant for holder/expiry.  PostgreSQL uses the equivalent `BYTEA`/`BIGINT` tables and row locking.  Neo4j uses one unique singleton `:ArtifactCasBarrierIdentity` node and a unique `:ArtifactCasSpaceLease { spaceId }` node.  Keep these objects out of `rebuild_projections_controlled`; rebuild must neither delete them nor reset `fence_epoch`.

Lease API, private to `HubDirectory`/the authority coordinator:

```text
acquire_artifact_cas_space_lease(space, holder_id, deadline, now)
  -> ArtifactCasSpaceLease { coordinator_id, space_id, fence_epoch, holder_id, expires_at_ms }
renew_artifact_cas_space_lease(lease, deadline, now) -> renewed lease
assert_artifact_cas_space_lease(lease, now) -> ()
release_artifact_cas_space_lease(lease) -> ()       // idempotent owner+epoch compare
```

Acquire atomically inserts epoch `1` or, only if the incumbent is expired, replaces the holder and increments the epoch.  The same holder may renew its epoch; all other callers receive bounded `Busy`, retry only after `OperationContext::checkpoint`, and stop at deadline/cancellation.  Generate the holder id inside the server coordinator, not from an HTTP/MCP/client request.  Use backend-authoritative milliseconds for the lease comparison (`SQLite` local database time, PostgreSQL server time, Neo4j `timestamp()`), and return the observed expiry.  Clock differences may affect liveness but cannot violate safety because of the CAS physical epoch below.

Use a fixed maximum lease of **5,000 ms**, cap renewals to the operation deadline, and never sleep uninterruptibly.  A physical action reasserts immediately before it begins.  Staging does not hold this lease for a 64 MiB pair: a durable reservation protects the pair while staging, and the physical epoch protects against a stale deleter.

### Dedicated CAS-private physical-fence schema

Every `ArtifactChunkCasStorage` implementation gains exactly one private physical-fence record per coordinator/space:

```text
ArtifactCasPhysicalFenceV1
  coordinator_id: [u8; 32]
  space_digest: SHA-256(space_id UTF-8)
  physical_epoch: u64, >= 1
  updated_at_ms: u64
```

SQL CAS stores use `hub_artifact_cas_space_fence` with `(coordinator_id, space_digest)` primary key.  Neo4j uses a unique `:ArtifactCasSpaceFence { coordinatorId, spaceDigest }` node.  The filesystem store keeps a domain-tagged, checksummed `fence-v1` metadata file below each space directory and a distinct stable `fence.lock` file.  Memory storage keeps the same map under the mutex that serializes its object operations; it is test parity only and deliberately loses all data/fences on process loss.

The directory coordinator identity is also recorded once at CAS open.  A CAS root/database/node set initialized for a different directory coordinator fails readiness rather than silently sharing a namespace.  This is essential because `OS_HUB_DIRECTORY_BACKEND` and `OS_HUB_STORAGE_BACKEND` are independent today (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2182-2220`).  Startup must connect the directory first, obtain its stable coordinator id, then open/validate CAS; the current order is the reverse (`:2276-2283`).

### Opaque in-process capabilities

Replace the partially added token fields with non-forgeable crate-owned capabilities.  Their fields are private and their `Debug` form is `<opaque>`.

```text
ArtifactCasSpaceLease        // directory epoch + opaque holder, never wire-encoded
ArtifactCasWritePermit       // reservation plus CAS physical epoch
ArtifactCasDeletePermit      // object + observed ledger generation + CAS physical epoch
```

`ArtifactCasDeletePermit` does **not** rely on a local expiry check in the storage implementation.  It is minted only after `assert_artifact_cas_space_lease` and the current reference/reservation query, then consumed by CAS `delete_if_epoch`.

## Backend realization and topology

| Topology | Required realization | Why it is safe |
| --- | --- | --- |
| Directory SQLite + CAS SQLite on the same explicitly constructed database handle | Collapse lease assert, current-reference query, physical-epoch update, and object delete into one `BEGIN IMMEDIATE` transaction. | One database serialization point; no two-phase commit. |
| Directory PostgreSQL + CAS PostgreSQL in the same explicitly constructed database/schema | Lock lease/fence rows with `FOR UPDATE`; check references/reservations and delete object in the same transaction. | Row locks make the fence/deletion conditional atomically. |
| Directory Neo4j + CAS Neo4j in the same explicitly constructed graph | Mutate the unique lease/fence node in the same Neo transaction before a conditional object delete. Retry Neo serialization/transient conflicts under the bounded context. | The graph transaction gives one commit order. |
| Filesystem CAS with SQLite, PostgreSQL, or Neo4j directory | Use the directory lease plus the filesystem `physical_epoch` protocol; no transaction spans the directory and filesystem. | `advance(E+1)` versus `delete_if_epoch(E)` is serialized in the filesystem, producing the two safe orders above. |
| Independently selected SQL/Neo directory and CAS databases | Use the same external fenced protocol, with matching coordinator ids. | No cross-database transaction is assumed. |
| Memory/test | Share one `Arc<MemoryArtifactChunkCasStorage>` and one directory backend across two independently created services; fence map and object map lock together. | Reproduces the interleaving law without claiming crash durability. |

Do not infer co-location by comparing URL strings.  Select a co-located implementation only from one factory that intentionally constructs a shared provider/transaction handle.  All other combinations, including the zero-touch default of SQLite directory plus filesystem CAS, are `ExternalFenced` and must use both layers.

### Filesystem fencing without a runtime dependency

The workspace MSRV is Rust 1.88 (`Cargo.toml:142-145`), so a newer standard-library file-lock convenience API cannot be assumed.  Implement a small in-repository `ArtifactCasFileFence` abstraction behind `cfg`:

- Unix: nonblocking advisory `fcntl` record lock on the stable `fence.lock` file.
- Windows: nonblocking `LockFileEx` on the stable `fence.lock` file.
- Retry from async code with bounded backoff, `OperationContext::checkpoint`, and no force-unlink/expiry stealing of the operating-system lock.

The lock is held only while reading/updating `fence-v1` or testing its epoch and deleting one object.  A process exit releases the OS lock; a paused process is never force-unlocked.  The successor may acquire the directory lease after expiry but waits at the physical lock.  If the old delete completes first, the successor stages after advancing its epoch; if the successor advances first, a stale delete fails.  Keep `fence.lock` stable: never replace it by rename.  Write `fence-v1` through a temp file, fsync it, atomically replace it while the lock is held, then fsync the directory.  Reject a symlinked root/space/fence path and fail closed on an unsupported lock operation.  This is zero-touch for native Windows, macOS, Linux, and the devcontainer; it does not require a lock server or external crate.

## Required operation sequences

### Reserve, stage, and publish

```text
1. validate pair and derive canonical ownership plan (one space only)
2. acquire directory space lease E (bounded/cancellable)
3. in one directory transaction: assert E, create or return exact live reservation
4. CAS advance_physical_epoch(coordinator, space, E)
5. release E; return reservation + opaque write permit only after step 4 succeeds
6. stage chunks/manifests and exact readback
7. acquire a current directory space lease F
8. in one directory transaction: assert F, validate exact unexpired reservation,
   append public event/private record/CAS publish journal, consume reservation, create reference
9. release F
```

If step 4 fails, the reservation remains live and protects any future partial writes; retrying the same canonical plan can activate a newer physical epoch.  If a stage is cancelled/fails, no publish occurs and expiry eventually permits cleanup.  If epoch `E` is already superseded at CAS activation, reacquire the directory lease and retry before deadline; never stage/publish through a stale grant.

### Retention and space deletion

Acquire the same space lease before appending `ArtifactRetentionAdvanced` or `SpaceDeleted`, then assert it in the existing atomic event/projection/release transaction.  A `SpaceDeleted` row leaves the barrier epoch intact.  Any old reservation may finish a physical write but cannot publish because the directory transition validates the deleted scope; later sweep safely reclaims it.

### Sweep

The existing opaque continuation remains locator-free and generation/mode/restart bound.  It is a scan cursor, not a locking grant.

```text
for each bounded candidate key, grouped by space in deterministic order:
  acquire space lease E (at most 16 objects per held lease)
  CAS advance_physical_epoch(E)
  directory: assert E and recheck current reference + unexpired reservation
  if eligible and execute: CAS delete_if_epoch(key, E)
  report committed result, checkpoint/yield
  renew only before E expires; otherwise stop/resume with a normal opaque cursor
  release E
```

Use a new `ARTIFACT_CAS_SWEEP_LEASE_OBJECT_MAX = 16`, independent of the existing 16-event page and 4,096-object request cap.  It bounds lease occupancy, cancellation latency, and maintenance interference.  Never retain a lease in a serialized continuation.  A cancellation before the delete commits leaves no deletion; after a conditional delete commits, report it as committed and check cancellation before the next object.

## Failure, crash, and recovery laws

| Cut point | Required result |
| --- | --- |
| Crash before reservation commit | No permission to stage; no reference. |
| Crash after reservation, before physical epoch advance | Reservation protects; successor advances an epoch before it stages. |
| Crash after epoch advance, before/during stage | No reference; reservation expires and later sweep can delete/miss objects. |
| Lease expiry while old sweeper is paused | Successor may own the directory lease, but stale delete either runs before successor's epoch advance (successor stages afterward) or fails after it. |
| Crash after physical delete, before progress | No reference existed at the checked point; retry reports `Missing` or completes later. |
| Crash after staging, before publish | Reservation protects until expiry; no public/private checkpoint is emitted. |
| Crash after directory publish commit | Reference is durable; later sweeps reject deletion regardless of stale cursor. |
| Rebuild | Rebuilds business/CAS projections only; barrier identity/epoch rows and physical fence records remain. Missing/mismatched coordinator identity fails readiness. |
| Coordinator/CAS mismatch | Fail closed at startup; never fall back to generic `PayloadStorage` or an unfenced sweeper. |

## Runtime ownership, scheduling, and readiness

Create an `ArtifactCasBarrierCoordinator` owned by `HubState`; it owns the directory control API, dedicated CAS store, per-process holder-id generator, and the publication/sweep adapters.  This is also the correct place to wire the currently unbound `CheckpointPublicationOrchestrator`/`HubVerifiedCheckpointPublisher` path identified in the post-implementation audit.

Create a cancellable `ArtifactCasMaintenanceSupervisor` after both directory and CAS coordinator-id validation succeed.  It owns no permanent lease.  Every tick takes short per-space leases as above; multiple processes are safe, though an optional private global maintenance lease may suppress duplicate work.  Default mode remains dry-run/observe.  Execute mode is an explicit server-owned maintenance setting, never a client route or a public event.  Stop it on shutdown, propagate deadlines/cancellation, and expose only aggregate counts/progress—not keys, locators, space ids, or lease metadata.

Readiness must distinguish:

```text
artifact_cas_codec_ready
artifact_cas_barrier_ready     // directory coordinator id matches every CAS store
artifact_publication_ready     // orchestrator uses reservation/write permit
artifact_sweeper_execute_ready // explicit execute mode + barrier ready
```

No service may claim the final two states from a compile-only backend path.

## Neutral fixtures, independent oracle, and race tests

Add a strict JSON fixture/schema at:

```text
🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-cas-global-barrier/🔣️.json
🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-cas-global-barrier/🧬️schema/🔣️.json
```

It must encode opaque symbolic owners/epochs rather than locators or real object digests.  The independent TypeScript oracle at `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts` should fold these interleavings and assert `referenced ⇒ physical_present` after every committed action:

1. stale delete races new reserve → physical-epoch advance → stage → publish;
2. stale delete obtains the physical action first, then new stage restores bytes;
3. expired holder resumes after successor physical advance and is rejected;
4. crash after reservation, epoch advance, delete, and publish;
5. two independently created directory services sharing one directory/CAS;
6. co-located and external topology semantics are the same;
7. 4,386-object continuation keeps no lease/identity in its public/debug representation.

Rust tests must run the same model against Memory, filesystem plus SQLite directory, and two independent directory/CAS connections where applicable.  Use process-level filesystem participants (not cloned in-process locks) to pause exactly before `delete_if_epoch`; the parent acquires a successor lease/epoch, then releases the child and proves it cannot delete a staged/published key.  PostgreSQL and Neo4j need their own two-connection live integration gates before parity is claimed.

## Exact implementation packet for Sol

Landing a token-only change is unsafe.  The smallest safe sequence is:

1. **Fail closed immediately:** keep dry-run; make execute sweep unavailable until `ArtifactCasBarrierCoordinator` is configured.  Remove/revert the incomplete token-only fence change rather than repairing constructors piecemeal.
2. Add the neutral barrier fixture/schema and TypeScript oracle first.
3. In `🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs`, add opaque lease/write/delete permits, CAS physical-fence APIs, and Memory/filesystem/SQLite/PostgreSQL/Neo4j implementations.  Do not modify generic `PayloadStorage`.
4. In `🌎️hub/📇️directory/🦀️.rs`, add the private lease APIs and move reservation/publication/retention/space-delete/sweep orchestration behind them.  Update `HubDirectory` dispatch in the same change.
5. Add the private schemas and transactional implementations in `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`, and `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`; preserve barrier rows during rebuild.
6. In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, connect directory before CAS, validate coordinator identity, construct the barrier/publication coordinator, and register the bounded maintenance supervisor/readiness facets.
7. Add the two-service/process races and cancellation/crash laws in the existing Rust tests; use `🌎️hub/📦️packages/🦀️rust/📜️script.ts` only if a permanent focused Nx target is needed.

This packet changes only dedicated directory/CAS authority code, fixtures, and tests.  It neither exposes a locator nor weakens the generic payload boundary.  It is bounded by one schema/version and one topology-independent state machine; co-located database paths are an optimization, not a second correctness model.

