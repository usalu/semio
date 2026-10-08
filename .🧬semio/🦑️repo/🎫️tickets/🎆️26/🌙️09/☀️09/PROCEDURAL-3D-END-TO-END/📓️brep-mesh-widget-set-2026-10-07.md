# 🧊️ Procedural 3d — B-Rep and Mesh Widget Set on Mutations and Inferences

Session 6 (2026-10-07, Opus 5.5 coordinator, Sonnet 5.5 execution lanes, Haiku 5.5 audits).

## Request

> Procedural 3d is still extremely ad hoc. Make it feature-complete and usable for the end user: a full-blown 3d B-Rep and mesh widget set for a complete creating, editing and analysis experience for arbitrary shapes. Only use the mutation and inference systems, not separate modules.

## Evidence (audit reports, this folder)

| Report | Topic |
|---|---|
| `📓️wsx-mutation-system-2026-10-07.md` | framework mutation lifecycle, 20 generation3d leaves, 40 commands classified (6 snapshot-diff routes, mesh-as-JSON-text leaves, `set-contributions` registry side channel) |
| `📓️wsx-inference-system-2026-10-07.md` | `Inference<P>` + `InferredField`/`DepHash` cache (inactive in production), `ArtifactInferenceService` (budgets, cancel, resume, progress); generation3d's only inference `topology` is descriptor-only; 8 geometry bypasses |
| `📓️wsx-generation3d-pipeline-2026-10-07.md` | end-to-end chain today: `change-widget-input` → UI-hop `flowEvalTick` chain → neural engine → `ContributedExtensionStub` → `ExtensionInvocation evaluate/tessellate` to the separate `flow-extension-brep.sxt` guest → JSON/base64 → renderer; ad-hoc inventory; end-user gaps |
| `📓️wsx-kernel-capabilities-2026-10-07.md` | 98-method `BrepKernel`, 93 B-Rep + 42 mesh flow nodes, maturity gaps, only fuse/cut/intersect are jobs, missing end-user capabilities |
| `📓️wsx-reference-plugins-2026-10-07.md` | CAD/lowpoly/FEM/puzzle patterns: intent leaves, tool-machine gumball, tool runs, stdio oracles |
| `📓️wsx-ui-viewport-2026-10-07.md` | UI component union, World3d host picking/gumball/push-pull, missing snapping/dimensions/section/heatmap, wgpu face ids zeroed |
| `📓️wsx-build-test-launch-2026-10-07.md` | crate/nx names, test runners, oracles, launch.json naming, pitfalls |

## Diagnosis

Procedural 3d is a flow graph whose geometry never touches the artifact's own schema families:

1. **Separate module.** Every geometry operator is a `ContributedExtensionStub` answered by the `flow-extension-brep` guest through `ExtensionInvocation`s, JSON strings and a process-wide extension registry written by `set-contributions` (outside history).
2. **No inference.** Geometry, tessellation, bounds, selection sets and analysis are recomputed by ad-hoc tick hops, JSON re-parsing and renderer code. `Generation3dInference` holds only an unused `topology`.
3. **Mutation intent lost.** Six commands edit a scratch `FlowHost` and diff whole snapshots into leaves; mesh edits write a whole serialized mesh as a text `change-widget-input`.
4. **End-user gaps.** No catalogue-driven creation flow, operator labels hand-written in four places, analysis only as raw inspector values, B-Rep sub-element editing reaches only fillet/chamfer/shell, no measure tool, no typed controls for vectors/planes/selections, English-only faults.

## Architecture Decisions

### AD1 — Document stays the flow widget graph, widget kinds become generation3d schema

- The persisted document stays `Generation3dSnapshot { hostSnapshot: FlowHostSnapshot, generation }`. A geometry widget is `Widget::Neuron { neuronKind, params, inputPorts, outputPorts, preview }`.
- Every geometry widget kind is declared schema-first by generation3d in `🧬️schema/🗂️catalogue/` (one `🔣️.json` per category plus the catalogue meta-schema; Rust and TypeScript typed loaders). A kind declares: `id`, `category`, `emoji`, EN/DE `label` and `description`, typed input ports (`number|integer|angle|length|boolean|text|enum|vector|point|plane|shape|shapes|mesh|selection(face|edge|vertex)|list<…>` with `default`, `min`, `max`, `step`, `unit`, `enum` options, `optional`), typed output ports, `quality` (the kernel's `OpQuality` or the mesh fidelity), and `interaction` hints (which sub-element pick seeds which selection port, which gumball motion maps to which port).
- Kind ids are domain-scoped and handcrafted: `brep.<group>.<name>` for B-Rep kinds, `mesh.<group>.<name>` for mesh kinds (no more `brep.mesh.*`), `math.<name>` for the scalar/vector core, `analysis.<name>` for measuring kinds. All examples are rewritten by hand to the new ids (no migration code).
- The catalogue is the single source for the palette, inspector controls, history labels, a11y names, docs and validation. No operator label is hand-written anywhere else.

### AD2 — Geometry is the generation3d `geometry` inference

- `🧬️schema/💡️inferences/📐️geometry/` implements `InferredField<Generation3dSnapshot>`: `Key` = widget id, `plan` = topological order of widgets over synapses, `dep_input` = the widget's kind + params + incoming synapse wiring (honesty contract), `compute` = dispatch on the catalogue kind to a pure compute function over the parents' already-computed values. `Value` = `WidgetEvaluation { outputs: BTreeMap<port, GeometryValue>, fault: Option<WidgetFault>, quality }`.
- `Generation3dInference { topology, geometry, analysis }`; `InferenceSpec::fields()` declares honest `reads`.
- `Generation3dDiff` implements `DiffRegions` so the tier-1 gate and the `DepHash` cache skip every widget whose dependency chain did not change. The instance owns an enabled `InferenceCache` (budgeted).
- The expensive path is an executable `ArtifactInferenceService` `s.procedural.generation3d.geometry` (progress unit `widget-step`, work-unit budget, cancellation id, resumable state, cache modes), registered through `.inference_services([...])` on the generation3d artifact. Kernel jobs (booleans, tessellation, mesh modeling, and the newly job-backed B-Rep features) are stepped inside it.
- Procedural 3d never uses `neural_engine` evaluation, `ContributedExtensionStub`, `ExtensionInvocation`, `flow_extension_sdk` or `set-contributions`. The `flow-extension-brep` crate stays only for the flow plugin and is not a dependency of generation3d.

### AD3 — Kernel-owned geometry values

- `semio_framework_3d::brep` gains an owned, `ToValue`/`FromValue` standalone shape value (`ShapeValue`: a minimal `Body` plus root entity references and kind, persistent labels preserved) with `Brep::import_shape` and `Brep::export_shape`. Mesh values are `HalfedgeMesh` (typed, never JSON text). `GeometryValue` (generation3d) = `Number | Integer | Boolean | Text | Vector | Point | Plane | Shape | Mesh | Selection | List`.
- Sub-element references are B-Rep `PersistentLabel`s (stable for an unchanged upstream chain) and mesh component indices validated against the inferred topology of the referenced widget output.

### AD4 — Analysis is the generation3d `analysis` inference

- `💡️inferences/📊️analysis/` (`InferredField` keyed by widget id, depends on `geometry`): mass properties (volume, area, centroid, inertia tensor, principal moments and axes), bounding box, topology counts, Euler characteristic, validity report, closed/manifold/watertight verdicts, per-face area/normal/kind tables (for picking and inspection), mesh quality (boundary and non-manifold edges, degenerate faces, edge length and aspect ratio ranges). Catalogue `analysis.*` kinds (distance, angle, area-of-selection, interference) produce document-addressable measurements.

### AD5 — Preview reads the inference

- The preview window renders `geometry` outputs of widgets whose `preview` flag is set (or the selected widget). Tessellation is view-time from inference values (`TessellationJob` for shapes, triangulation for meshes) retained per window keyed by the widget's `DepHash`. The framework tool run (`previewEval`) drives the geometry inference service with progress and cancel. All `flowEval*`/`flowTessellate*`/`setContributions` commands and the extension request plumbing are deleted.

### AD6 — Every edit is a declared mutation emitted directly

- Commands emit `Generation3dMutation` leaves directly; the scratch-`FlowHost` snapshot-diff route is deleted. Composite user actions (insert a feature on a picked face, boolean the selection, delete with wires) are one transaction of leaves. New leaves only where intent is missing (e.g. `duplicate-widget`, `reorder-widgets`), each with descriptor, payload schema, diff, inverse, TS mirror, fixture quintet and tests. The descriptor-only `select-generation`/`change-generation-preview` folders are resolved (implemented or removed).

### AD7 — End-user widget set (UI)

- **Create**: catalogue-driven palette (categories, search, keyboard, EN/DE, icons/emoji) in the existing `🛍️catalogue` panel and a create rail in the preview window; adding a kind wires it to the current selection when the kind's ports accept it, selects the new widget and opens the inspector.
- **Edit**: typed inspector controls from port schemas (number with unit and range, angle, vec3/point, plane, enum, boolean, text, selection with pick button), contextual actions on picked faces/edges/vertices/shapes (fillet, chamfer, shell, draft, offset, push-pull/extrude face, split, booleans; mesh extrude, inset, bevel, subdivide, loop cut, knife, delete, weld, flip, triangulate, smooth), gumball mapped to transform widgets via catalogue interaction hints, undo/redo.
- **Analyse**: analysis panel (properties, validity diagnostics, mesh quality), measure utility (distance, angle, area with in-scene labels), selection filter (shape/face/edge/vertex), display options.
- Everything accessible (roles, keyboard reachability, live regions) and localized EN/DE from the catalogue.

## Delivery Waves

Wave 1 (parallel, disjoint): K kernel shape values · J kernel jobs for expensive B-Rep features · A kernel analysis queries · C catalogue schema · M mutation hygiene · V viewport primitives.
Wave 2: G geometry inference engine + service; O1–O4 widget computes per category (contract from G); Y analysis inference.
Wave 3: P preview rewire and extension-path removal; U1 palette, U2 inspector, U3 analysis panel and measure utility, U4 contextual edit actions; X examples rewritten; T language-agnostic fixtures + third-party oracles (three.js, manifold-3d); L launch.json entries; B browser verification (React, then wgpu).

## Lane Log

(appended by the coordinator as lanes land)
- 2026-10-07 — Wave 1 dispatched (Sonnet): K shape values, J operation jobs, A analysis queries (framework `🧊️3d`), C catalogue (`🧬️schema/🗂️catalogue`), M mutation hygiene + `DiffRegions`, V viewport primitives (annotations, heatmap, pick filter, section, wgpu face ids). Haiku audits for Wave 2/3: preview removal map, editor UI internals, inference execution path, example assets.
- Preview removal map landed (`📓️wsx-preview-removal-map-2026-10-07.md`). Extra consumer found: the playbook procedural extension (`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/`) links `neural_engine` and the brep/math flow extensions directly — decision: it must evaluate through the generation3d geometry inference too (lane P scope). The `previewEval` status contract (`world3dComputeStatusV1`: phase, phaseLabel, hint, progress, inFlight, ratio, cancellable, cancelAction/cancelArgs) is kept; only its source changes.
- Inference execution path landed (`📓️wsx-inference-execution-path-2026-10-07.md`). Decisions for lane G: (1) geometry runs inside the existing tool-run run job holding the instance operation owner, calling the generation3d contextual `ArtifactInferenceService` via `infer_with_context` each step (fuel → work units, `PreviewReady` progress, `CheckpointReady` resume, abort → cancel); (2) framework gap G1: add a stepped `InferredField` driver (`infer_field_step` with cursor + fuel, `infer_field` = unbounded fuel, fallible compute) in `💡️inference`; (3) framework gap: the `InferenceCache` text-encodes values with private get/insert — replace with a typed in-memory cache (cache-transparency law unchanged) so shape/mesh values are not re-encoded per hit; the cache lives in `Generation3dInstanceOperationOwner`; (4) fix G3: the interactive `semio.infer` result must forward `complete`/`validity`/`quality` faithfully; (5) G2 (MCP parity: contextual services through the gateway with the instance owner) is in scope for full inference-system usage; (6) renders fall back to the last retained evaluation when the owner is busy; any kernel call above the 8 ms step ceiling must be a nested interactive job (lane J jobs).
- Editor UI internals (`📓️wsx-editor-ui-internals-2026-10-07.md`) and example assets (`📓️wsx-example-assets-2026-10-07.md`) landed. Lane X scope additions: `🗣️.dsl.semio` is the only real example source — the `🎒️.pack`/`📡️.spr`/`🔧️.op` files are zero-payload placeholders (remove them or make them real derived encodings, no placeholders); unify the ~9 example enumeration sites into one catalogue-like example registry; mesh-workbench lacks fixture/TS/test/budget entries; untranslated German example labels; stale cached preview value in sphere-cut; mislabelled "Torus Major Radius" slider; composition manifest pins a test file sha256. Lane U hazards: gumball shown for every selection and fallthrough to translate (gate per utility), whole-list replacement in `window_kind_utilities`/`action_refs`, post-hoc builder calls no-op on undeclared ids, 14 command folders missing from taxonomy.
- Lane K landed (`📓️wsx-lane-k-shape-values-2026-10-07.md`): `ShapeValue { root, body }`, `Brep::export_shape`/`import_shape`/`import_shape_mapped`, `ShapeValue::{kind,label,components,content_hash,check,tessellate,tessellate_job}`, `HalfedgeMesh::{positions,polygons,edge_ids,vertex_normal}`; boolean `group_shells` made deterministic (BTree). 9/9 shape tests + 366 filtered regressions + TS three oracle green. Open: `handle_for_label` for wire/compound/curve/surface labels.
- Lane G dispatched (geometry inference engine + framework stepped driver + typed cache + service + G3 fix). Compute lanes O1–O4 start when G publishes `📓️wsx-lane-g-contract-2026-10-07.md`.
- Lane V landed (`📓️wsx-lane-v-viewport-2026-10-07.md`): `World3dScene::{with_annotations, with_scalar_field, with_pick_filter, with_section, with_highlight, with_modelling_options}`, `World3dAnnotation::{dimension,angle,marker,leader}`, `World3dScalarField`, React `World3dHost/📏️modelling`; 184 Rust + 91 TS + 38 mounted modelling checks green; wgpu: sub-element GLB ids, pick filter, heatmap colours done. Lane V2 dispatched for wgpu annotation/legend overlays, section cap, highlight tokens, React reading GLB ids.
- Lane G contract published (`📓️wsx-lane-g-contract-2026-10-07.md`). Compute lanes dispatched: O1 math-arithmetic/vector/list + analysis-measure/check, O2 brep curve/surface/solid/boolean, O3 brep feature/transform/intersect/evaluate/topology/interchange, O4 all mesh categories. Live: G, J, A, C, M, V2, O1–O4 (11 Sonnet lanes).
- Lane A landed (`📓️wsx-lane-a-analysis-queries-2026-10-07.md`): `brep::queries::analysis::{analyze_shape, mass_properties, bounding_box, topology_counts, validity, manifold_report, surface_differential, face_differential, face_table, edge_table, shape_distance, point_distance}` + `Brep::*_sync` twins, `HalfedgeMesh::quality_report`, `analyze_polygon_soup`, `semio_framework_3d::inertia`; fixed the adaptive mass integrator (centroid/inertia of curved solids were wrong), `validate` now per shape, compat watertight stub deleted. 104 Rust + 14 TS oracle (three, manifold-3d) green. Found a boolean bug (sphere∩box through poles) → lane KB1 dispatched (boolean matrix vs manifold-3d, validator hardening, exact curved section/split, no hull fallback).
