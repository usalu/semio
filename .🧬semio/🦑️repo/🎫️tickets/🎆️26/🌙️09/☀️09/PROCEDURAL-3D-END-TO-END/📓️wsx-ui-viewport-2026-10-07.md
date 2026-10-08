# WSX UI and 3D Viewport Inventory for Procedural 3D Modelling

Date: 2026-10-07. Scope: read-only audit. No source files, builds, tests, git state, or tickets were touched.

Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`. Purpose: inventory what an artifact editor can already use to build an end-user B-Rep/mesh modelling experience in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`.

Path note: the task named some paths differently from the tree. Actual locations:
- Editor panels: `✏️editor/📌️panels/🔍️inspection` (not `inspection`)
- Viewer preview: `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview` (the mode folder is `👁️view`, not `view`)
- The generation3d subset root is `🏅️standards/🔖️1/🪆️subsets/✳️any/`. All generation3d paths below are relative to `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/` unless shown absolute.

Abbreviations used below:
- `FW` = `🧰️framework/`
- `UI` = `FW/🔨️modules/🖱️ui/`
- `OS` = `FW/🛍️products/💻️os/🔨️modules/`
- `R` = `OS/📺️renderer/🧑‍🎨engine/`
- `W3H` = `R/🧱️elements/🌐️World3dHost/🟦️.tsx`

---

## 1. Generic UI elements (usable from Rust guest code)

### 1.1 Schema: closed `Component` union

Source: `UI/🧬️contract/🧬️schema/🦀️.rs` (SchemaMetadata table). Each entry below is one `Component` variant; line numbers are the `name:` lines.

| Component | Schema line | Renders | Value types and notes |
|---|---|---|---|
| `container` (ContainerProps) | 203 (union) | Layout box, row/column/stack/grid/absolute/scroll | Layout via `LayoutSpec` (StackLayout, GridLayout, ScrollLayout, AbsoluteLayout) |
| `text` (TextProps) | 203 | Text | Label |
| `button` (ButtonProps) | 203 | Button | Icon string, action via `Trigger::Activate` |
| `input` (InputProps) | 344 | Text, long text, number, date, color, file | `InputKind` at line 328: `text`, `longText`, `number`, `date`, `color`, `file`. Value is string. Number adds `min`, `max`, `step`, `precision`, `snaps`, `displayFactor`, `limits`, `draftTarget`. `commit` convention (e.g. `blur`). React renders a native color picker for `color` (`R/🧱️elements/🗣️Interpreter/🟦️.tsx`:1531-1532) |
| `select` (SelectProps) | 654 | Dropdown | `value` string, `items: SelectItem[]`, `appearance` = `menu` or `segmented` (radio row) |
| `toggle` (ToggleProps) | 876 | Button toggle or checkbox | `on: boolean`, `icon`, `text` label, `appearance` = `button` or `checkbox` |
| `numberStepper` (NumberStepperProps) | 463 | Unbounded numeric entry with +/- | `value: number`, `step`, `min`/`max` optional, `precision`, `snaps`, `unit`, `displayUnit` |
| `slider` (SliderProps) | 702 | Range slider or rotary dial | `value`, `min`, `max`, `step`, `snaps` (detents), `scale` = `linear` or `log`, `appearance` = `track` or `dial` (693), `unit`, `displayUnit`, `displayFactor`, `limits` (soft travel vs hard limits, 1148) |
| `ring` (RingProps) | 203 | Ring indicator | Progress-like |
| `iconSelect` (IconSelectProps) | 203 | Icon picker | `value` string, `uniform`, `classifierKind` |
| `progress` (ProgressProps) | 536 | Determinate or indeterminate bar | `completed`, `total` (null = indeterminate), `valueText` (localized) |
| `tree` / `treeSection` / `treeItem` (TreeProps 964, TreeItemProps) | 964 | Hierarchical tree with windowed children | TreeItem: `label`, `description`, `icon`, `defaultOpen`, `draggable`, `dragData`, `dimmed`, `selected`, `granularity`, `inlineToolbar`, `detail`, `rowActions`, `target` |
| `table` / `tableRow` (TableProps 798) | 798 | Column-headed table with windowed rows and row actions | `columns`, `rowLabel`, `columnLabel`, `window`, `columnWindow` |
| `image` (ImageProps) | 320 | Image | `src`, `alt` |
| `surface` (SurfaceProps 773) | 773 | Embedded product surface | `kind` from `SurfaceKind` (763): `canvas-2d`, `world-3d`, `node-graph`, `text-editor`, `table`, `paint-2d`, `virtual-file-system`, `tiled-map`, `board-2d`, `icon-render`, `ink-canvas`, `graph-timeline`, `block-list`, `diff-view`, `event-feed`. Also `docSchema` `"<kind>@<version>"`, opaque `doc`, `bindings` |
| `extension` (ExtensionProps) | 276 | Escape hatch to a named extension slot | `extension` string address plus `props: UiValue` |
| `separator` | 203 | Divider | - |
| `keyValueList` | 203 | Key/value list | - |

Supporting types:
- `Label` = plain string (409). Localization happens upstream, before the wire.
- `UiValue` (1209) = JSON-shaped value (the one recursive type).
- `Trigger` (1009): `activate`, `change`, `commit`, `delta`, `drop`, `submit`, `abort`, `repeatLast`, `hoverPreview`.
- `ActionBinding` (104): `{trigger, action, args, capability}`. Every element declares its behaviour as bindings, not per-field callbacks.
- Density/Tone/Emphasis/SizeToken/Anchor exist in the same schema for customization (theme-driven).

Element implementations (React and wgpu) live in `UI/🧱️elements/` (66 entries) and `R/🧱️elements/`. Names relevant to modelling: `Input`, `Slider`, `Select`, `Toggle`, `Tree`, `Table`, `Tabs`, `Dialog`, `Popover`, `ContextMenu`, `Wizard`, `Form`, `Stepper`, `Chip`, `Ring`, `Progress`, `Skeletons`, `Canvas`, `Scene`, `Diagram`, `LayeredOverview`, `Resizable`, `Collapsible`, `Log`, `Ports`, `HistoryTable`. Note: there are two `Panel` elements (`🛰️Panel` and `🖼️Panel`), a naming inconsistency to resolve.

There is no dedicated `vec3`, `color`-with-alpha, `inspector`, or `dropdown` element. Composition used instead (see 1.3).

### 1.2 Localization (EN and DE)

- Every declaration label is a `LocalizedLabel::native(en, de)` pair. Example: `✏️editor/🎭️modes/🧬️generate/🦀️.rs`:20 `LocalizedLabel::native("Generate", "Generieren")`. Same pattern for modes (`edit`, `view`), windows (preview `Vorschau`, `…/👁️preview/🦀️.rs`:34), utilities (`Move`/`Verschieben`), actions (`✏️editor/🦀️.rs`:3095-3117), and panel tabs (`✏️editor/📌️panels/🔍️inspection/🦀️.rs`:28). A native pair is mandatory, so there is no default language.
- Runtime UI label model: `UI/🧱️elements/📚️I18n/🟦️.tsx` lines 15 and 18 (`UiLabelPair` with `normal` and `beginner` tiers; `UiLabelValue` with optional `manual` and `tutorial`).
- Totality test: `UI/🧱️elements/📚️I18n/🧪️tests/🔬️translation-totality/🟦️.ts`:9 asserts every frozen outcome code has distinct EN and DE labels for both tiers.
- Number display formatting: `UI/🧬️contract/🔢️number-format/` (shared).

### 1.3 How the inspector composes its fields today

`✏️editor/📌️panels/🔍️inspection/🦀️.rs`:
- Line 64: `input(InputKind::Number)` for numeric values.
- Line 99: `LongText` for notes.
- Lines 148 and 473: vector-like values (vec3-like) are built as one number input per axis, with an axis suffix (`x`, `y`, `z`) on the key and label. This is the only vec3 path. There is no vector widget.
- Lines 212 and 231: `editable_input` and `editable_field`. A missing `kind` produces a checkbox toggle (line 231-245); otherwise an `input` bound by `Trigger::Commit` (blur) to `setWidgetInput`.
- Line 385: material base color is an array of numbers (per-channel), not a color picker. Not verified whether the picker is used elsewhere.
- Flow-level widgets (`🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`:103-132): `NodeChrome` variants `Plain`, `Slider`, `Note`, `Image`, `Variable`.

### 1.4 Accessibility

- `UI/🧬️contract/♿️accessibility/🦀️.rs`: `Liveness` (line 23: `off` / `polite` / `assertive`) and `AccessibilitySpec` (line 37: `label`, `description`, `live`, `shortcut`, `hidden`).
- Role is implied by the component, never sent (comment at line 37). `accessibility_role` (66) and `accessibility_is_focusable` (133) are shared by all renderers.
- Disabled row actions stay focusable and carry a `reason` (`RowAction` doc comment, `UI/🧬️contract/🧬️schema/🦀️.rs`:585).
- The World3d preview announces its canvas through `labels.preview_canvas` and `labels.preview_canvas_hint` with `Liveness::Polite` (`✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`:99).
- Tests exist: `R/🧪️tests/♿️native-accessibility`, `R/🎯️targets/🧊️wgpu/♿️accessibility-mirror`, `R/🧪️tests/♿️wgpu-accessibility-interaction`.

---

## 2. 3D viewport capabilities

Two renderers exist. The React World3d host (`W3H`, 8302 lines) carries component (face/edge/vertex) picking. The native wgpu target (`R/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`) has no sub-element id tables on the GLB path (see 2.6).

### 2.1 Payload the renderer accepts

`W3H`:
- `WorldMeshData` (158-176): `positions`, `normals`, `indices`, `colors` (per-vertex RGBA, linear), `uvs`, `faceIds`, `vertexIds`, `edgePositions`, `edgeIds`, `componentReferences: Record<granularity, string[]>`, `attributes` (vertex/corner/face/edge domain; semantic normal/uv/color/material/custom), `materials`, `textures`, `edgeUvs`, `edgeIsSeam`.
- `WorldInstanceRecord` (200): `id`, `meshId`, `position`, `rotation` (quaternion), `scale`, `selected`, `hovered`, `highlighted`, `disabled`, `provisional`, `smoothShading`, `interactionId`, `componentSource`, `interactionGranularityId`.
- Built-in mesh kinds (1329-1352): `plane`, `sphere`, `uvSphere`, `icoSphere`, `cylinder`, `cone`, `torus`, `vortex-marker`, `vertex-marker`. Default is a box. Resolved client-side by `meshDataFromKind` (1358).
- Selection record (250): `ids`, `hoveredId`, `hoveredComponent {objectId, mode, id}`, `targets {mesh, vertex, edge, face}`, `selectionMode`, `componentIds`, `transformMode`, `gumballConfig`, `gumballLiveDispatch` (276), `faceDragActive` (281), `showEdges`.
- Camera: `parseCameraState` (745), projection `perspective` or `orthographic` (731-780), FOV, zoom. Orthographic framing at 1288-1316.

Component reference resolution: `world3dComponentInteractionTarget` (5997) and `componentReferences` lookup at 6003 map a picked face/edge/vertex to a domain granularity.

### 2.2 Sub-element picking, hover, selection

- Face pick: 3378-3425. Uses `faceIndex` to look up `faceIds`. Sets `hoveredComponent` with mode `face`.
- Edge pick: 3443-3490. Edge layer shown when `targets.edge`, `showEdges`, or mesh selection is active. Uses `edgeIds`.
- Vertex pick: 3497-3530. Uses `vertexIds` (built by `buildVertexOverlayGeometry`, 2386).
- Overlay highlights for components: `buildFaceOverlayGeometry` (2316), `buildEdgeOverlayGeometry` (2372), `buildVertexOverlayGeometry` (2386). Hover paints with the secondary "highlighted" style, selection with primary (523-536 comment).
- Marquee selection of components by mode: vertex 4773-4779, edge 4781-4786, face 4788-4796 (`pointsSatisfyMarquee` with method and coverage).
- Push/pull: `faceDragActive` (281) and `onFaceDragStart` (3377) start a face drag gesture, `worldFaceDragEnd` on release.
- Mesh style states (`resolveMeshStyle`, 475): `disabled`, `provisional`, `celebrated`, `selected`, `highlighted`, `hovered`, `neutral`.

### 2.3 Gumball / transform gizmo

- Handles: `translate`, `rotate`, `scale`. Config from `gumballConfigForTransformMode` (3138): mode `transform` gives move axes and planes plus rotate; `rotate` gives rotate only; `scale` gives scale axes, planes, and uniform. A plane restriction can be applied from the window projection (`worldGumballConfigForProjection`).
- Live streaming: `gumballLiveDispatch` (276) sends `phase: stream` ticks inside one open transaction; `commit` on release; `abort` with `reason` on blur. Gesture state machine `worldGumballStep` (2957). Preview is computed locally (`applyGumballLivePreviewDeltaToPose`, 3029).
- Verbs are window-kind owned (`R/…/📇️world3d-surface-verbs` fixture): a window kind must declare `translateSelection`, `rotateSelection`, or `scaleSelection` for its gumball to appear. The procedural preview declares all three (`W3H` fixture `🧫️fixtures/📇️surface-verbs.json`).
- Generation3d commands: `✏️editor/🎮️commands/↔️translate-selection/🦀️.rs` (36 lines), `🔄️rotate-selection`, `📏️scale-selection`, `🧭️transforms`.

### 2.4 Snapping

- Grid snap only: `snapWorldPointToGrid` (5199-5202) rounds to a `gridFactor`. Used for catalogue drop origins (5280-5284) and the volume brush origin (5262-5267, always snapped).
- No vertex, edge, midpoint, face-centre, or axis/angle snapping was found in `W3H`.

### 2.5 Measurement, annotations, sections, display modes, overlays

- Measurement or dimension labels: none. There is no in-scene text (`Html`, `Text3D`, troika, billboard, `drei` text): zero hits in `W3H`. "measure" in the codebase means the window measures rail (`WindowMeasure`), not geometric measurement.
- Section or clipping planes: none (no `clippingPlanes` or `localClipping` in `W3H` or the wgpu renderer). Also none in the grep across `R/` (`section`, `clip` hits were UI-level).
- X-ray: none. The only transparency is a 0.35-opacity wireframe volume-brush preview (`W3H`:4603).
- Display modes exist at the app level, not the renderer level. `GENERATION_3D_SHOW_MODES` = `shaded`, `shaded+edges`, `wireframe`, `points` (`✏️editor/🎚️config/🦀️.rs`:50). Wireframe and points are realized by stripping triangles in the tessellated payload (`✏️editor/🎭️modes/.../🧵️preview-eval/🦀️.rs`:600-620), not by the renderer. Renderer show edges is a boolean (`showEdges`, 278; `(showEdges ?? true)` at 3443).
- Flat vs smooth shading: `flatShading` (2410-2444) and `smoothShading` (218).
- Heatmap or curvature coloring: no UI element, legend, or colour-ramp component. The payload can carry per-vertex `colors` (161) and custom `attributes` (171), so scalar-field colouring is achievable by payload, but nothing produces or labels it. Zero "heatmap" hits in `R/`.
- Engagement preview overlay (`EngagementPreviewLayer`, 4569-4620): non-raycastable `point`, `segment` (line), `box-preview` (wireframe box), and `linear-handle` (cylinder). Preview only.
- Grid and LOD: `lod.gridFactor`, `lodGridStepWorld` (1127-1146).
- Sun/light: `WorldSunConfig` (`🔌️plugin/🦀️.rs` around line 47884+) with azimuth, elevation, intensity, color, toggle.

### 2.6 Camera

- Orbit controls, perspective and orthographic projections, FOV, zoom, frame-to-bounds (`world3dFrameCameraFromBounds`, 1158; `world3dFrameCameraFromInstances`, 1187; bounds margin `WORLD3D_FRAME_BOUNDS_MARGIN`, 972).
- Camera state round-trips through `setCamera` (`✏️editor/🎮️commands/📷️set-camera`).
- Not found: named views (front/top/iso), view cube, orientation gizmo. Zero hits for `isometric`, `frontView`, `topView`, `ViewCube`, `viewPreset`.

### 2.7 Native wgpu renderer caveat

`R/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`:2774: the GLB outline materializer writes `face_ids: 0, vertex_ids: 0`. The native (desktop) path therefore carries no sub-element id tables for GLB content, so face/vertex picking is React-host only in this tree. The AGENTS priority is desktop, then mobile, then tablet, so this is a parity gap.

### 2.8 Modelling capability in the kernel and mesh engine (not wired to the viewer)

- B-Rep kernel `FW/🔨️modules/🧊️3d/📐️brep/`:
  - Operations: booleans (`🛠️operations/🔀️boolean`), blend/fillet/chamfer (`🎨️blend`), offset (`↔️offset`), sweep and loft (`➡️sweep`), Euler ops (`🔺️euler`), primitives (`🧱️primitives`), intersections.
  - Queries: `solid_volume` (`📏mass-properties/🦀️.rs`:49), `solid_surface_area` (85), `solid_center_of_mass` (99), `solid_bounding_box` (124), `solid_mass_properties` (157), `face_area` (372), `edge_length` (388), `distance_solid_solid` (404), `closest_point_on_solid` (516). Tessellation with report and progress: `🧩tessellation/🦀️.rs` (`tessellate_solid_with_report` 88, `TessellationJob` 183).
  - Finding: these query functions are referenced only inside the kernel (`boolean`, `⚙️engine`, `mass-properties`). Nothing in `🌊️flow` or generation3d calls `solid_volume`, `solid_mass_properties`, or `face_area`. Analysis values exist in the kernel but are not exposed to the artifact or UI.
- Mesh modelling `FW/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs`: budgeted, cancellable jobs (`MeshModelingStep::{Working, Done, Cancelled}`, line 23; `step(budget)`, line 88). Work kinds (around line 40): `Bevel`, `Decimate`, `Mirror`, `Merge`, `Subdivide`, `LoopCut`, `Orient`, `FillHoles`, `Inflate`, `Import`, `Knife`, `Transform`. This satisfies the progress and cancel rule.
- Generation3d already uses this: `editMeshSelection` (`✏️editor/🦀️.rs`:2862 args; describe 3104) inserts an adjustable mesh modifier on selected faces, edges, or vertices. `knifeMeshSelection` (2895 args; describe 3105) cuts one face along a line through two points.

### 2.9 Gaps for a modelling app (summary)

| Need | Status | Evidence |
|---|---|---|
| Per-face highlight | Exists | `buildFaceOverlayGeometry` 2316 |
| Edge picking | Exists | 3443-3490 |
| Vertex picking | Exists | 3497-3530 |
| Marquee of components | Exists | 4773-4796 |
| Push/pull face drag | Exists | 281, 3377 |
| Gumball translate/rotate/scale, live | Exists | 2957, 3138 |
| Plane-constrained gumball | Exists | `gumballConfig.plane` |
| Snapping to vertex/edge/midpoint | Missing | Grid only, 5199 |
| Axis/angle constraint guides | Missing | Not found |
| Dimension labels, in-scene text | Missing | No text primitive in `W3H` |
| Section/clipping planes, caps | Missing | Not found |
| X-ray / transparent display mode | Missing | Only a 0.35-opacity brush preview, 4603 |
| Heatmap/curvature/deviation colouring with legend | Missing (payload path exists) | `colors`, `attributes` 161, 171; no legend |
| Analysis readout (volume, area, centroid) | Kernel exists, not wired | `📏mass-properties` |
| Named views, view cube | Missing | No hits |
| Sub-element picking on native wgpu | Missing (GLB zeroes ids) | `wgpu/🧊️renderer/🦀️.rs`:2774 |
| Topology adjacency (face-edge-vertex) for "select loop/ring", grow, shrink | Missing | Payload has id tables, no adjacency |

---

## 3. How an artifact declares tools, utilities, actions, panels

### 3.1 Taxonomy (member kinds and placement)

File: `FW/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

Member emoji and slugs (SSOT):
- panes `🍱️` (4633), widgets `🪀️` (4638), utilities `🪛️` (4643), actions `🎬️` (4648), configuration/options `🎚️` (4693), tools `🛠️` (4698), panels `📌️` (4703).
- Modes dir `🎭️modes` (28286). Windows dir `🪟️windows` (28287).
- Commands are `🎮️commands` (`"kind": "command"`, around line 28033). Options as `☑️options`.

Structure rules:
- `modeChildDirs` (28969): `🪟️windows`, `🎮️commands`, `🎚️config`, `👥️presence`, `🫧️transient`. All five required.
- `windowChildDirs` (29003): `🍱️panes`, `🪀️widgets`, `🪛️utilities`, `🎬️actions`, `☑️options`, `🎚️config`, `👥️presence`, `🫧️transient`.
- `windowRequiredChildDirs` (29013): `🎬️actions`, `🪛️utilities`, `☑️options`, `🎚️config`, `👥️presence`, `🫧️transient`. Panes and widgets are optional.
- `taxonomyLeafParentDirs` (29030): leaf parents include panes, widgets, utilities, actions, options, commands, tools, panels, config, presence, transient.
- Required empty directories carry `📌️.empty.md` markers. Example: the edit preview's `🎬️actions`, `☑️options`, `🪛️utilities` are empty in `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/`. Declarations for those members live in Rust (section 3.2), not in the files. The directory is the placement authority, the Rust builder is the declaration authority.

### 3.2 Registration (Rust, generation3d editor)

Declaration types (framework manifest, `FW/🔨️modules/🛂️manifest/🦀️.rs`):
- `ModeDefinition` (5256): `id`, `label: LocalizedLabel`, `icon_id`, `tools: Vec<ToolRef>`, `layout_id`, `commands`.
- `WindowKindDefinition` (5412): `id`, `label`, `body_key`, `surface_kind`, `icon_id`, `options`, `actions`, `utilities`, `initial_utility_id`, `interactions`, `params_schema`, `output_schema`, `capabilities`.
- `UtilityDefinition` (3418): `id`, `label`, `icon_id`, `group`, `keys`, `cursor`.
- `CommandDefinition` (3501): `id`, `label`, `category`, `icon_id`, `kind`, `args`, `keybindings`, `in_palette`, `semantics`.
- `ActionDefinition` (2465), `ToolRef` (3670), `PanelGroup` (5461: `Workbench`, `Details`, `Display`, `Settings`), `PanelTabKind` (5499), `PanelTabDefinition` (5539).

Builder methods (plugin crate `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`): `mode` (5206), `window_kind` (5252), `window_kind_actions` (5333), `window_kind_action_refs` (5342), `window_kind_utilities` (5352), `panel_tab` (5388), `view_action` (5417), `action_args` (5435), `action_describe` (5447), `command` (5564), `utility` (5585), `interaction` (5591), `tool` (5604). The `Editor::builder` (41023) wraps this.

Generation3d registration, `✏️editor/🦀️.rs`:
- Entry: `Editor::builder(...).document(["semio","procedural","3d"])` at 2721.
- Tool: `.tool(crate::preview_eval::preview_eval_tool_definition())` at 2752. Modes must reference every declared tool, or the builder refuses (doc comment in `✏️editor/🎭️modes/✏️edit/🦀️.rs`:12-18).
- View action and label: `.view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))` at 2793.
- Action args: `.action_args("editMeshSelection", …)` at 2862; `setActiveExample` at 2928; `exportDocument` at 2924.
- Utilities: `.utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move","Verschieben"), "move") })` at 2941-2943, then `.window_kind_utilities(edit_preview::…PREVIEW, vec!["move","rotate","scale"])` at 2944.
- Window-action ownership: `.window_kind_action_refs(...)` at 2971-2999, with a test `every_emitted_action_is_declared_on_its_window_kind` (comment at 2971-2983). Actions not listed are copied to every window.
- Descriptions: `.action_describe(...)` at 3095-3117 (EN/DE, one per action).

Modes (`✏️editor/🎭️modes/…/🦀️.rs`):
- `generate` (label line 20): `ModeDefinition` with `label: LocalizedLabel::native("Generate","Generieren")`, `tools: [PREVIEW_EVAL]`, `layout_id`, and `create_default_layout` with three windows at widths 22/43/35.
- `edit` (label line 22): `ModeDefinition` with `Edit`/`Bearbeiten`, flow window (68%) and preview window (32%).
- `view` (viewer, label line 26): `View`/`Ansicht`, single stack with the preview window.

Panels: `✏️editor/📌️panels/🔍️inspection/🦀️.rs`:25 `PanelTabDefinition { kind: PanelTabKind::App(...), label: native("Details"-group label, "Inspektion"), group: PanelGroup::Details, body_key, children: [] }`.

Windows: `…/👁️preview/🦀️.rs` (edit mode) line 30 `definition()` returns `WindowKindDefinition` with `surface_kind: SurfaceKind::World3d`, `actions: Vec::new()`, `utilities: Vec::new()`, `interactions: Vec::new()`. The window's actual actions, utilities, and measures are declared on the app builder, not in the window file. Show-mode measure: `show_mode_measure` (63) returns `WindowMeasure::Select`. Sun group: `world3d_sun_measures(...)` (`FW/…/🔌️plugin/🦀️.rs`:47934). Render: `render` (89) builds `world3d_scene(...)` (48426) inside `accessible_scene_surface(...)`.

### 3.3 Minimal concrete example (generation3d `setShowMode`)

Command file `✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs` (18 lines):
- `#[dsl(keyword = "show-mode")] pub struct SetShowMode { pub value: String }` (DslRecord, ToValue, FromValue).
- `pub fn handle(payload, doc, cfg, session) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault>` returns `Emit::config(vec![Generation3dConfigMutation::SetShowMode(...)])`.

Wiring:
- Command enum mapping in `✏️editor/🦀️.rs`:139: `"setShowMode" as "show-mode" => set_show_mode::SetShowMode`.
- Dispatch: `✏️editor/🦀️.rs`:2393 `"setShowMode" => Ok(Generation3dCommand::SetShowMode(...))`.
- Declaration: `.view_action("setShowMode", …)` at 2793 and `.action_describe("setShowMode", …)` at 3103.
- Window ownership: listed under `window_kind_action_refs(edit_preview::…PREVIEW, …)` at 2984 and the generate preview at 2999.
- Taxonomy placement: `🎮️commands/👁️set-show-mode/` (command directory, `🦀️.rs`, `🧬️schema` mutation type).

This is the minimum loop: DSL keyword, handler returning config mutation, action descriptor with EN/DE text, window ownership, and a measure that calls it.

---

## 4. Recommendation: components for "create, edit, analyse arbitrary shapes"

### 4.1 Already exists (reuse)

1. World3d surface with mesh, instance, and component payloads (2.1), face/edge/vertex hover and selection, marquee, push/pull (2.2).
2. Gumball translate, rotate, scale with live transactional streaming and plane constraints (2.3).
3. Inspector pattern: `PanelTabDefinition` plus tree items with number, toggle, text, and select inputs (1.3). vec3 composed from three number fields.
4. Slider and number entry with detents, soft and hard limits, display units, precision (1.1, `SliderProps`, `NumberStepperProps`).
5. Select, toggle, segmented control, and tree and table with row actions (1.1).
6. Tools and utilities: `move`, `rotate`, `scale` utilities with window-kind ownership; `knife`, `editMeshSelection` as mode commands (3.2, 2.8).
7. Budgeted, cancellable mesh modelling jobs (`MeshModelingJob`, 2.8) and B-Rep tessellation with progress (`TessellationJob`).
8. Localization (EN/DE native pairs, totality test) and accessibility (Liveness, role implication, a11y tests) (1.2, 1.4).
9. Camera: orbit, ortho, framing to bounds (2.6).
10. Show modes (shaded, edges, wireframe, points) at the payload level (2.5).

### 4.2 Must build or extend (priority order)

1. Topology adjacency in the mesh payload. Add face to edge to vertex adjacency (and face normals per face) so the UI can do select-loop, select-ring, grow, shrink, and invert. Today there are id tables and `componentReferences`, but no adjacency.
2. Snapping layer. Vertex, edge-midpoint, face-centre, and on-axis or on-plane snapping with visible guides. Keep grid snap as one option.
3. Dimension and annotation overlay. An in-scene text and leader-line layer (billboarded labels, dimension arrows) that uses the existing number formatter and `displayUnit`/`precision` from the UI contract. This is the largest missing piece for analysis.
4. Section planes with cap rendering, plus an x-ray display mode (transparent faces, no depth write). Add as viewport state (per window), not as a payload mode.
5. Scalar-field colouring for analysis: a generic scalar attribute on vertices or faces (curvature, face area, draft angle, deviation), a colour-ramp element with a legend (min/max labels, localized, with a textual description for accessibility), and a switch in the show-mode measure.
6. Analysis readout panel. Wire the existing kernel queries (`solid_mass_properties`, `solid_surface_area`, `face_area`, `edge_length`, `distance_solid_solid`) into a flow or generation3d node, shown in the inspector with units.
7. Native wgpu parity for sub-element ids (fix the zeroed `face_ids`/`vertex_ids` at `wgpu/🧊️renderer/🦀️.rs`:2774) before shipping the desktop experience, since desktop is the priority.
8. Named views and an orientation gizmo (front, top, right, iso), as camera presets using the existing `setCamera` command.
9. Keyboard component cursor for face, edge, vertex navigation (accessibility). Not verified whether a keyboard cursor already exists in `W3H`; check before building.

### 4.3 Verification status

- Read-only audit. No tests were run and no builds were executed. Claims about "missing" mean "no hits in the paths searched", which is a grep-based inference, not a runtime check.
- Zero hits were checked with `/usr/bin/grep` over the relevant trees. Background greps over the full renderer tree returned only unrelated `xray` hits in wasm binaries and UI-level "section" and "measure" hits.
- The B-Rep kernel query wiring claim (2.8) is a grep-based finding: no references to `solid_volume`, `solid_mass_properties`, or `face_area` outside the kernel.
- Not verified: whether the React Input color picker is used by generation3d's inspector; whether a keyboard face or edge cursor exists in `W3H`; the exact gumball config type file (defined outside the paths searched).

---

## Key file index

- UI schema: `UI/🧬️contract/🧬️schema/🦀️.rs` (Component 203, InputKind 328, InputProps 344, SelectProps 654, SliderProps 702, TableProps 798, SurfaceProps 773, Trigger 1009)
- Accessibility: `UI/🧬️contract/♿️accessibility/🦀️.rs` (Liveness 23, AccessibilitySpec 37, accessibility_role 66)
- Labels: `UI/🧱️elements/📚️I18n/🟦️.tsx` (15, 18); `…/🧪️tests/🔬️translation-totality/🟦️.ts`:9
- World3d host: `R/🧱️elements/🌐️World3dHost/🟦️.tsx` (WorldMeshData 158, WorldInstanceRecord 200, selection 250, mesh kinds 1329-1358, overlays 2316-2386, gumball 2957-3138, picking 3378-3530, marquee 4773-4796, engagement overlay 4569, grid snap 5199)
- wgpu renderer (zeroed ids): `R/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`:2774
- Interpreter (color, number): `R/🧱️elements/🗣️Interpreter/🟦️.tsx`:1192-1195, 1531-1532
- B-Rep queries: `FW/🔨️modules/🧊️3d/📐️brep/💡️queries/📏mass-properties/🦀️.rs` (49-516); tessellation `…/🧩tessellation/🦀️.rs`
- Mesh modelling: `FW/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` (23, 88)
- Taxonomy: `FW/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (4633-4703, 28286-28287, 28969, 29003, 29013, 29030)
- Manifest declarations: `FW/🔨️modules/🛂️manifest/🦀️.rs` (ActionDefinition 2465, UtilityDefinition 3418, CommandDefinition 3501, ModeDefinition 5256, WindowKindDefinition 5412, PanelTabDefinition 5539)
- Plugin builder and world3d helpers: `OS/🔌️plugin/🦀️.rs` (builder 5206-5604, world3d_host 47884, sun measures 47934, world3d_scene 48426)
- WindowMeasure: `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` (1067: Select, Slider, Number, Toggle, Group)
- Generation3d editor root: `…/✏️editor/🦀️.rs` (builder 2721, tool 2752, utilities 2941-2944, window action refs 2971-2999, describe 3095-3117, setShowMode 139, 2393, 2793)
- Generation3d modes: `…/✏️editor/🎭️modes/🧬️generate/🦀️.rs` (label 20); `…/✏️editor/🎭️modes/✏️edit/🦀️.rs` (label 22, tool reference doc comment 12-18); `…/👁️viewer/🎭️modes/👁️view/🦀️.rs` (label 26)
- Generation3d windows: `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` (24-89); `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` (55, 78, 95, 377)
- Generation3d panel: `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` (25, 37, 64, 99, 148, 212, 231, 385, 473)
- Generation3d commands: `…/✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs` (18 lines); `…/↔️translate-selection/🦀️.rs` (36 lines); `…/🥽️edit-mesh-selection/`; `…/🔪️knife-mesh-selection/`
- Show-mode ladder: `…/✏️editor/🎚️config/🦀️.rs`:50; wireframe and points payload: `…/🧵️preview-eval/🦀️.rs`:600-620
- Flow widget chrome: `OS/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` (103-132)
- Surface verbs fixture: `R/🧱️elements/🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json`
