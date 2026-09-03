# P2-D Artifact Chunk-CAS Post-Implementation Safety Audit

## Verdict

**REJECT — do not accept P2-D yet.**

The current tree has the requested dedicated, bounded CAS format and a substantial local test/oracle surface.  It does **not** make delete-if-unreferenced safe across more than one `DirectoryService` instance or process.  The only interval protecting the directory recheck and physical delete is a per-instance Tokio mutex; the physical storage port has no live reference/reservation check or globally held lease.  A concurrent publisher through another service can therefore stage an object after the sweep fence and before the sweep's delete, then publish a reference to missing bytes.

This was a read-only source and fixture audit.  No production/test source was changed and no build, test, or runtime command was run.  The final report, `📓️sol-artifact-chunk-cas-reference-retention.md`, appeared during this audit.  It attributes a green focused TypeScript oracle run (1 passed, 9 skipped) to its author, explicitly leaves the focused Rust rerun **pending** a shared Cargo build lock, and explicitly does not claim PostgreSQL/Neo4j runtime parity.  `📓️sol-artifact-chunk-cas-retention.md` remains pre-implementation/foundation evidence, so its historical test and compile statements do not cover this ledger/sweeper implementation.

## High-severity blocker

### H1 — the delete fence is local, not a deletion barrier

`DirectoryService::new` creates a new `write: Mutex<HubClock>` for every service instance (`🌎️hub/📇️directory/🦀️.rs:1398-1423`).  Reservation obtains that particular mutex (`:1482-1489`).  The sweeper obtains the same particular mutex, asks the directory for a fence, and awaits `storage.delete_if_unreferenced` while it holds it (`:1604-1629`).  That is sufficient only when every reserve/publish/sweep call shares the same `DirectoryService` object.

The fence itself carries only an object key and a nonzero ledger generation.  Its only storage-side predicate is `fence.permits(key)` (`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:119-137`).  Memory, filesystem, SQLite, PostgreSQL, and Neo4j deletion implementations subsequently issue a raw physical delete; none rechecks directory references/reservations or holds a shared directory transaction/lease (`:555-562`, `:640-649`, `:766-783`, `:848-858`, `:915-924`).  The deployed configuration independently selects directory and CAS backends (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2182-2206`, `:2209-2219`), so a cross-store transaction cannot be inferred either.

The failing ordering is possible without forging any token:

```text
A / service-1: recheck K -> unreferenced; receive fence(K, G)
B / service-2: reserve plan containing K -> durable reservation
B / service-2: stage K -> physical K exists
A / service-1: delete_if_unreferenced(K, fence(K, G)) -> removes K
B / service-2: atomic publish -> durable reference now names missing K
```

The existing race test intentionally uses one `Arc<DirectoryService>` for both actors (`🌎️hub/📇️directory/🦀️.rs:2959-3008`), so it proves only the local-mutex case.  It cannot exercise the counterexample.  The final generation read is also under the local mutex but merely rejects a generation moving **backward**, accepting a newer generation (`:1650-1657`); it is diagnostic after deletion, not a cross-instance fence.

Required remediation: define one durable/global artifact-CAS deletion barrier which is acquired by **reserve, publish, and sweep**.  For directory and CAS data in one database, perform reference/reservation recheck and object deletion in one serializable transaction.  For filesystem or independently configured stores, use a durable/advisory per-space (or globally ordered per-object) lease held from recheck through physical delete, and make reservation/publication acquire the same lease.  The storage delete capability must be bound to that live barrier, not merely to an object-shaped token.  Add a two-service/two-connection race for each configured directory/CAS pairing that pauses exactly between fence and delete and proves publication cannot leave a missing object.

## Requirement-by-requirement static assessment

| Requirement | Current source evidence | Assessment |
| --- | --- | --- |
| Space-scoped 256 KiB CAS | Chunk size is `256 * 1024`; chunks/manifests include the space in their identity; object keys use `space_id`, kind, digest (`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:11-33`, `:331-353`). | Meets locally. |
| Canonical manifests | Decoder checks scope, fixed chunk size, ordinals, lengths, raw digest, trailing bytes, canonical re-encoding, and manifest digest; manifests are capped at 64 KiB (`:200-328`). | Meets locally. |
| Pair budget and ownership bound | Authority validates the combined 64 MiB pair before planning; ownership is canonical and deduplicates chunks plus two manifests (maximum 258 objects, not 256) (`🌎️hub/🗿️artifact-authority/🦀️.rs:437-459`; `🗂️chunk-cas/🦀️.rs:356-465`). | Meets locally. |
| Reserve before stage | The orchestrator derives ownership, calls `publisher.reserve`, then stages pack/SPR and exact-readbacks (`🌎️hub/🗿️artifact-authority/🦀️.rs:458-489`). | Meets locally.  Failed/cancelled stage is reclaimable only after reservation expiry and a later sweep. |
| Atomic public/private publication and reference replacement | SQLite/PG/Neo append methods validate the live exact reservation and transact public event, private authority journal, projection, ledger publish record, reservation removal, and reference insertion together; SQLite is visible at `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1088-1135`. | Meets by static parity review.  No live PG/Neo confirmation. |
| Expiry, supersession, and rebuild | Reservation TTL is 1..=300,000 ms; replacement advances the write epoch; rebuild replays the immutable CAS ledger after clearing its projections (`🪶️sqlite/🦀️.rs:1022-1085`, `:1232-1305`; analogous PG/Neo implementations). | Meets locally; SQLite has static test coverage. |
| Retention and `SpaceDeleted` release | Retention/space-delete append a CAS ledger operation in the same directory transaction and remove references/private projections (`🪶️sqlite/🦀️.rs:444-491`, `:1197-1212`; PG/Neo parallel this). | Meets locally; physical reclamation remains subject to H1 and requires an invoked sweep. |
| Generic `PayloadStorage` isolation | CAS is a separate port/table/root, not the generic payload API; the retention test preserves an identical generic payload while sweeping CAS (`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:154-163`, `🌎️hub/📇️directory/🦀️.rs:2805-2860`). | Meets locally. |
| Delete-if-unreferenced fence | Immediate directory recheck exists and all stores require a nonforgeable in-crate fence. | **Fails globally: H1.** |
| Final generation recheck under writer lock | A final read occurs while holding `self.write`, and verifies monotonicity (`🌎️hub/📇️directory/🦀️.rs:1650-1657`). | Present, but insufficient for H1 because it is not a shared barrier and does not make deletion conditional on the final generation. |
| Dry-run default | `ArtifactCasSweepRequest::default()` sets `execute: false`; execution is explicit (`🌎️hub/📇️directory/🦀️.rs:318-329`). | Meets locally. |
| Bounded progress and cancellation | Request cap is 4,096 objects; pages are at most 16 ledger entries; stage/read and sweep checkpoint, report committed progress, and yield (`🌎️hub/📇️directory/🦀️.rs:291-298`, `:1555-1668`; `🗂️chunk-cas/🦀️.rs:992-1049`). | Meets locally.  A committed delete remains committed if cancellation arrives after it, which is the documented progress ordering. |
| Opaque continuation beyond 4,128 objects | The 53-byte MACed token carries mode/generation/page offset, no object identity.  Fixture drives 17 × 258 = 4,386 objects, split 4,096 + 290 (`🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json`; `🌎️hub/📇️directory/🦀️.rs:2908-2956`). | Meets locally. |
| Restart/generation/mode invalidation | Token MAC uses per-service secret; parser binds execute mode and continuation use requires equal observed generation (`🌎️hub/📇️directory/🦀️.rs:1425-1453`, `:1561-1575`). | Meets locally.  Restart invalidation is intentional; a fresh sweep is required. |
| No private locator in public schema | `ArtifactBlobRef.storage_key` remains private; `PublishedArtifactCheckpoint` omits it.  The directory test serializes public event output and asserts no `storageKey`/locator (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:423-473`; `🌎️hub/📇️directory/🦀️.rs:3045-3050`). | Meets locally. |
| Verified read path | Rebootstrap resolves public active checkpoint plus private verified record, reconstructs through `ArtifactChunkBlobStore`, and rechecks raw/aggregate identities (`🌎️hub/🛰️lag-rebootstrap/🦀️.rs:197-279`). | Meets locally.  H1 can make this correctly fail with unavailable/integrity after a bad cross-instance deletion. |
| SQLite/PostgreSQL/Neo4j semantics | All three contain the ledger/reservation/reference/release/rebuild methods and dedicated CAS stores. | Static source resemblance only; see M2. |

## Medium-severity integration and evidence gaps

### M1 — the production authority has no publication orchestrator or sweeper owner

The runtime opens and retains the CAS and `VerifiedRebootstrapSource` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2276-2299`), but its `HubState` stores only `ValidatingCanonicalArtifactAuthority` as `_artifact_authority` (`:190-203`, `:272-283`).  There is no production construction/use of `CheckpointPublicationOrchestrator` or `HubVerifiedCheckpointPublisher`; searches locate those only in the authority/directory modules and test helpers.  The binary test helper manually reserves, stages, and publishes (`:2638-2672`).  Likewise, `sweep_artifact_cas` has no production trigger/schedule in the binary.

Consequences: the authoritative runtime path is not yet proven to publish through reserve → CAS stage/readback → atomic directory seam, and retention cannot reclaim bytes unless an unshown host calls the internal service method.  Wire the orchestrator and a bounded maintenance sweeper into the owned authority/runtime lifecycle only after H1's barrier exists.  The caller must supply cancellation/deadline/progress and must never expose CAS keys on public routes.

### M2 — backend parity is source-only, not operational evidence

SQLite has direct in-memory and filesystem restart/rebuild tests in the current source (`🌎️hub/📇️directory/🦀️.rs:2805-3008`, `:3171-3210`) and CAS module tests cover memory/filesystem/SQLite (`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:1089-1237`).  PostgreSQL and Neo4j have parallel code.  The final report attributes source review plus all-feature compile evidence to those implementations but explicitly records no reachable PostgreSQL/Neo4j service and no runtime parity; its exact focused Rust rerun remains pending.  The older foundation report predates reservations/references/sweeping and must not be read as proof of them.

After H1 is fixed, run and retain evidence for SQLite, PostgreSQL, and Neo4j covering: exact collision/readback, reservation expiry/supersession, atomic public/private/reference append rollback, retention and space deletion, cancelled rebuild rollback, durable restart, 4,386-object continuation/resume, and the cross-instance deletion race.  Include both same-backend and independently selected directory/CAS configurations, because the runtime permits the latter.

### M3 — failure coverage is local and no current execution result is available

The neutral fixture and independent TypeScript oracle are good coverage, not a replacement for execution.  `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json` defines manifest vectors, retention snapshots, and the 4,386-object cursor law; `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:295-461` independently validates it with AJV/WebCrypto.  The final report attributes a green focused TypeScript command to its author; this audit did not rerun it.  Rust source adds cancellation, local sweep/reservation race, generic-payload isolation, and SQLite restart/rebuild cases, while that report leaves the focused Rust command pending.

Missing is an oracle-backed, fault-injected two-service/process test at the exact fence-to-delete boundary, plus failure injection between directory transaction completion and physical store operations.  This audit deliberately did not run either the Rust or TypeScript test suite; do not turn static assertions or report-attributed results into broader runtime claims.

## Current ordering laws and limits

The source currently establishes the following bounded local ordering:

```text
validate 64 MiB pair / identities
  -> canonical ownership plan (<=258 objects)
  -> durable reservation (TTL <=300,000 ms)
  -> chunk put-if-absent + exact readback, then manifest + exact readback
  -> exact pair readback
  -> one directory transaction: public event + private checkpoint + ledger publish + reference replacement

retention or SpaceDeleted
  -> one directory transaction: release ledger entry + relevant references/private projection removal

sweep (default dry run; 1..=4,096 objects)
  -> page <=16 ledger events, cursor MAC binds mode/generation/offset
  -> local writer mutex -> directory fence -> physical delete -> committed progress -> yield
```

The prepublication failure path intentionally leaves immutable bytes behind, with the reservation making them nondeletable until expiry; this is safe only when H1 is repaired.  A cancellation checked before a durable action aborts it; a cancellation after a completed delete/publication is observed at the next safe boundary and progress remains committed.  Rebuild replay is transaction-wrapped in each backend's source, so a cancellation/error should roll back its projection clear/replay, but that claim is source-level pending the backend executions above.

## Difference from the original P2-D audit

The original audit correctly described a missing dedicated CAS, generic-payload ceiling, no ownership ledger, and no sweeper.  The current tree now has the planned chunk/manifest codec, dedicated storage types, private ownership journal, reservation/reference projections, retention/space release, opaque cursor, rebootstrap reader, neutral fixture, and TypeScript oracle.  It also correctly uses 258 as the maximum deduplicated pair ownership set (256 chunks plus two manifests), which is more precise than a 256-object shorthand.

The remaining difference is consequential: the original required the delete fence and final recheck to close the rewrite race.  The current local mutex closes that race only inside one `DirectoryService`; it is not a durable/shared CAS barrier.  This is the acceptance blocker, not a reason to weaken the scope to a single-process assumption.

## Acceptance gate, in order

1. Fix H1 with a barrier shared by every reserve/publish/sweep actor and bind physical deletion to it.
2. Add the two-service/process fence-to-delete race and prove no successful reference can name missing CAS bytes; run it for supported directory/CAS backends.
3. Wire the real authority publication path and bounded owned sweep lifecycle (M1), retaining public locator secrecy.
4. Execute and report the focused SQLite/PG/Neo gates and the independent TypeScript oracle (M2–M3).  Attribute actual live-backend availability explicitly.
5. Re-audit the exact ordering and only then accept P2-D.
