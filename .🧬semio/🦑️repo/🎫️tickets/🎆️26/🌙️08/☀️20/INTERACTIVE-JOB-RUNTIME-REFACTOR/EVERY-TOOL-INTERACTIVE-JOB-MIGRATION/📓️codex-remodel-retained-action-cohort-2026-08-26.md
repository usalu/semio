# Remodel Retained Action Cohort

## Outcome

This source-only checkpoint retains exactly two honestly bounded Remodel host-request routes and keeps the remaining thirty-nine routes fail-closed. It does not claim config, document, or global work is bounded, and it does not certify the reconstruction implementation while it retains process-global session state.

The production taxonomy owner is `✏️s/🔌️plugins/📸️remodel/🗿️artifacts`. An accidental lookalike `🗀️artifacts` path was created during an escaped-path edit, contained only the two new fixture files, and was removed completely before further work. No parallel taxonomy root remains.

## Exact Owned Paths

- Production owner: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- Language-neutral schema: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.schema.json`
- Language-neutral fixture: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🎯️retained-command-limits.json`
- Reconstruction blocker: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs`
- Validation transcript: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/🧪️codex-remodel-retained-catalog-source-audit-2026-08-26.txt`

## Retained Bounded Routes

The bounded cohort is limited to two O(1) host-request emissions:

1. `importFrames`
2. `importVideo`

The source registers one concrete `RemodelCommandJobFactory` for those exact tool keys. Its wire admission rejects payloads above 65,536 bytes and rejects checkpoints. `ArtifactEditor::build_tool_job` rejects tool/command mismatches, admits a declared extent of one work item, mounts `BoundedArtifactCommandWork`, and carries the app operation context into the retained payload. The owner-local proof macro, manifest `Migrated` annotations, and exact `HostOnly` publication contracts name the same two routes in the same order.

## Fail-Closed Resumable Cohort

The fixture classifies thirty-nine routes as resumable, but production deliberately has no `Migrated` annotation or registered factory key for them yet:

- Reconstruction and lifecycle: `runReconstruction`, `retryStage`, `runStage`, `advanceReconstruction`, `cancelReconstruction`.
- Payload and output work: `importFramePayload`, `importVideoFramePayload`, `importVideoBytesPayload`, `exportQcReport`.
- Document ingestion: `importVideoDone`, `addStream`, `removeStream`, `setStreamSync`.
- Calibration and control points: `editCalibration`, `calibrateCameras`, `addGcp`, `removeGcp`, `placeGcpObservation`.
- Document parameters: `setIngestParams`, `setFeatureParams`, `setMatchParams`, `setSfmParams`, `setDenseParams`, `setMeshParams`, `setMotionParams`, `setGeoParams`.
- Document result work: `resetPlaceholderMesh`, `clearSparse`, `clearDense`, `clearMeshResult`, `clearTracks`, `clearGeoProducts`, `clearResult`.
- Config publication: `setCamera`, `setLayerVisibility`, `setFrameCursor`, `setReportTable`, `setActiveUtility`, `setLocale`. Their reducer bodies are small, but no exact retained config preparation factory is installed, so they are not publishable end to end.

These routes require operation-owned resumable work. Collection lookups must advance through bounded microcursors, payload decode and report output must be page-owned, and reconstruction lifecycle state must not be stored in a process-global registry.

## Blocking Evidence

`run-reconstruction/🦀️component.rs` still declares `RECONSTRUCTION_SESSIONS: OnceLock<Mutex<ReconstructionSessions>>`. The artifact root also retains `REMODEL_PRIVATE_ASSET_STAGING` and `REMODEL_PRIVATE_MESH_STAGING` as process-global mutable child-content maps. These violate operation/instance-owned payload requirements. The existing reconstruction steps may be internally incremental, but they cannot be honestly registered as retained jobs until their state moves behind the app-instance operation owner and cancellation/close/replay are verified there.

## Schema-First Oracle

The JSON Schema is Draft 2020-12 and fixes the controller, document schema, bounded factory, exact publication-contract identities/lanes, contract shapes, route count, and expected `41 / 2 / 39` split. The fixture lists every route with its execution disposition and intended feature boundary. Hostile missing, extra, wrong-tool, and wrong-lane publication-contract fixtures are rejected.

The production test module defines the owned `RemodelRetainedCatalogOracle` interface. Its test-only `SerdeJsonRemodelRetainedCatalogOracle` implementation parses the language-neutral fixture using the existing third-party `serde_json` dependency and returns only repository-owned summary values. The test compares that oracle result with the live command-id surface and bounded factory keys; no third-party type crosses the interface.

## Source-Only Validation

- AJV 2020 validated the fixture against the schema: `schemaValid=true`.
- Census: `routeCount=41`, `bounded=2`, `resumable=39`, `unique=true`, `commandCount=41`.
- Exact parity: fixture bounded IDs equal the production bounded constant, manifest annotations, and owner-local proofs.
- Fail-closed check: no fixture-resumable route is annotated `Migrated`.
- Bounded command-body scan: no iteration, collection rewrite, serialization, or process-global session pattern occurs in the two retained command owners.
- Official-parser scoped reproduction: the concrete factory now uses the exact qualified `semio_framework::ToolJobFactory` implementation and `semio_framework_plugin::EditorApp<RemodelPlayApp>` owner spelling required by the verifier; exact registration, builder, job type, and execution contract checks are green.
- Host-request audit: `importFrames` and `importVideo` enqueue one fixed request each, perform no payload clone/decode/serialization or loop, and leave the video request payload `None`; the latter reads only three fixed ingest scalars.
- Canonical-path check: no `🗀️artifacts` directory remains under Remodel.
- Scoped `git diff --check` returned exit code 0. The shared files already had staged/modified state; this work did not alter the index or run a modifying Git command.

## Pending Runtime and Compiler Validation

No compiler, Cargo, Nx, rustfmt, or runtime test was run because the Store cohort holds the exclusive compiler lease. Therefore this report does not claim compilation or runtime behavior.

The latest cross-cohort official report, `📊️codex-official-tool-jobs-shooting-source-2026-08-27.json`, recognizes the exact Remodel factory and the two owner-local routes with zero Remodel-scoped forged/publication failures. All 41 Remodel rows remain centrally fail-closed while the shared Store `fullToolOperationBounded` prerequisite is false; that shared prerequisite is outside this cohort.

When the Remodel compiler lease is granted, run through the repository task surface:

1. `bun nx run @semio-tech/remodel-plugin:test-quick`
2. A focused retained catalog/oracle test for `retained_command_catalog_matches_the_serde_json_oracle` through the Remodel test target if the task router supports test filtering.
3. The official repository `verify interactivity tool-jobs` source gate after all concurrently owned cohorts settle.
4. Runtime retained-job tests for progress, cancellation, close, replay, oversized wire rejection, checkpoint rejection, exact controller routing, and cross-app-instance isolation for the two retained routes.
5. After the thirty-nine resumable implementations exist, runtime tests proving one-unit progress, bounded checkpoint/output pages, prompt cancellation, replay equivalence, and absence of process-global payload/session ownership.
