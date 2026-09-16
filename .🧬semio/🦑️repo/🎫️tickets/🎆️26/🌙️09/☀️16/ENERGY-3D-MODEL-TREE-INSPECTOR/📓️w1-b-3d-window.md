# W1-B — the World3d "Model" window for the energy editor (and its viewer twin)

Lane W1-B of ticket `26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR`. Session: Opus 5 (1M context).
Everything below is repo-relative to `/Users/ueli/Documents/semio`.

## 0. What now exists

A `SurfaceKind::World3d` window kind `energy.model.3d` on BOTH energy-model surfaces, rendering
every `Surface`, every `Fenestration` and every `ShadingSurface` of the working `crate::model::Model`
as one mesh + one pickable instance, bound (on the editor only) to the shared `energyModel`
interaction domain so a 3d pick and a tree pick are the same event.

## 1. Files

### New

| Path | What |
|---|---|
| `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🎬️scene/🦀️.rs` | The ONE pure `Model -> World3d payload` builder, shared by both windows (≈300 lines) |
| `…/⚙️engine/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` | 8 unit tests over the builder |
| `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs` | Editor window: constants, `definition()`, `render()`, local `with_caption` |
| `…/✏️editor/…/🪟️windows/🧊️model/🧪️tests/🔬️unit/🦀️.rs` | 8 window tests |
| `…/✏️editor/…/🪟️windows/🧊️model/🟦️.ts` | TS twin of the payload shapes |
| `…/✏️editor/…/🪟️windows/🧊️model/{🎬️actions,🪛️utilities,☑️options,🎚️config,👥️presence,🫧️transient}/📌️.empty.md` | The six `windowRequiredChildDirs` markers, mirroring `🌳️structure` |
| `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs, 6 facet markers}` | The read-only twin |

### Edited (small, targeted)

| Path | Edit |
|---|---|
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs` | 3 mounts: `pub mod scene;` at the crate root (next to `schedule`), `pub mod model;` under the editor's `windows`, `pub mod model;` under the viewer's `windows` |
| `…/✳️any/✏️editor/🦀️.rs` | import line; `.window_kind_def(model_window::definition())`; `.window_kind_interactions(...)`; render split into `render` + `render_with_request_context` + a shared free `render_body` carrying the new `model_window::BODY_KEY` arm |
| `…/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs` | layout: 3d viewport left at 0.55, structure/zones/simulation stacked in a 0.45 column |
| `…/✳️any/👁️viewer/🦀️.rs` | import line; `.window_kind_def(model_window::definition())`; render arm |
| `…/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs` | same layout shape as the editor's |

Nothing else was touched. No file was reformatted or rewritten; no peer change was reverted.

## 2. Two deliberate deviations from the brief — please read

### 2a. The shared builder is NOT under `🧬️schema/💡️inferences/🧊️scene`

The brief suggested `🧬️schema/💡️inferences/🧊️scene/🦀️.rs` mounted as `crate::scene`. I put the same
module at `🔨️modules/⚡️simulation/⚙️engine/🎬️scene/🦀️.rs`, still mounted at the crate root as
`crate::scene`, for two hard reasons:

1. **Taxonomy.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`'s
   `semanticDirectoryMemberKinds` registry enumerates the legal member names per owner kind.
   `members-of-inferences` does NOT contain `🧊️scene` (it lists `🗃️entries`, `📐️shape`, `🪟️window`, …),
   so a new dir there is an unregistered member. `members-of-engine` DOES already contain `🎬️scene`
   (and `🖥️app-surface`, fem's own UI helper) — so the location I chose is registry-clean today.
   `members-of-windows` already contains `🧊️model`, so both window dirs are clean as well.
2. **Contract.** `💡️inferences/🦀️.rs`'s own docstring says each `<emoji><slug>/` child is a named
   inference and must satisfy `protocol::Inference<Snapshot>`, with a matching field on
   `EnergyModelInference` and a matching entry in `🔣️.json`/`🛰️.proto`/`🔗️.graphql`/`🟦️.ts`. The
   plugin crate root's own docstring records that the 50 engine domains were moved OUT of inferences
   for exactly that reason. A scene builder is pure, but it is not a snapshot inference, and wiring
   it as one would have touched four shared schema spec files other lanes are editing.

`crate::scene` is the import path the brief asked for, so no call site differs.

### 2b. `ArtifactEditor::render` has no `interaction` parameter

The brief said "the render fn receives `_interaction: &InteractionView` today; rename it". It does
not — `ArtifactEditor::render(body_key, doc, cfg, view_state)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31093`) takes four arguments and no
interaction. The live domain reaches a render through the OPTIONAL
`ArtifactEditor::render_with_request_context(owner, body_key, doc, cfg, view_state, transient,
interaction)` (`…/🔌️plugin/🦀️.rs:31104`), whose default impl discards `interaction` — this is exactly
how puzzle3d does it (`🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:8215-8233`).

So I **added that override** to `EnergyModelEditor` and factored the body-key match into a free
`fn render_body(body_key, doc, cfg, view_state, interaction: &EnergyModelInteractionSnapshot)`.
`render` passes `EnergyModelInteractionSnapshot::default()`; `render_with_request_context` passes
`EnergyModelInteractionSnapshot::from_interaction(interaction)`.

**Lane C / lane D:** if you need live selection in the inspector or the panels, take it from
`render_body`'s `interaction` argument — do not add a second `render_with_request_context`.

## 3. The contract the window publishes

```rust
// ✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs
pub const WINDOW_KIND_ID: &str = "energy.model.3d";
pub const BODY_KEY: &str = "energy.model.3d";
pub fn definition() -> WindowKindDefinition;  // World3d, "Model"/"Modell", icon "box"
pub fn render(
    model: &crate::model::Model,
    interaction: &crate::editor::model::interaction::EnergyModelInteractionSnapshot,
    overlay: Option<&std::collections::HashMap<u32, [f64; 3]>>,
    caption: Option<&str>,
) -> UiAssemblyResult<BuiltNode>;
```

- `scene.domain_id = Some("energyModel")`, `scene.domain_granularity_id = Some("surface")`.
- `scene.fit_json = Some(world3d_fit_json(energy_model_fit_revision(model), 1.25, None))` — the
  revision hashes the model name plus every rendered id/vertex, so the camera auto-frames once per
  distinct geometry and never takes back a camera the user moved.
- **Instance id = the raw `EntityId`** (`crate::scene::energy_scene_target_id`, asserted equal to
  `interaction::energy_target_id` by a unit test). Mesh ids are namespaced
  (`energy-surface-<id>` / `energy-window-<id>` / `energy-shading-<id>`); instance ids are NOT.
- Each instance also carries `label` (entity name), `objectKind`
  (`surface`/`fenestration`/`shading`) and the `selected`/`hovered` booleans.
- `overlay` is keyed by the RAW `u32` entity id (`EntityId.0`) → linear-rgb triple in `0..=1`, and
  REPLACES that entity's family swatch before any selection/hover tint. **Lane D: that is the map
  shape to produce.** `caption` is the legend string rendered as a `Label::data` row above the
  viewport by a local `with_caption` (fem3d's shape — there is no framework caption helper).

### Colour, and why it is baked per-vertex

`World3dHost`'s `PaintTexturedMesh`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:2100-2143`)
renders a data mesh with `side={DoubleSide}` and, **when the geometry carries a `color` attribute,
forces the material colour to white and the emissive to black** so the baked colours pass through
unmodified. Two consequences I had to design around and that the next lane should know:

1. **The host's own selected/hovered paint does not tint a vertex-coloured mesh.** Selection and
   hover tint are therefore baked in `crate::scene` (65 % towards `#3b82f6`; hover 25 % towards
   white). The per-instance `selected`/`hovered` flags are still published so the host chrome stays
   right.
2. **`WorldInstanceRecord` has no `color` field** (grep: nothing reads `instance.color`), so a
   per-instance colour lane does not exist — the mesh is the only colour channel. **There is also no
   alpha lane**: `transparent` follows the style's own opacity, never the payload. Glazing is
   therefore a SOLID light blue (`[0.537, 0.745, 0.898]`), not translucent. The report's §1c claim
   of an optional instance `"color"` is wrong for the current host.

`side={DoubleSide}` also means **no back-face duplication is needed** — a wall is visible from inside
the zone with one fan triangulation. That is why the quad triangle-count law is 2 triangles, not 4.

### Window geometry

`crate::scene::fenestration_polygon(host, window, index, count)` and
`crate::scene::model_fenestration_polygon(model, window)` **delegate to lane A's engine helpers**
`precompute::fenestration_polygon_in_bay` / `precompute::model_fenestration_polygon` (which prefer
`Fenestration.vertices_m` when non-empty and otherwise place the area/height/sill rectangle in the
window's bay), then lift the polygon `ENERGY_SCENE_WINDOW_OFFSET_M = 0.005 m` along the host's
outward Newell normal so the glazing never z-fights its wall. I first wrote a local re-derivation of
the exporter's `aperture_rectangle`; lane A's `vertices_m` + helpers landed mid-session, so I deleted
it — **what the viewport draws and what the solar model shades now come from one function.**

## 4. Layouts

Both modes are now `row[ Model (0.55) | column[ Structure, Zones, Simulation/Results ] (0.45) ]`,
built from the same `model_window_stack(kind, title, size)` helper the modes already had (it grew a
`size` argument). Both new window kinds are declared by their own manifest, so
`try_build_definition`'s "every layout window kind must be declared" rule holds.

## 5. Tests added (16)

`⚙️engine/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` — 8 plain `#[test]`s:
`a_quad_fan_triangulates_into_two_triangles`, `every_instance_id_is_the_raw_entity_id`,
`a_degenerate_surface_is_skipped_rather_than_faulting`,
`a_window_polygon_lies_on_its_host_plane_offset_by_the_lift`,
`selection_and_hover_retint_and_an_overlay_replaces_the_class_swatch`,
`the_fit_revision_follows_the_geometry_and_the_empty_model_renders`,
`every_surface_class_and_family_carries_its_own_swatch`,
`the_bestest_600_example_renders_every_surface_and_every_window`.

`✏️editor/…/🧊️model/🧪️tests/🔬️unit/🦀️.rs` — 8 `async_test`s decoding the built node back through
`semio_framework_plugin::artifact_app_laws::built_surface_scene`:
definition law (id/body/World3d/no actions/really translated),
domain+granularity+fit law, one instance per surface (6) and per window (2) of `bestest_600` keyed by
`energy_target_id`, scene-id ⇄ interaction-id equality, selection retint + selection lane, overlay
replace + caption wrapping (2 children, not a bare `Surface`), empty model, quad = 2 triangles,
window polygon on the host plane.

`👁️viewer/…/🧊️model/🧪️tests/🔬️unit/🦀️.rs` — 3 `async_test`s: definition law, `domain_id`/
`domain_granularity_id` both `None` **plus a byte-equality assertion that the viewer's payload equals
`crate::scene`'s own output** (one builder, two windows), empty model.

## 6. Gates — honest status

| Gate | Result |
|---|---|
| `cargo check -p semio-s-artifact-energy-model --lib` | ✅ green. Zero errors, zero warnings from any file I wrote (the 2 crate warnings are lane C/D's: an unnecessary qualification in `🪟️windows/⚡️simulation/🦀️.rs:40` and a dead `surface_options` in `📌️panels/🔍️inspection/🦀️.rs:204`). |
| `cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2` | ✅ green (2 m 33 s), same two peer warnings. |
| `cargo check -p semio-s-artifact-energy-model --lib --tests` | ⚠️ **the lib-test target does not compile**, for reasons entirely outside my files — see below. Zero of the remaining errors are in a file I authored or edited. |
| `cargo test -p semio-s-artifact-energy-model --lib -- windows::model scene viewer` | ❌ **NOT RUN — blocked by the above.** My 16 tests are WRITTEN, NOT RUN. |
| whole `cargo test -p semio-s-artifact-energy-model --lib` | ❌ **NOT RUN**, same blocker. |

Errors blocking the test target at the time of writing (peer lanes, mid-flight):
`✏️editor/🧪️tests/🔬️unit/🦀️.rs:634/802/807` (`EnergyModelMutation::semantics` does not exist) and
`:788` (`DslValue::from_pairs` / `DslValue: From<&str>` do not exist). Earlier in the session the same
target also failed on ~100 missing `🧫️fixtures/🧬️mutations/…/🔣️.json` files for the new glazing/gas
mutation leaves and on an undeclared `ResultField`/`ActionArgOption` in `🪟️windows/⚡️simulation/🦀️.rs`;
those cleared while I worked, so this set is expected to clear too.

**Owed:** once the crate's test target compiles, run
`cargo test -p semio-s-artifact-energy-model --lib -- windows::model scene viewer` and then the full
`cargo test -p semio-s-artifact-energy-model --lib`, and fix anything my 16 tests turn up. I filtered
`cargo check --lib --tests` output by file on every iteration and my three test files are error-free
at type-check level, but type-check is not execution.

## 7. Also still owed / known gaps

- **Zone volumes are not drawn.** `Zone` carries only a scalar `volume_m3` and no shape (see
  `📓️explore-energy-geometry-and-results.md` §1), so DoD item 1's "zones as volumes" is not met by
  this lane. The honest options are a per-zone convex hull of its member surfaces or wiring
  `geometry::zone_volume_from_surfaces`' pyramid decomposition — neither is in this lane's scope.
- **Fenestration/shading picks report the `surface` granularity.** The brief fixed
  `domain_granularity_id = "surface"` for the whole scene; a per-instance granularity lane does not
  exist on `World3dScene`, so a window pick arrives as `(energyModel, "surface", "<windowId>")`. The
  ID is right (which is what the tree link needs) and `energy_entity_kind` recovers the real family,
  but a consumer that switches on the reported granularity must not trust it.
- **No persisted per-window camera.** `fit_json` frames the model once per geometry revision;
  there is no `🎚️config` camera state (process3d/generation3d don't have one either).
- **`caption`/`overlay` have no producer yet** — the edit-mode dispatch passes `None, None`. Lane D
  supplies both; the plumbing and the tests are in place.
- **No browser probe.** Per the lane brief I ran no wasm activation and no dev server; lane E owns
  the restage + console/screenshot proof that the window actually paints.
