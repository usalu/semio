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
| `…/✏️editor/…/🪟️windows/🧊️model/🧪️tests/🔬️unit/🦀️.rs` | 12 window tests |
| `…/✏️editor/…/🪟️windows/🧊️model/🎚️config/{🦀️.rs,🧬️schema/…,🧪️tests/🔬️unit/🦀️.rs}` | The per-window camera config, its `set-camera` mutation and 6 tests (§8) |
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

## 5. Tests added (25)

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

## 6. Gates — RUN, with results

All four gates were run in the foreground after the follow-ups below landed. Nothing here is
"written, not run".

| Gate | Result |
|---|---|
| `cargo check -p semio-s-artifact-energy-model --lib` | ✅ green, zero errors and zero warnings from any file this lane wrote. |
| `cargo test -p semio-s-artifact-energy-model --lib -- windows::model scene viewer` | ✅ **40 passed, 0 failed** (0.02 s). That is this lane's 25 tests plus the viewer's pre-existing ones. |
| `cargo test -p semio-s-artifact-energy-model --lib` | ⚠️ **6221 passed, 4 failed, 2 ignored** (31.6 s). The four are NOT this lane's: the three declared pre-existing reds `sim::tests::p7c1_weather_owner…`, `p7c2_preview_typed_view…`, `p7c2_restored_commit_bytes…`, plus lane D's own new `sim::tests::per_surface_conduction_losses_close_the_zone_air_balance_against_the_heating_meter`. Every editor/viewer/window/panel test passes. |
| `cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2` | ✅ green (52 s), only peer warnings. |

Two SHARED laws needed a one-line widening because two new verbs (lane D's and mine) had outgrown
them; both were failing before I touched them or would have started failing on `setCamera`:

- `✏️editor/🧪️tests/🔬️unit/🦀️.rs::document_verbs_publish_to_the_artifact_lane_settings_to_the_config_lane_and_examples_to_the_host`
  fell through to `HostOnly` for anything that is not a document verb or `set-simulation-settings`.
  It was **already red** on lane D's `set-result-field` (declared `Config`) before my change. The match
  now also names `SET_RESULT_FIELD_ACTION_ID => Config` and `model_window::SET_CAMERA_ACTION_ID =>
  WindowConfig`.
- `…::editor_declares_all_three_windows` / the viewer's twin now name the 3d window too (renamed to
  `…_declares_every_window`) and additionally assert that the 3d window is bound to
  `ENERGY_MODEL_INTERACTION_DOMAIN` on the editor and that the viewer declares no domain at all.

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
- ~~No persisted per-window camera~~ — **done**, see §8.
- **`caption`/`overlay` have no producer yet** — the edit-mode dispatch passes `None, None`. Lane D
  supplies both; the plumbing and the tests are in place.
- **No browser probe.** Per the lane brief I ran no wasm activation and no dev server; lane E owns
  the restage + console/screenshot proof that the window actually paints.

---

# Follow-ups (same lane, after the first restage)

## 7. The caption `DuplicateSiblingKey` fault — fixed

**Symptom** (coordinator, `🗑️generated/energy-results-w1/console.txt`): as soon as a run ticked and
`render_body` started passing `Some(&colors), Some(&caption)`, every publication of the surface
faulted with `1:energy.model.3d [producer]: DuplicateSiblingKey parent=#0 key=#0` and the window went
blank. With `caption: None` it rendered fine.

**Cause**: my `with_caption` stacked two children that both defaulted to key `#0` — an unkeyed
`built_text_node` and an unkeyed `column()` wrapper. fem3d's `📊️results/🦀️.rs::with_caption`, which I
copied, has the same latent shape; it never bites there because that window is published through a
different path. The fault is at PUBLICATION, not at `try_build`, which is exactly why my original
"the caption wraps the scene" test passed type-check and would have passed at runtime too — it only
counted children, it never asked whether their keys differed.

**Fix** (`…/🪟️windows/🧊️model/🦀️.rs`): both siblings now carry explicit ids,
`CAPTION_NODE_ID = "energy.model.3d.caption"` and `CAPTION_SCENE_NODE_ID = "energy.model.3d.scene"`,
and the caption `Label` is built through `semio_framework_ui_contract::Label::try_from` +
`text(..).try_id(..)` instead of the id-less `built_text_node`. `render`'s signature is **unchanged** —
the coordinator's call site needs no edit.

**Guard**: `a_captioned_overlay_render_has_unique_sibling_keys_and_still_decodes` renders
`bestest_600` with an overlay for all six surfaces AND a caption AND a selection AND a hover, then
(a) walks the WHOLE built tree asserting every sibling set has distinct keys, (b) asserts the two root
children are exactly the two new ids, and (c) decodes the surface back through
`built_surface_scene` to prove the captioned scene is still domain-bound and still carries every
instance. A `surface_node` helper finds the surface inside the caption column, since
`built_surface_scene` needs the `Component::Surface` node itself.

## 8. `setCamera`

**Symptom**: orbiting dispatched `setCamera` from window kind `energy.model.3d` and the shell dropped
it — `no window kind declares it`. `setCamera` is NOT framework-reserved (unlike
`interactionSelect`/`interactionHover`): the react host sends it on its own after every gesture
(`worldCameraSetCameraDispatchArgs`, `🌐️World3dHost/🟦️.tsx:767`, debounced), and a window kind that
does not declare it drops every orbit.

**What landed** — the fem3d / wires-canvas `WindowConfigOwner` shape, per WINDOW INSTANCE:

| Piece | Where |
|---|---|
| `EnergyModelCameraPose { position:[f64;3], target:[f64;3], zoom }` + `EnergyModelWindowConfig { camera }` | `…/🧊️model/🎚️config/🧬️schema/🦀️.rs` |
| spec surface (`🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`) | `…/🧊️model/🎚️config/🧬️schema/` |
| `set-camera` mutation leaf + enum + text/binary op codecs | `…/🎚️config/🧬️schema/🧬️mutations/{🦀️.rs,🎥️set-camera/…}` (+ its 4 spec files) |
| `ArtifactDsl`/`ArtifactPack`/`impl_whole_record_config!`, `EnergyModelWindowConfigOwner`, `current()`, `addressed()` | `…/🧊️model/🎚️config/🦀️.rs` |
| `SET_CAMERA_ACTION_ID = "setCamera"`, `set_camera_action()` (`ActionKind::View`, `Migrated`, declared `camera` arg), `definition().actions` | `…/🧊️model/🦀️.rs` |
| `render_with_camera(.., Option<&EnergyModelWindowConfig>)`; `render` keeps its 4-arg signature and delegates with `None` | `…/🧊️model/🦀️.rs` |
| roster + command variant `SetCamera { camera: String }` + `action_id()` + bridge (`camera_pose_json`) + `camera_emit` + `reduce` guard + `WindowConfig` publication contract + `"setCamera"` proof row + `register_window_config_owners` + `render_body` | `✏️editor/🦀️.rs` |

Three things worth knowing:

- **Why the action had to be retained.** `InteractiveJobClassification::Migrated` is the ONLY
  UI-dispatchable classification, and a `Migrated` verb with no owned reducer is refused with
  `interactive-job.missing-owned-reducer`. So `setCamera` joins `ENERGY_MODEL_RETAINED_TOOL_IDS` (last,
  matching the command enum's declaration order, which `retained_roster_is_exact_and_exhaustive`
  asserts), the factory contracts and the proof rows — exactly like lane D's `set-result-field`.
- **Why the emit is not in `reduce`.** A camera is addressed at ONE window instance and `reduce(command,
  doc)` sees no `ViewModel`. Both dispatch routes intercept first through a new
  `camera_emit(command, view_state)`: `ArtifactEditor::handle` passes its own `_view_state`, and the
  retained `energy_model_reduce` passes `context.view_state`. `reduce`'s `SetCamera` arm is a fault
  ("a camera change requires a concrete 3d model window"), the same unreachable guard fem3d writes.
- **Why the arg is declared.** `effective_action_args` keeps ONLY declared arg ids once an action
  declares any, so an undeclared `camera` would have been filtered out before the bridge saw it.
  The bridge validates the pose (`FromValue` + finite + positive zoom) and refuses a malformed one
  rather than silently ignoring the gesture. The emit carries `UiDirtyScope::Partial` with nothing
  listed — the pane that sent the pose already holds it — and a `coalesce_key` per window instance so a
  burst of debounced orbit ticks collapses.

**`fit_json` does not re-frame after an orbit — checked.** The host's one-shot framing keys on
`world3dAutoFitKey(fit.revision, sceneCameraAttachJson, autoFitBounds)` and is additionally gated by
`userMovedFitRevision`; `sceneCameraAttachJson` only advances when
`shouldReattachWorldViewportCamera(previous, next, lastDispatchedWorldCamera)` says the change is
EXTERNAL, which it never is for a pose the host itself just sent. On the producer side both
`energy_model_fit_revision(model)` and the published `camera_json` are pure functions of the model (and
now of the stored pose), so re-rendering the same model republishes byte-identical values.
`a_re_render_of_the_same_model_never_re_arms_the_auto_fit` asserts exactly that: a selection changes
neither `fit_json` nor `camera_json`, while switching to `bestest_610` does change `fit_json`.
`a_stored_camera_replaces_the_model_derived_one` additionally asserts that a stored pose changes
`camera_json` and NOTHING else (meshes, instances, fit, domain all byte-identical).

**Deviation from the brief, again in the honest direction:** none on this one — this is the
per-window config the follow-up asked for, not an app-config shortcut. `EnergyModelConfig` was ruled
out on purpose: its own law `defaults_and_ranges_follow_the_schema_source_of_record` requires every
property to be numerically bounded or enumerated, which a camera pose is not, and it is lane D's live
file.

## 9. Still owed after these follow-ups

- **The VIEWER's 3d window has no `setCamera`.** Orbiting the read-only twin will drop the same
  dispatch the editor used to drop. `ArtifactViewer` has its own
  `bounded_first_step_tool_proofs` hook for exactly this case ("a viewer that declares view-only
  actions (camera orbit, show-mode, sun) needs them… a `Migrated` verb without an exact app-owned
  reducer is rejected with `interactive-job.missing-owned-reducer`"), and a viewer's publication
  contracts may never name the artifact lane — so the same `WindowConfigOwner` would have to be
  registered on `EnergyModelViewer` with a viewer-side command enum. Not done: the viewer is not on
  the ticket's restage path.
- **Zone volumes are still not drawn** (§7 of the original list) — `Zone` carries only a scalar
  `volume_m3`, so DoD item 1's "zones as volumes" needs either a per-zone convex hull over its member
  surfaces or `geometry::zone_volume_from_surfaces`' pyramid decomposition.
- **No browser probe from this lane.** The coordinator's restage already proved the window paints, the
  8 instances and the pick; what is NOT yet re-proved in the browser is (a) the caption path after the
  `DuplicateSiblingKey` fix and (b) an orbit surviving a re-render now that `setCamera` persists. Both
  are unit-tested; neither has been seen in the running app since the fix.
