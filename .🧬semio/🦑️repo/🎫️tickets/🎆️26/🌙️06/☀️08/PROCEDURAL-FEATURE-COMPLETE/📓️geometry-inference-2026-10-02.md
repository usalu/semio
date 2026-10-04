# Geometry Inference Integration — 2026-10-02

Geometry computation is moved behind the existing named artifact inference service boundary. The Brep extension keeps its explicit SessionCapture resource owner, and FlowEvalSession and PreviewEvalRun retain orchestration and publication. The existing SDK retained operator jobs and existing node cache remain the only evaluation/cache implementation.

Owned files: the plugin ArtifactInferenceService context variant and its registry tests; Brep geometry payload schemas and Generation3d preview invocation assembly; Brep owned inference worker, registration and tests; existing flow SDK retained job input identity if needed. No inspector/viewport/scene-lane files.

Validation pending.

## Current implementation

- `ArtifactInferenceService` adds a contextual executable variant. Pure executables keep their existing calling behavior. Missing context is an explicit refusal, and contextual identities remain conflict-checked in the existing registry.
- `ExtensionResourceOwner` optionally supplies its retained inference context. The standard wire gateway resolves this only for contextual services and preserves existing metadata/resource/cancellation/result/provenance validation. `ExtensionBundle::artifact_infer` provides the equivalent explicit owner route.
- Brep registers `s.flow-extension-brep.geometry` as a real `ArtifactContribution` targeting its declared dependency `s.flow.flow`. The named provider calls the existing SDK evaluator with its captured SessionCapture; the owned extension evaluate handler resolves this service before executing geometry.
- Both generic Flow and Generation3d preview invocations provide the same SDK projection of semantic upstream sources, parameters, incoming wiring and operator registry/algorithm/policy versions. Projection excludes presentation fields and is sorted by stable identities.
- Existing SDK retained operator jobs now compare canonical input/source/wiring/version identity before resuming. Changed dependencies cancel the old job at that key and start fresh. The same retained job registry and node cache are reused; disposable handles remain transient. Explicit inference dependencies and policy bytes also contribute to dependency identity. Handle-only requests without canonical source/dependency evidence are refused.

## Verification to date

The existing Brep canonical-architecture Nx oracle succeeded with 7 new AJV cases. Three.js triangulated BoxGeometry independently computes the same 24 and 60 fixture volumes. A native build succeeded, and four exact laws passed: real bundle identity, actual session/tessellation retirement, actual owned extension evaluate transport, and named inference call observer plus input/dependency/version invalidation and volume outputs. Public standard gateway and retained Boolean progress/cancellation are under repair/verification; no passing claim for those yet.

The ordinary plugin test runner is independently blocked before Cargo by a stale first-party fixture path: `artifactAdmissionOracle` reads nonexistent `✏️s/🔌️plugins/🌿️vcs/🦀️.rs`. The plugin canonical-architecture route bypasses that unrelated oracle; native compilation exposed stale diagnostic and JSON test imports, repaired narrowly in the associated test sources.

## Resource, Quality and Budget Contract

The public named gateway can use the installed extension's explicit resource owner, or the same gateway can be called through `ExtensionBundle::artifact_infer`. The Brep provider advertises full request/result JSON schemas and uses `SessionCapture` only for transient geometry resources. Canonical dependencies include actual upstream values and wiring; live handles alone are refused. This does not serialize or reconstruct the resource session into an independent portable document.

Kernel `NODE_KERNEL_METHOD`/`operation_quality` metadata supplies BRep result quality, independently of `complete`. The mesh owner supplies `PolygonMesh`, `TessellatedMesh` for fromBrep and `MeshDerivedBRep` for toBrep through one operation-quality contract shared by registered summaries and inference results. Unknown unregistered operators remain Unsupported. Every literal Brep operator registered in the root maps to the existing kernel quality table; polygon operators are covered by their own representation contract. The provider sets the existing evaluator's per-round work grant ceiling from the named inference work budget. Cold starts retire the parked job at the same key; bypass removes retained addressing; changed source/input/version/policy identities cancel parked work before restart. The existing retained entry includes the cancellation identity, and the installed resource owner retires its matching jobs when public inference cancellation arrives. No second cancellation registry, geometry cache or evaluator is introduced.

## SDK Suite Blocker

The SDK canonical-architecture build currently cannot compile unrelated existing test API mismatches: `dsl::json` references in reactor continuation, empty-state, progress and verdict tests; `ViewModel::default`; mutation `warn`; retained-window `dispatch` with four arguments. The narrow diagnostic import and world3d JSON type errors encountered earlier were corrected using the existing first-party APIs. The contextual service law is registered in that suite, but is not claimed to have executed there. Equivalent missing-context refusal and pure/contextual executable registry conflict checks are included in the real Brep worker law.

## Changed Sources

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: contextual executable and same-registry public gateway/owned context cancellation.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs`: explicit resource context/cancellation contract and owned named gateway.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-artifact-inference-service/🦀️.rs` and its existing Rust `📜️script.ts`: contextual registry law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` and `🔬️world3d-host-unit/🦀️.rs`: narrow first-party test API compile repairs.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs`: one shared dependency projection, canonical identity, existing retained entry/cancellation and round grant cap.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`: real contribution registration and resource-owner invocation/cancellation route, existing quality map available at runtime.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/`: request/result schemas, portable fixture and provider leaf.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🦀️.rs`, `🟦️.ts`, existing `📦️packages/🦀️rust/📜️script.ts`: real worker/gateway/retained laws and AJV/Three.js oracle.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🦀️.rs`: old fake bundle evaluate test now uses real owned extension.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`: source/version payload before publication orchestration.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`, `🧫️fixtures/📐️geometry-dependencies.json`, `🧪️tests/🔬️unit/🦀️.rs`: same shared source/version payload, presentation exclusion fixture/law.

A concurrent shared store constructor repair (`DslValue::UInt` → `uint`) was coordinated with the mesh output owner; no separate store ownership is claimed here. No inspector, viewport or scene-lane files were edited.

The real named gateway law also executes mesh.construct with one-unit grants, checks PolygonMesh on every working result, resumes using the standard Incremental/previousState route, and requires a complete polygon output after multiple progress hops. The manifest regression checks that all registered brep.* summaries declare a supported quality. Mesh helper and registration summary lines were coordinated with the mesh output owner.

## Preview Dependency Validation

`bun nx run @semio-tech/procedural-generation3d-rs:test --skip-nx-cache -- quick --locked --lib geometry_inference_dependencies_include_sources_and_wiring_without_presentation` passed: Nextest 1 passed, 305 skipped. This ran the actual shared SDK projection through the Generation3d helper and its language-agnostic fixture.

Mesh quality was synchronized into the Brep static descriptor `🔣️.json` (164 summaries across four embedded manifest copies). Its old descriptor does not yet advertise artifact contributions; existing `describe_extension` does correctly serialize live contributions. The final package/current closure refresh must regenerate descriptor metadata and hashes from current source before runtime route advertisement. Root owns that build.

## Gateway Receipt and Test Correction (2026-10-03)

Native artifacts `exact-cargo-laws-Q3dueq/00` show actual automatic named gateway execution with `[DEBUG] named geometry automatic gateway success=true`. The same law first passed schema-wrapper quality/validity and retained one-unit polygon construction assertions. Its final assertion compared two independently allocated box handles and failed despite equivalent geometry. The law now measures the automatic result through the public named volume query before disposing the installed bundle and compares volume against the fixture (24), rather than equating opaque resource identities. Fresh canonical run is pending; this is not yet a passing whole-law receipt. Generated schema operators now report `SchemaValue` through their existing owned registry contract.

## Public Gateway Native Receipt

Exact native run `exact-cargo-laws-OjwMv9/00` passed the first five laws, including `named_geometry_inference_standard_gateway_uses_registered_extension_context`. The real installed public gateway supplied its actual owned SessionCapture, computed box geometry, and a second public named volume query verified the fixture value 24. The same law validated schema wrapper `SchemaValue`, proper validity/completion, and retained one-unit mesh construction with honest `PolygonMesh` quality through ordinary/Incremental wire cache modes. Temporary debug prints were removed after observed runtime evidence.

The sixth retained Boolean law then hit the same opaque-handle equality mistake before its cancellation section. Its assertion now compares completed tessellation meshPack chunks from ordinary versus retained Boolean outputs. This is a direct geometry-output parity check with the existing tessellation owner; retained cancellation verification is still pending the fresh exact native run.

## Current Replay (2026-10-03)

One fresh registered provider replay after the shared graph owner green stopped before owned tests on Playbook generation close_step String/ValueError mismatch and DAG snapshot missing ValueRefusalKind constructors. Exact paths and log are recorded in 📓️mesh-attributes-2026-10-03.md. Previous first-five exact gateway receipts remain valid observations of their snapshot; the current retained ordinary-output parity/cancellation and worker/pack laws are still pending. Root owns the shared migration gate and final runtime closure.
