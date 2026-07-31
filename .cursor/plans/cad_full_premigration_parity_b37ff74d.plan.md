---
name: CAD Full Premigration Parity
overview: Replace the three remaining stubbed areas of the wgpu CAD program (transformations, interaction engagement, save/load) with real implementations that reproduce premigration behavior exactly, using the existing Rust BREP kernel instead of restoring any JS/React code.
todos:
 - id: typology-alignment
   content: Rename TYPOLOGY_CATALOG/fixtures to premigration-qualified typology ids scoped per pane (cad/plugin/rs/lib.rs)
   status: completed
 - id: face-analytics-helpers
   content: Add face-centroid + facePlaneGroupKey helpers on top of kernel_3d_brepkit (CAD-side, no kernel crate changes)
   status: completed
 - id: persist-object-solids
   content: Extend typology_brep_mesh construction to keep GeometryHandle per CadObject instead of discarding after tessellation
   status: completed
 - id: derive-transformation-engine
   content: Implement run_derive_transformation (fuse + classify + merge + ensure) mirroring runDeriveTransformation, replace clone/relabel stub
   status: completed
 - id: transformation-appliers
   content: Implement from_building custom applier and typology-whitelist fallback so all CAD_TRANSFORMATION_SPECS route through the correct of 3 paths
   status: completed
 - id: interaction-spec-types
   content: Add Rust InteractionSpec/StateDefSpec/TransitionSpec/DisplayItemSpec/CommitOperationSpec types mirroring cad/schema/json/interaction.json
   status: completed
 - id: statechart-interpreter
   content: Implement statechart interpreter (applyTransition/canCommitFromState/runCommit) and embed the 4 ported specs (box, externalwall, slab, column)
   status: completed
 - id: engagement-rewire
   content: Rewire cad_window_engagement/engagementSubmit/possible_engagements/REPL parsing onto the statechart interpreter instead of flat typology matching
   status: completed
 - id: preview-display-items
   content: Render active interaction display items (box-preview/linear-handle) via world3d_scene_extended
   status: completed
 - id: native-file-dialogs
   content: Add rfd crate; implement native download_media_export save branch and a requestFileOpen operation + native open-dialog handling
   status: completed
 - id: step-export-wiring
   content: Wire saveInPlay/saveCurrent to export_step_sync on real pane solids; keep saveSelected as SpatialExchangeBundle JSON
   status: completed
 - id: load-envelope-unwrap
   content: Match handleLoadRaw's modelSpace/model/raw/root envelope-unwrap priority before importSpatialJson deserialize
   status: completed
 - id: tests-extend
   content: Extend existing cad/rs and cad/plugin/rs test modules to cover derive classification, statechart commits, and save/load round-trips
   status: completed
isProject: false
---

# CAD Full Premigration Parity

## Context

The prior pass (ticket `cadwgpupremigrationparity`, now closed) implemented the _shape_ of premigration parity — schema, VCS, hierarchy chrome, multi-selection, references, a transfer toolbar, save/load commands, and an engagement REPL — but three areas are **stubbed, not behaviorally identical** to premigration (`f8376e8486`):

1. **`applyTransformation`** (`cad/plugin/rs/lib.rs:623-665`) clones objects and relabels typology to `"energy.energy.hull"` — it never fuses BREP solids or classifies faces like premigration's `runDeriveTransformation` (`cad/core/js/index.ts:3531-3627`).
2. **Interaction engagement** (`cad_window_engagement`, `engagementSubmit`, `cad/plugin/rs/lib.rs:1488-1567,2229-2256`) is a flat typology-string matcher that instantly creates a placeholder object — not premigration's per-typology `InteractionSpec` statechart (states/transitions/guards/effects/commit, `cad/core/js/index.ts:5173-6665`) with REPL keyed-transition parsing (`cad/renderer/js/index.tsx:5493-5525`).
3. **Save/load** (`export_spatial_json`, `pending_export`, `cadLoadRequest`, `cad/plugin/rs/lib.rs:667-723,2124-2149`) only stashes/reads JSON in-memory — `pending_export` has no consumer and `cadLoadRequest` is unhandled by the wgpu renderer, unlike premigration's real file-picker + STEP/JSON export (`cad/renderer/react/index.tsx:1162-1433`).

Feasibility check confirmed the Rust BREP stack (`kernel_3d_brepkit`, `kernel/3d/brep/rs/lib.rs`) already provides boolean fuse (`fuse_sync`), face/surface normals (`surface_normal_sync`), and STEP export/import (`export_step_sync`/`import_step_sync`) — so real geometry-backed transformations and STEP export are buildable without a JS kernel. It lacks face centroid and plane-grouping helpers (`faceCentroid`/`facePlaneGroupKey` in premigration `cad/kernel/brepjs/js/index.ts:1210-1352`), which must be added. No native file-dialog crate exists in the workspace; none is needed for wasm32 (existing `download_media_export` wasm branch already does anchor-download) but the **native desktop branch is a no-operation stub** (`framework/renderer/wgpu/rs/lib.rs:11594-11595`) and must be implemented for real load/save.

## Phase 1 — Typology/model-definition alignment (`cad/rs/lib.rs`, `cad/plugin/rs/lib.rs`)

Premigration typologies are namespaced per model definition (`energy.energy.externalwall`, `structure.structure.onewayreinforcedconcreteslab`, `structure.structure.reinforcedconcretecolumn`, `spatial.shape.primitive.box`, `aec.building.*`). The current `TYPOLOGY_CATALOG` (`building.building.slab/column/beam/wall`, `spatial.shape.box`) does not match these ids, so transformations/interactions can't be scoped by model definition. Rename the catalog and any fixture data (`FOREST_LEFT_MODEL_JSON`, `make_object_for_typology`) to use the real premigration-qualified typology ids, scoped per pane/model-definition, so downstream classify rules and interaction lookups key off identical strings to premigration.

## Phase 2 — Real derive-transformation engine (`cad/plugin/rs/lib.rs`)

Port `runDeriveTransformation` (`cad/core/js/index.ts:3531-3627`) using `kernel_3d_brepkit`:

- Add `face_centroid_sync`-equivalent helper (vertex-average via existing `deconstruct`/point-query APIs) and a `face_plane_group_key` helper (port of `facePlaneGroupKey`, `cad/kernel/brepjs/js/index.ts:1337-1352`) — implemented CAD-side (no kernel crate changes needed) since the kernel already exposes normals + boolean operations.
- Build each source-pane `CadObject` as a real kernel solid (extending `typology_brep_mesh`'s existing `box_prim`/`cylinder_prim` construction pattern, `cad/plugin/rs/lib.rs:133-146`, to keep the resulting `GeometryHandle`s instead of discarding them after tessellation).
- Implement `run_derive_transformation(spec, source_objects) -> Vec<CadObject>`: fuse touching solids (`fuse_sync`), find external faces (port `fuseSolidsToExternalFaces` contact-pair logic, `cad/kernel/brepjs/js/index.ts:1301-1334`), classify via the same rule chain as `from_geometry` (z-dominant → roof/baseplate, axis-dominant → externalwall, opening override → windows, fallback → hull), merge coplanar faces by plane group, and `ensure` an empty windows typology row when none classified — mirroring `cad/asset/modelDefinition/aec.building.energy/transformation/from_geometry/transformation.json` exactly.
- Replace `apply_transformation_to_envelope`'s clone/relabel branch with: registered custom applier for `from_building` (typology remap, mirroring `cad/module/aec-building-structure/js/index.ts:82-99`), derive engine for `from_geometry`, and typology-whitelist fallback (mirroring `applyTransformationFallback`, `cad/core/js/index.ts:3518-3529`) for the remaining specs (`classic`, FEM variants) — selected per `CadTransformationSpec` the same way `applyTransformation`'s three-path dispatch works (`cad/core/js/index.ts:3631-3637`).
- Keep `dispatch_cad_operations`/VCS routing (`CadOperation::SetPaneObjects`) as the mutation path — only the objects computed for the target pane change.

## Phase 3 — Real per-typology Interaction state machine (`cad/rs/lib.rs`, `cad/plugin/rs/lib.rs`)

Add Rust types mirroring `cad/schema/json/interaction.json`/`cad/core/js/index.ts:699-829` (`InteractionSpec`, `StateDefSpec`, `TransitionSpec`, `DisplayItemSpec`, `CommitOperationSpec`, `InteractionSpatialConfig`), plus a small statechart interpreter (`applyTransition`, `canCommitFromState`, `runCommit` — mirroring `cad/core/js/index.ts:5173-6665`) that:

- Evaluates guards/effects declaratively (a small expression subset is sufficient — enough to cover the ported specs' actual guards, not a general expression language).
- Embeds the same 4 concrete specs found in premigration (`primitive.box`, `energy.energy.constructExternalWall`, `structure.structure.constructOneWayReinforcedConcreteSlab`, `structure.structure.constructReinforcedConcreteColumn`) as Rust constants (ported field-for-field from their JSON assets) inside `cad/plugin/rs/lib.rs`, scoped to the correct pane via Phase 1's aligned typology ids.
- On commit, runs the corresponding action (`primitive.createBoxFromCorners`, wall/slab/column construction) against the kernel to build a real solid/object and dispatches through `CadOperation::AddObject` + VCS, replacing today's one-shot `make_object_for_typology` placeholder in `engagementSubmit`.

Rewire the engagement chrome:

- `possible_engagements`: idle → all interactions scoped to the active pane's model definition (mirrors `listSpatialInteractionsForModelDefinition`); active session → current state's keyed transitions (mirrors `listKeyedInteractionTransitions`, `cad/core/js/index.ts:5631-5647`).
- REPL `engagementInput`/`engagementSubmit`: parse in the same priority order as `trySubmitLine` (`cad/renderer/js/index.tsx:5493-5525`) — numeric value lines (`set.height`, `dist`, `footprint`) → start-interaction-by-id → keyed transitions — replacing the current flat "typology string match" logic.
- `status`/`control`: surface `state name`, selection count, and last-response OK/Error the same way `cadPlayEngagementMirror` does (`cad/renderer/core/js/index.ts:868-924`), using the framework's existing `WindowEngagement.control`/`status` fields (already present per the earlier gap analysis — no framework change needed).
- In-canvas previews (`box-preview`, `linear-handle` display items) render through the existing wgpu `world3d_scene_extended` API the same way reference overlays already do (Phase 7 of the prior ticket) — add a "preview" entry per active interaction snapshot.

## Phase 4 — Save/load with real file I/O

**Native desktop (non-wasm32):**

- Add the `rfd` crate (cross-platform native file dialogs, also supports wasm32) to `cad/plugin/rs/Cargo.toml`/`framework/renderer/wgpu/rs/Cargo.toml` as appropriate.
- Implement the native branch of `download_media_export` (`framework/renderer/wgpu/rs/lib.rs:11594-11595`, currently a no-operation) to open an `rfd::FileDialog` save dialog and `std::fs::write` the bytes — this single fix also makes CAD's existing OBJ/GLB export work on native desktop for the first time.
- Add a matching `requestFileOpen`-style operation (handled the same way as `downloadMediaExport`) so `loadRawRequest` opens a native open-dialog and reads the file via `std::fs::read_to_string`, replacing the currently-unhandled `cadLoadRequest` operation.

**Persisted BREP geometry for STEP export:**

- Extend `CadObject` construction (Phase 2's kept `GeometryHandle`s) so `saveInPlay`/`saveCurrent` can call `export_step_sync` on the pane's real solids instead of hand-built JSON — matching premigration's STEP output for these two actions (`cad/renderer/react/index.tsx:1169-1189`). `saveSelected` keeps producing the `SpatialExchangeBundle`-shaped JSON (`{ model, modelSpace, activeModelDefinitionId }`, `cad/renderer/react/index.tsx:152-156`) since premigration also uses JSON there, not STEP.
- `loadRawRequest`'s loaded JSON is parsed the same way as `handleLoadRaw`'s envelope-unwrap priority (`modelSpace` → `model` → `raw` → root, `cad/renderer/react/index.tsx:1389-1433`) before being applied via `importSpatialJson`'s existing `CadScene` deserialize path.

**wasm32:** no change needed beyond wiring the new `requestFileOpen` operation — `rfd` supports wasm32 via the browser's file picker, and `download_media_export`'s wasm32 anchor-download branch is already correct.

## Process notes

- Reopen ticket `cadwgpupremigrationparity` (same goal, `🎯️r2602🎯️runningsketchpad`) once implementation starts, per repo rules — this is a continuation of the same task, not a new one.
- Extend the existing test modules in `cad/rs/lib.rs` and `cad/plugin/rs/lib.rs` (no new test files) to cover: derive-transformation face classification on a known box fixture, interaction statechart commit for each of the 4 ported specs, and save/load round-trips (JSON for selected, STEP for model-space/current).
- No React/JS renderer restoration — everything above is implemented in Rust against the existing wgpu plugin/kernel stack.
