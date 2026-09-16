# Explore: how a plugin editor renders a 3D scene (contract for adding 🔋️energy's 3D "model" window)

Scope: read-only survey of the World3d rendering contract (framework SDK + react renderer) and five
precedent plugins (fem3d, process3d, generation3d, puzzle3d, cad), aimed at adding a 3D window to
`✏️s/🔌️plugins/🔋️energy`'s editor (`🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`).

## 0. Executive summary

- A World3d window is declared like any other window kind (`SurfaceKind::World3d`, a `body_key`), and
  its `render()` returns a `semio_framework_ui_scene::World3dScene` wrapped through
  `semio_framework_plugin::scene_surface(id, SurfaceKind::World3d, &scene)`.
- The scene's big string fields (`meshes_json`, `instances_json`, `selection_json`, plus ~16 optional
  side-channel lanes) ride OUTSIDE the 32 KiB fixed-capacity surface doc as individually paged text
  carriers ("lanes") — this is what lets a >32 KiB mesh set publish at all.
- **fem3d is the simplest correct precedent** (`🧱️model/🦀️.rs`, 32 lines) but it is also the
  *outdated* pattern: legacy scalar `.window_kind(...)` builder call, hand-baked per-vertex colour, and
  **zero picking wired up** (no `canvas-pointer-down` command, no `InteractionDefinition`).
- **process3d / generation3d / puzzle3d / cad all share one modern pattern** instead: a
  `WindowKindDefinition` built via `definition()` and stitched with `.window_kind_def(...)`, a
  `window_kind_interactions(window_kind_id, vec![InteractionRef::new(<domain>)])` binding to a
  **framework-owned interaction domain** declared once via `.interaction(InteractionDefinition{...})`,
  `scene.domain_id`/`scene.domain_granularity_id` set on the `World3dScene`, and
  `scene.fit_json = Some(world3d_fit_json(revision, padding, None))` for auto-framing. **None of them
  have a plugin-owned `canvas-pointer-down` command** — picking is entirely generic: `World3dHost` (the
  React renderer) resolves the raycast hit itself and dispatches the framework-reserved
  `interactionSelect`/`interactionHover` actions directly; the plugin only declares the domain.
- **cad already has a window named "Energy"** (`🔥️energy`, an *energy analysis* pane inside the CAD
  building app — unrelated to the `🔋️energy` plugin, but a byte-for-byte structural precedent) that is
  a 59-line thin wrapper delegating to a shared `build_world_scene_for_pane` — the closest template of
  the five to literally copy.
- The `🔋️energy` plugin's own `Surface { id, name, zone_id, class: SurfaceClass, vertices_m: Vec<[f64;3]>, ... }`
  (`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:299-311`) is already exactly
  "a list of coloured polygons with ids" — each `Surface` is a planar polygon in meters with an id and a
  class that can drive fill colour, matching the ticket's ask almost exactly.

---

## 1. Window-kind definition + simplest full render function

### 1a. Two ways to declare a World3d window kind

**Legacy/scalar** (fem3d only) — `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs:1156-1159`:
```rust
.window_kind(window_model::FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"),
             window_model::FEM3D_BODY_MODEL, semio_framework_ui_contract::SurfaceKind::World3d, "fem-model")
.window_kind(window_results::FEM3D_WINDOW_RESULTS, ..., SurfaceKind::World3d, "bar-chart-3")
```
Builder signature (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:4886`):
```rust
pub async fn window_kind(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>,
    body_key: impl Into<String>, surface_kind: SurfaceKind, icon_id: impl Into<IconName>) -> Self
```

**Modern/object** (process3d, generation3d, puzzle3d, cad, and **already used by 🔋️energy's own
existing windows** `structure`/`zones`/`simulation`) — a `WindowKindDefinition` struct literal returned
by a `definition()` fn, stitched with `.window_kind_def(def)`. Full struct (fields seen in cad's
`🔥️energy/🦀️.rs:24-38`):
```rust
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(), label: LocalizedLabel::native("Energy", "Energie"),
        body_key: BODY_KEY.into(), surface_kind: SurfaceKind::World3d, icon_id: "sun".into(),
        options: WindowOptions::default(), actions: Vec::new(), utilities: Vec::new(),
        interactions: Vec::new(),   // <- filled in later via .window_kind_interactions(), OR inline here
        params_schema: None, artifact_snapshot_schema: None, input_event_schema: None,
        output_schema: None, capabilities: Vec::new(),
    }
}
```
`🔋️energy`'s own `📊️zones/🦀️.rs:42-45` shows the idiomatic way to derive a definition from a shared
kit and only override a few fields:
```rust
pub fn definition() -> WindowKindDefinition {
    let kit = TableWindowKit::editable_window_kind();
    let mut actions = kit.actions.clone();
    actions.extend(self::actions());
    WindowKindDefinition { label: LocalizedLabel::native("Zones", "Zonen"), icon_id: "table-2".into(), actions, ..kit }
}
```
There is no `MeshWindowKit`-derived helper this clean for World3d (`MeshWindowKit` exists —
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30571-30602` — but only process3d uses it via
`MeshWindowKit::render(&MeshView{camera_json, meshes_json, instances_json, selection_json})`; everyone
else hand-builds `World3dScene`). Prefer the object pattern — it's what 🔋️energy already uses
everywhere else, and it's what every 3D precedent except legacy fem3d uses.

### 1b. Simplest complete render function (fem3d model window, minus progress-lease bookkeeping)

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` (50 lines total):
```rust
pub const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
pub const FEM3D_BODY_MODEL: &str = "fem3d.play.model";

pub fn render_with_progress(doc: &Fem3dSnapshot, camera: &Viewport3dOrbit, visual: Option<&crate::live_visual::Fem3dPageVisualLease>)
    -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::editor::fem3d::fem3d_scene_parts;
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    let mut scene = semio_framework_plugin::world3d_scene(
        crate::viewport::scene_camera_json(camera), meshes_json, instances_json,
        semio_framework_plugin::world3d_selection_json("rectangle", &[], None),
        &semio_framework_plugin::WorldSunConfig::default());
    scene.snapshot = visual.map(crate::live_visual::Fem3dPageVisualLease::snapshot);
    crate::app_surface::world_3d_surface(FEM3D_BODY_MODEL, &scene)
}
```
`world3d_scene`/`world3d_selection_json`/`WorldSunConfig` are framework helpers
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `world3d_host` module, lines 38478-38482,
38314ff, 37962-37970). `world_3d_surface` is a **shared fem-plugin helper** (not framework-level!) —
`✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:32-35`:
```rust
pub fn world_3d_surface(id: impl Into<String>, scene: &semio_framework_ui_scene::World3dScene) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    semio_framework_plugin::scene_surface(&id.into(), semio_framework_ui_contract::SurfaceKind::World3d, scene)
}
```
i.e. it's just a one-line wrapper over the real framework primitive `scene_surface`. 🔋️energy should
add its own equivalent (or call `scene_surface` directly).

### 1c. The mesh/instance JSON shapes (what actually goes over the wire)

`meshes_json`: `Vec<{ "id": string, "data": { "positions": [f64], "normals": [f64], "colors": [f64], "indices": [u32] } }>`
— flat per-triangle arrays (3 floats/vertex for positions/normals/colors, one duplicated vertex triple
per triangle so colours/normals can differ per-face-corner). Built in fem3d's
`fem3d_solid_mesh_entries` (`✏️editor/🦀️.rs:636-676`):
```rust
for &[a, b, c] in &solid.boundary_tris {
    let (pa, pb, pc) = (vertex_pos(a), vertex_pos(b), vertex_pos(c));
    // ...compute face normal n...
    let base = (positions.len() / 3) as u32;
    for (idx, p) in [(a, pa), (b, pb), (c, pc)] {
        positions.extend_from_slice(&p);
        normals.extend_from_slice(&n);
        let (r, g, bl) = vertex_color(idx);
        colors.extend_from_slice(&[r, g, bl]);
    }
    indices.extend_from_slice(&[base, base + 1, base + 2]);
}
let mesh_id = format!("solid-{}", solid.solid_id);
meshes.push(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
```
A mesh can ALSO be a built-in procedural kind (`{"id":"box","kind":"box"}`, via
`world3d_meshes_json_from_kinds`) or a URL-backed shared GLB (`{"id":"mesh:<slug>","url":...}`, via
`world3d_mesh_id_from_url`/`world3d_meshes_json_from_urls`) — both resolved client-side
(`meshDataFromKind` in `🌐️World3dHost/🟦️.tsx:1236`), so neither ships tessellated geometry over the wire.

`instances_json`: `Vec<{ "id": string, "meshId": string, "position":[f64;3], "rotation":[f64;4] (quaternion),
"scale":[f64;3], "label": string, "disabled"?: bool, "color"?: string, "selected"?: bool, "hovered"?: bool, "objectKind"?: string }>`.
fem3d's instance for a solid (`✏️editor/🦀️.rs:1108-1114`):
```rust
instances.push(dsl::json!({
    "id": format!("solid-inst-{}", solid.solid_id), "meshId": mesh_id,
    "position": [0.0, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0],
    "label": solid.solid_id,
}));
```
**Per-mesh ids for picking**: `instances_json[i].id` is the identity a domain-bound
`interactionSelect` reports back; when NO domain is bound, `worldPick`'s `id` argument is instead the
plain ARRAY INDEX into `instances_json` (puzzle3d's `🧊️main/🦀️.rs:202-204` comment: *"Hidden objects
stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero
scale... Selection/hover paint is driven by `selectionJson`... never baked here so instance geometry
stays stable across picks."*) — so for a domain-bound window (the modern pattern, recommended) give
every polygon instance a real, stable string id (`format!("surface-{}", surface.id.0)`), not an index.

---

## 2. How mesh bytes reach the browser; size limits; re-render triggers

### 2a. Scene → surface node → carriers

`scene_surface` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:503-514`):
```rust
pub fn scene_surface<T: semio_framework_ui_scene::SceneDoc>(id: &str, kind: SurfaceKind, scene: &T) -> UiAssemblyResult<BuiltNode> {
    let (spine, lanes) = scene.split_lanes();
    let props = semio_framework_ui_scene::encode(kind, &spine).map_err(|error| ui_assembly_error_because("scene-surface.encode", error))?;
    let carriers = lanes.iter().map(|lane| paged_text_carrier(lane.key, &lane.payload)).collect::<UiAssemblyResult<Vec<BuiltNode>>>()?;
    surface(props).try_id(id)?.try_children(carriers)?.try_build()
}
```
`World3dScene::split_lanes` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:773-789`) pulls
**20 named lanes** (`meshes`, `instances`, `instancesDelta`, `selection`, `vortices`, `attractions`,
`targetVolumes`, `references`, `brushPreview`, `interaction`, `engagementPreview`, `lod`, `chunking`,
`environment`, `frame`, `fit`, `terrain`, `points`, `status`, `toolRunTrace` — full list at
`🎬️scenes/🦀️.rs:665-682`) out of the struct into separate carriers rooted at
`framework.scene.world3d.<lane>`; only `camera_json`, the wgpu `snapshot` lease, `domain_id`/
`domain_granularity_id`, and the `lanes` manifest itself stay in the doc's SPINE (bounded, always
present, cheap-per-frame). Each carrier is a `SceneLaneRef { lane, bytes, hash }` — the hash is what
makes a re-published surface node differ (and therefore refresh) exactly when that one lane's content
actually changed; an unchanged lane costs nothing on a partial refresh.

### 2b. Paging / size limits

`paged_text_carrier` (`🔌️plugin/🦀️.rs:467-485`) chunks a lane's payload string via
`section_text_chunks` (`🔌️plugin/🦀️.rs:419-": each text chunk ≤ `UI_TEXT_MAX_BYTES = 512` bytes
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:18`), packed `1 + UI_FIXED_LIST_ITEMS` (=33,
`…/🎬️action/🦀️.rs:21`) chunks per UI node (one as the node's own `Label`, up to 32 as numbered
attributes), then paged so no UI node has more than `UI_BUILT_CHILDREN_MAX = 32` children
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:51`) — recursing into deeper page-of-pages
generations (`p0-0`, `p1-0`, …) if a single level would still overflow. This is what removes the plain
32 KiB `UiFixedBytes` doc ceiling: the doc comment on `scene_surface` cites a real 57 281-byte Nakagin
world that the old bounded-doc path refused outright and this paging admits.

There's a SEPARATE, smaller capacity system for the wgpu snapshot LEASE (`World3dScene.snapshot`,
used by fem3d's live-progress path, `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs:9-13`):
`WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY = 64` items/page, `WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY = 16 KiB`/page,
`WORLD3D_SNAPSHOT_PAGE_CAPACITY = 256` pages/slot, `WORLD3D_SNAPSHOT_CAPACITY = 8` slots — this is for a
*mounted, incrementally-written* mesh (used by long-running solver jobs), not the plain
`meshes_json`/`instances_json` string lanes a static render like the recipe below needs. Don't reach
for it unless you need incremental/paged writes from a background job.

### 2c. Re-render triggers

- **After a document mutation**: the standard `ArtifactEditor::render(body_key, doc, cfg, view_state)`
  dispatch (energy's own `create_energy_model_editor`'s `render` match at
  `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1258-1266`)
  is invoked by the reactor whenever the addressed window is dirty — no special wiring needed for a
  static (non-progressive) 3D window; it's the same path `structure::render`/`zones::render` already
  use.
- **After tool-run progress** (a background job wants to push partial results before the mutation
  commits): the `ArtifactEditor::pending_effects` hook — fem3d's is one line,
  `crate::live_visual::reconcile(doc)` (`✏️editor/🦀️.rs`, `Fem3dPlayApp::pending_effects`) — and the
  render path threads an `Option<AppRenderOperationContext>` (`doc.render_operation()`,
  `🔌️plugin/🦀️.rs:7825`) through to pick up a live visual lease and stamp it onto
  `scene.snapshot` (see 1b above, `render_with_progress`'s `visual.map(...)`). This machinery
  (`🧵️session/🦀️.rs`, 3925 lines) is fem3d's own bounded-job reactor integration — substantial
  overkill for a first cut; skip it for the initial energy 3D window and add it later only if a
  long-running energy simulation needs to stream partial geometry into the same window.

---

## 3. Picking

### 3a. The framework-reserved path (what every modern precedent uses — recommended for 🔋️energy)

Declaring a window's picking is TWO builder calls, no plugin command:
```rust
// once per app, e.g. next to other .mutation()/.action_with() calls:
.interaction(InteractionDefinition {
    id: "energy-model".into(),                     // your domain id, any string
    label: LocalizedLabel::native("Model", "Modell"),
    granularities: vec![GranularityDefinition { id: "surface".into(), label: ..., icon_id: "square".into() }],
    hierarchy: HierarchyProvider::Topology,          // or ::Flat if there's no parent/child structure to walk
    hover: HoverSpec { enabled: true, transitive: false, channels: vec!["pointer".into()], broadcast: true },
    selection: SelectionSpec {
        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
        methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
        transitive: false, broadcast: true,
    },
})
// then per window kind that shares this domain:
.window_kind_interactions(model::WINDOW_KIND_ID, vec![InteractionRef::new("energy-model")])
```
(`InteractionDefinition`/`GranularityDefinition`/`InteractionRef` types:
`🧰️framework/🔨️modules/🕹️interaction/🦀️.rs:43-96`; builder methods `.interaction()`/
`.window_kind_interactions()`: `🔌️plugin/🦀️.rs:5124-5129`, `4991-4998`.)

Then in `render()`, set on the `World3dScene`:
```rust
scene.domain_id = Some("energy-model".into());
scene.domain_granularity_id = Some("surface".into());
```
That's it — **no `canvas-pointer-down` command exists in process3d, generation3d, puzzle3d, or cad**.
The React host (`World3dHost/🟦️.tsx`) does its OWN raycast against the real Three.js geometry
(`raycastGroundPoint`/`onPointerDown` handlers, lines 2866, 6059-6089, 6613 ff.) and — because
`domain_id` is set — dispatches the framework-RESERVED actions directly:
```
dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, [target.id], merge));
dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target.granularity, target.id));
```
(`🟦️.tsx:5685`, `5713`, `6085`, `6811`). `interactionSelect`/`interactionHover` are **framework-reserved
route ids** with their own retained job factories (`FrameworkInteractionSelectJob`/
`FrameworkInteractionHoverJob`, `🔌️plugin/🦀️.rs:15638-15639`) — the plugin never implements a handler;
the framework's `next_selection`/`next_hover` state machine
(`semio-framework-replication`, re-exported via `🕹️interaction/🦀️.rs:24`) merges the pick into the
domain's `InteractionState` and the reconciler republishes `selection_json`/`interaction_json`
automatically. **Correction to the task's framing**: `Effect::ReplayShellCommand` is NOT part of this
path — grepping the whole framework, `ReplayShellCommand` is used only for undo/redo replay
(`🔌️plugin/🦀️.rs:25564,25591,25682,35724`), never for `interactionSelect`.

Without a bound domain (fem3d's situation), picking falls back to the LEGACY, non-domain verbs
`worldPick`/`worldSelect`/`setHover` against the OS's own generic "world" board — fem3d has no handler
for these either (confirmed: no `canvas-pointer-down`/pick command anywhere under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d`), so **fem3d's 3D windows currently support no picking at all**;
its `removeSelection` mutation (`.../🎮️commands/🗂️remove-selection/🦀️.rs`) just consumes whatever id
list some OTHER generic selection surface reports.

### 3b. Hover/highlight colours

Entirely React-side, theme-token-driven, and keyed off per-instance boolean flags — NOT baked into the
mesh's own vertex `colors` array. `resolveMeshStyle` (`🌐️World3dHost/🟦️.tsx:423-438`):
```ts
function resolveMeshStyle(state): MeshStyleKind {  // priority order
  if (state.disabled) return "disabled";
  if (state.provisional) return "provisional";
  if (state.celebrating) return "celebrated";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}
```
mapped through `MESH_STYLE_PAINT` (`🟦️.tsx:377-386`) to live CSS custom properties (`var(--panel)`,
`tokenVar("primary")`, etc.), theme/dark-mode aware via `resolveMeshStylePalette`/`useCanvasAppearanceSync`.
The `selected`/`hovered`/`highlighted`/`disabled` booleans an instance carries come from
`selection_json`/`interaction_json` (parsed client-side, matched by instance id), not from the
`meshes_json`/`instances_json` payload the plugin writes — so a plugin never has to compute hover paint
itself; it only has to publish stable ids.

---

## 4. Per-mesh colouring for results (scalar → colour, legend)

**There is no framework-level colour-map helper.** Every plugin that needs scalar→colour hand-rolls it:

- **fem3d/fem2d** (shared in `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`, used by BOTH `fem2d_ui`
  and `fem3d_ui`): a fixed **8-stop discrete band ramp**,
  `VON_MISES_BANDS: [&str; 8] = ["#1d4ed8","#2563eb","#0ea5e9","#22c55e","#eab308","#f97316","#ef4444","#b91c1c"]`
  (line 17, blue→green→yellow→red), plus:
  ```rust
  pub fn von_mises_color(value: f64, min: f64, max: f64) -> &'static str {
      let span = (max - min).max(1e-9);
      let t = ((value - min) / span).clamp(0.0, 1.0);
      let index = ((t * (VON_MISES_BANDS.len() - 1) as f64).round() as usize).min(VON_MISES_BANDS.len() - 1);
      VON_MISES_BANDS[index]
  }
  pub fn hex_to_rgb01(hex: &str) -> (f64, f64, f64) { /* "#rrggbb" -> (r,g,b) in 0..1 */ }
  ```
  fem3d applies this PER-VERTEX (baked into the mesh's `colors` array, see §1c's `vertex_color`
  closure) — a true per-triangle-corner gradient, not per-instance. fem2d applies the same 8-band table
  to filled 2D triangle/polygon layers plus a small vertical legend swatch stack
  (`von_mises_legend_layers`, `📊️results/🦀️.rs:99-115`) with min/max text labels — this is the ONLY
  "legend" implementation found anywhere in the five precedents.
- **puzzle3d**: no scalar legend at all — colours are catalogue lookups (`vortex_color`/
  `object_kind_color`, resolving a hex string from the example fixture's `kind_catalogs` metadata) or
  fixed swatches per entity class (attractions `#60a5fa`, target volumes `#f472b6`).
- **cad**: colour is pure selection-state, two colours only (`"#3b82f6"` selected vs `"#64748b"` idle) —
  no scalar mapping.
- **process3d/generation3d**: no per-instance colour field set at all in the JSON the render function
  builds (relies entirely on the client-side selection/hover paint of §3b).

**Recommendation for energy**: since `SurfaceClass` (ExteriorWall/InteriorWall/Roof/Ceiling/Floor/
Interzone/Adiabatic/Ground) is a small closed enum, a fixed per-class swatch table (cad/puzzle3d style)
is the natural fit for the "model" window; reserve a fem3d-style continuous `von_mises`-shaped ramp
(copy `hex_to_rgb01` + the band-index formula verbatim — it's plugin-local code, not something to
import) for a later energy-RESULTS window (e.g. per-surface heat-loss/solar-gain colouring), plus a
legend modeled on `von_mises_legend_layers` if that window is 2D, or a small on-screen HUD `Label`
node if it's 3D (fem3d's 3D windows have no legend overlay at all — only fem2d does).

---

## 5. Camera

Framework helper (`ui_wgpu::wgpu::world3d_camera_json`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:4564-4572`):
```rust
pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
    json!({ "position": position, "target": target, "up": [0.0, 0.0, 1.0], "fov": fov }).to_string()
}
```
**Up is always Z** (`[0,0,1]`) — matches energy's own `vertices_m` convention (z is presumably "up" for
a building model too). `world3d_default_camera()` = `world3d_camera_json([4,-4,3],[0,0,0],45.0)`
(`🔌️plugin/🦀️.rs`, `world3d_host::world3d_default_camera`). Per-precedent defaults observed:
| plugin | position | target | fov/zoom |
|---|---|---|---|
| framework default | `[4,-4,3]` | `[0,0,0]` | fov 45 |
| fem3d (`🪟️viewport/🦀️.rs`) | `[24,-20,14]` | `[6,3,3]` | zoom 1.0 |
| process3d config default | `[3,-3,2]` | `[0,0,0]` | fov 45 |
| generation3d preview default | `[4,-4,3]` | `[0,0,0]` | fov 45 |
| cad (`CadCamera::default`) | `[12,-12,8]` | `[0,0,0]` | zoom 1.0, fov 50 |

React-side orbit/pan/zoom is the standard `@react-three/fiber` `OrbitControls`-style camera store
(`useThree`), with the actual pose round-tripped back to the doc via `setCamera`
(`worldCameraSetCameraDispatchArgs`, `🟦️.tsx:765-767`) — units are whatever the plugin's own geometry
uses (fem3d/energy: meters).

**Fit-to-bounds** is the `scene.fit_json` lane, framework helper:
```rust
// 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:3040
pub fn world3d_fit_json(revision: u32, padding: f64, bounds: Option<([f64; 3], [f64; 3])>) -> String
```
Used identically by generation3d/puzzle3d/cad, e.g. cad
(`✏️editor/🎭️modes/✏️edit/🦀️.rs:285`): `scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.document, pane, objects), CAD_FIT_PADDING, None))`
where `world_fit_revision` hashes together the document id + the pane's own object ids, so the camera
is auto-framed exactly ONCE per revision ("frame once per document delivery, never take back a
user-moved camera" — generation3d's doc comment, `preview-eval/🦀️.rs`). Typical padding constants:
`CAD_FIT_PADDING = 1.25`, `PUZZLE3D_FIT_PADDING = 1.25`, `PREVIEW_FIT_PADDING` similar. Client-side math
twin: `world3dFrameDistanceForRadius`/`fitCameraFromBounds` in `🌐️World3dHost/🟦️.tsx:907-945`, Rust twin
`frame_distance_for_radius`/`frame_orbit_to_bounds` in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs:287-307`
(shared margin constant `WORLD_FRAME_BOUNDS_MARGIN = 1.12`/`WORLD3D_FRAME_BOUNDS_MARGIN` on the TS side).
fem3d and process3d do NOT use fit_json at all (fixed manual camera only) — auto-fit is a "modern
pattern" feature, not universal; recommended for energy given building geometry varies wildly in scale
per example.

---

## 6. Concrete minimal recipe for 🔋️energy's 3D "model" window

Target: a `World3d` window rendering every `Surface` in the model as a coloured, pickable polygon.
`Surface { id: EntityId, name, zone_id, class: SurfaceClass, vertices_m: Vec<[f64;3]>, construction_id, ... }`
already lives at `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:299-311`.
Energy's editor crate root mounts modules at
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs:5648-5658` (mirrors fem3d's own crate-root
`#[path]` mounting style) and its manifest builder lives at
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1330-1345`
(`create_energy_model_editor`); the edit-mode layout is
`.../✏️editor/🎭️modes/✏️edit/🦀️.rs` (`layout()`, currently a 3-column row of `structure`/`zones`/
`simulation`).

Ordered steps (mirrors the cad `🔥️energy`/`📐️shape` window pair, the closest of the five precedents):

1. **Create the window directory** `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs` (sibling of
   the existing `🌳️structure`, `📊️zones`, `⚡️simulation` dirs). Give it the sibling facet dirs the
   others have (`🎚️config/` if the window needs persisted per-window state like a camera pose —
   yes, it will — plus empty `👥️presence/`, `🎬️actions/`, `☑️options/`, `🪛️utilities/`, `🫧️transient/`
   marker files if unused, matching `🌳️structure`'s layout exactly).
2. **Constants + `definition()`** — copy cad's `🔥️energy/🦀️.rs` shape:
   ```rust
   pub const WINDOW_KIND_ID: &str = "energy-model-model";   // or similar, must be manifest-unique
   pub const BODY_KEY: &str = "energy.model.model";
   pub fn definition() -> WindowKindDefinition {
       WindowKindDefinition { id: WINDOW_KIND_ID.into(), label: LocalizedLabel::native("Model", "Modell"),
           body_key: BODY_KEY.into(), surface_kind: SurfaceKind::World3d, icon_id: "box".into(),
           options: WindowOptions::default(), actions: Vec::new(), utilities: Vec::new(),
           interactions: vec![InteractionRef::new(ENERGY_MODEL_INTERACTION_DOMAIN)],  // or leave empty and use .window_kind_interactions() in the manifest, like process3d/cad
           params_schema: None, artifact_snapshot_schema: None, input_event_schema: None,
           output_schema: None, capabilities: Vec::new() }
   }
   ```
3. **Add a per-window camera config** if you want persisted orbit state — copy the SHAPE of fem3d's
   `Viewport3dOrbit`/`scene_camera_json`, or simpler: just call `world3d_camera_json([...],[...], 45.0)`
   with a fixed default and rely on `fit_json` to auto-frame (skip persisted camera for v0 — none of
   process3d/generation3d persist a per-window camera either; only fem3d/cad do, via `🎚️config`).
4. **Write `render()`** (mirrors §1b/§1c above):
   ```rust
   pub fn render(model: &crate::model::Model) -> UiAssemblyResult<BuiltNode> {
       let mut meshes = Vec::new();
       let mut instances = Vec::new();
       for surface in &model.surfaces {
           let (positions, normals, indices) = triangulate_polygon_fan(&surface.vertices_m); // fan-triangulate; compute one flat normal
           let (r, g, b) = class_color(surface.class);  // fixed per-SurfaceClass swatch table, cad/puzzle3d style
           let colors: Vec<f64> = positions.iter().enumerate().filter(|(i,_)| i % 3 == 0).flat_map(|_| [r,g,b]).collect();
           let mesh_id = format!("surface-{}", surface.id.0);
           meshes.push(json!({ "id": mesh_id, "data": { "positions": positions, "normals": normals, "colors": colors, "indices": indices } }));
           instances.push(json!({ "id": format!("surface-{}", surface.id.0), "meshId": mesh_id,
               "position": [0.0,0.0,0.0], "rotation": [0.0,0.0,0.0,1.0], "scale": [1.0,1.0,1.0], "label": surface.name }));
       }
       let meshes_json = dsl::json::to_string(&Value::Array(meshes));
       let instances_json = dsl::json::to_string(&Value::Array(instances));
       let mut scene = semio_framework_plugin::world3d_scene(
           semio_framework_plugin::world3d_default_camera(), meshes_json, instances_json,
           semio_framework_plugin::world3d_selection_json("rectangle", &[], None),
           &semio_framework_plugin::WorldSunConfig::default());
       scene.domain_id = Some(ENERGY_MODEL_INTERACTION_DOMAIN.into());
       scene.domain_granularity_id = Some("surface".into());
       scene.fit_json = Some(semio_framework_ui::wgpu::world3d_fit_json(model_fit_revision(model), 1.25, None));
       semio_framework_plugin::scene_surface(BODY_KEY, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
   }
   ```
   (Vertices are already planar polygons in meters — a simple fan triangulation from `vertices_m[0]`
   is enough for convex surfaces; if concave surfaces are possible, reuse whatever polygon
   triangulator the energy simulation engine itself already has for area calculations, if any.)
5. **Declare the interaction domain once**, in `create_energy_model_editor` (next to the other
   `.mutation(...)`/`.action_with(...)` calls, §3a's shape):
   ```rust
   .interaction(InteractionDefinition { id: ENERGY_MODEL_INTERACTION_DOMAIN.into(), label: ...,
       granularities: vec![GranularityDefinition{ id:"surface".into(), label:..., icon_id:"square".into() }],
       hierarchy: HierarchyProvider::Flat,   // energy has no obvious pick-hierarchy above "surface" yet
       hover: HoverSpec{ enabled:true, transitive:false, channels:vec!["pointer".into()], broadcast:true },
       selection: SelectionSpec{ modes:vec![SelectionMode::Multiple,SelectionMode::Single],
           methods:vec![SelectionMethod::Pick,SelectionMethod::Rectangle],
           merges:vec![MergeMode::Replace,MergeMode::Additive,MergeMode::Subtractive,MergeMode::Invertive],
           transitive:false, broadcast:true } })
   .window_kind_def(model::definition())
   .window_kind_interactions(model::WINDOW_KIND_ID, vec![InteractionRef::new(ENERGY_MODEL_INTERACTION_DOMAIN)])
   ```
6. **Mount the module** in the crate-root `#[path]` tree,
   `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`, alongside the existing
   `pub mod simulation;`/`pub mod structure;`/`pub mod zones;` under BOTH the editor's `windows` mod
   (line ~5651) and, if a read-only viewer twin should also show the 3D model, the viewer's `windows`
   mod (line ~5681):
   ```rust
   #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs"]
   pub mod model;
   ```
7. **Add the `render(body_key, ...)` dispatch arm** in the `ArtifactEditor` impl
   (`.../✏️editor/🦀️.rs:1259-1263`): `model::BODY_KEY => model::render(&crate::energy_model(doc.snapshot))?,`
8. **Add the window to the edit-mode layout** — extend `.../✏️editor/🎭️modes/✏️edit/🦀️.rs`'s `layout()`
   from a 3-way row to a 4-way row (or a 2x2 grid) including `model::WINDOW_KIND_ID`, same
   `model_window_stack(...)` helper pattern already there.
9. **Tests to add beside the render function** (precedent: fem3d's model window keeps
   `🧪️tests/🔬️unit/🦀️.rs` right beside `🦀️.rs`, testing (a) that the surface encodes as `"world-3d"`
   and (b) that specific mesh/instance ids appear in the built scene via
   `semio_framework_plugin::artifact_app_laws::built_surface_scene(&node)` to decode a `BuiltNode` back
   into a typed `World3dScene` and assert on `scene.meshes_json`/`scene.instances_json` substrings —
   see `🧱️model/🧪️tests/🔬️unit/🦀️.rs`'s two tests, and puzzle3d's richer
   `🧊️main/🧪️tests/🔬️unit/🦀️.rs` for lane-ordering/selection-state assertions once colour/selection
   logic grows). cad's own `🔥️energy`/`📐️shape` window files have NO adjacent test file (tested only
   via the shared editor-level test file) — flagged above as the weakest precedent; don't copy that
   gap, keep a real `🧪️tests/🔬️unit/🦀️.rs` next to energy's `model::render`.
10. **Skip for v0** (all confirmed non-essential from the precedent survey): fem3d's `🧵️session`
    live-visual/progress-lease machinery (§2c) — only needed once a long energy simulation streams
    partial geometry; a persisted per-window camera config — only fem3d and cad bother, and
    `fit_json` alone gets you a reasonable framed view; a scalar-to-colour legend — only fem2d has
    one, and only for a results window, not a model window.
