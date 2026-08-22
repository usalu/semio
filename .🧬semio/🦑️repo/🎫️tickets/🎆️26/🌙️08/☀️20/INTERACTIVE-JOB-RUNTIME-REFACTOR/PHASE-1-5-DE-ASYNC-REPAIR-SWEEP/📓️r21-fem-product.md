# R21 — FEM Product De-async Repair and P6f Visual Language

## Scope

This packet owns the FEM engine tree except `🧮️analyses/🦀️component.rs`, plus the FEM plugin tree and its local Rust glue. Shared framework, other plugins, renderer, stdio, and the excluded analyses implementation remain untouched.

## Implementation

- Converted the pure FEM numerical leaf engine to synchronous calls, matching the already-synchronous `Element` contract and removing decorative futures from element stiffness, recovery, mass, meshing, solve, modal, buckling, and their tests. True framework suspension boundaries (`ArtifactBuilder`, `ArtifactAnalysis`, `ArtifactComposition`, `ArtifactEditor`, and `ArtifactViewer`) remain asynchronous.
- Re-exported the plugin-owned `FemApps` closed runtime enum at crate root so artifact declaration modules can use the stable `crate::FemApps` type.
- Replaced removed plugin-level `world3d_default_selection_json` references with the public `world3d_selection_json("rectangle", &[], None)` scene selection contract.
- Migrated FEM Canvas2d/World3d windows from obsolete plugin-local scene nodes to the schema-backed semantic UI scene encoder and `BuiltNode` surface contract.
- Repaired compiler-exact ready-future boundaries with the framework-owned `resolve_ready` bridge where a synchronous definition or test calls an intentionally asynchronous SDK boundary.
- Corrected all six FEM protocol fixture headers so their protocol identifiers begin with a valid alphabetic namespace (`fem2d`/`fem3d`) rather than the illegal numeric-leading `2d`/`3d` tokens.
- Made the FEM editor test host apply emitted `LoadDocument` effects through the real pack-loading boundary. Command fixtures now create every referenced node, material, and section through semantic commands before testing dependent mutations; no validation was bypassed.
- Rebased Canvas2d/World3d assertions on typed `SurfaceDoc` decoding. Renderer payload data is opaque in `BuiltNode` JSON by design, so tests decode `Canvas2dScene`/`World3dScene` before checking mesh, contour, reaction, instance, and vertex-color content.

## P6f Visual Language

The 2D editor model now exposes a deterministic replaceable `Fem2dLiveVisual` surface input and `render_with_progress` entry point. Its stable layer identifiers and colors distinguish:

- `unmeshed`, `coarse`, `refined`, and `final` region quality;
- elements currently being assembled;
- load arrows and support glyphs;
- live displacement and residual vectors;
- `unconverged`, `converged`, and `validated-final` solve status.

The overlay sorts regions, assembly ids, and node fields before encoding, so worker completion order cannot perturb the rendered packet. Ordinary `render` remains a settled model surface and does not invent a live-job state; the worker/session owner can pass its newest accepted preview to `render_with_progress`.

Focused coverage replays the same reverse-ordered accepted preview twice and requires identical layer values. A 256-field adversarial overlay measures its complete deterministic layer build and fails at `8,000 µs`; this is the P6f surface-side micro-step boundary, distinct from the already-covered P6b–P6e compute-job steps.

## Verification

- Owned engine census, excluding analyses: zero `async fn`, `.await`, or async-test attributes.
- Focused Rust formatting of the engine and changed glue/scene files: passed.
- Native library check: **passed**, zero diagnostics (`📝️r21-fem-native-4.txt`).
- Full FEM library test compile (`cargo test -p semio-s-plugin-fem --lib --no-run`): **passed**, zero diagnostics (`📝️r21-fem-test-no-run-4.txt`).
- Post-P6f native check: **passed** (`📝️r21-fem-native-5-p6f.txt`).
- P6f deterministic replay and 256-field `<8 ms` visual build: **2 passed, 0 failed** (`📝️r21-p6f-focused-tests-2.txt`).
- P6f load/support glyph coverage: **1 passed, 0 failed** (`📝️r21-p6f-glyph-tests.txt`).
- Full FEM library execution after fixture repairs: **754 passed, 0 failed** with `--test-threads=1` (`📝️r21-fem-tests-5-serial.txt`). The single-threaded runner is intentional for wall-clock assertions: a parallel run under concurrent workspace compilation measured the otherwise-green root-owned assembly timing test at `14,134 µs`, then `8,814 µs`; its isolated rerun passed (`📝️r21-assembly-timing-rerun-1.txt`). No product semantics or threshold were weakened.
- Native release library check: **passed** (`📝️r21-fem-release-check.txt`, `Finished release profile` in 12m58s).
- Official Nx wasm component/descriptor gate: **blocked upstream** after FEM compilation reached the component link (`📝️r21-fem-nx-describe-wasm.txt`). `semio-s-plugin-stdio`, built as FEM's no-default-feature dependency, does not export the required `semio:framework/reactor@1.0.0#poll` function. The earlier raw wasm `cargo check` log (`📝️r21-fem-wasm-check.txt`) also records why `check` is not a valid substitute for a component build: component guest exports are materialized only by the link pipeline.
- Nx quick routing was attempted twice with the ticket-local target. Both invocations reached the framework-defined hard `15,000 ms` nextest budget while rebuilding concurrently modified upstream dependencies, before the FEM test binary executed (`📝️r21-fem-nx-test-quick.txt`, `📝️r21-fem-nx-test-quick-rerun.txt`). This is a routing-budget result, not a test failure; the same package's authoritative direct execution is the **754/754** result above.
- Clippy with the same ticket-local target: **passed** (`📝️r21-fem-clippy.txt`, exit 0). The log retains the workspace's existing lint warnings; no warning-deny policy was requested or weakened.

## Final mounted-tree rerun

- The editor testkit now exposes genuine async dispatch and every owned caller awaits it; no
  synchronous executor bridge was reintroduced.
- The sparse buckling path no longer Cholesky-factorizes an indefinite projected geometric stiffness
  matrix. It uses a deterministic stiffness-metric basis, a symmetric projected operator, ordered
  positive reciprocal modes, and a finite `f64::MAX` sentinel for null/non-positive modes.
- `📝️p6-subspace-k-metric-final.txt`: **4/4** focused subspace, dense-differential,
  checkpoint/replay, and deterministic sentinel tests passed.
- `📝️p6-example-fixture-final-2.txt`: **2/2** real 2D/3D example solve tests passed; the 3D fixture
  asserts finite, positive, monotonically ordered buckling factors.
- `RESUMABLE-FEM-JOB-GRAPH/📝️p6f-visual-final-2.txt`: **3/3** P6f visual-language tests passed.
- `RESUMABLE-FEM-JOB-GRAPH/📝️p6f-live-visual-timing-measured.txt`: isolated P6f overlay step measured
  **1,470 µs** against the unchanged **8,000 µs** ceiling; the temporary `[DEBUG]` probe was removed.
- `RESUMABLE-FEM-JOB-GRAPH/📝️p6-full-serial-final.txt`: authoritative current native suite passed
  **756/756** in **1.81 s**.
- `RESUMABLE-FEM-JOB-GRAPH/📝️p6-release-check-final.txt`: current-tree release check passed in
  **5m 27s**, with 25 existing warnings and zero errors.
- Final owned FEM engine/plugin source census is zero for `[DEBUG]` markers.
- `RESUMABLE-FEM-JOB-GRAPH/📝️p6-wasip2-describe-final-3.txt`: current FEM and its dependencies
  compile/link for `wasm32-wasip2`. The owned wasm-bindgen stores now expose async `create` factories
  and await genuine async store operations; no deprecated async constructor remains. The describe
  executor subsequently fails in shared descriptor assembly because it treats the two distinct
  scoped standards `s.fem.fem2d@1` and `s.fem.fem3d@1` as a global key conflict on `"1"`. No FEM
  source or link diagnostic remains, and the descriptor tool correctly refuses a placeholder.

## Descriptor registry closure

The shared format descriptor now retains its existing global registry while keying each scoped
standard with the already-canonical composite kind identity (`artifact-kind@standard`). The focused
fixture passed **1/1** (`RESUMABLE-FEM-JOB-GRAPH/📝️p6-format-descriptor-key-exact-2.txt`). The official
Nx describe command then passed and emitted the real FEM component descriptors
(`RESUMABLE-FEM-JOB-GRAPH/📝️p6-describe-composite-key-final.txt`). The earlier descriptor blocker is
therefore closed without weakening either FEM StandardId.
