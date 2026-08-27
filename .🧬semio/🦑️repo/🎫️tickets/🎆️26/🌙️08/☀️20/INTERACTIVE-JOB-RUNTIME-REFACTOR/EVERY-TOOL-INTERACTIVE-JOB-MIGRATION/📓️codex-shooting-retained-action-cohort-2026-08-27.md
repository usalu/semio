# Shooting Retained Action Cohort

## Outcome

The canonical Shooting command surface has 39 action identities. Exactly two fixed host-request reducers now have a concrete owner-local retained factory, exact bounded-first-step proofs, and exact `HostOnly` publication contracts. The other 37 routes remain intentionally fail-closed because they require artifact/config preparation authority, retained document/selection cursors, payload codecs, render/export pages, or a real interaction implementation.

This is a source-only `2 / 37` checkpoint. The official verifier recognizes both owner-local routes with no Shooting-scoped forged or publication-contract failure, but still reports all 39 Shooting rows as remaining because the central full prepare/job/commit operation is not bounded. No end-to-end runtime admission is claimed.

## Exact Owned Paths

- Runtime owner: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- Language-neutral fixture: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.json`
- Draft 2020-12 schema: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.schema.json`
- Emblem payload owner: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs`
- Validation transcript: `🧪️codex-shooting-retained-catalog-source-audit-2026-08-27.txt`
- Official verifier ledger: `📊️codex-official-tool-jobs-shooting-source-2026-08-27.json`

## Retained Bounded Routes

- `loadRequest` emits exactly one fixed `RequestFileOpen` effect for a document file. It does not read or clone the document, invoke a codec, traverse a collection, or publish a Store lane.
- `importAssetRequest` emits exactly one fixed `RequestFileOpen` effect for a GLB file. It does not clone or decode the eventual asset payload.

Both routes use `maxRawBytes=65,536`, `maxDecodedItems=64`, `maxWorkUnitsPerStep=1`, `maxOutputBytes=262,144`, and `maxStepMicros=7,500`. The factory rejects checkpoints and oversized raw wire input. `ArtifactEditor::build_tool_job` rejects command/tool mismatches, mounts `BoundedArtifactCommandWork`, and retains the exact app operation context.

## Fail-Closed Routes

The fixture carries the exact per-route disposition. The blocker groups are:

- Whole-document and codec work: `importSnapshotJson`, `setActiveExample`, `resetFixture`, `saveDownload`, `importAsset`, `exportActiveShot`, and `exportAllShots` decode, clone, encode, render, or serialize document/media state. They need fixed byte/item grants, progress, cancellation, freshness, ACK, incremental close, and a terminal-empty witness.
- Artifact publication: document mutations such as shot/asset creation, lighting/material edits, shot field edits, and `saveCamera` have no exact Shooting artifact preparation factory.
- Collection and selection work: patch, transform, saved-camera lookup, and all-shot export need persistent app-owned microcursors rather than one monolithic reducer call.
- Config publication: `setActiveAsset`, `setCamera`, `setCameraDraftLabel`, `setCenterModel`, `setActiveUtility`, `setLocale`, and `setShotSelection` have no exact Shooting config preparation factory.
- Placeholder interaction routes: `worldPointerDown` and `worldPointerMove` remain no-ops and therefore are not given a meaningless migrated classification.

## Instance-Owned Payload Closure

The mutable process-global `SHOOTING_EMBLEM_SCRATCH` thread-local map was removed. `shooting_set_emblem_from_base64` now attaches the immutable `SemioImageSnapshot` to the exact composed `ArtifactChild` owner with `with_local_owner`; reads use the child handle's typed `local_owner`. A hostile regression reconstructs the same child identity in a second snapshot without transferring the materialization and verifies that it cannot observe the first snapshot's bytes.

The remaining Shooting `OnceLock` values hold immutable language and composer registries. They are not payload/session owners.

## Schema-First and Third-Party Oracle

The language-neutral fixture enumerates all 39 identities, the `2 / 37` execution and admission split, both exact publication contracts, and every fail-closed feature boundary. Its strict schema fixes the controller, document schema, factory, counts, contract shape, and exact publication tool/lane order. AJV 2020 validates the canonical fixture and rejects hostile missing, extra, wrong-tool, and wrong-lane contract variants.

The production test module defines the repository-owned `ShootingRetainedCatalogOracle` interface. Its test-only implementation uses the existing third-party `serde_json` dependency, and compares the fixture's exact route set, bounded set, and HostOnly set with the live command surface and factory constants. No third-party type crosses the interface.

## Pending Runtime Tests

No Cargo, Nx, rustfmt, or Rust test was run because the compiler lease is held by the Store cohort. When the coordinator grants the lease, run the focused Shooting test target covering:

1. `retained_command_catalog_matches_the_serde_json_oracle`.
2. `emblem_materialization_is_owned_by_the_exact_snapshot_child`.
3. Both retained host-request jobs through progress, cancellation, replay, close, ACK, terminal emptiness, oversized wire rejection, checkpoint rejection, exact controller routing, and cross-instance isolation.
4. The official verifier after the central Store/full-operation gate is green.
