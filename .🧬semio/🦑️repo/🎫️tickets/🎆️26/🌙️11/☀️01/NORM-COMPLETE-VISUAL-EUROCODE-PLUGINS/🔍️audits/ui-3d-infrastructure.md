# 🧊 UI and 3D Infrastructure

Read-only audit for the norm plugin (`✏️s/🔌️plugins/📕️norm`). Paths are relative to `/home/deck/semio` unless absolute. Framework roots: `🧰️framework/`. Shorthand used below: **FW** = `🧰️framework`, **ENG** = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

## 1. Plugin window body

Plugin side (Rust, guest code):
- `WindowKit` trait: `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🦀️.rs:460`. Members: `type ViewModel`, `const KIND_ID`, `fn window_kind() -> WindowKindDefinition`, `fn editable_window_kind() -> WindowKindDefinition`, `fn render(view: &Self::ViewModel) -> UiAssemblyResult<BuiltNode>`. Reference impl: `🪟️window-kits/🧊️mesh/🦀️.rs` (`MeshWindowKit`, `KIND_ID = "framework.window.mesh"`, `SurfaceKind::World3d`, icon `"box"`). TS twin: `🪟️window-kits/🧊️mesh/🟦️.ts` (`renderMesh`).
- `UiAssemblyResult<T> = Result<T, PluginAssemblyError>` and `ui_assembly_error(stage)`: `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🦀️.rs:32-34`.
- `window_kind_definition(id, en, de, surface_kind, icon_id, actions)`: `🔌️plugin/🪟️window/🦀️.rs:212`.
- `scene_surface(id, kind, &scene)`: `🔌️plugin/🪟️window/🦀️.rs:139`. Splits a scene into its spine (`SurfaceProps.doc`) and payload lanes (paged-text carrier children), so large scenes are not refused by the 32 KiB fixed doc.
- `TreeWindows<'a>` (`🪟️window-kits/🌳️tree/🦀️.rs:379`): `TreeWindows::for_body(view, body_key)` and `TreeWindows::unhosted()`. Shares one body-wide node budget (`TREE_WINDOW_BODY_NODE_BUDGET`) with the React host.
- `FormPanelBuilder` (`🪟️window-kits/🦀️.rs`): namespaced `Section > Field rows > submit Button` builder.
- Plugin SDK re-export crate: `semio-framework-plugin` (`🔌️plugin/📦️packages/🦀️rust`). Its `lib` re-exports `BuiltNode`, `UiAssemblyResult`, `scene_surface`, `window_kind_definition`, `world3d_scene`, `world3d_selection_json`, `WorldSunConfig`, `Canvas2dScene`, `built_text_node`, `ui_label`, `ui_text`, and the tree/window kit surface.

Contract side (generated, do not edit): `FW/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`. Rust crate `semio-framework-ui-contract` (`FW/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust`), builder DSL in `semio-framework-ui-contract-builder` (`🧬️contract/🏗️builder`).

- `BuiltNode` (`ui-contract/🟦️.ts:69`): `{ key, component, layout, style, activity, disabled, accessibility, bindings, menu, children }`. Children nested inline.
- `Component` (`ui-contract/🟦️.ts:100`), closed union of 21 `type` tags: `container` (roles `plain|section|group|field|form|toolbar`), `text`, `button`, `separator`, `input` (kinds `text|longText|number|date|color|file`), `select`, `toggle`, `keyValueList`, `slider`, `numberStepper`, `ring`, `iconSelect`, `progress`, `tree`, `treeSection`, `treeItem`, `image`, `surface`, `extension`, `table`, `tableRow`.
- `LayoutSpec` (`ui-contract/🟦️.ts:266`): `leaf | stack | grid | overlay | scroll | absolute`.
- `StyleSpec` (variant, size, density, tone, emphasis), `AccessibilitySpec` (label, description, live, shortcut, hidden). Labels are localized via `LocalizedLabel` (`semio-framework-ui-locale`).
- Window shell: `WindowLayout { root: WindowLayoutNode }`, `WindowLayoutNode = window | stack | split` (`ui-contract/🟦️.ts:902`).
- `WindowKindDefinition`: generated at `FW/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:1600`; source `FW/🔨️modules/🛂️manifest/🧱️app-surface/🪟️window-kinds-and-panels/🦀️.rs:10`. Fields: `id, label, body_key, surface_kind, icon_id, options, actions, utilities, initial_utility_id, interactions, params_schema`.
- `ViewModel` (`FW/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs:24`), with `tree_windows` and `tree_viewport_rows`.
- `BuiltNode` is also re-exported to TS as `@semio-tech/framework` (`type BuiltNode`).

Renderer host (what consumes the tree): `ENG/🧱️elements/🗣️Interpreter` (node-tree, `🪟️tree-windows`, `🖼️surface-views`, `🌉️scene-host-contract`), `ENG/🧱️elements/🔌️PluginRuntime` (`🪟️retained-ui-patch`, `🧺️handle-ui-intake`), `ENG/🧱️elements/🏛️ShellHost/🧱️built-node-stores`. Two targets: React (`ENG/🎯️targets/⚛️react`) and wgpu (`ENG/🎯️targets/🧊️wgpu`).

Norm today: `📇️registry/🧬️contract/🖥️app-surface` renders the document as JSON (per fleet brief). No norm window currently builds a `WindowKit` 3D or 2D surface.

## 2. 3D viewport and scene node

Yes. Surface kind `world-3d` (`SurfaceKind::World3d`, doc schema `world3d@1`). Set by `SurfaceProps { kind, docSchema, doc, bindings }` (`ui-contract/🟦️.ts:519`).

Scene payload: `World3dScene` at `FW/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🌍️world3d/🦀️.rs:60` (crate `semio-framework-ui-scene-world3d`). Main fields:
- `camera_json`, `meshes_json`, `instances_json` (authoritative), `instances_delta_json` (incremental), `selection_json`
- `pick_targets_json` (sub-object pick targets: vertex, edge, face, object), `interaction_json`, `brush_preview_json`, `engagement_preview_json`, `target_volumes_json`, `references_json`, `vortices_json`, `attractions_json`, `lod_json`, `chunking_json`
- `presentation_json` (`World3dPresentation { show_grid, show_gizmo, interactive, viewport_mask, clear, source_aspect }`), `snapshot`, `tool_run_trace`, lane manifest.

Constructors used by plugins (`FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌍️world3d/`):
- `world3d_scene(camera_json, meshes_json, instances_json, selection_json, &WorldSunConfig) -> World3dScene` (`☀️sun/🦀️.rs:27`)
- `WorldSunConfig { enabled, azimuth, elevation, intensity, color }` (default disabled, az 45, el 35)
- `world3d_meshes_json_from_kinds(&[String])` and `world3d_meshes_json_from_kinds_and_urls` (GLB, `world3d_mesh_id_from_url`) (`🌍️world3d/🦀️.rs:417, 459`)
- `world3d_selection_json(method, ids, hovered_id)` and `world3d_selection_json_with_granularity(...)` (`🦀️.rs:473-476`)

Mesh payloads:
- Built-in kinds by reference, `{ id, kind }`: `box` (used by mesh kit), `cone` (used by fem), plus `cylinder`, `sphere`, `torus` in the host (`ENG/🧱️elements/🌐️World3dHost/🎨️mesh-render/🧱️mesh-parsing-and-residency/🟦️.ts`).
- Inline geometry, `{ id, data: { positions, normals, colors, indices } }`. Used by fem for meshed solids with per-vertex stress colours (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/🦀️.rs`, `fem3d_solid_mesh_entries`).
- GLB by URL: `ENG/🧱️elements/🌐️World3dHost/🧩️glb-surface`.

Instances: `{ id, interactionGranularityId, meshId, position [x,y,z], rotation [x,y,z,w], scale [x,y,z], label }`. The `interactionGranularityId` makes a raycast hit resolve to a domain entity (fem uses `node, element, solid, support, load, material, section, loadCase, combination`).

Host components (`ENG/🧱️elements/🌐️World3dHost/`): `🎨️mesh-render`, `🎛️marquee-and-gumball`, `🧮️gumball-math`, `👆️pointer-and-drop`, `👇️hover-and-placement`, `🔗️instance-selection`, `🧭️fit-and-presence`, `🔀️paint-and-projection`, `📏️modelling`, `🌍️scene-lanes`, `🧱️instance-layers`, `🪟️window-view-store`. Engine-side renderer tests: `ENG/🧪️tests/📇️world3d-surface-verbs`.

Camera and picking math: `FW/🔨️modules/🖱️ui/🎬️scene/📐️math/` (`🎥️camera`, `🎯️picking`, `🧮️algebra`, `🕸️mesh`, `🎞️pass`).

Plugins using 3D windows (via `world3d_scene` or `WindowKit`): fem (`🏗️fem`), block (`🧱️block`, `🌐️world`), cad (`📐️cad`, picking under `⚙️engine/🧲️picking`), lowpoly (`💠️lowpoly`, `🌐️model`), bim (`🏙️bim`, `🧊️world`), procedural (`🌀️procedural`), wfc (`🀄️wfc`), puzzle (`🧩️puzzle`). The `space` plugin also uses `semio-framework-ui-scene`.

Minimal real example, fem read-only 3D viewer (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs`):

```rust
let camera = semio_s_artifact_fem_3d_scene::viewport::INITIAL;
let (meshes_json, instances_json) = semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None)?;
let mut scene = world3d_scene(scene_camera_json(&camera), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
scene.snapshot = visual.map(Fem3dPageVisualLease::snapshot);
world_3d_surface(BODY_KEY, &scene)   // -> scene_surface(id, SurfaceKind::World3d, &scene)
```

Instance construction, from the same crate (`🎬️scene/🦀️.rs:171`):

```rust
json!({ "id": identity, "interactionGranularityId": granularity, "meshId": mesh_id,
        "position": position, "rotation": rotation, "scale": scale, "label": label })
```

Utilisation and colour-field support (domain neutral, in `FW/🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling/`):
- `World3dScalarField { mesh_id, domain, values: Vec<Option<f64>>, ramp, range, legend }` (`🌡️scalar-field/🦀️.rs`). Per-value colouring with a legend.
- `World3dColorRamp { Viridis, Inferno, Coolwarm, Grayscale }` (`string_enum!`, `stops()`, `sample(t)`).
- `World3dScalarRange { min, max }`, `World3dScalarLegend { title, unit, ticks }`, `World3dLegendTick`, `format_world3d_legend_value(value, locale)`, `world3d_modelling_strings(locale)`.
- Annotations: `World3dAnnotationLayer` with `World3dAnnotation::{Text, Dimension, Angle, Marker, Leader}`; `project_world3d_annotations(layer, view_proj, w, h)` (`📏️world3d-modelling/🦀️.rs` and `🧭️overlay-projection`).
- Section plane and caps: `World3dSection`, `World3dSectionCap`. Sub-element paint: `World3dSubElementPaint`, `World3dHighlight`, `World3dPickTargets`.

Rust crate for all of this: `semio-framework-ui-scene-modelling` (`📏️world3d-modelling/📦️packages/🦀️rust`).

## 3. 2D vector, diagram and chart primitives

There is no chart or plot widget in the contract. Plots are built from path layers plus text and tables.

- `SurfaceKind::Canvas2d` (`"canvas-2d"`): `Canvas2dScene { camera_x, camera_y, zoom, layers_json, framing, snapshot, tool_run_trace, lanes }` at `FW/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🔲️canvas2d-scene/🦀️.rs:8`. Crate `semio-framework-ui-scene-plane`.
- `layers_json` is JSON. Each layer: `{ id, transform: [a,b,c,d,e,f], segments: [...], fill: { color: [r,g,b,a] , evenodd? }, stroke? }`. Segment kinds in the painter (`ENG/🧱️elements/📐️Canvas2dHost/🎨️paint/🦀️.rs:282-289`): `move`, `line`, `quad`, `cubic`, `arc` (with `rx, ry, rotation, large_arc`), `close`.
- Painter files: `ENG/🧱️elements/📐️Canvas2dHost/🎨️paint/` (Rust and TS), `🖍️layer-paint`, `🧭️gumball`, `📷️camera-math`.
- Example of a computed 2D plot layer: fem 2D results `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`, `filled_triangle_layer(...)` for stress contours and moment diagrams (`MOMENT_SCALE_2D`).
- TS side: `@semio-tech/framework/2d` (`FW/📦️packages/🟦️typescript/◻️2d/🟦️.ts`) exports `paintDrawingScene`, `pathSegmentsToSvgD` (SVG export), `drawingTextLines`, `constantGradientColor`, types `PathSegment`, `FillStyle`, `StrokeStyle`, `Vec2`.
- Other 2D surfaces: `board-2d`, `paint-2d`, `ink-canvas`, `node-graph`, `text-editor`, `table` (`Component::table` with `TableProps` and `TableRowProps`, `ui-contract/🟦️.ts:542`), `tiled-map`, `graph-timeline`, `diff-view`, `block-list`, `event-feed`, `icon-render`, `virtual-file-system`.
- No standalone 2D legend type found. For a 2D legend, use `Canvas2d` text and fill layers or a `table`.
- Text: `semio-framework-2d-text` (`FW/🔨️modules/◻️2d/📝️text`), fonts in `semio-framework-2d-text-font`.

Geometry crates for 2D (`FW/🔨️modules/◻️2d/`): `semio-framework-2d` (root), `semio-framework-2d-engine`, `semio-framework-2d-booleans`, `semio-framework-2d-trace` (raster to vector), `semio-framework-2d-path-flatten`, `semio-framework-2d-path-stroke` (`StrokeOutlineJob`), `semio-framework-2d-compute`, `semio-framework-2d-regions`, `semio-framework-2d-retire`. Raster and pixels: `semio-framework-raster`, `semio-framework-pixels`.

Gap: no axis, tick, log-scale or curve-plot primitive exists in the contract. Interaction curves, M-N diagrams, S-N curves and spectra need a norm-owned generator that emits `Canvas2d` path layers (plus table/text labels). A reusable 2D axis helper would be new work.

## 4. Direct manipulation, events and fast feedback

Event path (CQRS / event sourcing):
1. Node-level bindings: `BuiltNode.bindings: Array<ActionBinding>` (`ui-contract/🟦️.ts:21`). Each binding is `{ trigger, action: ActionId, args, capability }`. `ActionId = { scope, name, version }` (`ui-contract/🟦️.ts:36`).
2. `Trigger` (`ui-contract/🟦️.ts:710`): `activate | change | commit | delta | drop | submit | abort | repeatLast | hoverPreview`. `delta` and `abort` are the streaming/cancel pair. `hoverPreview` is non-committing.
3. Surface-level intents: `SurfaceProps.bindings` (for host chrome around an embedded surface).
4. Host to plugin: `UiIntent { surface, revision, node, ... }` (`ui-contract/🟦️.ts:771`). Plugin turns it into an action, a command, or a mutation through `WindowKindDefinition.actions: Vec<ActionDefinition>` (`ActionKind::Mutation` etc., `bounded_catalog(...)`).
5. Mutations and history: `Fem3dMutation` (`✏️s/🔌️plugins/🏗️fem/.../🧬️mutations`), `ToolTransaction` / `ToolStep` / `ToolYield` (`semio-framework-tool-machine`, `FW/🔨️modules/⏯️tool-run`, `FW/🔨️modules/⏪️time-travel`).
6. Plugin to host: `UiPatch { surface, baseRevision, revision, ops }` with `UiPatchOp = upsert | setComponent | setLayout | ...` (`ui-contract/🟦️.ts:840-845`). Full state via `UiSnapshot`. Host intake: `ENG/🧱️elements/🔌️PluginRuntime/🪟️retained-ui-patch` and `🧺️handle-ui-intake`.
7. Scene refresh: `ViewModel` projection, `render`, then `scene_surface`.

Gizmo and pointer:
- World 3D: `ENG/🧱️elements/🌐️World3dHost/🎛️marquee-and-gumball`, `🧮️gumball-math`, `👆️pointer-and-drop` (`handle-pointer-up`, `catalogue-drag-enter`, `active-utility`), `👇️hover-and-placement`, `🎯️marquee-and-bridges`. Gizmo visibility controlled by `World3dPresentation.show_gizmo`.
- 2D: `ENG/🧱️elements/📐️Canvas2dHost/🧭️gumball` (schema `🧬️schema/🔣️gumball-meta`), `👆️pointer-lane`, `🗂️layers-session`.
- Plugin-side gumball tool (reference): `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../✏️editor/🕹️interaction/🧭️gumball/🦀️.rs` (429 lines). It uses `semio_framework_tool_machine::{GestureTool, GesturePhase, ToolMachineRunner, ToolTransaction, ToolYield, ToolAbortReason}`. A streamed gesture accumulates ticks into one open transaction, previewed by every window, and commits the net leaf once. Abort leaves no trace.
- Picking: `World3dScene.pick_targets_json` (sub-object) and per-instance `interactionGranularityId`. Interaction domain declared via `InteractionDefinition` (`FW/.../🔌️plugin/🕹️interaction/`), dispatched as `interactionSelect`.

Transient and presence lanes (fast feedback, ephemeral):
- Transient state per window: `FW/.../🔌️plugin/🪟️window/🫧️transient/🦀️.rs` ("exact-window transient state with independent generation authority"). Types: `WindowTransientMutation`, `WindowTransientOwner`, `WindowTransientSnapshot`, `TransientView` (`🪟️window/🎚️config` re-exports).
- Artifact viewer associated types: `type Presence`, `type PresenceMutation`, `type Transient`, `type TransientMutation` (`FW/.../📜️declaration/🪟️surface/👁️artifact-viewer-trait/🦀️.rs:99-102`).
- Publication lanes: `Presence(...)` and `Transient(...)` (`FW/.../🧩️composition/📬️publication/🦀️.rs:36-37`). Ephemeral emission `EphemeralEmit` (`FW/.../📜️declaration/🧩️app`), and `EphemeralSnapshot` (`⚛️reactor/🔌️app`).
- Persisted vs ephemeral and local vs shared is the AGENTS requirement. The gumball gesture above is persisted only in the transient facet and is never history.
- The norm plugin already has empty `🫧️transient` facets in `🗿️artifacts/⚖️en1990/.../✏️editor/🎭️modes/✏️edit/🪟️windows/{📊️results,📥️inputs}/🫧️transient/` and `⚡️din18599` (placeholders, `📌️.empty.md`). They are slots, not implementations.
- Progress and cancel: `ProgressProps { completed, total, valueText }`, `Activity = waiting|loading|idle|finished`. Job lanes: `semio-framework-plugin-job`, `semio-framework-job`, `semio-framework-plugin-operation` (cancellation).

## 5. Geometry kernels

B-rep kernel `FW/🔨️modules/🧊️3d/📐️brep/` (directories are the fleet taxonomy; crate names in parentheses):
- Engine: `semio-framework-3d-brep-engine` (`⚙️engine`), `semio-framework-3d-brep-kernel-contract`, `semio-framework-3d-brep-shape-value`, `semio-framework-3d-brep-mesh-io`. Sync and job variants: `extrude_wire_sync`, `sweep_sync`, `revolve_sync`, `offset_solid_sync`, `intersect_sync` (`⏱️operation-jobs`).
- Representation: `semio-framework-3d-brep-representation`, `-vector`, `-curve`, `-surface`, `-polynomial`, `-tolerance`.
- Operations (`🛠️operations/`):
  - Construction and primitives: `semio-framework-3d-brep-construction`; `make_box(body, w, d, h, rec)` (`🧱️primitives`).
  - Extrusion, sweep, revolve (`➡️sweep`, `🌀️revolve`): `semio-framework-3d-brep-sweep`; `extrude_face`, `extrude_wire`, `sweep_along_path(body, profile, path, rec)`, `revolve_face`.
  - Booleans (`🔀️boolean`): `semio-framework-3d-brep-boolean`; `boolean_solid(body, a, b, op, tol, rec)`, `boolean_job` (resumable), `boolean_solid_mesh_preview`.
  - Offset (`↔️offset`): `semio-framework-3d-brep-offset`; `offset_solid`, `offset_face`.
  - Blend (`🎨️blend`): `fillet_edges`, `fillet_variable`, `blend-job`.
  - Also: euler (`🔺️euler`), sew (`🧵️sew`), transform (`🔁️transform`), `staged`.
- Queries: `semio-framework-3d-brep-queries` (mass properties, bounding volume, classification), `-tessellation` (`tessellate_solid`, `tessellate_solid_with_report`, `tessellate_face`, `tessellate_wire`), `-analysis`, `-intersect` (`intersection_test`, `intersect_surface_surface`), `-validation`.
- Cad uses: `semio-framework-3d-brep-engine`, `-representation`, `-tessellation` (`📐️cad/Cargo.toml`).

Mesh kernel `FW/🔨️modules/🧊️3d/🥽️mesh/`:
- `semio-framework-3d-mesh-document` (halfedge mesh ops, polygon triangulation `triangulate_polygon`, tessellation job, OBJ export `⏱️tessellation-job-and-obj-export`), `semio-framework-3d-mesh-journal`, `semio-framework-3d-mesh-quality`, `semio-framework-3d-mesh-clipping`, `semio-framework-3d-mesh-reconstruction`, `semio-framework-3d-mesh-contract`.
- Modelling operations: bevel and decimate, inflate and fill holes, loop cut, subdivide (`subdivide_until_flat`), knife, mirror. In `🛠️modeling/`.
- Lowpoly uses `semio_framework_3d::mesh::` (root crate `semio-framework-3d`, `FW/🔨️modules/🧊️3d/📦️packages/🦀️rust`).
- TS entry `@semio-tech/framework/3d` (`FW/📦️packages/🟦️typescript/🧊️3d/🟦️.ts`): `kernelGeometry`, `emptyMeshTransfer`, `solidRef`, types `MeshTransfer`, `FaceGroup`, `EdgeGroup`, `EdgeInfo`, `FaceInfo`, `EdgeCurve`, `ArcPlaneFrame`, `Vec3`.
- Other: `semio-framework-3d-rigid` (`🌀️rigid`), `semio-framework-3d-collision` (`🧿️collision`).
- Also `🧮️math/🧮️algebra/🔲️dense` (dense linear algebra, `🔲️dense/↔️inverse`).

FEM solver mesh: `semio_s_artifact_fem_3d_engine::fem3d_engine::mesh_preview::fem3d_mesh_preview(doc)` returns solid boundary triangles (`boundary_tris`, `points`, `node_ids`, `solid_id`). Used by the fem scene builder.

Practical note for norm: for an extruded I-section or any beam solid you do not need a kernel. Build the inline `{ id, data: { positions, normals, colors, indices } }` mesh directly (fem does this). Use the brep kernel only if you need true booleans, fillets or sweeps.

## 6. Reusable structural visual components

No beam, column or section renderer exists in the framework. Reusable pieces:
- Fem 3D scene builder (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../🎬️scene/🦀️.rs`, 368 lines): `fem3d_scene_parts(doc, displacements, deform_scale, nodal_stress)` returns `(meshes_json, instances_json)`. Nodes are boxes, members are box prisms oriented by `quat_z_to(dir)`, supports are cones (`quat_y_to`), loads are arrows (cone head plus box shaft, `quat_roll_z`). Deformed positions via `fem3d_deformed_position`. Solids are meshed boundaries with per-vertex colours. Crate `semio-s-artifact-fem-3d-scene`.
- Colour bands: `semio_s_fem_app_surface::von_mises_color(value, min, max)` maps to 8 discrete hex bands `VON_MISES_BANDS` (blue to red), `hex_to_rgb01(hex)`. Path: `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`. `world_3d_surface(id, &World3dScene)` wraps `scene_surface`.
- Result modes: `DisplayMode { Static, Modal(usize), Buckling(usize) }`, `normalize_mode_shape`, `MODE_SHAPE_AMPLITUDE_RATIO`, `ResultDisplay` (same file).
- Fem 2D: moment diagrams (`MOMENT_SCALE_2D`), filled-triangle stress contours (`filled_triangle_layer`), `fem2d_region_mesh_triangles`, `fem2d_deformed_shape_layers` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/...`).
- Domain-neutral utilisation colouring: `World3dScalarField` + `World3dColorRamp` + legend (section 2). A norm utilisation map can feed `values` per element or per vertex with `range` 0 to 1 (or 0 to 100 percent) and a `Coolwarm` or `Viridis` ramp. This is the closest existing fit; the fem band palette is an alternative.
- Section cap and plane (`World3dSection`, `World3dSectionCap`) and annotation types (`World3dDimension`, `World3dLeader`, `World3dText`, `World3dMarker`, `World3dAngle`) with `project_world3d_annotations`. These cover dimension labels and member callouts.
- Other plugins with section-type windows (not audited in detail): `📐️cad` (`🏛️structure-classic`, `🏢️building`, `📐️shape`), `🏙️bim` (`📄️sheet`, `🗺️plan`).

## 7. Constraints and dependencies for the norm plugin

Repo rules (AGENTS and fleet brief):
- No runtime external libraries. Use framework crates and system libraries only. External libs are allowed only behind an interface.
- The UI vocabulary goes through the choke point `semio-framework-ui-contract` plus `semio-framework-plugin`. The wgpu host (`semio-framework-ui-wgpu-*`) must stay out of the guest graph (comment in `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🦀️.rs`).
- Exports must not expose external types. Re-export explicitly.
- Decision: the norm `Cargo.toml` workspace deps currently list `semio-framework-plugin`, `semio-framework-ui-contract`, `semio-framework-ui-locale`, `semio-framework-value`, `semio-framework-pack-json`, `semio-framework-os-kernel`, `semio-framework-job`, `semio-framework-dsl-record`, `semio-framework-schema-registry`, `serde`, `serde_json` and others (`✏️s/🔌️plugins/📕️norm/Cargo.toml`).
- Not yet listed, but needed to use scene types directly: `semio-framework-ui-scene` (`FW/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust`), `semio-framework-ui-scene-world3d`, `semio-framework-ui-scene-plane`, `semio-framework-ui-scene-modelling`. These are framework crates and fit the constraint. `semio-framework-plugin` already re-exports `world3d_scene`, `WorldSunConfig`, `world3d_selection_json`, `scene_surface`, `Canvas2dScene`, `BuiltNode`, `UiAssemblyResult`, so `semio-framework-plugin` alone may be enough for 3D and 2D windows.
- Kernel crates (`semio-framework-3d-brep-*`, `semio-framework-3d-mesh-*`, `semio-framework-2d-*`) are framework-internal and allowed. Add them only if needed (see section 5).
- `serde` and `serde_json` are already in the norm workspace. They are third-party, so keep them behind the existing pack/JSON facade if the rule is applied strictly.

Recommended approach for the norm plugin (for the next step, not implemented here):
1. Structural 3D: build `meshes_json` with inline `{id, data}` meshes (beam prism, column, plate, foundation block, joint) and instances tagged with `interactionGranularityId` per member or joint. Colour with `World3dScalarField` (`Coolwarm`, utilisation 0 to 1) and a `World3dScalarLegend`. Use `world3d_scene` plus `scene_surface` (via `semio_framework_plugin`).
2. 2D diagrams: a norm-owned plot generator emitting `Canvas2dScene.layers_json` path layers (curves, axes, labels). No built-in plot primitive exists.
3. Direct manipulation: pick via `pick_targets_json` and `interactionGranularityId`, edit via `ActionBinding` (`drop`, `delta`, `commit`, `abort`), gesture through `semio_framework_tool_machine` transients, following the fem gumball pattern.

## Key file index
- `FW/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (BuiltNode, Component, Trigger, SurfaceKind, UiPatch, WindowLayout)
- `FW/🔨️modules/🛂️manifest/🧱️app-surface/🪟️window-kinds-and-panels/🦀️.rs` (WindowKindDefinition)
- `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🦀️.rs` (WindowKit trait, FormPanelBuilder, ActionFactory)
- `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌍️world3d/🦀️.rs` and `☀️sun/🦀️.rs` (world3d_scene, mesh and selection helpers)
- `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🦀️.rs` (MeshWindowKit)
- `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🦀️.rs` (scene_surface, window_kind_definition)
- `FW/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🌍️world3d/🦀️.rs` (World3dScene)
- `FW/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🔲️canvas2d-scene/🦀️.rs` (Canvas2dScene)
- `FW/🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling/` (scalar field, legend, annotations, section)
- `ENG/🧱️elements/🌐️World3dHost/` (3D host), `ENG/🧱️elements/📐️Canvas2dHost/` (2D host), `ENG/🧱️elements/🗣️Interpreter/`, `ENG/🧱️elements/🔌️PluginRuntime/`
- `FW/🔨️modules/🧊️3d/📐️brep/` and `FW/🔨️modules/🧊️3d/🥽️mesh/` (kernels), `FW/🔨️modules/◻️2d/` (2D geometry)
- Fem reference: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/` (`🎬️scene`, `👁️viewer`, `✏️editor/🕹️interaction/🧭️gumball`) and `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/`
- Norm: `✏️s/🔌️plugins/📕️norm/Cargo.toml`, `📇️registry/🧬️contract/🖥️app-surface/`, `🗿️artifacts/*/…/🫧️transient/` placeholders
