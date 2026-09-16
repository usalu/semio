# W2-A — finishing the 3d model window: zone volumes, ramp legend, hover, viewer camera

Lane W2-A of ticket `26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR`. Session: Opus 5 (1M context). Paths are
repo-relative to `/Users/ueli/Documents/semio`.

All five items were implemented. Every gate in §7 was RUN in the foreground and its real output is
quoted. Nothing in this report is "written, not run". What was NOT done is in §8.

---

## 1. Zones as volumes ✅

`Zone` carries a scalar `volume_m3` and no shape at all, so the drawable volume had to be derived.

**What landed** (`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🎬️scene/🦀️.rs`, new
`//#region 🔖️Hull` plus one loop in `energy_model_scene_parts`):

- `zone_hull_points(model, zone) -> Vec<[f64;3]>` — every corner of every `Surface` whose
  `zone_id == zone.id`, in model order. Empty for a zone with no member surface, and empty (i.e. no
  volume) above `ENERGY_SCENE_ZONE_HULL_MAXIMUM_POINTS = 4_096` corners.
- `convex_hull_triangles(points) -> Option<Vec<[[f64;3];3]>>` — a standard incremental 3d hull: seed a
  non-degenerate tetrahedron (farthest point → farthest from that line → farthest from that plane),
  then for each remaining point delete the faces it can see, walk the horizon of the hole (directed
  edges that are not cancelled by their reverse) and cone it back to the point. Coning preserves the
  removed faces' winding, so every face is outward with no second orientation pass. Every tolerance is
  RELATIVE to the cloud's own bounding extent (`scale * 1e-9`), so a room in millimetres and a district
  in kilometres are judged degenerate by the same rule.
- One translucent instance per zone that encloses a volume: `id = energy_scene_target_id(zone.id)`
  (= `interaction::energy_target_id`), `label` = zone name, `objectKind = "zone"` (the granularity id),
  `meshId = energy-zone-<id>`, colour `ENERGY_SCENE_ZONE_COLOR` (a soft violet no envelope class uses).

**Why `geometry::zone_volume_from_surfaces` was NOT reused.** It answers a SCALAR — a sum of signed
face-pyramid volumes about an interior reference point. There is no surface in it to render. The hull
is the cheapest honest shape whose boundary encloses exactly the corners that integral is taken over,
and for the rectangular zones every authored example uses the hull IS the zone box. The law
`the_box_hull_encloses_exactly_the_zone_volume_the_engine_integrates` asserts that: the
divergence-theorem volume of the DRAWN shell and `zone_volume_from_surfaces` over the same authored
faces agree to `1e-9` (both exactly 30 m³ for a 4 × 3 × 2.5 m zone).

### Translucency and picking — the honest answer to "alpha ≈ 0.15"

**There is no numeric alpha lane.** `PaintTexturedMesh`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:2126-2143`)
sets `transparent={style.opacity < 1} opacity={style.opacity}`, and `style` comes from the fixed
`MESH_STYLE_PAINT` table keyed by style KIND, never from the payload. Lane B's §3 note ("there is also
no alpha lane") is right about a number, but there IS exactly one payload lane that reaches it:

`WorldInstanceRecord.disabled` (`🟦️.tsx:184`). It is the only style kind with `opacity < 1`
(`disabled: … opacity: 0.45`, `🟦️.tsx:387`), and it ALSO does the second half of the brief's
requirement: `instancePickEnabled = pickEnabled && !instance.disabled && !provisional` (`:2831`) and
`raycast={worldInstanceMeshRaycast(instancePickEnabled)}` (`:1906-1908`) makes the mesh answer
`() => null` to every raycast. So a zone hull is **structurally invisible to picking** — it cannot
occlude a wall inside it even at the cursor's exact pixel — and it blends at 0.45.

**0.45, not 0.15.** That is the whole of what a payload can ask for today; reaching 0.15 needs a host
change (a numeric `opacity` on `WorldInstanceRecord` threaded into `MeshStyleColors`). Flagged in §8.
Note also that `hasVertexColors` forces the material colour white and the emissive black, so the baked
zone swatch still shows through at that opacity rather than being replaced by the disabled grey.

Zones are appended LAST in `meshes_json`/`instances_json`: three.js draws transparent materials after
opaque ones and sorts them back-to-front, so the walls are already in the colour buffer when the hull
blends over them — and appending keeps every opaque family at the index it had before volumes existed,
so no pre-existing index-based test moved.

**Empty and flat zones draw nothing.** No member surface → no instance. Only coplanar corners (a zone
with just a floor) → `convex_hull_triangles` returns `None` rather than emitting a zero-thickness
shell. Non-finite corners are refused, never rendered.

`energy_model_fit_revision` now also hashes each zone id and each surface's `zone_id`, because zone
membership is geometry now: it decides which corners a zone's drawn volume hulls.

## 2. Legend strip ✅

**The UI contract has no colour swatch.** `Component` is a closed 19-variant enum with no
badge/chip/swatch/rect; `StyleSpec` is "closed enums over ui_styling tokens, never raw values" with no
node-level `background`; `[data-tone]` is styled by no stylesheet in the repo; and
`TextProps::data_attributes` is a payload-PACKING device that the react `TextView` never renders as DOM
attributes — so the brief's fallback ("eight text cells with the hex in a data attribute") would have
rendered nothing visible. fem2d's own 8-swatch legend is drawn as Canvas2d path layers
(`🏗️fem/…/🪟️windows/📊️results/🦀️.rs::von_mises_legend_layers`, two filled triangles per band), i.e.
scene geometry, which does not port to a World3d scene.

**What landed** — `ImageProps::src` is the one lane that carries a plugin-chosen colour into a rendered
rectangle in BOTH targets (react's `ImageView` passes `src` through verbatim; the wgpu target maps it
to `UiImageNode`). In `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs`:

- `pub fn legend_swatch_src(hex) -> String` — an inline SVG data URI, ~160 bytes, well inside
  `UI_TEXT_MAX_BYTES = 512`. `<`, `>` and `#` are percent-encoded, so the URI is valid without relying
  on lenient parsing and without a base64 dependency.
- `legend_strip(minimum, maximum)` — a `row` of `[min label, 8 decorative swatch images, max label]`,
  ten siblings with ten explicit ids (`LEGEND_MINIMUM_NODE_ID`, `LEGEND_SWATCH_NODE_ID_PREFIX{0..7}`,
  `LEGEND_MAXIMUM_NODE_ID`) and one accessible label on the row itself, so a screen reader hears one
  legend rather than eight unnamed rectangles.
- `with_caption(scene, caption, bounds)` — the caption line and the strip now live in a `column` with
  id `LEGEND_NODE_ID`, which takes the first root-child slot the bare caption used to take. The root
  is still exactly two children (`LEGEND_NODE_ID`, `CAPTION_SCENE_NODE_ID`), so lane B's
  `DuplicateSiblingKey` guard and its `surface_node` helper still hold unchanged.
- `pub fn render_with_legend(model, interaction, overlay, caption, bounds, camera)`. `render` and
  `render_with_camera` keep their exact signatures and delegate with `bounds: None`.

**The shared ramp.** Appended to lane D's `✏️editor/📊️results/🦀️.rs` (append-only, nothing edited):

```rust
pub fn legend_bands() -> [&'static str; 8]                       // == SURFACE_ENERGY_BANDS
pub fn legend_bounds_labels(min: f64, max: f64) -> (String, String)  // ("0.0", "412.3 kWh")
```

The strip's swatches come from `legend_bands()` and the meshes' colours come from `band_color`, which
indexes the same array; the window law walks all eight bands asserting
`band_color(value_at_band_i) == legend_bands()[i]`, so the strip can never describe a ramp the scene is
not using, and `legend_bounds_labels` formats the two ends exactly as `legend_caption` does so the
caption line and the strip never round differently.

**Hidden without an overlay.** No overlay ⇒ the coordinator passes `caption: None` ⇒ the render is the
bare `Component::Surface`, no legend column at all. A caption with no bounds is the caption line alone
(the old shape). Asserted by `without_an_overlay_there_is_no_legend_at_all`.

**One call site changed** (`✏️editor/🦀️.rs`, `model_window::BODY_KEY` arm, ~line 2304):
`render_with_camera(…)` → `render_with_legend(…, painted.as_ref().map(|(_, min, max)| (*min, *max)), …)`.
That is the whole edit in lane W2-B's live file — re-read immediately before editing, nothing else
touched.

## 3. Hover highlight ✅

**The host really does deliver hover.** `HoverSpec::default()`
(`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1935-1940`) is
`{ enabled: true, transitive: false, channels: ["pointer"], broadcast: true }`, i.e. enabled on exactly
the `ENERGY_POINTER_CHANNEL` that `EnergyModelInteractionSnapshot::from_interaction` reads back. It was
already correct — but it was correct *by default*, so `✏️editor/🕹️interaction/🦀️.rs` now states the four
fields explicitly with the reason: a silently changed framework default would make every hover arrive
empty with nothing to point at. (`transitive` must stay `false` while `hierarchy` is `Flat`.)

**The tint.** Hover and selection are baked in `crate::scene` because a vertex-coloured mesh ignores the
host's own selected/hovered paint entirely — so the bake is the *entire* hover feedback, not a
supplement to it. `ENERGY_SCENE_HOVERED_MIX` was raised 0.25 → 0.35 for legibility. The new law
`hover_lightens_every_channel_and_selection_never_does` asserts the real distinction rather than mere
inequality: hover moves EVERY channel strictly UP (towards white), selection moves red down and blue up
(towards the primary blue) — two directions, never confusable — and it checks the paint actually
reaches the payload for a surface, a fenestration AND a zone volume, with the per-instance `hovered`
flag still published so the host chrome stays right.
`a_selected_and_hovered_entity_reads_as_selected` pins the precedence so a picked entity never flickers
under the cursor.

## 4. Viewer `setCamera` ✅

The viewer's 3d window dropped every orbit the way the editor's used to. This is now fixed, and it is
the first viewer in the repo to persist a window config (`grep -rln WindowConfigOwner ✏️s/🔌️plugins/`
→ 121 files, none under `👁️viewer`).

**Why it could NOT go through `handle`, as the brief assumed.** `ViewEmit`
(`🔌️plugin/🦀️.rs:31770`) has three fields — `config_mutations`, `effects`, `ui_dirty` — and
`ViewerApp::handle` maps them into `Emit { …, ..Default::default() }` (`:32289`), so a
`window_config_mutations` written in `ArtifactViewer::handle` would be discarded by the adapter. The
verb travels the RETAINED route instead, whose reducer returns a full `Emit`. Precedent:
generation3d's viewer already runs `bounded_first_step_tool_proofs!` with `owner:
ViewerApp<Generation3dViewer>` (its own `setCamera` publishes onto its app `Config` lane).
`ArtifactViewer::handle` now returns a loud refusal (`energy.model.3d.viewer.retained-route-required`)
rather than silently dropping a camera that reached the wrong route.

**What landed:**

| Piece | Where |
|---|---|
| `EnergyModelViewerCameraPose` / `EnergyModelViewerWindowConfig` / `…Mutation` / `SetCamera` leaf / `…Owner` / `current` / `addressed`, + the 4 spec surfaces per level (15 files) | `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/…`, dsl id `energy.model3dviewerwindowconfig` |
| `SET_CAMERA_ACTION_ID`, `set_camera_action()` (`ActionKind::View`, `Migrated`, declared `camera` arg), `definition().actions`, `render_with_camera` | `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` |
| `EnergyModelViewCommand::SetCamera { camera }` (was an inert `Noop`), `dsl::DslOps` + hand-written `OpBinary` with `TOOL_JOB_IDS`, `action_id`, `command_id`, `command_from_action`, `camera_emit`, `energy_model_view_reduce`, `EnergyModelViewCommandJobFactory`, `bounded_first_step_tool_proofs!`, `build_tool_job`, `register_tool_job_factories`, `register_window_config_owners`, `render` passing `config::current(cfg)` | `👁️viewer/🦀️.rs` |

An independently authored twin, not an import: a viewer may never reach through `✏️editor`
(`policyViewerPurityBreaches`), so the record, the verb and the owner are spelled again under their own
dsl id and their own store.

**The viewer emits no document mutation, by construction and by law.** The single
`PUBLICATION_CONTRACTS` row is `{ setCamera, [WindowConfig] }` — never `Artifact`, never the app
`Config` — and `a_camera_gesture_becomes_an_addressed_window_config_write_and_nothing_else` asserts the
built `Emit` has one `window_config_mutation` and empty `artifact_mutations`, `config_mutations` and
`effects`. `a_camera_gesture_without_a_concrete_window_is_refused_rather_than_written_anywhere` covers
the four refusals (no window, wrong window kind, empty pose, non-positive zoom).

⚠️ Note for whoever debugs this in the browser: the command carries the **orbit** json the args bridge
canonicalizes (`{position,target,zoom}`), NOT the **scene camera** json the render publishes
(`{position,target,up,fov}`). Only the first has a `zoom`. I wrote the first version of that law with
`scene_camera_json()` and it failed with `missing field zoom`.

## 5. Fenestration polygons ✅

**Confirmed, and now pinned by tests.** `crate::scene::model_fenestration_polygon` →
`precompute::model_fenestration_polygon` → `precompute::place_aperture`, whose first statement is
`if !own_vertices.is_empty() { return own_vertices.to_vec(); }`. An authored `Fenestration.vertices_m`
is returned verbatim; only the 5 mm outward lift is applied on top.

`an_authored_five_vertex_window_renders_as_authored` authors a gabled FIVE-corner window on the south
wall of a host quad, with `area_m2`/`height_m`/`sill_height_m` all zero so a rendered polygon can only
have come from `vertices_m`, and asserts: the rendered polygon has **5 vertices**; each rendered corner
is its authored corner displaced by exactly `ENERGY_SCENE_WINDOW_OFFSET_M` along the host normal AND by
nothing else (the displacement's full length equals the lift); and the published mesh carries **9
indices = 3 triangles** with 27 position floats, with the window as one pickable instance.
`an_authored_polygon_wins_over_the_area_height_sill_rectangle` pins the fallback: the same scalars
alone still derive a 4-vertex bay rectangle, and adding `vertices_m` replaces it.

---

## 6. Files

### New

| Path | What |
|---|---|
| `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/**` (15 files) | The viewer's own window config: schema + `set-camera` leaf + mutations enum + owner, each with its `🔣️.json`/`🔗️.graphql`/`🛰️.proto`/`🟦️.ts` surface, plus 6 unit tests |

### Edited

| Path | Edit |
|---|---|
| `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🎬️scene/🦀️.rs` | zone swatch/prefix/objectKind/point bound consts; `sub3`/`cross3`; `triangles_mesh`; `zone_instance`; the whole `🔖️Hull` region; the zone loop in `energy_model_scene_parts`; zone membership in `energy_model_fit_revision`; `ENERGY_SCENE_HOVERED_MIX` 0.25 → 0.35; two docstring corrections (the `disabled` alpha lane, the zone family) |
| `…/⚙️engine/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` | 3 fixtures (`authored_window`, `box_zone_surfaces`, `zone`); the bestest-600 instance count now includes zones; **9 new tests** |
| `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs` | 5 legend node-id consts; `legend_swatch_src`; `legend_strip`; `with_caption` grew a `bounds` argument and the legend column; `render_with_legend` (`render`/`render_with_camera` signatures unchanged) |
| `…/🪟️windows/🧊️model/🧪️tests/🔬️unit/🦀️.rs` | the root-child-id assertion now names `LEGEND_NODE_ID` and additionally pins the caption-only shape; 2 walk helpers; **3 new tests** |
| `…/✏️editor/📊️results/🦀️.rs` | **append only** — `legend_bands()`, `legend_bounds_labels()` |
| `…/✏️editor/🕹️interaction/🦀️.rs` | `HoverSpec` stated field by field with its reason |
| `…/✏️editor/🦀️.rs` | ONE call site: `render_with_camera` → `render_with_legend` with the ramp bounds |
| `…/👁️viewer/🦀️.rs` | imports; the whole command/camera/retained-command region; 6 trait members |
| `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` | `config` mount, `SET_CAMERA_ACTION_ID`, `set_camera_action()`, `definition().actions`, `render_with_camera` |
| `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🧪️tests/🔬️unit/🦀️.rs` | the definition law now pins the camera verb; **1 new test** |
| `…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | **5 new tests** (`🎥️ViewerCamera` region) |

No file was reformatted or rewritten except the viewer's 3d window module (a 57-line file, rewritten to
add the camera; its three existing tests all still pass unchanged). No peer change was reverted, and
nothing under `🗑️generated` was touched.

## 7. Gates — commands run, with their real output

```
cd /Users/ueli/Documents/semio && cargo check -p semio-s-artifact-energy-model --lib --tests
  → clean. Zero errors. Zero warnings from any file this lane wrote.

cargo test -p semio-s-artifact-energy-model --lib -- scene windows::model viewer interaction results
  → test result: ok. 83 passed; 0 failed; 0 ignored; 6212 filtered out; finished in 0.06s

cargo test -p semio-s-artifact-energy-model --lib
  → test result: FAILED. 6290 passed; 3 failed; 2 ignored; finished in 15.50s
    failures: sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one
              sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total
              sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology
    == exactly the three declared pre-existing reds. No new reds.

cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2
  → Checking semio-s-plugin-energy … Finished `dev` profile in 56.66s. Zero errors.
```

**28 new tests** (9 scene, 3 editor window, 1 viewer window, 5 viewer root, plus the 6 that came with
the cloned viewer window-config leaf and 4 pre-existing ones widened).

**One flake worth recording.** The FIRST full-suite run also failed
`editor::model::component::tests::a_finalized_simulation_run_recolours_the_three_d_model_window` (lane
W1-D's new app-level recolour law). It passes in isolation and did not recur on the second or third
full run — the same in-process ordering effect lane D already documented in its §6 (the faulting p7c2
test leaves the shared Energy abandonment registry dirty for a later test in the same binary). It is
NOT a legend regression: my legend change is on that exact render path and the law is green on every
run but one.

**Cross-lane note.** For ~40 minutes the crate's `--tests` build was red on lane W2-B's in-flight
`📌️panels/🗿️artifact` + `📌️panels/🔍️inspection` signature change and their not-yet-written
`🗻️change-material-roughness` fixtures. I did not touch any of it; `cargo check --lib` stayed green
throughout and I used it to validate my own half until W2-B landed.

## 8. Still owed / known limits

1. **A zone volume is 45 % opaque, not 15 %.** `MESH_STYLE_PAINT.disabled.opacity = 0.45` is a fixed
   host constant and the only sub-1 opacity a payload can reach. A real per-instance alpha needs a host
   change: a numeric `opacity?: number` on `WorldInstanceRecord`, resolved into `MeshStyleColors` in
   `WorldMeshInstance` and passed to `PaintTexturedMesh`. Roughly a ten-line edit in
   `🌐️World3dHost/🟦️.tsx`, owned by whoever owns the framework renderer.
2. **A zone whose surfaces are concave or non-convex draws its CONVEX hull.** An L-shaped room renders
   as the filled L-plus-notch. For the authored examples (all rectangular) the hull is exact — the
   volume law proves it — but a real non-convex zone would over-draw. The fix is not a better hull; it
   is to draw the zone's own member surfaces as one shell, which needs a closed-manifold guarantee the
   model does not currently carry.
3. **A marquee (rectangle) selection can still pick up a zone volume.** `disabled` kills the RAYCAST,
   which is what a click and a hover use, but `handleInstancePointerDown`'s `record?.disabled` branch
   and the screen-space marquee are separate paths; I verified the click path clears rather than
   mis-selects, and did not chase the marquee.
4. **The legend strip is eight `Image` nodes.** They are decorative and the row carries the accessible
   name, but the ramp itself conveys meaning only through colour — a colour-blind reader gets the two
   numeric bounds and nothing between them. A `title`/tooltip per band would need a swatch component
   the UI contract does not have (§2).
5. **Nothing in this lane was seen in a browser.** Per the lane brief I ran no wasm activation and no
   dev server. Unproven in the running app: the zone hull actually blending over the walls, the eight
   SVG data-URI swatches rendering (react `ImageView` passes `src` through verbatim, but an inline SVG
   data URI has no precedent in this repo — `📋️forms` uses base64 PNG), the hover tint at 0.35, and
   the viewer's `setCamera` surviving a re-render. **Lane E should probe at least the swatches**: if
   `ImageView` refuses the URI the strip degrades to two numbers and a gap, which no unit test can see.
6. **The viewer is still not on the restage path**, so its camera has never round-tripped through a
   real dispatch. The unit laws cover the reducer, the refusals, the codec and the render; what they
   cannot cover is `validate_tool_job_rows` accepting the new `bounded_first_step_tool_proofs!` rows
   at runtime registration.
7. **Fenestration/shading/zone picks all still report the `surface` granularity** (lane B's §7 gap is
   unchanged): `World3dScene` has one `domain_granularity_id` for the whole scene. The ids are right
   and `energy_entity_kind` recovers the real family, so a zone pick from the 3d window arrives as
   `(energyModel, "surface", "<zoneId>")` and the tree/inspector resolve it correctly — but a consumer
   that switches on the REPORTED granularity must not trust it.
