# 🖼️ API of the shared render module `crate::render` (r4, owner `u-viewer`, consumers `u-editor`, `u-viewer`)

`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. Module dir `S/🖌️render/`, mounted at the artifact root `🦀️.rs` as
`pub mod render` (next to `editor` and `viewer`), Rust path `crate::render`. It is the ONE place the editor and the viewer share rendering: the viewer must not
import `crate::editor`, so everything both draw lives here. Files: `🦀️.rs` (queries + inference memo), `🧊️world/🦀️.rs`, `🗺️plan/🦀️.rs`, `🪟️window-config/🦀️.rs`
(the three submodules are declared with `#[path]` inside `🦀️.rs`, nothing else to mount). Tests: `🧪️tests/🔬️unit`, `🧊️world/🧪️tests`, `🗺️plan/🧪️tests`.

## 1. Queries and the inference memo (`crate::render::*`)

```rust
pub struct StoreyRow { pub id: String, pub building: String, pub name: String, pub level: i32 }
pub fn storeys(&ModelSnapshot) -> Vec<StoreyRow>;                 // by (building, level, id): pickers, plan selection, visibility toggles
pub fn plan_storey(&ModelSnapshot, stored: &str) -> Option<String>; // the stored storey while it exists, else the lowest storey
pub fn element_ids(&ModelSnapshot) -> Vec<String>;                // walls, curtain walls, columns, beams, slabs, roofs, openings, stairs, railings (no spaces)
pub fn storey_of<'a>(&'a ModelSnapshot, element: &str) -> Option<&'a str>; // an opening stands on its host's storey
pub fn element_name<'a>(&'a ModelSnapshot, element: &str) -> &'a str;
pub fn solids(&ModelSnapshot) -> Rc<BTreeMap<String, ElementSolid>>;   // element-solids, memoised for the LAST snapshot (single entry, compared by equality)
pub fn plans(&ModelSnapshot) -> Rc<BTreeMap<String, PlanLinework>>;    // plan-linework per storey id, same memo
```
The memo is a thread-local single entry: re-rendering for a camera/selection change never re-infers; a document change replaces it. It computes `compute_element_solids` / `compute_plan_linework`
directly (not the whole `ModelInference`), so a window pays only for the field it draws.

## 2. World3d (`crate::render::world`)

```rust
pub const ELEMENT_DOMAIN: &str = "elements";      // interaction domain of drawn elements (declare it in the app manifest with `.interaction(..)` + `.window_kind_interactions(window, [InteractionRef::new(ELEMENT_DOMAIN)])`)
pub const ELEMENT_GRANULARITY: &str = "element";  // one granularity; target id = ELEMENT ID
pub struct WorldView<'a> { pub orbit: Option<&'a store::Viewport3dOrbit>, pub projection: &'a WorldProjectionConfig, pub hidden_storeys: &'a [String], pub selected: &'a [String], pub hovered: &'a [String] }
pub fn scene(&ModelSnapshot, &BTreeMap<String, ElementSolid>, &WorldView) -> World3dScene;
```
`scene` writes: one inline mesh `{id: <element id>, data: MeshData}` per drawn solid (vertex colours per group from `snapshot.materials[group.material].color`, glass `[.62,.78,.9,.35]`, else a family colour);
one instance per element (`id` = `meshId` = `interactionId` = element id, `interactionGranularityId` = `element`, `position` = solid placement x,y,z, `rotation` = quaternion about +Z of the placement rotation, `label` = element name,
`selected`/`hovered` from the view); `selection_json` with the marks; `scene.domain_id = elements`, `scene.domain_granularity_id = element`; camera = `world3d_camera_projection_json(orbit, projection)`;
`fit_json = {enabled: orbit.is_none(), revision: 0, padding: 1.25}` (a one-shot fit to the model while the window has no stored orbit). With `orbit == None` the camera is `overview_orbit(world_bounds(drawn solids))`
(south-east above the centre). Solids on `hidden_storeys` (by `ElementSolid.storey`) and empty solids are left out. Other exports: `visible`, `mesh_data`, `group_color`, `family_color`, `world_bounds`, `overview_orbit`.
The editor adds its own overlays (gumball, section box, previews) AFTER calling `scene` by setting more fields of the returned `World3dScene`.

## 3. Canvas2d plan (`crate::render::plan`)

```rust
pub fn scene(plan: Option<&PlanLinework>, viewport: &store::Viewport2d, framed: bool, selected: &[String]) -> Canvas2dScene;
pub fn layers(&PlanLinework, selected: &[String]) -> Vec<DslValue>;     // layer records in paint order
pub fn canvas_bounds(&PlanLinework) -> Option<[f64; 4]>;
pub fn canvas_point(x, y) -> [f64; 2];  pub fn plan_point(x, y) -> [f64; 2];   // the ONLY coordinate conversions
pub fn framing_revision(storey: &str) -> u32;
```
Conventions: canvas coordinates are plan coordinates in METRES with the y axis mirrored, `canvas = (x, -y)` (north up on screen). The editor must convert pointer positions with `plan_point(world_x, world_y)`
(`canvas_point_to_world` first, then mirror) before building mutations. Stroke widths are metres (the host scales strokes with the camera): cut 25 mm, projection 12 mm, hidden 10 mm dashed, annotation 8 mm.
Bulged edges are flattened within 0.5 mm. Paint order: regions (poche) by style `Hidden < Projection < Cut < Annotation`, then polylines by the same order, then texts. Layer ids are the `PlanLinework` primitive ids
(`<element>/<Kind>/<n>`), so a record id starts with `<element id>/`. An element in `selected` is drawn in the accent colour `[0.95, 0.5, 0.1, 1.0]`. While `framed == false` the scene carries a `Canvas2dFraming` to the plan bounds
(revision = hash of the storey id, padding 48 px) so the host fits once per storey; the first pan/zoom gesture must set `framed = true` in the window config (the host reports it through `setCamera`).
Note the host clamps a fit to `zoom <= 32` px per metre.

## 4. Window configuration macro (`crate::bim_window_config!`)

```rust
#[derive(DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, DslArtifact)]
#[value(rename_all = "camelCase")] #[dsl(layout = "lines")] #[artifact(id = "s.bim.model.<name>.config", extension = "bim<name>cfg")]
pub struct MyWindowConfig { #[dsl(block)] pub viewport: store::Viewport2d, pub framed: bool, .. }   // + `impl Default`
// sparse form (preferred): the configuration travels as a diff with one optional per field
crate::bim_window_config! { config: MyWindowConfig, diff: MyWindowConfigDiff, mutation: MyWindowConfigMutation, owner: MyWindowConfigOwner, window: super::WINDOW_KIND_ID,
    schema: "bim.model.<name>.config", owner_path: "<taxonomy path>", display: "Set ...", bytes: 4_096, fields: { viewport: store::Viewport2d, framed: bool } }
// whole-record form (no `diff:`/`fields:`): `impl_whole_record_config!`, the diff is the configuration itself (what `✏️editor/🧰️kit` uses today)
crate::bim_window_config! { config: .., mutation: .., owner: .., window: .., schema: .., owner_path: .., display: .., bytes: .. }
```
Both forms expand (in the invoking module) to the `ArtifactDsl`/`ArtifactPack` codecs, `enum MyWindowConfigMutation { Snapshot { config } }` with its concrete inverse, the text/binary op codecs, the `WindowConfigOwner`
and `pub fn current(&ConfigView) -> Config`, `from_snapshot(Option<&WindowConfigSnapshot>) -> Config` (for retained tool jobs, `context.window_config`), `addressed(&ViewModel, Config) -> Result<WindowConfigMutation, Fault>`.
The shared half is `crate::bim_window_config_common!`. Register the owner in `register_window_config_owners`. `crate::render::window_config::assert_window_config_laws(&base, &mutation).await` (`#[cfg(test)]`) proves apply,
inverse, op/dsl/pack round trips, the inverse sum law and, for the sparse form, the `between` law (a whole-record diff can never be empty, so `between(a, a)` is not empty by design: the law is skipped when the diff type is the
configuration type). The editor's whole-record tests `the_inverse_sums_to_the_negative_diff_and_between_is_the_state_delta` fail on exactly that law; the sparse form passes it.
Viewer examples: `👁️viewer/🎭️modes/👁️view/🪟️windows/{🧊️world,🗺️plan}/🎚️config/🦀️.rs` (sparse).
Projection preset bank: store `semio_framework_plugin::WorldProjectionConfig` as a `#[dsl(block)]` field; `BimViewerWorldWindowConfig::with_projection_action` shows how `setProjection`/`setProjectionParam` re-pose the orbit.

## 5. Adoption notes for the editor

- Do not re-implement scene building: call `render::world::scene` / `render::plan::scene`, then add editor overlays. Use `ELEMENT_DOMAIN`/`ELEMENT_GRANULARITY` for the interaction definition (a richer hierarchy may add granularities but the target ids stay element ids).
- Declare `interaction_topology` with `render::element_ids(snapshot)` (the viewer does exactly that), otherwise the framework prunes every world pick.
- `render::solids` / `render::plans` are the only inference reads a window needs; they are memoised for the last snapshot, so call them freely from `render` and `window_measures`.
