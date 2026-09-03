# Artifact Chunk-CAS and Retention Foundation

## Outcome

This packet lands the coherent manifest/storage/read foundation of P2-D. It does **not** complete P2-D: durable reservation/reference projections, retention release, `SpaceDeleted` release, `delete_if_unreferenced`, and the generation-fenced sweeper remain unimplemented. Physical CAS bytes are therefore append-only and may remain orphaned. No reachable object is deleted by this slice.

The former checkpoint `DbImmutableArtifactBlobStore` and its `db-payload/blake3/*` locator dialect have been removed after changing every production checkpoint read to the new manifest reconstruction path. Generic `PayloadStorage` and its 496 KiB policy remain unchanged and isolated from artifact CAS ownership.

## Canonical manifest

`ArtifactCasManifestV1` implements the audit's only byte representation:

- fixed 256 KiB chunks;
- space-scoped, domain-separated SHA-256 chunk identity;
- domain-separated SHA-256 manifest identity;
- exact raw SHA-256 and raw byte length;
- canonical `u64_be(length) || bytes` fields;
- exact domain/version, scope, raw identity, chunk width/count, contiguous ordinal, final-length, and trailing-byte validation;
- 64 MiB raw, 256 chunk, 64 KiB manifest, and 1,024-byte space-identity ceilings;
- strict lowercase-64-hex private locator `semio.artifact-cas.manifest/v1/<manifest-id>`.

Planning is pure and bounded. It accepts the neutral empty vector so the format itself has an unambiguous zero-byte identity; the existing artifact authority continues to require nonempty pack and SPR inputs. One-bit canonical-manifest and stored-chunk mutations fail closed.

## Dedicated storage

`ArtifactChunkCasStorage` is a hub-owned port independent of generic database payload storage. `put_if_absent` verifies the key from exact bytes before persistence and compares exact bytes after a key collision. `get` performs kind-specific length admission before returning data and recomputes the scoped identity.

Implemented namespaces:

- memory: `(space digest, object kind, object digest)` immutable map;
- filesystem: `artifact-cas/v1/<space digest>/<kind>/<digest>`, non-symlink canonical root, create-new temporary file, full write and sync, atomic hard-link installation, bounded exact readback;
- SQLite: dedicated `hub_artifact_cas_object` table, composite primary key, object-kind and byte-length constraints, transactionally collision-checked `INSERT OR IGNORE`;
- PostgreSQL: equivalent dedicated table and `ON CONFLICT DO NOTHING` transaction with `octet_length` preflight;
- Neo4j: dedicated `ArtifactCasObject` label/key constraint, `MERGE`, length preflight, and exact byte validation.

The production backend selector constructs the matching CAS beside the selected directory/database backend. PostgreSQL and Neo4j implementations received real all-feature source/bin compilation; their live services were not available because Docker could not connect to `/Users/ueli/.docker/run/docker.sock`.

`ArtifactChunkBlobStore` writes each missing chunk, reads it back exactly, writes and reads back the canonical manifest, and stores only the manifest locator in the private checkpoint projection. Reconstruction verifies every chunk, final raw length/SHA-256, and the checkpoint's expected public integrity. Cancellation/deadline checkpoints and executor yields occur before each chunk and durable transition.

## Authority and P2-C integration

`ImmutableArtifactBlobStore::{stage,read}` now receives the structural space id. The authority publication orchestrator passes the candidate's space through staging and exact readback. The old full-payload database adapter was removed rather than retained as a second private dialect.

`VerifiedRebootstrapSource` now owns `ArtifactChunkBlobStore<Arc<ArtifactChunkCasStores>>` and reconstructs pack and SPR from private manifests before the existing aggregate/checkpoint validation and public v1 bootstrap planning. The existing 4 KiB public wire chunks and public rebootstrap DTO remain unchanged. The existing JSON/schema rejection law still rejects `storageKey`, and no CAS locator is added to a public event, HTTP endpoint, or log.

Hub startup initializes the dedicated CAS for filesystem, SQLite, PostgreSQL, or Neo4j and injects it into the rebootstrap source. Test publication helpers now stage real manifests rather than inventing payload locators. The actual accepted-operation scheduler remains unavailable because the trusted production catalog/startup profile is independently incomplete; this slice does not fake that composition.

## Neutral fixture and independent oracle

The language-neutral fixture and schema cover raw lengths `0`, `1`, `262143`, `262144`, and `262145`, plus a deterministic 64 MiB pair represented by two 32 MiB entries. It records raw SHA-256, chunk count, first/last chunk identity, canonical manifest byte length, and manifest identity.

Rust consumes the fixture through the production planner/decoder. The permanent TypeScript test independently derives every byte field with Node `Buffer` and WebCrypto SHA-256, strict-validates the fixture with AJV, and agrees byte-for-byte. It does not call the Rust implementation or share its digest helper.

## TDD and validation evidence

The final pre-green Rust gate was red with `E0425` for an unqualified neutral test hash formatter and `E0596` for the newly added one-bit mutation vector. Both defects were corrected before the final run.

Permanent bounded Rust gate:

```text
RUST_MIN_STACK=16777216 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/artifact-chunk-cas-target" bun nx run os-hub:artifact-cas-check --skip-nx-cache
```

Final result:

- six CAS tests passed, zero failed, 38 filtered, runtime 3.39 seconds;
- memory, filesystem, and SQLite each round-tripped a 496 KiB + 1 artifact;
- filesystem reopened and read already persisted manifests/chunks, then rejected a one-bit stored-chunk mutation;
- canonical boundary, same-space idempotence, cross-space separation/rejection, locator strictness, cancellation-before-store, 64 MiB + 1, and 256 KiB + 1 laws passed;
- the same target completed `cargo check --all-features --bin os-hub` in 38.59 seconds, compiling PostgreSQL/Neo4j CAS and startup/rebootstrap wiring;
- warnings shown by the workspace were non-fatal and largely pre-existing; the bin also reports test-only CAS imports/state as unused in a production-only check.

Independent oracle:

```text
bun nx run os-hub-ts:test-quick --skip-nx-cache -- -t 'artifact chunk-CAS boundary'
```

Result: one test passed, nine skipped; Vitest duration 3.44 seconds and oracle execution 224 ms. The default 15-second `test` level had first exhausted its startup/transform budget without executing the assertion; the repository's existing `test-quick` level then completed green.

`git diff --check` passed for the touched tracked surfaces. A source search found no remaining `DbImmutableArtifactBlobStore`, `DB_ARTIFACT_BLOB_MAX_BYTES`, or `db-payload/blake3` use under `🌎️hub`.

## Exact residual order

P2-D remains open in this order:

1. Define schema-first expiring reservation and published-reference records keyed structurally by `(space, document, checkpoint)` plus exact manifest/chunk identities.
2. Implement atomic `reserve` and `publish_reserved` projection/journal transitions in memory, SQLite, PostgreSQL, and Neo4j; publication must consume only a live exact reservation and remain success-after-commit.
3. Add retention-floor and `SpaceDeleted` reference release while keeping public events locator-free and projection rebuild exact.
4. Add the reference-ledger oracle and `ArtifactChunkCasStorage::delete_if_unreferenced`; no backend may delete based on a caller assertion alone.
5. Implement the dry-run-default, generation-fenced, page-bounded, cancellable sweeper and generic-payload-isolation race laws.
6. Add per-storage-chunk progress beneath P2-A/P2-C stages and fault injection at every reservation/write/readback/commit boundary.
7. Run real PostgreSQL/Neo4j parity when services are available, plus real P2-C socket laws above 496 KiB and at the 64 MiB pair ceiling.

Until items 1–5 exist, there is intentionally no delete operation and no claim that P2-D retention or reclamation is complete.

## Files

- `🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🧬️schema/🔣️.json`
- `🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json`
- `🌎️hub/🛰️lag-rebootstrap/🦀️.rs`
- `🌎️hub/📇️directory/🦀️.rs` (private test locator dialect only)
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`

