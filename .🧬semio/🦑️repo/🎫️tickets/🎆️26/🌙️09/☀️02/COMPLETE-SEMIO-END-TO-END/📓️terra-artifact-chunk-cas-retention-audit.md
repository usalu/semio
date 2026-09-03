# P2-D Artifact Chunk-CAS and Retention Audit

## Decision

Adopt a **bounded, space-scoped chunk-manifest CAS**.  Do not raise the generic database payload ceiling to 64 MiB.

The authority accepts a canonical `(pack, spr)` pair totalling at most 64 MiB (`🌎️hub/🗿️artifact-authority/🦀️.rs:11-16`), while `DbImmutableArtifactBlobStore` rejects either blob above 496 KiB (`🔌️adapters/🦀️.rs:20-23,214-221`).  The generic `PayloadStorage` contract independently imposes that 496 KiB ceiling on one complete `put` or `get` for FS, SQLite, PostgreSQL, and Neo4j (`🧰️framework/🛢️db/🗄️storage/🦀️.rs:63-82,4515-4537`; SQLite `:63,699-725`; PostgreSQL `:104-107,379-413`; Neo4j `:66-69,590-630`).

Raising that ceiling coherently would enlarge WAL, snapshot, index, catalog, payload, driver-buffer, I/O-credit, and HTTP paths together.  It would also invalidate the deliberate pre-allocation protection built around `MAX_READ_BYTES`.  A narrow artifact-CAS namespace retains the existing page/driver limits, provides explicit ownership for reclamation, and has no runtime dependency outside the repository.

No build or test was run for this read-only audit.

## Present path and exact gaps

| Layer | Current behavior | P2-D consequence |
| --- | --- | --- |
| Authority P2-A1 | Candidate budget is 16,384 operations / 64 MiB operation bytes / 64 MiB pair bytes; it derives raw SHA-256 for pack/SPR and a concatenated aggregate hash. | The 64 MiB pair is valid at authority level but cannot cross the current one-blob adapter. |
| P2-A2 orchestrator | Stages pack, stages SPR, reads each back, then calls the one publisher (`🌎️hub/🗿️artifact-authority/🦀️.rs:415-464`). | Failure/cancellation after either `stage`, or publisher failure, leaks an immutable payload.  The port exposes only `stage/read`, with no reservation, reference, or release (`:249-263`). |
| Current adapter | Uses generic BLAKE3 `PayloadStorage`; private locator is `db-payload/blake3/<hash>` (`🔌️adapters/🦀️.rs:172-276`). | A public SHA-256 integrity identity is mapped to a separate BLAKE3 storage identity.  It hard-fails above 496 KiB and cannot safely call generic `delete`. |
| Generic payload substrate | Global, cross-document BLAKE3 CAS; `delete` is intended for DB compaction’s own ref tracing (`🧰️framework/🛢️db/🗄️storage/🦀️.rs:4516-4536`). | P2-D must not delete an unowned generic payload: an identical DB/WAL payload may share its BLAKE3 key.  A dedicated namespace is mandatory for safe collection. |
| Directory P2-B | Public event removes `storage_key`; private checkpoint projection retains it and bounds it to 4,096 bytes (`🌎️hub/📇️directory/🦀️.rs:795-808,874-892,1231-1263`).  Publication is serialized and atomic within the directory backend (`:1313-1327`). | Manifest locators remain backend-private.  The directory is the durable authority for references, but current checkpoint records do not contain CAS chunk ownership. |
| Retention / deletion | P2-B validates forward-only retention against checkpoint lineage (`🌎️hub/📇️directory/🦀️.rs:1189-1227`).  On `SpaceDeleted`, SQLite/PostgreSQL remove checkpoint private/public/retention projections (`🪶️sqlite/🦀️.rs:403-407`; `🐘️postgres/🦀️.rs:423-427`). | It removes metadata but never releases blob payloads.  There is no orphan sweep, reference accounting, or checkpoint-private locator release policy. |
| P2-C reader | `VerifiedRebootstrapSource` resolves active public+private checkpoint, then reads each full blob through `DbImmutableArtifactBlobStore`, verifies SHA-256/length/aggregate, and emits existing v1 bootstrap frames (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:187-280`). | Replace only its private storage reader with manifest reconstruction.  Preserve public wire v1. |
| P2-C bounds | 15-second read deadline; client transport uses 4 KiB chunks, 64 MiB total, at most 16,384 chunks (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:11-59,226-279`). | CAS chunks are storage chunks, not wire chunks.  The reader reconstructs/verifies before the existing 4 KiB transfer planner. |
| Legacy REST blob route | `/spaces/{space}/blobs/{hash}` calls generic payload `put/get/contains`; its HTTP body allowance is 1 MiB (`16 KiB × 64`) (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:552-616`). | It is not checkpoint authority storage and must never expose or delete P2-D CAS objects. |

The immediate failure is deterministic: a valid 496 KiB+1 pack or SPR reaches `DbImmutableArtifactBlobStore::stage` and returns `BlobIntegrity("stage input")`, even though the authority admits the pair.  This is a high-severity correctness failure, not a tuning issue.

## Canonical P2-D format

Use fixed **256 KiB** storage chunks.  It is below every current 496 KiB backend limit, aligns to 16 DB I/O pages, and caps a 64 MiB combined pair at 256 chunks.  At most four chunks may be resident/in-flight per store read/write operation (1 MiB); the already-authorized pair remains bounded at 64 MiB.

`ArtifactBlobRef.sha256` and `byte_length` remain the SHA-256 and length of the restored raw pack or SPR.  They retain their P2-A2/P2-B meaning.  Only the private `storage_key` changes to a manifest locator.

```text
CHUNK_DOMAIN_V1    = "semio.hub.artifact-cas.chunk.v1\\0"
MANIFEST_DOMAIN_V1 = "semio.hub.artifact-cas.manifest.v1\\0"

ChunkId = SHA-256(
  CHUNK_DOMAIN_V1 || field(space_id UTF-8) || field(chunk_length u64_be) || field(chunk_bytes)
)

ManifestV1 =
  MANIFEST_DOMAIN_V1 ||
  field(space_id UTF-8) ||
  field(raw_sha256[32]) || field(raw_byte_length u64_be) ||
  field(chunk_bytes u32_be = 262144) || field(chunk_count u32_be) ||
  for ordinal in 0..chunk_count:
    field(ordinal u32_be) || field(chunk_length u32_be) || field(chunk_id[32])

ManifestId = SHA-256(ManifestV1)
storage_key = "semio.artifact-cas.manifest/v1/" + lowercase-hex(ManifestId)
```

`field` is the already-used canonical `u64_be(length) || bytes` form from checkpoint identity encoding (`🌎️hub/🗿️artifact-authority/🦀️.rs:379-401`).  The decoder requires the exact domain/version, matching scope space, raw hash/length, exact `256 KiB` chunk size, contiguous zero-based unique ordinals, nonzero chunk lengths, full chunks except the final one, and `chunk_count == ceil(raw_byte_length / 262144)`.  The 64 MiB ceiling gives at most 256 records/manifest; a 64 KiB manifest limit therefore has substantial fixed headroom.  There is no optional ordering, JSON canonicalization, locator hash, or client-provided digest.

The space is part of every object identity.  This deliberately permits deduplication **between documents in the same space**, where authorization is already a shared tenant boundary, but prohibits cross-space physical deduplication and its timing/retention side channel.  Neither chunk IDs nor manifest locators leave the private checkpoint projection; public checkpoint events retain only raw SHA-256 and lengths.

## Required storage and publication model

### New owned port

Add an `ArtifactChunkCasStorage` port rather than extending `PayloadStorage`:

```text
prepare(scope, raw SHA-256/bytes) -> ArtifactManifestPlan     // pure, bounded
put_if_absent(object key, <=256 KiB bytes, reservation)      // idempotent
get(object key, reservation/control) -> <=256 KiB bytes
delete_if_unreferenced(object key, sweep fence) -> outcome
```

It owns a separate `artifact-cas/v1` namespace in FS and separate tables/nodes in SQLite, PostgreSQL, and Neo4j; `DbBackend` gains a concrete dispatch facet parallel to `PayloadRef`.  Use a `(space_digest, object_kind, object_digest)` unique key and store verified length with bytes.  The `put_if_absent` transaction/rename must compare length and bytes on a key collision and reject mismatch as corruption.  This preserves the present generic payload CAS for DB engine/compaction and makes P2-D deletion ownership unambiguous.

Backend parity is required:

- FS: `artifact-cas/v1/<space-digest>/<kind>/<digest>` temp-write, fsync/atomic rename, then exact bounded read.
- SQLite: dedicated object table with composite primary key and BLOB length constraint; `INSERT OR IGNORE`, followed by exact read validation.
- PostgreSQL: equivalent table/key and `INSERT … ON CONFLICT DO NOTHING`, bounded `octet_length` preflight.
- Neo4j: a dedicated labelled object node with the same composite uniqueness and `MERGE`; byte/property length is checked before driver allocation.
- Memory/fault: same key and collision semantics for neutral/failure tests.

### Reservation, publication, and retention

The directory remains the durable reachability ledger because it already atomically commits private locator plus public checkpoint event.  Add backend-private `artifact_cas_reservation` and `artifact_cas_reference` projections, both scoped structurally by `(space_id, document_id, checkpoint_id)` and object identity.  They are not public directory events and never include a raw locator in the public event log.

1. Authority materializes and validates the pair, then creates the two pure manifest plans.  Under the per-scope publication lease, `HubVerifiedCheckpointPublisher.reserve` atomically records a system-only reservation containing the checkpoint identity, both manifest IDs, every chunk ID, expiry, and directory write epoch.  Reservation expiry is bounded by the operation deadline plus a fixed grace; a non-expired reservation is a GC root.
2. The CAS writes missing chunks (at most four in flight), writes each canonical manifest, and reads every stored chunk/manifest back through its bounded port.  It reassembles raw bytes while hashing and verifies raw SHA-256/length before the existing P2-A2 pair/aggregate checks.
3. `publish_reserved` atomically validates the still-live reservation, writes private checkpoint locator records, emits the existing public `ArtifactCheckpointPublished`, inserts the two manifest references, removes the reservation, and advances the active checkpoint.  Repeated same-checkpoint calls are idempotent only if the plan and private locators match exactly.
4. P2-B advances the DB journal/WAL retention fence only after that publication succeeds.  `ArtifactRetentionAdvanced` then atomically releases references only for checkpoints strictly older than the new retained floor; it must retain both the active checkpoint and the retained-floor checkpoint if they differ.  Public lineage metadata may remain for audit without retaining private locators or CAS references.
5. `SpaceDeleted` atomically removes private checkpoint records, reservations, and all CAS references in every directory backend.  It revokes serving first; physical bytes are reclaimed only by the later sweep.

The physical CAS and independently selected directory backend cannot share a distributed transaction.  Thus the safe failure rule is **orphan data is permitted, lost referenced data is not**.  A publisher failure, cancellation, or process loss before step 3 leaves an expiring reservation and later-sweepable objects.  A crash after the directory transaction returns committed success; the caller must not convert it to cancellation.  The current P2-A2 five-stage progress has to become plan/chunk/manifest/readback/publication progress with monotonic total units, and cancellation/deadline checks occur before every chunk and every durable transition.

### Sweep without a retention race

The sweeper reads a single directory-ledger generation, marks all published private references plus all unexpired reservations, then candidates only CAS objects outside that set and older than the fixed orphan grace.  Immediately before deletion it rereads the object’s ledger status at or after that generation:

- a newly created reservation protects its planned objects before the first CAS write;
- publication is allowed only by atomically consuming that live reservation, so an expired/unreserved object cannot become referenced after the sweep’s final check;
- an expired reservation cannot be renewed or published; its operator receives an explicit expired/cancelled result and retries from materialization;
- failed delete, missing object, or corrupt object is recorded per object and does not alter a durable checkpoint.  A referenced corrupt/missing object makes P2-C return integrity/unavailable and requires re-materialization; it never falls back to a client snapshot.

Run sweep as bounded, cancellable system work: a maximum 256 objects per pair-plan, a fixed page of ledger objects per iteration, byte/object progress, lease renewal, dry-run default, and an audit event/result that reports only scoped counts/digests—not bytes or locators.  It must not call generic `PayloadStorage::delete`.

## P2-C and privacy rules

`VerifiedRebootstrapSource` must parse the private manifest locator, load/validate the manifest, read chunks in ordinal order under its existing 15-second control, incrementally reconstruct/hash the raw pack and SPR, then leave `ArtifactBootstrap`, its 4 KiB outbound chunking, descriptor/frontier checks, and atomic client installer unchanged (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:197-279`).  Add per-storage-chunk progress beneath `PackRead`/`SprRead`; keep final transfer `Chunk` progress separate.

CAS lookup is never an HTTP endpoint and is never keyed by a public SHA-256 URL.  It requires a private checkpoint lookup after document/space authorization.  A share/public spectator may receive only a current authorized bootstrap through existing policy, never a manifest/chunk probe.  Same-space dedup does not grant cross-document read authority; a private locator must be attached to an authorized checkpoint in that exact structural scope.

## Ordered implementation packet

1. Define `ArtifactCasManifestV1`, canonical byte fixture, digest helpers, limits, errors, and pure planner in the hub artifact-authority module.  Add neutral fixture vectors for 0/1/full/final chunk boundaries; pair input remains nonempty under existing authority rules.
2. Add the dedicated `ArtifactChunkCasStorage` facet and all five backend implementations.  Preserve generic `PayloadStorage` and its 496 KiB policy unchanged.  Add backend schema/setup and bounded driver/page admission before bytes are allocated.
3. Extend P2-B’s private checkpoint projection with reservation/reference state and add system-only reserve/publish/release/sweep query ports.  Fold SpaceDeleted and projection rebuilds through that state in SQLite, PostgreSQL, and Neo4j; never serialize private manifest IDs into `DirectoryEventBody`.
4. Refactor P2-A2 into `prepare → reserve → put/readback chunks+manifests → publish_reserved`.  Replace `DbImmutableArtifactBlobStore` for checkpoint authority; keep its old adapter only until all P2-C authority reads have changed, then remove it rather than retaining a second locator dialect.
5. Update P2-B journal retention orchestration to publish first, fence DB retention second, advance/release directory references third.  Implement the bounded sweeper last, because publication correctness must not depend on collection.
6. Replace P2-C’s `staged(&ArtifactBlobRef)`/full-payload reader with manifest reconstruction and add storage-chunk cancellation/progress.  Preserve wire v1 and prove descriptor/frontier/aggregate failures remain fail-closed.
7. Register narrow `catalog`/CAS check, backend parity, and real rebootstrap commands in the existing launch ordering only after the Nx targets invoke their directory `📜️script.ts` entries.

## Neutral and independent oracle tests

- A language-neutral manifest JSON/hex fixture is decoded independently by Rust, TypeScript/WebCrypto, and a small no-helper oracle.  It must agree on chunk IDs, manifest ID, raw SHA-256, boundaries at 1, 262143, 262144, 262145, and a 64 MiB pair.
- Run the same fixture through SQLite, PostgreSQL, Neo4j, FS, and memory.  Assert byte-for-byte reconstruction, dedup within one space, distinct identities across two spaces, and no backend accepts a 256 KiB+1 object.
- Fault-inject after reservation, after any chunk write, after manifest write, after each readback, before directory commit, and immediately after directory commit.  Assert either no public checkpoint/references or one fully readable checkpoint; every precommit artifact is eventually sweepable.
- Race an active sweeper against reservation, publication, cancellation, retention advance, restart, and space deletion.  The independent ledger oracle must never observe a referenced object deleted, may observe an orphan retained until grace, and must converge to zero unreachable objects.
- Seed a generic `PayloadStorage` object with the same BLAKE3 bytes as a CAS chunk, release the CAS reference, run sweep, and prove the generic payload remains.  This is the regression that rules out reusing generic `delete`.
- Exercise real P2-C socket/bootstrap transport at a pair just above 496 KiB and at the 64 MiB limit, verifying the existing 4 KiB wire chunk count/aggregate/descriptor tail and cancellation before completion.

## Focused post-implementation commands

Commands below are proposed, not run by this audit.

```sh
bun nx run os-hub:test -- artifact_authority::tests -- --nocapture
bun nx run os-hub:test -- lag_rebootstrap::tests -- --nocapture
bun nx run '@semio-tech/framework-os-kernel:test' -- --package semio-framework-os-kernel-db --features sqlite 'db_storage_sqlite::sqlite_storage::tests::payload_roundtrip_obeys_neutral_page_boundaries_and_arbitrary_bytes' -- --exact --nocapture
```

Run the new CAS parity target independently for FS/memory, SQLite, PostgreSQL, and Neo4j.  PostgreSQL/Neo4j require their already-configured local backend URLs; do not claim their parity without live backend evidence.  Finish with the existing hub all-feature target and a real socket rebootstrap test, not the legacy `/spaces/{space}/blobs/{hash}` route.

## Exit criteria

P2-D is complete only when a 64 MiB authority pair is stored as bounded, independently verified CAS objects; every published private locator resolves through a canonical manifest; retention/delete and failure paths leave no reachable object unaccounted for; sweep cannot race a live reservation/publication; same-space-only dedup preserves scope authorization; and all storage backends plus the unchanged public bootstrap protocol pass the same neutral fixture.  The current 496 KiB `DbImmutableArtifactBlobStore` and unreferenced generic payloads do not satisfy these conditions.
