# Procedural 3D (generation3d): end-to-end map and ad-hoc inventory

Date of audit: 2026-10-07. Mode: read-only static audit (grep and file reads). No build, no test run, no runtime console check. Every claim below comes from the cited file and line in the working tree at audit time. Anything not confirmed by reading code is listed in section 7.

## 0. Shorthand and scope

| Short | Path prefix |
|---|---|
| G3D | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/` |
| S | `G3D/🏅️standards/🔖️1/🪆️subsets/✳️any/` |
| CORE | `✏️s/🔌️plugins/🌀️procedural/🫀️core/` |
| G2D | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/` |
| FLOW | `✏️s/🔌️plugins/🌊️flow/` |
| BREP | `FLOW/🧩️extensions/📐️brep/` (B-Rep extension, wasm guest, 2218 lines) |
| MESH | `BREP/🥽️mesh/` (polygon-mesh operators, 977 lines Rust, 388 lines TS) |
| OSFLOW | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/` (framework flow host, registry, mesh codec) |
| KERNEL | `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/` (B-Rep kernel trait and engine) |

Sizes: G3D top `🦀️.rs` 875 lines; host `🔨️modules/🏠️host/🦀️.rs` 3538; editor `S/✏️editor/🦀️.rs` 3582; viewer `S/👁️viewer/🦀️.rs` 1787; preview-eval `S/🧵️preview-eval/🦀️.rs` 1475; io `S/🚪️io/🦀️.rs` 1164; schema `S/🧬️schema/🦀️.rs` 409; mutations `S/🧬️schema/🧬️mutations/🦀️.rs` 611; inference topology 81.

## 1. Document model

**Persisted document.** `Generation3dSnapshot { host_snapshot: FlowHostSnapshot, generation: GenerationPlayRoot }` (`S/🧬️schema/📸️snapshot/🦀️.rs`). The live artifact `Generation3dArtifact` has the same two fields (`S/🧬️schema/🦀️.rs` L23-33) and converts with `to_snapshot`/`from_snapshot`/`set_snapshot` (L58-76). Default is the three-widget flow demo from the framework, not a generation3d asset (`S/🧬️schema/🦀️.rs` L134-140).

**FlowHostSnapshot** (framework, `OSFLOW/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` L263-270): `schema`, `camera: CameraJson`, `widgets: Vec<Widget>`, `synapses: Vec<SynapseSpec>`, `layout: OrderedMap<WidgetLayout>`. Several of these types derive both `serde` and the value-contract `ToValue`/`FromValue` (e.g. L90), so two serializations exist for the same shape.

**Widget = flow node (a tagged enum, L190).** Variants: `Neuron { id, neuron_kind: String, params: Dictionary, input_ports, output_ports, preview }` (an operator), `InputSlider { value }`, `InputNote { text }`, `InputImage { src }`, `Variable { name }`, `OutputPreview`, `OutputAction`, `OutputExport`, `Cluster`. Sources and sinks are widgets too; there is no separate "shape" concept.

**Wires.** `SynapseSpec { id, from, to, from_port, to_port }`. Endpoints are plain strings `"nodeId@port"`, split by `split_endpoint` (`S/🧬️schema/🦀️.rs` L264) and by `rsplit_once('@')` in the flow window (`S/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` L210-213).

**Operators are identified by a string.** `Widget::Neuron.neuron_kind` holds e.g. `brep.prim3d.box`. The registry maps the string to `OperatorInfo` with `inputs: Vec<ChannelSpec>` (name, value types, default, cardinality). Lookup is `flow_neuron_kind_info_map()` (`OSFLOW/🗿️artifacts/🌊️flow/🗂️catalogue/🦀️.rs` L101).

**Where operator definitions and parameter schemas live:**
- B-Rep: `BREP/🦀️.rs`. `register()` at L994 registers the nodes. Example: `brep.prim3d.box` at L1031-1036 with `number_channel("width"|"depth"|"height", …, 1.0)` and `out_solid("BoxSolid")`. `NODE_KERNEL_METHOD` (L24-~120) maps each node id to a kernel method name by hand. `INTENTIONALLY_UNEXPOSED` (L125) lists kernel methods with no node. About 100 `brep.*` ids are registered (booleans `brep.bool.*` L1368, fillet `brep.solid.fillet` L1508, etc.). Operator bodies are hand-written structs with a macro (`geo_operation!`, L140-160).
- Packaged copies: `BREP/🔣️.json` (125 lines) and `BREP/🛂️.descriptor.semio` (99 lines, about 145 KB). No generator was found in the tree, so these look hand-maintained (section 7).
- Mesh: `MESH/🦀️.rs` `register_mesh` at L821. A hand-written `definitions` table (L823-830) gives id, English name, group and default parameters. Per-operator inputs, outputs and English summaries are `match` arms (about L840-960).
- Parameter values are stored per widget in `params: Dictionary` (neural value dictionary) and edited through the `change-widget-input` leaf (section 2).

**Generations (form presets).** `GenerationPlayRoot` / `FormGeneration { id, name, values: map(widgetId → value) }` (framework playbook). Only four widget kinds accept generation values: slider, note, image, variable (`generation_host_snapshot_for`, `S/🧬️schema/🦀️.rs` L194-236). Operator params cannot be driven by a generation.

**Config (not in the document).** `Generation3dConfig` (`S/✏️editor/🎚️config/🦀️.rs` L97+): `lod_mode: String`, `show_mode: String`, `camera` (node canvas), `preview_camera`, `sun_json: String` (documented as "JSON-encoded WorldSunConfig"), `selected_generation_id`. Config is written through `Generation3dConfigMutation`.

**Other state.** Transient gesture state (`S/✏️editor/🫧️transient`), selection (`S/✏️editor/🎯️selection`), presence (`S/✏️editor/👥️presence`). The retained evaluation session is `Generation3dInstanceOperationOwner.eval_session: FlowEvalSession` (`S/✏️editor/🦀️.rs` ~L179-200), outside the document.

**Schema family.** Snapshot, diff, mutations and inferences. Each facet is a set of leaves (`rust`, `typescript`, `graphql`, `json_schema`, `proto`) mounted with `include_str!` in `S/🧬️schema/🦀️.rs` L78-122 and `G3D/🦀️.rs` L18. Mutations are a 20-variant Rust enum (`S/🧬️schema/🧬️mutations/🦀️.rs` L224-240, count also hard-coded in host L36).

## 2. Evaluation: one input change to pixels

Worked example: the user types a new box width in the inspector.

1. **Inspector to command.** The inspector dispatches `setWidgetInput`. Its payload is `SetWidgetInput { widget_id, channel, value: String, component, operation, index, destination, facet, path }` (`S/✏️editor/🎮️commands/🎚️set-widget-input/🦀️.rs`). `handle` (L378) calls `input_leaf` (L93). `input_leaf` checks the port against `flow_neuron_kind_info_map()`, rejects connected inputs, and parses the string with `edit_input_value` (L69, `parse::<f64>`). It returns `Emit::mutations([ChangeWidgetInput])`.
2. **Mutation applied.** The editor's `prepare_generation3d_artifact` (`S/✏️editor/🦀️.rs` ~L1667) prepares the store write. `apply_generation3d_mutation` (`S/🧬️schema/🧬️mutations/🦀️.rs` L574) runs `protocol::Mutation::diff` and `MutationDiff::apply` from the framework kernel. The history label comes from `S/✏️editor/🗣️terminology/🦀️.rs`.
3. **Preview marked owed.** `Generation3dInstanceOperationOwner::owe_attached_previews_for_mutations` (`S/✏️editor/🦀️.rs` ~L217) calls `preview_eval::owe_attached_previews_for_mutations` (`S/🧵️preview-eval/🦀️.rs` L252). `applied_document_edits_digest` (L205, hand-rolled FNV) decides whether the document moved. `run_link.wake()` restarts the run.
4. **Eval loop of UI hops.** The preview run hands the host `Effect::DispatchAction { action: "flowEvalTick", req: 103 }` (`preview-eval` L129-160, `hop_effect` L153). Each dependency level is one dispatched action (comment at `evaluate_tick`, L~947-975). The run is therefore a chain of UI-dispatched commands, not a worker loop.
5. **Flow tick.** The `flowEvalTick` command (`S/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`, which uses `generation_host_snapshot_for` when a generation is selected) calls `evaluate_tick` (`preview-eval` L946-1027). That builds `flow_host_with_session` (`OSFLOW/🖥️host/🦀️.rs` L4634), calls `session.tick` and evaluates the DAG with the neural engine.
6. **Extension stub.** Any operator whose implementation lives in an extension is a `ContributedExtensionStub` (`OSFLOW/🗿️artifacts/🌊️flow/📔️registry/🦀️.rs` L134-150). It returns `EvalError::PendingExtension { extension_id, operator_id, node_hash }`. The host queues it (`take_pending_extension_evals`, `OSFLOW/🖥️host/🦀️.rs` L1481).
7. **Extension request.** `evaluate_tick` (L~960-1000) builds one `ExtensionInvocation(extension_id, "evaluate", request_json, "flowEvalResolve")`. `request_json` is a JSON string with `operatorId`, `inputJson` (a string), `dependencyJson` (a string), `operatorVersion` (`"registry:N;geometry:1;policy:1"`), `nodeHash`, `resume`, `budget` (8), `wallMicros` (2 000 000), `windowId`, `windowKindId`, `extensionId`. The address comes from `flow_extension_invocation_address` (`OSFLOW/…/📔️registry/🦀️.rs` L603), which maps flow id `brep` to plugin id `flow-extension-brep`.
8. **Wasm door.** The host emits `Effect::InvokeExtension`. The guest is the component `flow-extension-brep.sxt`, built by `BREP/📦️packages/🦀️rust/📜️script.ts` (`package`). Its `BrepExtensionResources::invoke` (`BREP/🦀️.rs` ~L2152-2178) dispatches on capability strings `evaluate`, `tessellate`, `evaluateCancel`, `tessellateCancel`. Request bodies are raw JSON bytes read by string key (`handle`, `tolerance`, `budget`, `wallMicros`, `chunk`). `evaluate` goes through the artifact inference service `GEOMETRY_ARTIFACT_KIND = "s.flow.flow"` with schema `s.flow-extension-brep.geometry` (`BREP/💡️inferences/📐️geometry/🦀️.rs` L6-7).
9. **Kernel.** The registered operator (for example `BoxPrim`, `Fuse`, `FilletEdges`) runs against `SessionCapture`. It calls the `BrepKernel` trait (`KERNEL/🦀️.rs` L125+, `box_prim` L128). The kernel is in-house Rust (no OCCT in the path), and it returns geometry as `GeometryHandle(pub String)` (L71). Mesh operators run in `MESH/🦀️.rs` `MeshOperation` (L596, `prepare` L690). They decode the polygon mesh from a JSON text field (`decode_mesh` L83, `encode_mesh` L115).
10. **Result back.** `flowEvalResolve` carries `FlowEvalResolve { window_id, window_kind_id, node_hash, output_json: String, extension_id, ok, fault_code, fault_message }` (`preview-eval` ~L67-88). `resolve_eval` (L1087) stores the answer and the tick continues.
11. **Preview tessellation.** For each preview channel the tick parks a `tessellate` invocation (`preview_tessellate_invocations`, L839-884). The guest runs `tessellate_step_envelope_json` with a 24-unit budget (`BREP/🦀️.rs` `TESSELLATE_STEP_BUDGET`). The kernel tessellates, and the mesh is encoded as a base64 "pack" body by `OSFLOW/🎒️mesh/🦀️.rs` (uses the external `base64` crate). The answer comes back as `FlowTessellateResolve { output_json: String }` and is applied by `resolve_tessellate` (L1159).
12. **Mesh to renderer.** `session_preview_mesh` / `decode_preview_mesh_pack` (L626-640) decode the pack into typed arrays. The edit preview calls `preview_payload` (`S/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` L312-363), which yields `meshes_json: String` and `instances_json: String`. `preview_fit_json` (L576) parses `meshes_json` again to get bounds (`preview_payload_bounds`, L505). `preview_selection_json` and `preview_status_json` are also JSON strings.
13. **Render.** `S/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` `render` (L89-111) wraps everything into `world3d_scene(camera_json, meshes_json, instances_json, selection_json, sun)` inside `accessible_scene_surface`. The framework turns that into a wgpu scene. LOD maps to tolerance in `preview_tolerance` (`preview-eval` L318-324): coarse 0.15, medium 0.05 (default), fine 0.02.

Variants:
- **Generate mode** (`S/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` L97): `generation_host_snapshot_for` patches a clone of the document in memory. No mutation is written. The same tick then runs.
- **Viewer** (`S/👁️viewer/`): same tick and mesh path, no document mutations. It writes config only.
- **Contributions**: `setContributions` writes the process-wide flow extension registry (`OSFLOW/…/📔️registry`, `flow_extension_state()`). `may_rearm` (`preview-eval` L174) gates the loop on that registry.

## 3. Windows, panels, modes and commands (what a user can do)

**Editor app `procedural3d-play`** (app id `GENERATION3D_EDITOR_APP_ID = "s.procedural.generation3d@1/*#editor"`, `S/✏️editor/🦀️.rs` L95-98). It has 42 command rows in `app_commands!` (L122-168). Each command has a camelCase tool id and a kebab-case DSL keyword.

| Mode (layout) | Window (body key) | User can |
|---|---|---|
| Edit (`edit`, row split 68/32, `S/✏️editor/🎭️modes/✏️edit/🦀️.rs` L24-35; window titles "Flow", "Preview" are hard-coded English) | Flow (`procedural-main`, `procedural.play.main`), `🕸️flow/🦀️.rs` | Add operator nodes (`addWidget`); move nodes (`nodeGraphEdit` move, `move-nodes`); connect, disconnect, insert port and delete (`nodeGraphEdit`, `deleteSelection`, `removeWidget`); reorganize layout (`reorganize`); keyboard traversal and activate (`selectNext/Previous/Upstream/DownstreamNode`, `activateSelection`); pan and zoom (`nodeGraphViewport`); find items (`graph_find_items`); see per-node status queued, computing, error, blocked; inline slider overlay (`nodeGraphEdit` `setSlider`; the inspector's number field uses `patchFlowWidgets`) |
| Edit | Preview (`procedural-preview`, `procedural.play.preview`), `🪟️windows/👁️preview/🦀️.rs` | Camera (`setCamera`, stored in config); show mode shaded, shaded+edges, wireframe, points (`setShowMode`, `cycleShowMode`); LOD coarse, medium, fine (`setLodMode`, `cycleLodMode`); sun toggle, azimuth, elevation, intensity; gumball translate, rotate, scale on shape instances (`translateSelection`, `rotateSelection`, `scaleSelection`) and on mesh components; component selection of vertex, edge, face on indexed meshes (`setInteractionGranularity` is framework-injected); mesh operations on selection (`editMeshSelection`) and one-face knife cut (`knifeMeshSelection`); evaluation status pill with cancel (`flowTessellateCancelResolve`, `toolRunAbort`) |
| Generate (`generate`, layout 22/43/35, `S/✏️editor/🎭️modes/🧬️generate/🦀️.rs`) | Generations (`generations`) | Add, remove, rename and select generations (`addGeneration`, `removeGeneration`, `renameGeneration`, `selectGeneration`) |
| Generate | Form (`form`) | Edit the selected generation's values for sliders, notes, images, variables (`updateGenerationValues`) |
| Generate | Generate preview (`generate-preview`) | See the selected generation evaluated in memory (no document change) |
| Inspector panels (`S/✏️editor/📌️panels/`) | Catalogue (`catalogue`), Artifact (`artifact`), Inspection (`inspection`) | Catalogue: browse all registered operators, add by activation or drag (`addWidget`). Artifact: document-level info. Inspection: edit selected node's inputs (number, text, boolean, point and vector coordinates, list add/set/remove/move via `setWidgetInput`); edit mesh source JSON by path (`facet: meshSource`); import texture; see reflected outputs as a tree; export buttons for OutputExport widgets |
| Document (palette/menu) | — | Import a file (`importDocumentRequest` file picker, `importDocument`); export as STL, OBJ, PLY, glTF, LAS, DWG or Semio text (`exportDocument`); switch among 9 bundled examples (`setActiveExample`) |
| Config | — | Install flow extension pages (`setContributions`, process-wide registry) |

**Viewer app `procedural3d-view`** (`S/👁️viewer/🦀️.rs`): one mode "View" with one window "Preview" (`procedural.view.preview`, `S/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`). It has 15 command modules: show mode, LOD, camera, sun (4), toggle sun, set active example, export document, set contributions, and the flow eval hop tools. No document mutations.

## 4. Ad-hoc inventory

Each item gives what it is, where it is, and why it matters. "Bypass" means the document changes without a declared mutation, or a derived fact is computed outside inferences.

### 4.1 Document changes that bypass declared mutations (state-driven diffing)

- **Snapshot diffing as the write path.** `commit_host_snapshot` (`S/🧬️schema/🦀️.rs` L260) and `generation3d_host_snapshot_operations` (`S/🧬️schema/🧬️mutations/🦀️.rs` L462-518) compare two whole graphs and emit mutations. Callers include node-graph-edit (`S/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs` L62), add-widget (`🧩️add-widget` L92), delete-selection (`❌️delete-selection` L26), remove-widget (`➖️remove-widget` L24), reorganize (`🗺️reorganize` L19), edit-mesh-selection (L90, L96), transforms (`🧭️transforms` L450), import-document (`📥️import-document` L110), set-active-example (`🎨️set-active-example` L65). Handlers clone a `FlowHost`, edit it directly, then diff. This is state-driven, not event-driven, against the AGENTS rule.
- **Example switch writes a full rewrite.** `set-active-example` deletes and re-creates every widget and synapse by diff (L65). History therefore records a full graph rewrite, not one declared action.
- **No reorder verb.** The mutation vocabulary has no reorder. `reordered_survivors` (`mutations/🦀️.rs` L520-543) uses a longest-increasing-subsequence heuristic to choose delete-and-recreate for survivors. Its doc comment says the fixture swap "silently kept" entry order before the fix.
- **Graph surgery in command code.** `ensure_component_node` (`🧭️transforms/🦀️.rs` ~L583-612) calls `host.insert_between`. `ensure_gumball_node` (`S/🚪️io/📝️text/📸️snapshot/🦀️.rs` ~L563) does the same for gumball nodes. Both are outside any mutation.
- **Camera has two owners and one dead path.** The document has `host_snapshot.camera`. `UpdateCamera` exists (`mutations/📷️update-camera`) and is replayed by the text and binary io codecs, but no editor command emits it (grep). The editor's `set-camera` writes config `SetPreviewCamera` (`🎮️commands/📷️set-camera/🦀️.rs` L18). The diff function also never compares `camera`.
- **Generation values are applied in two places.** `generation_host_snapshot_for` (`S/🧬️schema/🦀️.rs` L194-236) patches typed widget values in memory. `evaluate_generation_preview` (`S/🚪️io/📝️text/📸️snapshot/🦀️.rs` L534-540) patches a JSON copy through `apply_generation_values_to_host_snapshot` (`OSFLOW/🌿️vcs/🦀️.rs` L2049, matches widget kind by string). The two implementations cover the same four kinds and can drift.
- **Duplicate mutation concepts.** Slider value exists as `ChangeSliderValue`, as `ChangeWidgetInput`, and as `UpdateWidget`. `patch-flow-widgets` (`🩹️patch-flow-widgets/🦀️.rs`) handles only `field == "value"` on `InputSlider`. Generation values exist as `ChangeGenerationValue` and as the playbook's own `GenerationMutation` bridge (`mutations/🦀️.rs` L440-460).
- **Relative gestures recompose operators by kind string.** `generation3d_transform_diff` (`mutations/🦀️.rs` L371-419) and `generation3d_transform_inverse` (L421) compose params per target. The targets are selected by the constants `GENERATION3D_TRANSLATE_KINDS`, `ROTATE_KINDS`, `SCALE_KINDS` (L308-310).
- **Global process state.** `set-contributions` installs into `flow_extension_state()` and `LINKED_FLOW_EXTENSION_INSTALLERS` (`OSFLOW/🗿️artifacts/🌊️flow/📔️registry/🦀️.rs`, static mutexes). This is process-wide state outside the document, and it changes what the evaluator can compute.
- **Dead placeholder in the framework.** `install_builtin_flow_extensions` (`📔️registry/🦀️.rs` ~L127-129) has an empty body with a comment that the packs are "runtime-installable".

### 4.2 Derived facts computed outside inferences

- **The generation3d inference is unused.** `Generation3dInference { topology }` (`S/🧬️schema/💡️inferences/🦀️.rs` L17-30) computes DAG counts and a topological order (`🧭topology/🦀️.rs` L42-81). No non-test consumer in the plugin was found (grep). The editor, viewer and io do not read it.
- **Mesh facts are computed in renderer and editor code.**
  - Bounds: `preview_payload_bounds(meshes_json)` (`preview-eval` L505-525) re-parses JSON.
  - Camera fit: `preview_fit_json` (L576) and `preview_fit_revision` (L538-562, hand-rolled FNV over topology).
  - Analysis: the mesh `analyze` operator (`MESH/🦀️.rs` L139) and TS `analyzePolygonMesh` (`MESH/🟦️.ts` L331) both compute area, volume and counts.
  - Selection vertex sets: `selectedMeshVertices` (`S/✏️editor/🎯️selection/🟦️.ts` L39) and Rust `component_vertices` (`MESH/🦀️.rs` L51).
  - Gumball centre from indexed topology: `S/✏️editor/🎯️selection/🦀️.rs` L128.
- **Geometry inference lives in the flow artifact, not in generation3d.** B-Rep evaluation goes through `s.flow-extension-brep.geometry` on artifact kind `s.flow.flow` (`BREP/💡️inferences/📐️geometry/🦀️.rs` L6-7). The procedural artifact has no geometry inference of its own, so ownership is split across two plugins.
- **Mesh analysis is an operator output only.** It is shown as the raw "reflected output" tree in the inspector (`S/✏️editor/📌️panels/🔍️inspection/🦀️.rs` L279). There is no inference, no overlay and no derived-field declaration.

### 4.3 Hard-coded tables and magic strings

- **Example list repeated in five places.**
  - `S/🧬️schema/🦀️.rs` L125-133 (ids), L156-170 (`is_generation3d_example_id`, which also accepts the alias `"demo"`, a generation2d example id).
  - `S/✏️editor/🦀️.rs` L2930-2938 (option labels) and L3145-3155 (`examples()`).
  - `S/👁️viewer/🦀️.rs` L1648-1656 (option labels).
  - `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` L19-27 (text constants) and L475-483 (match arms, `PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo"`).
  - Each example module (9 files, 8-14 lines) repeats `ID`, `label()`, `ICON`, `PRIMARY_TEXT`.
- **Same files included twice.** The text constants in `io/📝️text` `include_str!` the same `🗣️.dsl.semio` files that the example modules include.
- **Inconsistent asset sets.** Eight examples have `.op.semio`, `.spr.semio` and `.pack.semio` files. `mesh-workbench` has only the DSL. Asset directory names do not match example ids (`hexagonal-mushroom-column` vs asset dir `🍄️hexagonal-mushroom`).
- **Default document is the hex column example.** `default_snapshot` (`S/🚪️io/📝️text/📸️snapshot/🦀️.rs` L469-471) parses it. `default_generation3d_snapshot` (`S/🧬️schema/🦀️.rs` L134-140) is the framework's three-widget demo, and its doc comment admits the old name was misleading.
- **Gumball kind strings.** `gumball_xform_kind` (`S/🚪️io/📝️text/📸️snapshot/🦀️.rs` L546-552) falls back to translate for any unknown operation. `validate_component_gesture` (`🧭️transforms/🦀️.rs` L74) lists `"brep.mesh.translateComponents"` and siblings. `format!("brep.mesh.{operation}Components")` builds kinds at ~L596. `gumball_widget_id` builds ids from strings (L555).
- **Mesh operation allowlist.** `edit-mesh-selection` (`S/✏️editor/🎮️commands/🥽️edit-mesh-selection/🦀️.rs`) hard-codes the granularity table (L40), the allowlist (L49: extrude, inset, subdivide, flip, deleteFaces, moveVertices, loopCut, bevel, dissolveEdges, dissolveVertices, mergeVertices, moveProportional, snapVertices, filletEdges, chamferEdges, shell), and the parameter mapping (L59, L63).
- **Mesh extension dispatches by operator string.** `MeshOperation(&'static str, …)` (`MESH/🦀️.rs` L596) matches on names inside `prepare` (L690-968). `operation.ends_with("Components")` (registration) is a naming heuristic.
- **Brep operator ids repeated across four places.** `register()` literals (`BREP/🦀️.rs` ~L1031, L1368, L1508…), `NODE_KERNEL_METHOD` (L24-~120), packaged `🔣️.json` and `🛂️.descriptor.semio`, and the generation3d literal arrays. The editor also has 142 hand-written operator label entries (`S/✏️editor/🗣️terminology/🦀️.rs`, e.g. L27 and L423). Unknown ids fall back to the extension's English name (`generation3d_catalogue_name`, L402-559, `_ => name`).
- **Preview geometry recognised by string prefix.** `is_brep_geometry_handle` (`preview-eval` L340-361) accepts `solid-`, `shell-`, `face-`, `wire-`, `edge-`, `vertex-`, `compound-`, `curve-`, `surface-`, or a 64-character hex digest.
- **Stringly ids.** Selection id is `instance.granularity.component` (`selection/🦀️.rs` L38, parsed with `rsplit_once('.')` at L53). Mesh component address is `"{id}@meshOut#0.{mode}.{component}"` (`🧭️transforms/🦀️.rs` L437). Port endpoint is `"node@port"`.
- **LOD and show modes are strings with literals.** `preview_tolerance` (`preview-eval` L318-324) matches `"coarse"`, `"fine"` with literals 0.15, 0.02 and default 0.05. The same default 0.05 is repeated in the guest (`BREP/🦀️.rs` `tessellate`, `unwrap_or(0.05)`). Show modes and LOD modes are constant string arrays (`S/✏️editor/🎚️config/🦀️.rs` L50-54).
- **Budget constants duplicated.** `EVALUATE_STEP_BUDGET = 8` (`preview-eval` L134, "mirrors flow_extension_sdk"). `PREVIEW_TESSELLATE_STEP_BUDGET = 24` (L305) and `TESSELLATE_STEP_BUDGET = 24` (`BREP/🦀️.rs` `extension_guest`). Wall time `2_000_000` (L141). The cancellation id `"previewEval"` is a literal in the guest.
- **Magic request id.** `PREVIEW_EVAL_HOP_REQUEST: u64 = 103` (`preview-eval` L129).
- **Hand-rolled hashes.** FNV-1a 64 appears in `preview-eval` L206, L539, L1029 and `⏯️tool-run/🦀️.rs` L149. The G2D copy has the same pattern (`G2D/…/preview-eval/🦀️.rs` ~L232).
- **Layout magic numbers.** `AUTOMATIC_GAP = 48.0` and default placement 120.0 (`🧩️add-widget/🦀️.rs` L4, L64). Default export format "gltf" for OutputExport (`descriptor_fields`, L~35).
- **Two naming vocabularies per command.** Each command has a camelCase tool id and a kebab-case DSL keyword in the same macro row (`S/✏️editor/🦀️.rs` L122-168). Two artifact kind spellings for one artifact: id `3d.generation`, source format `generation.3d`, component kind `generation3d`, schema `generation.3d`, dialect `s.procedural.generation3d` (`G3D/🦀️.rs` L82-99 and L27).
- **Subset representation differs.** `GENERATION3D_DIALECT` at `G3D/🦀️.rs` L27 uses `SubsetId::ANY`. The io module uses `SubsetId("*")` (`S/🚪️io/🦀️.rs` L885-1001). Other dialect constants are inline struct literals.

### 4.4 JSON strings where a typed schema should be

- **Command payloads.** `SetWidgetInput.value: String` (parsed per kind). `NodeGraphEdit.operations_json: String` (`✏️node-graph-edit` L15-19) carries a whole batch of connect, disconnect, insertPort, delete and move rows as JSON text. `AddWidget.descriptor_json()` builds a JSON string (`🧩️add-widget` L45) that the host re-parses. `ImportDocument` payloads are text.
- **Extension answers.** `FlowEvalResolve.output_json` and `FlowTessellateResolve.output_json` are `String` (`preview-eval` ~L67-88, L91-100). `FlowTessellateCancelResolve.output_json` likewise.
- **Extension requests.** `request_json` for `evaluate` nests `inputJson` and `dependencyJson` as strings inside a JSON object (`preview-eval` L946-1000). The `tessellate` and `evaluateCancel` capabilities read raw JSON bytes by string key (`BREP/🦀️.rs` guest `invoke`).
- **Renderer inputs.** `meshes_json`, `instances_json`, `selection_json`, `fit_json`, `status_json` and the eval JSON are all `String` (`preview-eval` L1446; `S/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` L174-191; editor `render` L345-390 takes `preview_eval_text: Option<&str>`).
- **Config.** `sun_json: String` and `lod_mode`, `show_mode` are strings in the typed config (`S/✏️editor/🎚️config/🦀️.rs` L97-111).
- **Mesh as text.** Mesh schema fields are `data: Text` and `preview: Text` (`MESH/🦀️.rs` L822). A polygon mesh is encoded as a JSON string in the document and re-decoded on each edit (`decode_mesh` L83, `encode_mesh` L115). The `construct` operator takes JSON vertices as text (`MESH/🦀️.rs` `source_text` L387).
- **Mesh pack transport.** Tessellation output is base64 text inside the response (`OSFLOW/🎒️mesh/🦀️.rs`; `decode_preview_mesh_pack` `preview-eval` L626-630).
- **Runtime-parsed declarations.** The `previewEval` tool label, icon and run definition are parsed at runtime from an embedded 7.5 KB JSON (`S/🧵️preview-eval/⏯️tool-run/🦀️.rs` L24, L127-141) with `expect(...)` panics. A compile-time typed declaration would avoid this.
- **Import helpers.** `polygon_import_payload(...) -> String` and `import_polygon_meshes(Vec<String>)` (`S/🚪️io/🦀️.rs` L405, L435). `generation3d_document_from_mesh -> pack_json::Value` (L305).

### 4.5 Schema drift and duplicate schema sources

- **Mutation proto and GraphQL describe the snapshot, not the mutation union.** `S/🧬️schema/🧬️mutations/🛰️.proto` (L8-13) and `🕸️.graphql` (L3-6) define `Generation3dMutation { host_snapshot, generation }`. The Rust enum (20 variants), the TS union (`mutations/🟦️.ts` L23+) and the JSON Schema (`mutations/🔣️.json`, `oneOf` of 20 `$ref`s to an external `json.schemas.assets.semio-tech.com` host) describe the real union. Proto and GraphQL are wrong.
- **Two hand-kept variant lists.** `KINDS` (`mutations/🦀️.rs` L252-277) and `GENERATION3D_RETAINED_MUTATION_OWNERS` (`host/🦀️.rs` L397-419), with count `GENERATION3D_MUTATION_VARIANT_COUNT = 20` (L36). The code says the framework never parses Rust, so a test keeps them in step.
- **Hand-written binary decoder per mutation.** The retained decoder `Generation3dRetainedMutationOwner::accept` (`host/🦀️.rs` L1412-1744) and the field owners (L872-1000) are hand-written per variant. Nothing generates them from the schema.
- **Four grammars for one binary format.** `S/🚪️io/💾️binary/📸️snapshot/` contains `.ksy`, `.spicy`, `.abnf` and `.protocol.semio`. Only the `.protocol.semio` file is embedded as a constant in Rust (`COMPONENT_PROTOCOL_SEMIO`). The other three are not referenced by Rust in this artifact.
- **SQLite columns by string.** `columns(table)` (`CORE/🧬️generation/🪶️sqlite/🦀️.rs` L17) matches table names such as `"generation_host"` and `"generation_slider_widget"` to column indices. The table list is hand-maintained.
- **Text DSL and its parsers.** `S/🚪️io/📝️text/🦀️.rs` (1164 lines) hand-writes a DSL grammar plus `snapshot_widget_keyword` and per-variant `*_to_dsl`/`*_from_dsl` functions.
- **Packaged brep descriptors.** `BREP/🔣️.json` and `BREP/🛂️.descriptor.semio` duplicate `register()` (no generator found).
- **Persistence formats.** The same snapshot can be stored as text DSL, binary, SQLite, generation JSON and pack. Each format has its own codec and tests.

### 4.6 Duplicate implementations

- **Rust and TypeScript run the same mesh algorithms.** Rust (`MESH/🦀️.rs`) and TypeScript (`MESH/🟦️.ts`) both implement `componentVertexIds`/`component_vertices`, `transformMeshComponents`/`transform_components`, `loopCutMesh`/loop cut, `knifeCutMesh`/knife cut, `analyzePolygonMesh`/`analyze` and `inspectMeshComponent`/`inspect`. The TS file is a "portable contract" and is also imported at runtime by the generation3d editor.
- **Editor TS duplicates Rust edit logic.** `editInputValue` (`S/✏️editor/🎮️commands/🎚️set-widget-input/🟦️.ts` L7) mirrors `edit_input_value` (`set-widget-input/🦀️.rs` L69). `editCollectionValue` (TS L160) mirrors `edit_collection_value`. `editMeshSource` (TS L27) mirrors Rust `edit_mesh_asset` and friends (`set-widget-input/🦀️.rs` ~L140-260). `editMeshTexture` (TS L201) mirrors `import_mesh_texture`.
- **Procedural depends on flow internals.** Three generation3d TS files import `🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts` directly: `set-widget-input/🟦️.ts` L4, `selection/🟦️.ts` L1, `io/🟦️.ts` L2. The io TS also imports stdio gltf, ply and obj snapshot types.
- **generation2d and generation3d copies.** `G2D/…/preview-eval/🦀️.rs` (246 lines) and `G3D/…/preview-eval/🦀️.rs` (1475 lines) share structure and have diverged (about 1 500 differing diff lines). The same holds for `⏯️tool-run` (579 vs 645 lines) and `editor/🎚️config` (248 vs 193 lines).
- **Viewer repeats editor scaffolding.** The viewer's instance owner, job factories, work structs and flow eval work (`S/👁️viewer/🦀️.rs` L153-841) repeat the editor's (`S/✏️editor/🦀️.rs` L179-1600). The viewer also repeats the flow eval tool id list (viewer L283, editor L440).
- **Two functions named `import_document`.** `S/🚪️io/🦀️.rs` L330 `import_document(neuron_kind, data_text) -> Snapshot` and L871 `import_document(name, payload) -> Result<Snapshot>`. Different semantics, same name.
- **Local STL writer beside the stdio bridges.** `stl_ascii_bytes` (`io/🦀️.rs` L321) is local, while the module comment says mesh writers come from `semio-s-artifact-stdio-semio` `conversion-mesh`. The two were not compared line by line.
- **Dag workflow translation.** `dag_host_snapshot_to_workflow` (`S/🧬️schema/🦀️.rs` L269-305) translates a DAG host from another artifact kind inside generation3d. The flow window has its own `document_operator_records` (`S/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` L240).

### 4.7 Special cases per example

- Example switch is a diff-based rewrite (4.1). Preview framing revision hashes the topology, not the parameters, so the framing changes only when the example changes (`preview-eval` L538-562). This is deliberate, but it is an example-level special case inside the renderer path.
- `is_generation3d_example_id` treats `"demo"` as an example (`schema` L156-170) while `"demo"` is a generation2d example.
- `add-widget` defaults `outputExport` to `gltf` (`🧩️add-widget` ~L35).
- The `mesh-workbench` example has no compiled `.op.semio` or `.spr.semio` assets, unlike the other eight (4.3).

### 4.8 English-only and inline bilingual strings

- **Command faults are English only.** Examples: `Fault::from("Choose a widget")`, `"Choose an input"`, `"Input value is missing"` (`S/✏️editor/🦀️.rs` L2265-2331, ~127 literals in that file). Also `Err("Select a collection input".into())` and similar in `set-widget-input/🦀️.rs` (~107 literals). `format!("unknown widget kind: {}")` (`add-widget` `descriptor_json`). `"The B-Rep operation requires matching edge or face selection"` (`edit-mesh-selection` L104). None has a German label, unlike `GENERATION3D_WIDGET_ADD` notices (`G3D/🦀️.rs` L64-76).
- **Inline `if is_de` literals bypass terminology.** `show_mode_row` (`S/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` L53-58) and the show-mode measure label (L67).
- **Bilingual `match` in core.** `generation_tree` (`CORE/🖼️semantic-ui/🦀️.rs` L127-150) hard-codes English and German for each key in a `match (key, locale)`, not through terminology.
- **Layout titles are English.** `create_default_layout(..., Some(&["Flow", "Preview"]))` (edit mode) and `Some(&["Generations", "Form", "Preview"])` plus `create_named_layout(..., "Generate", ...)` (generate mode).
- **Fallback to English.** Missing terminology falls back to the extension's English name (`_ => name` at `terminology/🦀️.rs` L559, L695).
- **Operator descriptions are English only.** Brep summaries (`q("…", "Split a compound into its member solids")`) and mesh summaries (the `match` in `MESH/🦀️.rs` ~L880-955) are English. Labels (`"Explode"`, `"Export Step"`) are English.
- **Trace logs.** Two `eprintln!("[TRACE] …")` in `render_body` (`S/✏️editor/🦀️.rs` ~L359) and one in `flowTessellateCancelResolve` (`preview-eval` L1143). The AGENTS rule requires a `[DEBUG] ` prefix.

### 4.9 Side channels and global state

- **Eval loop as UI dispatch.** The preview run re-dispatches `flowEvalTick` as `DispatchAction` hops (`preview-eval` L153-160). The window id travels as an argument, so every hop depends on the shell routing it to the right window.
- **Scratch session outside the retained one.** `with_scratch_session` (`S/✏️editor/🦀️.rs` L286-295) builds a throwaway `FlowEvalSession` for the marks-free render and the non-retained `handle`. Its doc says these paths never compute anything that must survive a turn.
- **Port fallback.** `mesh_data_for_preview_handle` (`preview-eval` L642-660) calls `session.geometry_port()?.tessellate_step(...)` after the chain's own mesh is missing. The comment says callers without an authority get no native fallback. Whether production sessions ever carry a port was not verified (section 7).
- **Registry is process-global.** `flow_extension_state()` and `LINKED_FLOW_EXTENSION_INSTALLERS` (framework registry) are static mutexes. Contributions installed by one command affect all later evaluations in the process.
- **Mesh bytes through the document.** Mesh payloads sit inside the document as text (4.4). A mesh edit therefore re-encodes and re-diffs the whole mesh.
- **Retention by hand.** Retained owner catalogues and close-step ladders (`host/🦀️.rs` L326-420, L2957-3460) are hand-maintained to match the store; a missed variant is a silent leak.
- **Ticket-specific comments in code.** Many doc comments cite a ticket number and report file (for example `26/09/09/PROCEDURAL-3D-END-TO-END`) to explain behavior. The explanations are useful, but they tie code to a process artefact that will rot.

### 4.10 Dead, unused or placeholder items

- `ChangeSliderValue` and `ChangeWidgetInput` overlap (4.1).
- `UpdateCamera` has no emitter in the editor or viewer (4.1).
- `Generation3dInference.topology` has no consumer in the plugin (4.2).
- `install_builtin_flow_extensions` is empty (4.1).
- `OutputAction` and `OutputExport` widget kinds: export works through the inspector, but no document-level declaration says which format list an export uses (it is read from `EXPORT_FORMATS`, `io/🦀️.rs` L585). Not a dead item, but a second source for formats.
- `dag_host_snapshot_to_workflow` is used only to adapt another artifact (4.6).
- `FlowArtifact` in the framework (`OSFLOW/…/📸️snapshot/🦀️.rs` L70-74) is a second document type next to `FlowHostSnapshot`. Whether it is still used by generation3d was not checked.

### 4.11 AGENTS rule issues visible in the code

- **Runtime external crates.** The framework flow package declares `base64 = "0.22"` (`OSFLOW/📦️packages/🦀️rust/Cargo.toml` L53) and uses it in the mesh codec (`OSFLOW/🎒️mesh/🦀️.rs` L2). The generation3d crate depends on `serde_json` at runtime, used to read the embedded tool definition (4.4). These are runtime third-party dependencies, which the AGENTS rule forbids unless wrapped behind an interface.
- **Parser duplication.** TS and Rust each parse the same mesh and input formats (4.6).
- **Commented-out or stale schema.** Proto and GraphQL mutation schemas do not match the code (4.5).
- **Comments inside bodies.** Many function bodies carry inline `//` comments (for example `evaluate_tick`, `preview-eval` L946-1027). The AGENTS rule asks for concise code with docstrings only.

## 5. What an end user cannot do yet

**Create**
- Draw or sketch a B-Rep in the viewport. There is no sketch tool. Geometry starts from primitives or from typed numbers and point/vector inputs (`brep.prim3d.*`, `brep.curve.*`, `brep.surf.*`).
- Place a point or vertex by dragging. Points are typed values in the inspector, not gizmo handles (`WidgetInputValue::Point` in the inspector).
- Create a new empty document. The command enum has no "new" command. Only `setActiveExample` (9 examples) and `importDocument` exist.
- Create a mesh by painting or modelling from scratch. A mesh is created from JSON vertices and face indices (`construct`), or from primitives (`box`, `plane`, `sphere`, `cylinder`, `cone`), or from B-Rep (`fromBrep`).
- Import B-Rep STEP into the document. STEP import and export exist as flow operators (`brep.io.importStep`, `brep.io.exportStep`), but `IMPORT_FORMATS` and `EXPORT_FORMATS` (`io/🦀️.rs` L585-611) do not list STEP.
- Export a native B-Rep. Document export writes STL, OBJ, PLY, glTF, LAS, DWG or Semio text. All except the Semio text form are tessellated meshes.

**Edit**
- Select and edit B-Rep faces or edges directly. Analytic labels select faces and edges, but only `filletEdges`, `chamferEdges` and `shell` are reachable from the selection UI (`edit-mesh-selection` L104, L149-150). Extrude, offset, move and boolean on a selected B-Rep face are not reachable from selection.
- Multi-select components of two meshes at once. The UI requires one mesh (`edit-mesh-selection` L47, `selection` L63-68).
- Use most mesh operators interactively on a selection. Decimate, triangulate, weld, orient, fill holes, mirror and merge coplanar faces are reachable only as graph nodes through the catalogue (labels exist in terminology). Knife cut works on one face only.
- Gumball a B-Rep sub-element. Gumball targets whole shape instances (`brep.xform.*`) and mesh components only.
- Edit generation values for operator parameters. Generations only drive sliders, notes, images and variables (4.1, `generation_host_snapshot_for`).
- Use a generation to switch the document's camera or show mode. Generations do not touch config (verified: generation values are applied to a clone only).
- Edit a mesh's texture or material in a structured way beyond path-based fields and texture import (`set-widget-input` facets `meshSource` and `path`).

**Analyse**
- See interactive measurements or overlays. Volume, area, bounds, distance and validity are operator outputs. They appear in the inspector's reflected-output tree, not as overlays or measurement tools in the viewport. No measure command exists in the enum.
- Highlight boundary edges, non-manifold edges, inconsistent edges or degenerate triangles on the mesh. The `analyze` operator computes counts (`MESH/🦀️.rs` L139-144), but no view renders them.
- Run a B-Rep validity check interactively. `brep.measure.validate` exists as a node only.
- Get a section view or a cross-section plane. `brep.intersect.section` exists as a node only.

**Organise and share**
- Share a document in a native B-Rep format. Only meshes and the Semio text form leave the editor.
- Save a named analysis or view. Camera, show mode and LOD are config and persist per viewer, not per analysis.

## 6. Recommendations (for the next ticket, not implemented)

These are the highest-leverage changes the inventory points to. They are suggestions only, since this audit did not change code.

1. Replace snapshot diffing (4.1) with declared mutations per command. Start with `set-active-example`, `add-widget`, `delete-selection` and `remove-widget`.
2. Add a reorder mutation so `reordered_survivors` can go.
3. Make the mutation proto and GraphQL schema match the 20-variant union (4.5). Generate the per-variant decoder from the schema instead of hand-writing it (`host/🦀️.rs` L1412-1744).
4. Put derived mesh facts (bounds, fit, analysis, selection sets) behind a typed inference, so renderer and editor read them instead of re-parsing JSON.
5. Replace JSON-string payloads with typed records (4.4), starting with `operations_json`, `output_json` and the extension request body.
6. Keep one mesh algorithm implementation (Rust in the guest). Generate the TS side from schema or drop it from the editor path (4.6).
7. Route all command faults and inline labels through terminology (4.8).
8. Derive the example list from one source (4.3).
9. Merge the generation2d and generation3d preview-eval and tool-run copies behind a shared module, or delete the duplicate (4.6).
10. Remove the empty `install_builtin_flow_extensions` and move process-global registry state behind an explicit owner (4.1, 4.9).

## 7. Unverified points and follow-ups

- No runtime check was run. All behavior claims are from code reading. Shell verification was grep only, and the search tool was not used.
- Whether `FlowEvalSession.geometry_port` is set in production (`preview-eval` L642-660). If it is, the editor has a second, non-extension geometry path.
- Whether `BREP/🔣️.json` and `BREP/🛂️.descriptor.semio` are checked against `register()` by a test. A test may exist under `BREP/🧪️tests/`; not read.
- Whether `Generation3dInference` is read by any registry consumer outside the procedural plugin. Only the procedural plugin tree and the framework were searched, not the whole repo.
- Whether the viewer and editor share all of their scaffolding through a common module that was not read.
- Whether `stl_ascii_bytes` (io L321) duplicates the stdio STL writer byte for byte. Not compared.
- Whether `FlowArtifact` (framework flow snapshot L70-74) is still used on the generation3d path.
- Whether the mesh and brep operator label lists (terminology) are complete for every registered operator. Only a count (142 entries) was taken.
- The count of 42 editor command rows and 15 viewer command modules was read from the enum and the `commands` module list. A mismatch between enum rows and directory names was not reconciled.
