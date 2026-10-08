# WSX Reference Plugins: 3d B-Rep, Mesh, Editing and Analysis Patterns

Read-only audit for the rebuild of `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`.
All paths are relative to `/Users/ueli/Documents/semio/`. Line numbers were checked against the files.
Where a claim was not verified it is marked as such.

## 1. Plugin inventory

### 1.1 CAD (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

**Document schema.** The CAD artifact is a composition root, not a geometry store.
- `CadSnapshot` holds four fixed composed-child slots: `shape_model`, `building_model`, `energy_model`, `structure_classic_model`. Each slot is an `s.stdio.semio.model` child document. Geometry lives in stdio child documents, not in the CAD snapshot.
- `🧬️schema/📐️geometry/🦀️.rs` states that `CadGeometry` and `CadObject` are ephemeral import intermediates. They are parsed from `spatial.model` JSON and fed into the live `Brep` kernel (`import_geometry_handles`, line 321). They are never persisted (module doc, lines 1-12).
- Schema family present: `🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🦀️.rs`, `🟦️.ts`, `📸️snapshot`, `🔺️diff`, `🧬️mutations`, `💡️inferences`.

**Mutations (`🧬️schema/🧬️mutations/`, 21 kinds, `CadMutation` enum at `🧬️mutations/🦀️.rs:61`).**
- Composition and slot mutations: `🧊️create-brep` (line 9, `CreateBrep { child_id, target, index }`, "inserts an exact owned topology sibling"), `🧹️delete-brep`, `🧱️create-shape-model` (line 14, sets a composed slot, inverse restores the prior handle), `🧨delete-shape-model`, `🏢create-building-model`, `💥delete-building-model`, `🏛️create-structure-classic`, `💣delete-structure-classic-model`, `⚡create-energy-model`, `🔌delete-energy-model`.
- Document and reference mutations: `➕create-node`, `🗑️delete-node`, `🏷️rename-node`, `📍move-reference`, `📏change-reference-width`, `🔒change-reference-locked`, `👁️change-reference-hidden`, `📎replace-references`, `🖇️replace-reference-media`, `📐️create-drawing`, `🧹delete-drawing`.
- Every descriptor `🔣️.json` has `"editable": false` and `"invertibility": "explicit-mutation"`. The descriptor fields include `binaryTag`, `diffParticipation`, `outcomeClasses` and `composition: "atomic"`.
- Geometry edits (move-vertex, create-face, create-solid, ...) are NOT CAD mutations. They are the stdio `s.stdio.semio@v1/🧊️brep` mutations (see 2.3).

**Inferences (`🧬️schema/💡️inferences/`).**
- One inference: `📦bounds`. `CadInference` (`💡️inferences/🦀️.rs:19`) has `object_count`, `vertex_count` and `bounds`, all `#[derived]`. The `Inference` impl is at line 28.
- Degraded. `📦bounds/🦀️.rs` states that the bounds inference now returns `None` (`scene_bounds`), `vertex_count` returns `0` (line 38), and `object_count` counts only the four slots (line 33). Real values need the composed children to be resolved. This means the CAD bounds inference currently reports no geometry.
- There is no `brep.mesh` inference in CAD. Tessellation is a view-time derivation (see 1.4).

**Geometry derivation.**
- `🧬️schema/📐️geometry/🦀️.rs`: `mesh_from_owned_brep` (line 55), `import_geometry_handles` (321), `tessellate_geometry_handle` (515), which calls the framework B-Rep tessellation.
- Derived per render, not cached, not an async job.

**Interaction (`✏️editor/⚙️engine/🕹️interaction/🦀️.rs`).**
- A generic interpreter over declarative `spatial.interaction` JSON assets. There is no hand-written statechart per primitive (module doc, lines 1-7).
- Assets: `📚️examples/🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🕹️interactions/*.json`, 60 files across eight model definitions. Example: `📦️box.json` declares `pickDisabledStates`, `groundPointerMoveStates`, `lengthEntry` (stepper controls with unit) and `commit.operation.action`.
- Commit actions map to `🎬️actions/*.json` (e.g. `📦createBoxFromCorners.json`, `🧮selectGeometries.json`).
- The model definition descriptor is `📐️spatial.shape/🔣️modelDefinition.json`. It declares `baseObjectTypology`, `kernelTypologies` (anchor, vertex, edge, wire, face, shell, solid) and `kinds: [action, attribute, interaction, property, stat, typology]`.

**Picking (`✏️editor/⚙️engine/🧲️picking/🦀️.rs`).**
- `SpatialPickKind` (line 28) with pointer down or move. Sub-object targets (`SpatialPickTargetKind`, line 45), visibility toggles, hover-key aliases, and merge modes via `protocol::MergeMode`.
- Target-neutral by its own doc (lines 1-12). Screen-space ray math is delegated to the framework `ui_scene::math` module.

**Other editor pieces.**
- `✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/` (transform tool).
- `✏️editor/🎭️modes/✏️edit/☑️options/` (sun, projection, dislocate).
- `✏️editor/🎮️commands/🗺️model-definition/🦀️.rs` (line 27 `SetActiveExample`, which selects the model definition and bundled example).
- `⚙️engine/🧬️typology`, `🎰️stately`, `🏃️runtime`, `🎬️actions`, `📺️renderer` (TypeScript).

**Third-party runtime.** `⚙️engine/🧱️brepjs/🟦️.ts` imports `brepjs` and `brepjs-opencascade` behind an adapter block (`#region 🔌️Adapters`). Its owned B-Rep contract types are defined locally.

### 1.2 Lowpoly (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

**Document schema.**
- `LowpolySnapshot` holds `objects`. Each object carries a `transform` (position, rotation, scale), a name, a smooth-shading flag, paint layers, and a `mesh` child slot.
- `🕸️mesh/🦀️.rs` defines `LowpolyMeshState` (vertices, halfedges, faces, uv_seams, attributes, materials, textures). It is a persisted half-edge state in a child document, reusing `semio_framework_3d::mesh` attribute enums.

**Mutations (`🧬️schema/🧬️mutations/`, 21 kinds).**
- Object and transform: `🌱️create-object`, `💀️delete-object`, `🏷️rename-object`, `↗️move-object`, `🔄️rotate-object` ("sets an absolute Euler rotation"), `📐️scale-object`, `🔀️reorder-objects`, `🔘️change-object-smooth`.
- Selection-intent (relative): `🚚️move-selection` (`MoveSelection { object_id, vertex_ids, offset }`, line 14), `🌀️rotate-selection`, `🔍️scale-selection`. Their doc says: "the geometry is derived on every application from the object's persisted mesh, so the drag replays onto whatever mesh the object holds by then".
- Mesh child: `🕸️create-mesh` (sets the `mesh` child slot, overwrite-aware), `🧨delete-mesh`.
- Paint: `🖌️apply-paint-stroke`, `➕️insert-paint-layer`, `➖️remove-paint-layer`, `🎨️edit-paint-layer`, `🌫️change-paint-layer`, `🎛️change-paint-layer`, `👁️change-paint-layer`, `🔖️rename-paint-layer`.

**Inferences.** One inference: `📦bounds` (`💡️inferences/📦bounds/🦀️.rs`). It reads only `transform.position` of each object. The mesh is not visible to the inference (the doc says mesh content lives session-side). No `InferredField`.

**Gumball (`✏️editor/🎮️commands/🧲️transform/🦀️.rs`).**
- `LowpolyGumballMotion` (translate, rotate, scale). `lowpoly_gumball_leaves` (line 33) binds the motion to the selection's objects and vertices and returns `Vec<LowpolyMutation>`. `gumball_emit` (line 72).
- Per the module doc (lines 1-9), every gesture runs through the lowpoly tool statechart under the `🛠️tool-machine` runner. The host accumulates a drag locally and dispatches the NET pose delta once on release. One gesture is one dispatch, one `ToolTransaction`, one history row.
- The tool glue is `✏️editor/🖌️session/🦀️.rs`: `LowpolyScratch` (line 154, session-local mesh workspace cache and paint texture cache, never serialized), `LowpolyTransient` (line 298, per-window open gesture and its provisional leaf), `lowpoly_tool_emit` (line 512).

**Mesh edit (`✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs`).**
- Commands `extrude` (handler at line 30), `inset`, `bevel`, `loopCut`, `subdivide`, `triangulate`, `mirror`, `decimate`, `flipFaces`, `merge`, `dissolve`, `snap`, `toggleSmooth`.
- Each handler calls `mesh_edit` (`✏️editor/🖌️session/🦀️.rs:121`). That builds a compute session, runs the framework kernel op (for example `extrude_faces`), syncs meshes back, and diffs before/after.
- The diff is turned into one semantic mutation by `semantic_mutation_for_patch` (`🖌️session/🦀️.rs:66`). Priority: name, then smooth shading, then transform axis, then mesh. Mesh changes are persisted as `create-mesh` with the FULL `mesh_state`, or `delete-mesh`.

**Editor layout.** `✏️editor/🎭️modes/` has `🎨️paint` and `✏️edit`. `✏️edit` has `🪟️windows/🌐️model`, `🎮️commands` and `⚙️engine`. Shared commands live in `✏️editor/🎮️commands/` (`🗂️selection`, `🧲️transform`, `🔷️mesh-edit`, `🧵️uv`, `🌞️sun`, `🖌️paint`, `➕️add-primitive`, `🎥️camera`, `🗑️object`). Options are in `✏️editor/🛠️options/` (`🧲️gumball`, `🗂️select`, `🧲️snap`, `👁️show-edges`, `🌞️sun`, `🖌️paint-params-brush`). Panels are in `✏️editor/📌️panels/` (`🗿️artifact`, `🔍️inspection`, `🗂️layers`, `🛍️catalogue`).

**Tooling.** `🧪️tests/🔬️unit` and `🧫️fixtures/🧪️interactive-job` exist. The `.mesh.json` fixture is in the stdio example set.

### 1.3 Puzzle 3d (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- Edit-mode windows: `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/`. Utilities are `🔄️transform` (the world gumball; `🪛️utilities/🔄️transform/🦀️.rs` header lines 1-8 say it is also the transform TOOL, a `🔄️machine` statechart driven by `🛠️tool-machine`), `🖌️brush`, `🧊️volume-brush`, `🚚️world-relocate`.
- Tools: `✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`. Declared as a framework tool run with `ToolRunDefinition`. The framework owns start, pause, resume, step, abort, finalize and dismiss, plus provisional placements in its ledger, and publishes one edit on finalize.
- Brush suggestions are a read-only tool run (`🖌️brush`, same doc).
- Options: `☑️options/🎯️select/🦀️.rs` (which entity kinds a pick may reach; marquee method and merge mode moved to the framework `vortex` interaction domain).
- Precompute jobs: `✏️editor/⏳️precompute/` (`📐️geometry`, `🖌️brush`, `🪣️fill`).
- Gumball flag command: `✏️editor/🎮️commands/🕹️set-transform-gumball-flag/`.
- Mutations: `🧬️schema/🧬️mutations/` includes `📐scale-target-volume`, `🪢connect-vortices`, and others (not inspected in full).

### 1.4 FEM 3d (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/`)

- Gumball tool: `✏️editor/🕹️interaction/🧭️gumball/🦀️.rs`. Header (lines 1-8): a `🔄️machine` statechart whose effects are `ToolYield`s, driven by `🛠️tool-machine`. Each translate, rotate or scale dispatch builds one relative `move-selection` tick leaf. A streamed gesture accumulates into one open transaction in a gumball transient (previewed by all windows). It commits the NET leaf once. An abort leaves zero trace.
- Helpers: `fem3d_transform_targets` (line 39), `fem3d_transform_pivot` (86), `fem3d_gumball_tick` (124, returns a `MoveSelection`), `fem3d_gumball_then` (137, folds a tick into the net leaf).
- Mutation: `🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection` (a same-named leaf to lowpoly's `🚚️move-selection`, see section 4).
- Windows with utilities: `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🪛️utilities/` and `🪟️windows/📊️results/🪛️utilities/`. Analysis results live under `📈️analysis/` (benchmark fixtures such as `solves-fem3d-1-benchmarks`).
- Commands: `🎮️commands/🧭️gumball`, `📏️add-member-udl`.

### 1.5 Block (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/`)

Partially inspected. Found `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🪛️utilities/` and `☑️options/📏️spacing`. Did not read the 3d world's gumball or picking code. Treat block as unverified for sub-element selection and measurement.

### 1.6 Procedural generation3d (target, `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- Already a parametric graph. Schema: `🧬️schema/` with `🧬️mutations` (29 leaf dirs, `Generation3dMutation` enum at `🧬️mutations/🦀️.rs:224`) and `💡️inferences` (one inference: `🧭topology/`).
- Widget model is the flow framework's `semio_framework_artifact_flow_flow::Widget` (not a procedural-only type). Widget mutations: `🌱️create-widget` (insertion index, upsert by id), `🩹update-widget`, `🧹delete-widget`, `🎛️change-widget-input`, `🎚️change-slider-value`, `🔗️connect-synapse`, `✂️disconnect-synapse`, `✋️drag-transforms`, `📍️move`, `🔃️rotate-transforms`, `📏️scale-transforms`, `🚚️move-nodes`.
- `change-widget-input` doc (line 1-3): "the ABSOLUTE typed literal one unconnected operator input is set to ... so editing it in history replays exactly that value on any base."
- Mesh edits as widgets: `✏️editor/🎮️commands/🥽️edit-mesh-selection/` splices a typed operator with default params, then sets every chosen input as an absolute `change-widget-input` leaf. The doc says "ONE edit ... so a history edit changes extrude distance = 0.1, never a whole operator record".
- Preview: `🧵️preview-eval/⏯️tool-run/🦀️.rs` (read-only `previewEval` tool run over a retained `FlowEvalSession`).
- Flow extension for B-Rep and mesh nodes: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/` with `🥽️mesh/` (indexed mesh widgets, B-Rep preview conversion, uses `HalfedgeMesh`, `MeshModelingJob`, `TessellationJob`) and `💡️inferences/📐️geometry/🦀️.rs` (a geometry inference provider registered as `s.flow-extension-brep.geometry`, lines 6 and 41, executed through an `ArtifactInferenceExecution`).

### 1.7 stdio standard (generic B-Rep and mesh subsets)

Located at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`.

- `🧊️brep/🧬️schema/🧬️mutations/` (15): `🧊create-solid`, `🔷create-face`, `🖇️create-edge`, `🏗️create-vertex`, `🐚create-shell`, and their deletes; `📍move-vertex`, `🗺️replace-surface`, `➰replace-curve`, `📸️set-snapshot`, `🩹️patch-snapshot`.
- `🧊️brep/🧬️schema/💡️inferences/✅validation-report/`: referential-integrity inference. Pure geometric validation is in `semio_framework_3d::brep::queries::validation`.
- `🔺️mesh/🧬️schema/🧬️mutations/` (18): `🔺create-primitive`, `🕸️create-mesh`, `📍move-vertex`, `✂️delete-primitive`, `📐replace-primitive-geometry`, `🔀set-primitive-topology`, materials and textures (`🎨create-material`, `🖼️create-texture`, `📀replace-texture-bytes`, `🧲set-primitive-material`, and similar).
- `🔺️mesh/🧬️schema/💡️inferences/📦aabb/`: per-primitive AABB, keyed `"{meshId}:{primitiveId}"`. This is the only mesh inference.
- Generators (`🏭️generator/`) and oracles (`🔮️oracles/`) are test fixture producers. The mesh generator builds shapes with `manifold-3d`, exports and re-imports with `three`, and writes expected values. The brep generator uses `brepjs`/OpenCASCADE. They are not runtime code. This matches the repo rule "same output with a third-party library".

## 2. Mechanisms: how the mature plugins do it

### 2.1 Geometry derivation

| Concern | CAD | Lowpoly | Procedural |
|---|---|---|---|
| Persisted B-Rep or mesh | Yes, in stdio `brep`/`model` child documents | Yes, half-edge state in `mesh` child, per object | No; widgets persist parameters only |
| Derived geometry | Ephemeral import intermediate, view-time tessellation (`tessellate_geometry_handle`) | Session-side `mesh_workspace` cache (`LowpolyScratch`) | Flow evaluation (`FlowEvalSession`), preview as read-only tool run |
| Inference over geometry | `📦bounds` (currently degraded to `None`/`0`) | `📦bounds` (transform only, no mesh) | `🧭topology` (graph shape, not geometry) |
| Cache | None | Session cache, never serialized | Flow session retained across hops |
| Async or job | Tessellation is synchronous per call | Mesh edit is synchronous | Preview is an interactive run with progress |

Framework B-Rep tessellation is a resumable job (`semio_framework_3d::brep::queries::tessellation::TessellationJob`, `📐️brep/💡️queries/🧩tessellation/🦀️.rs`): `for_solid` (line 228), `step(body, budget)` (line 325, budgeted), `cancel` (line 307), `progress` (line 289), `into_mesh` (line 413). `tessellate_solid` (line 80) is the synchronous wrapper.

Framework mesh modeling is a resumable job too (`🥽️mesh/🛠️modeling/🦀️.rs:30` `MeshModelingJob`, with `MeshModelingStep::{Working, Done, Cancelled}`). Bevel, decimate, mirror, knife cut and about 35 other `*_job` builders exist. The synchronous `HalfedgeMesh` methods (`🥽️mesh/🦀️.rs`) wrap them with `finish_with_progress`.

### 2.2 Mutations to edits: three styles in use

1. **Intent leaves (best).** Lowpoly `🚚️move-selection` (`offset` plus `vertex_ids`), FEM `🧭️move-selection`, procedural `change-widget-input`. Geometry is derived at apply time, so the edit replays on any base. History edits change the parameter, not a result.
2. **Structural leaves.** CAD `🧊️create-brep` and `🧱️create-shape-model` attach composed children. Stdio `🧊️brep` mutations are topology edits (`create-face`, `move-vertex`). These are exact but not parametric.
3. **Result leaves (weakest).** Lowpoly mesh-edit commands (`extrude`, `bevel`, `loopCut`, ...) compute the new mesh and persist `create-mesh` with the full `mesh_state` (`🖌️session/🦀️.rs:66-96`). The history stores whole meshes per edit. The classifier keeps only one facet per commit ("first populated field wins", doc on line 66), so a commit that changes two facets silently loses one.

### 2.3 Gumball and drag

- Framework: `semio_framework_tool_machine` (`🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs`). `ToolYield` (line 19) is `Upsert{key, mutation}`, `Retract{key}`, `Commit`, `Abort`. `ToolMachine` (line 242) is a `Machine` whose effects are yields. `ToolMachineRunner` (line 287) folds yields into one open `ToolTransaction`. `GestureTool` (line 485), `drive_press` (line 755), `GestureLedger` (line 794) hold one gesture slot per window.
- Host events (`GestureHostEvent`, line 554): `blur`, `captureLost`, `retired`, `baseMoved`, `frozen`. Each aborts with zero trace.
- Used by lowpoly `🧲️transform`, FEM `🧭️gumball`, puzzle `🔄️transform` utility, procedural (via the same contract).
- Gumball math is generic: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`, `gumball_extent` (line 2564), `gumball_eye` (2568), `gumball_axis_drag_plane_normal` (2584), `gumball_project_ray_onto_axis` (2597). The 2D board variant is `TransformGumballFlags` in `🎲️board/🦀️.rs:585`.

### 2.4 Picking

- Ray picking primitives are generic: `RayMeshHit` (`📐️math/🦀️.rs:2046`: distance, triangle index, barycentric u and v, point, normal), `ray_pick_mesh_detail` (line 2055), `ray_pick_instance` (line 2028). Scene picking for 3d lives in `semio_framework_ui_scene`.
- Sub-object picking (vertex, edge, face, solid) is plugin-specific. CAD: `⚙️engine/🧲️picking/🦀️.rs` (`SpatialPickTargetKind`, line 45, visibility and hover-key logic). Lowpoly: selection in `🗂️selection` and `🛠️options/🗂️select`. FEM: `fem3d_transform_targets` maps selection ids to node or solid targets.
- Puzzle's `☑️options/🎯️select` shows that marquee and merge mode were moved into the framework `interaction` domain. This is the direction to follow.

### 2.5 Analysis and measurement

- Generic B-Rep queries (`semio_framework_3d::brep::queries`): `mass_properties` (`solid_volume`, `solid_signed_volume`, `shell_signed_volume`, `solid_surface_area`, divergence theorem, `📏mass-properties/🦀️.rs` lines 49-85), `bounding_volume` (`FaceBvh`, `EdgeBvh`, `build_face_bvh` line 491), `classification` (`point_in_solid` line 120, `point_in_face_uv` line 55), `validation` (`validate_body`, `✅validation/🦀️.rs:388`).
- Mesh analysis: `📦aabb` inference (stdio mesh subset).
- FEM analysis: `📈️analysis/` subset with benchmark fixtures, and the results window. Measurement overlays were not traced to a generic overlay API. The generic scene API is `semio_framework_ui_scene::{World3dScene, World3dSceneLane, SceneDoc}`, used by the tool-run ledger (`⏯️tool-run/🦀️.rs:3-9`).

### 2.6 Tools, utilities and windows

- `ArtifactApp` trait (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14607`). Doc block at lines 14585-14600 and 14607-14615 states the contract:
  - Actions are non-interactive and run once. Mutation actions emit operations. View and shell actions must emit zero operations.
  - Utilities are live pointer modes. Exactly one per window kind. The active utility arrives in `view_state.active_utility_id` and is never stored in the document.
  - Gestures are tool machines (press, drag, release) with an `ActionEmit::commit_transaction`.
- Hooks in the trait: `bounded_first_step_tool_proofs` (line 14643), `register_tool_job_factories` (14670), `build_tool_run_job` (14674), `mounted_job_maintenance_step`, `mounted_job_close_step`.
- Tool runs (visible process, cancel, pause): `semio_framework_tool_run`. `ToolRunJobRequest` (`⏯️tool-run/🦀️.rs:91`), `ToolRunDriver` (line 695), `ToolRunLedger` (line 713, provisional ops overlay), finalize publishes one edit. Used by puzzle fill and brush, and procedural preview-eval.
- Window-level host: `semio_framework_plugin::world3d_host` (`🔌️plugin/🦀️.rs:47884`), used by lowpoly and FEM instance previews.
- Windows, utilities and options are plugin-declared folders: `✏️editor/🎭️modes/<mode>/🪟️windows/<window>/🪛️utilities/` and `☑️options/`. Commands are `✏️editor/🎮️commands/<cmd>/🦀️.rs` with `handle(payload, doc: ArtifactView, cfg: ConfigView, ctx) -> Result<Emit<M, C>, Fault>`.

## 3. Generic framework versus plugin-specific

### 3.1 Generic framework APIs (reusable by any artifact)

| Concern | API | Location |
|---|---|---|
| Mutation leaf | `protocol::MutationKind<P, Op>` (`SEMANTICS`, `diff`, `inverse`, `label`, `target`) | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:219` |
| Mutation derive | `dsl::MutationLeaf`, `dsl::Mutations`, `semio_framework_value_derive::{ToValue,FromValue,RetireOwned}` | framework derive crates |
| Mutation descriptor | `protocol::mutation::validate_mutation_leaf_descriptor` (`binaryTag`, `invertibility`, `composition`, `outcomeClasses`) | re-exported at `spr/🎮️command/🦀️.rs:~180` |
| Inference | `protocol::Inference<P>` (`infer`) | `spr/🎮️command/🦀️.rs:32` |
| Inference spec | `InferenceSpec<P>` (`inference_schema_id`, `schema_version`, `fields`), `InferenceFieldSpec { id, reads }` | `spr/🎮️command/🦀️.rs:90, 98` |
| Tier-1 invalidation | `DiffRegions::touches() -> TouchedPaths` (prefix intersection, coverage law) | `spr/🎮️command/🦀️.rs:41, 83` |
| Dependency-cached inference | `InferredField<P>` (`plan`, `dep_input`, `compute`, `reads`), `infer_field`, `InferenceCache` (disabled by default), `DepHash` (blake3 merkle chain) | `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs:20, 84, 254` |
| Gesture or tool machine | `ToolYield`, `ToolMachine`, `ToolMachineRunner`, `GestureTool`, `drive_press`, `GestureLedger` | `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs:19, 242, 287, 485, 755, 794` |
| Scrub and typing | `ScrubMachine`, `TypingMachine`, `ChartGesture` | `🛠️tool-machine/🦀️.rs` |
| Visible-process tool run | `ToolRunDefinition`, `ToolRunDriver` (695), `ToolRunLedger` (713) | `🔌️plugin/⏯️tool-run/🦀️.rs` |
| Interactive job | `semio_framework_job::InteractiveJob`, `ToolRunRetargetableJob` | framework job crate, `⏯️tool-run/🦀️.rs:137` |
| Artifact app contract | `ArtifactApp` (`Snapshot`, `Mutation`, actions, utilities, tool job factories) | `🔌️plugin/🦀️.rs:14607` |
| Plugin commands | `ArtifactView`, `ConfigView`, `Emit`, `Fault` | `semio_framework_plugin` |
| 3d kernel: B-Rep | `semio_framework_3d::brep::{engine::{Brep, BrepKernel}, queries::{tessellation, mass_properties, bounding_volume, classification, validation}}` | `🧰️framework/🔨️modules/🧊️3d/📐️brep/` |
| 3d kernel: mesh | `semio_framework_3d::mesh::{HalfedgeMesh (263), VertexId, FaceId, EdgeId, MeshModelingJob (`🛠️modeling`)}` plus sync ops `move_vertices` (584), `bevel_edges` (672), `loop_cut` (683), `knife_cut` (688), `decimate` (751) | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` |
| 3d scene math | `RayMeshHit` (2046), `ray_pick_mesh_detail` (2055), gumball math (2559-2678) | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` |
| 3d host | `semio_framework_plugin::world3d_host` | `🔌️plugin/🦀️.rs:47884` |
| Flow graph | `semio_framework_artifact_flow_flow::Widget`, `semio_framework_os_flow::{FlowEvalSession, FlowHost}`, `ArtifactInferenceService` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/` |
| Stdio B-Rep and mesh standard | `s.stdio.semio@v1/🧊️brep`, `🔺️mesh`, with mutations and inferences | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/` |

### 3.2 Plugin-specific code

- CAD: model definitions, typologies, `spatial.interaction` JSON assets, commit-action runner, sub-object picking logic (`🧲️picking`), composition slots.
- Lowpoly: paint layers and brush, mesh-edit command set, the patch-to-mutation classifier (`semantic_mutation_for_patch`), session `mesh_workspace` cache.
- Puzzle: volume and vortex domain, fill and brush tool runs, selection kinds.
- FEM: node and solid gumball targets, load and support mutations, analysis solves.
- Procedural: widget kinds (`Generation3dMutation` set), mesh-edit-to-operator splice.

### 3.3 Observations

- The CAD picking module is titled "target-neutral" and lives in the plugin. Its ray math is correctly in the framework. The module could be promoted if a second spatial plugin needs it.
- Lowpoly's `🚚️move-selection` and FEM's `🧭️move-selection` are the same semantic leaf with different emoji. Pick one name and emoji for the shared concept, per the taxonomy rule.
- The stdio generators (`🏭️generator/`) and `🔮️oracles/` are third-party oracle producers, not runtime code. Good pattern for the procedural tests.

## 4. Findings

1. **CAD bounds is degraded.** `📦bounds` returns `None` and `0` after the composition refactor (`💡️inferences/📦bounds/🦀️.rs:33, 38`). CAD has no working `brep.mesh` inference; tessellation is a view-time call (`📐️geometry/🦀️.rs:515`). The task brief assumed a `brep.mesh` inference exists. It does not.
2. **Lowpoly mesh-edit persists results, not intent.** `extrude`, `bevel`, `loopCut` and others persist `create-mesh` with the full `mesh_state` (`🖌️session/🦀️.rs:66`). Only gumball `move-selection`, `rotate-selection` and `scale-selection` are intent leaves. The classifier can drop a multi-facet change.
3. **Procedural already has the right model for widgets.** Parametric `create-widget` and `change-widget-input` with absolute values, plus an edit-mesh command that creates a widget with default params and then absolute input leaves (`🥽️edit-mesh-selection`). The rebuild should keep this shape and avoid result-patch leaves.
4. **The gumball contract is generic and tested across four plugins.** Tool machine plus net-leaf commit plus transient preview is used by lowpoly, FEM, puzzle and the 2D board. A new procedural gumball should use `semio_framework_tool_machine` directly, not a bespoke statechart.
5. **Long operations are resumable jobs with progress and cancel.** `TessellationJob` and `MeshModelingJob` (framework) plus `ToolRunDriver` (framework) cover the "interaction-friendly" rule. Use them for procedural evaluation and tessellation.
6. **Inferences are whole-snapshot scalars today.** None of the plugins use `InferredField` with `DepHash` for geometry. The `dependency-aware cache` exists in the framework but is off by default (`InferenceCacheConfig::default`, `enabled: false`) and no 3d plugin uses it. For procedural graphs (per-widget DAG), it is the natural fit.

## 5. Recommendation for the procedural 3d artifact

Proposed structure, based on the most mature parts (lowpoly intent leaves, FEM tool gumball, puzzle tool runs, procedural widget mutations, stdio oracles).

**Persisted document (event-sourced, schema-first).**
- Keep the flow `Widget` and synapse collections as the only persisted geometry description. Widgets carry typed inputs as absolute literals. No mesh or B-Rep is persisted in the generation snapshot.
- Each widget kind is declared by a JSON schema with `x-semio-ui` (label, description, group, order, en and de), the way CAD's `🔣️.json` mutation schemas do.
- Use a model-definition descriptor (CAD `spatial.modelDefinition` pattern) to list the widget kinds, typologies and interactions that a generation3d document may use.

**Mutations (`🧬️mutations/`, one leaf per directory, `MutationKind` plus descriptor `🔣️.json`).**
- Keep: `create-widget`, `update-widget`, `delete-widget`, `connect-synapse`, `disconnect-synapse`, `change-widget-input`, `change-slider-value`, `move-nodes`.
- Add intent leaves for gumball moves on components: `move-selection`-style (selection ids plus offset, pivot), so that history edits the intent, not the derived mesh. Do not add result-patch leaves like lowpoly's `create-mesh` with full state.
- Each composite user action (for example "edit mesh selection") is one edit that creates the widget with default params, then sets each chosen input as an absolute `change-widget-input` leaf. Reuse the procedural `edit-mesh-selection` shape.

**Inferences (`💡️inferences/`).**
- `topology` (keep, already present).
- A geometry inference (`s.procedural.generation3d.geometry`) that evaluates the widget DAG through `InferredField<P>`: `plan` is the topological order of widgets, `dep_input` is the widget's own inputs plus its incoming synapse edge parameters, `compute` calls the flow operator. Declare `reads` per field (`InferenceFieldSpec`) and implement `DiffRegions::touches()` for the mutation diff, so the tier-1 gate can skip unchanged widgets. Enable the cache (`InferenceCacheConfig { enabled: true, persistence: None }`) per host, not per artifact.
- Mesh and B-Rep tessellation is a view-time derivation, not an inference. Use `TessellationJob` or `MeshTessellationJob` in a transient job, not persisted. Mirror the lowpoly `LowpolyScratch` and FEM gumball transient split.

**Interaction.**
- Gumball and drag: `semio_framework_tool_machine` (`ToolMachineRunner` plus `ToolYield::upsert`/`commit`). One press is one transaction. Abort leaves zero trace.
- Evaluation with visible progress and cancel: `ToolRunDefinition` plus `ToolRunDriver` (puzzle fill pattern). Preview evaluation stays read-only (procedural `previewEval` pattern).
- Each widget edit that changes geometry must go through a mutation, never directly through the cache.

**Testing (repo rules).**
- Language-agnostic fixture per feature under `🧫️fixtures/`, and the same expected output produced by a third-party oracle (`🔮️oracles/`, as in stdio `🏭️generator/` with `manifold-3d`, `three`, or `brepjs`). Generation and execution are separate commands (stdio pattern).
- Unit tests per mutation leaf (`🧪️tests/🔬️unit`), and a mounted test per tool run (`🧪️tests/🔬️app-fixture`).

## 6. Verification scope

- Verified by reading: file listings, mutation descriptors (`🔣️.json`) for CAD, stdio brep and stdio mesh, lowpoly mutation list, CAD bounds inference source, lowpoly gumball and mesh-edit source, lowpoly semantic classifier source, FEM gumball header and helpers, puzzle utility and fill-tool headers, framework tool-machine, tool-run, inference, mutation and tessellation headers and signatures, procedural widget mutation headers.
- Not verified by reading the full body: block 3d editing, CAD `🎬️actions` commit runner, lowpoly paint, FEM analysis overlays, framework `world3d_host` internals, procedural flow evaluation internals (`FlowEvalSession`).
- No file was edited, no build was run, no git command was run.
