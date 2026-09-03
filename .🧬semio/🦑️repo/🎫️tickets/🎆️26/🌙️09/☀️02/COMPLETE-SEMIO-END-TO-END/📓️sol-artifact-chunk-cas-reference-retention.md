# Artifact Chunk-CAS Reference and Retention

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Scope: P2-D reservation, publication, reference release, topology-independent deletion fencing, bounded sweeping, maintenance wiring, and rebuild parity.

## Outcome

P2-D now has one schema-first private ownership lifecycle around a dedicated 256 KiB artifact chunk CAS:

```text
prepare exact ownership
  -> directory reservation advances a durable per-space fence epoch
  -> selected CAS durably advances the matching physical epoch
  -> immutable chunks/manifests are staged and read back exactly
  -> public checkpoint + private locators/references publish atomically
  -> retention or SpaceDeleted releases private references
  -> bounded dry-run preview or epoch-fenced physical collection
```

The deletion proof no longer depends on a `DirectoryService` process mutex. The mutex still serializes one service's local commands, but correctness across independent services and processes comes from a two-layer epoch invariant:

1. The directory backend owns one durable stable coordinator identity and one monotonic fence epoch per space. Every reservation and delete-lease acquisition advances that epoch transactionally.
2. The CAS persists a physical epoch per `(coordinator, space)`. Reservation/publication orchestration advances it before any staging. Sweep advances it after acquiring the directory lease and before deletion.
3. A delete is admitted only when the CAS physical epoch still equals the fence's captured epoch. A successor reservation therefore makes every older conditional delete stale before it can publish new bytes.
4. Sweep renews and revalidates the exact directory lease, coordinator, epoch, captured generation, and reachability immediately before conditional deletion. An old lease release conflict cannot mask the decisive stale-CAS-epoch error.

Dry-run is a read-only preview: it does not acquire a lease, consume an epoch, configure a coordinator, or delete bytes. Execute fails closed if the directory coordinator cannot configure the selected CAS.

## Reservation, publication, and release authority

`ArtifactCasOwnershipPlanV1` canonically records `(space, document, checkpoint)`, both manifest identities, and a sorted unique object set. One pair owns at most 258 objects: up to 256 chunks across the combined 64 MiB pair plus two manifests.

SQLite, PostgreSQL, and Neo4j journal and project:

- an expiring reservation with exact plan, generation, epoch, token, and object membership;
- publication that consumes only the matching live reservation;
- checkpoint references and private manifest locators;
- retention release for checkpoints strictly older than the retained checkpoint; and
- `SpaceDeleted` release of reservations, references, locators, and serving projections.

Repeated reservation/publication is idempotent only for the exact plan and token. Expired tokens cannot publish, replacement reservations advance the per-space epoch, and released checkpoints cannot be reserved again.

`HubVerifiedCheckpointPublisher` owns the selected CAS. Its reservation boundary configures the durable coordinator and advances the returned physical epoch before returning a reservation to the staging orchestrator. `CheckpointPublicationOrchestrator` then stages both blobs, performs exact immutable readback, and calls `publish_reserved` only after the complete pair agrees with its public identities.

Public directory events remain locator-free. Manifest locators occur only in private authority projections. Generic BLAKE3 `PayloadStorage` is a separate namespace and is neither queried nor deleted by this sweeper.

## CAS implementations and filesystem boundary

Memory, filesystem, SQLite, PostgreSQL, and Neo4j implement the same space-scoped `(space digest, kind, SHA-256 digest)` contract. Writes recompute keys, accept only exact idempotence, and reject immutable collisions. Reads revalidate size, scope, digest, canonical manifest encoding, reconstructed raw length/SHA-256, and public pair identity.

All implementations persist or share the coordinator and physical epoch in the same synchronization boundary as conditional deletion. SQLite and PostgreSQL use database transactions; Neo4j uses transactionally locked nodes; memory uses one shared state mutex. Filesystem CAS uses stable coordinator and per-space lock files plus checksummed metadata.

Filesystem fence publication is crash-oriented: a unique temporary is written and file-synced, atomically replaces the destination, and the Unix parent directory is synced; Windows replacement requests replace-existing plus write-through. Leaf acquisition is atomic against symlink/reparse traversal:

- Unix calls owned `open(2)` with `O_NOFOLLOW | O_CLOEXEC`, validates the opened descriptor against `lstat` device/inode identity, and uses `flock` on the stable descriptor. A focused `fcntl(F_GETFD)` assertion proves close-on-exec.
- Windows opens with `FILE_FLAG_OPEN_REPARSE_POINT`, rejects `FILE_ATTRIBUTE_REPARSE_POINT`, and uses `LockFileEx`; Rust-created handles are non-inheritable by default.

Symlinked space directories, fence metadata, and lock leaves are rejected. The real filesystem process law uses separate test processes rather than a process-local mutex.

## Bounded sweeping and continuation

Sweep request/result carries a server-instance-bound opaque continuation. It authenticates the captured ledger generation, execute/dry-run mode, page generation, and intra-page offset with a private service secret. `Debug` renders `<opaque>` and the token contains no locator, space, document, checkpoint, object digest, coordinator, or lease identity.

A continuation is rejected after a ledger-generation change, service restart, mode substitution, malformed authentication, or invalid position. No implicit process-global cursor exists. The fixed captured generation is preserved across every page in a call.

The neutral/runtime convergence law contains 17 maximum ownership plans and 4,386 unique objects. The first request examines exactly 4,096, returns the intra-page continuation, and the second examines exactly 290 without revisit or starvation. The same law rejects `max + 1`, commits one monotonic progress item before cancellation, resumes exactly, and rejects generation/restart/mode-invalid cursors.

Production maintenance owns its continuation across scheduler ticks and keeps it after transient store failures. Only explicit continuation/generation invalidation restarts the scan. Each tick is bounded to 16 requests and a 30-second operation deadline, checking cancellation between requests. The supervisor retains its join handle; shutdown wakes it, awaits bounded completion, then aborts only after the deadline.

## Production wiring and readiness

Hub startup opens the directory first, creates `DirectoryService`, opens the chosen CAS, reads the durable directory coordinator, configures the CAS, creates the publication orchestrator, and mounts the maintenance supervisor in `HubState`.

Maintenance is dry-run unless `OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE` is exactly `true`/`1`; `false`/`0` or absence is dry-run and any other value fails startup. `/readyz` reports the real loopback/network bind classification, CAS barrier/publication/sweeper capability and mode, and dynamically becomes not-ready after a maintenance failure. A later successful page restores maintenance health.

## Backend and rebuild review

SQLite and PostgreSQL store a stable singleton coordinator plus `fence_epoch`, nullable lease token, and expiry per space. Their reservation and lease paths lock/update the per-space row transactionally. PostgreSQL additionally uses the existing transaction advisory lock. Neo4j persists the coordinator singleton and `ArtifactCasSpaceBarrier.fenceEpoch`; reservation/lease Cypher locks the barrier node through a write and advances its epoch in the same transaction.

The dedicated SQLite/PostgreSQL CAS tables and Neo4j CAS nodes persist coordinator/space physical epochs and condition deletion on exact equality. Filesystem persists equivalent checksummed coordinator and space-fence records.

Rebuild retains append-only directory/private-CAS journals, clears rebuildable projections, restores private checkpoint locators, then folds reservation/publication/release entries in generation order. Stable coordinator and monotonic space barriers are not reset by a projection rebuild. The SQLite-directory/filesystem-CAS restart law verifies exact private authority and bytes before and after rebuild.

PostgreSQL and Neo4j services were not provisioned on this host. Their P2-D result is source review plus the final all-feature Rust compile, not runtime parity.

## Neutral schema and independent oracle

The strict Draft 2020-12 schema/fixture covers raw boundaries `0`, `1`, `262143`, `262144`, and `262145`, the 64 MiB pair ceiling, canonical manifests, reservation/reference/retention/expiry/`SpaceDeleted`, 4,386-object continuation convergence, and both barrier orders with epochs `1 -> 2 -> 3`.

The independent TypeScript/Node oracle uses AJV for the neutral schema, Node `Buffer` for exact big-endian encodings, WebCrypto/Node SHA-256 for chunk/manifest and opaque-token checks, and a separate state-machine fold for both physical orders. It proves the successor-first order rejects the old delete and preserves an exact published read.

Final command:

```text
bun nx run os-hub-ts:test-quick --skip-nx-cache -- -t 'artifact chunk-CAS boundary'
```

Result: **green**, exit `0`; 1 passed, 9 skipped; focused test 147 ms, Vitest 1.37 s. The only diagnostic was the existing `NO_COLOR`/`FORCE_COLOR` warning.

Terminal evidence: session `6425`. No persistent command log was retained; the exact terminal output was reported before ticket-generated cleanup.

## Focused Rust evidence

The final Nx target was run with `CARGO_TARGET_DIR` pointed at this ticket's temporary generated directory to avoid unrelated shared-target lock contention:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/artifact-cas-continuation-target \
  bun nx run os-hub:artifact-cas-check --skip-nx-cache
```

Result: **green**, exit `0`:

- 16 focused library laws passed, 38 filtered, 0 failed, in 7.92 s;
- maintenance continuation law passed, 32 filtered, 0 failed, in 0.19 s;
- `cargo check --all-features --bin os-hub` completed in 8.10 s; and
- Nx reported target success.

Terminal evidence: session `52100`.

The 16 laws cover canonical chunk/manifest boundaries, neutral vectors, ownership codec, memory/filesystem/SQLite roundtrips, immutable collision/corruption, restart/rebuild, retention, `SpaceDeleted`, generic-payload isolation, dry-run non-mutation, expiry/supersession, cancellation/progress, `max + 1`, 4,386-object convergence, lease cleanup after CAS-advance failure, two independent `DirectoryService` instances, raw filesystem epoch processes, and the full SQLite-directory/filesystem-CAS process race.

The decisive process race also passed alone: parent, old-sweep child, and successor-publication child each reported one pass. The old child pauses after final directory validation at conditional CAS deletion; the successor child independently reserves after expiry, advances the CAS epoch, stages, publishes, and reads exact bytes. The old child accepts only the exact `artifact CAS deletion fence is stale` error. The parent reopens both stores, verifies the published private checkpoint, and reads exact pack and SPR bytes. Focused result: exit `0`, 0.38 s.

Exact focused command:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/artifact-cas-continuation-target \
  RUST_MIN_STACK=16777216 \
  cargo test --all-features --lib \
  artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes \
  -- --nocapture
```

Terminal evidence: session `87775`. The isolated Cargo target and failed-test temporary roots were deleted after all final-source gates completed; no generated command log remains in the ticket.

## Residuals and claim boundary

- PostgreSQL and Neo4j runtime parity remains unclaimed until real services are provisioned; all-feature compile-only coverage is explicit.
- The Windows no-reparse branch was source-reviewed but was not compiled or run on this macOS host. Unix no-follow, close-on-exec, crash-publication, and multi-process behavior ran on macOS.
- This packet does not claim the separate socket/bootstrap transport tests above 496 KiB and at 64 MiB; it verifies the private CAS, directory authority, restart, and maintenance path beneath the existing public P2-C wire.
- No ticket plan, acceptance matrix, metadata, or other ticket report was modified by this finalization.

## Exact source surfaces

- `🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🧬️schema/🔣️.json`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-artifact-chunk-cas-reference-retention.md`
