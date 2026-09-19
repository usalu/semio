# 🧲️ W14g — the World3d scene remainder: pick-target lane, curved edges, per-instance opacity, style rows, gumball config

Packet W14g of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Source + tests only: **no wasm build, no
activation** — W14d owns the puzzle3d live loop and picks this source up.

Five items were handed off by W2f §5, W8b §7.4 and W9b §7. Each was verified by grep against the
current tree before any edit; §6 records which were already done and were therefore **not** re-done.

## 0. The five items at a glance

| # | item | verdict | where |
| --- | --- | --- | --- |
| 1 | pick-target LANE for `World3dScene` | **new** — wire lane, producer, reader, ray hit test, dispatch | §1 |
| 2 | curved edges (`CadEdgeCurve` carried only `kind`) | **new** — curve parameters + the whole `edgeSamplePoints` ladder in Rust | §2 |
| 3 | per-instance opacity for locked dimming | **new** — `opacity` on the instance lane, applied at the bridge | §3 |
| 4 | the `celebrated`/`disabled` (and `highlighted`) `MESH_STYLE_PAINT` rows | **table already complete (W9b); the PRODUCER was missing** — three wire flags added | §4 |
| 5a | `hoveredComponent` JSON reader | **already done** — not re-done, see §6 | §6 |
| 5b | `gumballConfig` JSON reader | **new** — the config was PUBLISHED by `cad` and ignored here | §5 |
| 5c | vertex marker sizing | **was wrong** — world-unit crosses, now React's screen pixels | §5 |

---

## 1. The pick-target lane

### React source

- `✏️s/🔌️plugins/📐️cad/…/✏️editor/⚙️engine/📺️renderer/🟦️.tsx:2742` `SpatialPickGeometryLayer` — splits the
  overlay in two: `renderedTargets` (`resolveSpatialPickTargetsToRender`, which KEEPS a hidden/locked
  target so it can be drawn dimmed) and `selectableTargets`
  (`filterSpatialPickTargets` ∩ `filterSpatialPickTargetsForEntityFlags`), and only the second gets a
  `SpatialPickHitTarget`.
- `:2295` `targetRayScore` — the authoritative hit test: the AABB of `points` (or `[point]`),
  `expandByScalar(kind === "vertex" ? 0.12 : 0.08)`, ray-vs-box, score =
  `ray.origin.distanceTo(hit) + spatialPickPriority[kind] * 1e-4` with `vertex 0 / edge 1 / face 2 /
  object 3`. Lower wins, so **at equal distance the finest kind wins**.
- `:2428` `spatialPickTargetsFromRay` — sorts by that score.
- `⚙️engine/🧲️picking/🦀️.rs:301` `spatial_hover_key_aliases` / `:284` `pinned_pick_target_keys` — the
  `solid:` ↔ `object:`, `shell:` → `face:`, `wire:` → `edge:`, `anchor:` → `vertex:` aliasing.
- `:1150` `target_style` — the selected/hovered/locked/typology/default cascade, in theme TOKEN ids.

### The wire

A 21st `World3dScene` lane, `pickTargetsJson`, carrying
`{kind, id, point, points, typology, selectable, style{color,emissive,opacity,lineWidth}}`:

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:392` (field), `:666` (`World3dSceneLane::PickTargets`),
  `:695`/`:717`/`:743` (the three name tables), `:776` (`ALL`), plus `take`/`put`/`base`/`ToValue`/`FromValue`.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json` — the one
  language-neutral declaration both sides are pinned against (the existing Interpreter law
  `expect(WORLD3D_SCENE_LANES).toEqual(contract.lanes)` covers the new row for free).
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:295` (type) and `:499` (`WORLD3D_SCENE_LANES`).
- `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:132` — the render-plan size validator checks the new lane
  like every other optional JSON payload.

**Boundedness.** The producer caps the row count (`CAD_PICK_TARGET_LANE_BUDGET = 192`) and each row's
polyline (`CAD_PICK_TARGET_LANE_POINT_BUDGET = 64`); the consumer caps both AGAIN at parse time
(`WORLD_PICK_TARGET_CAPACITY = 256`, `WORLD_PICK_TARGET_POINT_CAPACITY = 64`,
`🌍️world/🦀️.rs:591`/`:597`/`:739`). No unbounded `Vec` reaches the wire or the retained state, and a
consumer never has to trust a producer's cap.

### The producer (`cad`)

`…/✏️editor/🎭️modes/✏️edit/🦀️.rs:414` `pick_target_lane_items` — React's own pipeline
(`create_spatial_pick_targets` → `filter_spatial_pick_targets_for_active_view` →
`resolve_spatial_pick_targets_to_render` with the hovered/selected keys pinned through
`spatial_hover_key_aliases`), each row styled by `target_style` and carrying
`selectable = SpatialEntityFlags::selectable()`. Published at `:496` behind the SAME gate as the
paint overlay — React's `hostSelectionEnabled`/`replHostGeometryPickingEnabled`: while the pane's
live session accepts a selection, and at no other time.

### The consumer (wgpu)

- `🌍️world/🦀️.rs:637` `WorldPickTargetRecord` / `:628` `WorldPickTargetStyleRecord`; parsed at
  `:11565` into `state.pick_targets` with the caps applied.
- `:707` `world_pick_target_ray_score` — React's `targetRayScore`, padding and priority weight
  included; `:686` `world_pick_target_bounds` is the padded AABB.
- `:717` `world_pick_key_alias` / `:731` `world_pick_keys_match` — the framework twin of
  `spatial_hover_key_aliases`, used to dedupe the sub-object hover.
- `:4501` — the **pre-pass** inside `WorldRayPickCursor::step`: one retained pick target per step
  (bounded by construction), only for the `Instance`/`Hover` purposes, only for `selectable` rows.
- `:4654` — one ordering across both: the pick target wins when `score <= mesh hit distance`, which
  is r3f's own rule (it raycasts `SpatialPickHitTarget` and the model in the same scene) with
  React's kind tie-break already folded into the score.
- `:4716` `pick_target_plan` — the dispatch: `interactionSelect`/`interactionHover` on the bound
  domain, **granularity = the target's own kind**, id = the kernel entity id.
- `:7196` — the `Hover` publish arm now updates `state.pick_hover_key` for a sub-object hover
  (`numbers[1]`) and `local_hover_id` otherwise; a clear clears both.
- `:9286` `append_pick_target_hover_lines` — paints the hovered target in its own `target_style`
  colour (`:9260` resolves the theme tokens), a screen-sized cross for a vertex and the hit box's
  wireframe otherwise, so what the user sees is what was hit.

### Law

`the_pick_target_lane_hit_tests_finest_first_skips_unselectable_rows_and_dispatches_react_payloads`
(`🌍️world/🧪️tests/🔬️unit/🦀️.rs`) — parses the lane off a real `UiComponentSceneNode`, asserts the
finest kind wins a tie by React's score, then drives a REAL click through
`enqueue_world3d_event` → `step_world3d_interaction` → the bounded action queue and reads the
published `interactionSelect`: `domainId` is the bound domain, `granularity` is `vertex` (not the
scene's `object`), `id` is the kernel entity id, and the unselectable row never appears.
Two more in the same region: `the_pick_target_lane_is_capped_at_parse_time_however_large_it_arrives`
(the boundedness law, and that a broken lane empties rather than faults) and
`pick_hover_keys_match_across_the_kernel_and_pick_aliases`.

---

## 2. Curved edges

### React source

`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts:262` `edgeSamplePoints` and the four samplers it
delegates to: `arcSamplePoints` (`:140`), `circleSamplePoints` (`:183`), `ellipseSamplePoints`
(`:196`), `nurbsDisplaySamplePoints` (`:209`), over `arcPlaneFrame` (`:117`),
`arcFrameFromRadiusPoint` (`:154`) and `arcSweepRadians` (`:131`).

### wgpu/Rust change

- `…/🚪️io/🗺️geometry-import/🦀️.rs:69` — `CadEdgeCurve` grew the curve parameters
  (`center`, `normal`, `radius`, `majorAxis`, `majorRadius`, `minorRadius`, `poles`, `degree`,
  `through`) **and** a `points` escape hatch for a producer that already has the kernel's own
  tessellation. It carried `kind` and nothing else, which is why every curved edge degraded to its
  two boundary vertices.
- `…/✏️editor/⚙️engine/🧲️picking/🦀️.rs:500`-`:557` — the whole ladder in Rust
  (`arc_sample_points`, `circle_sample_points`, `ellipse_sample_points`,
  `nurbs_display_sample_points`, `edge_sample_points`), with React's own segment floors
  (`EDGE_SAMPLE_SEGMENTS = 32`, `CLOSED_CURVE_SAMPLE_SEGMENTS = 64`) and React's `nurbs` span rule.
  `circleSamplePoints`' quirk — its frame is built from `center + normalize(normal) * radius`, so the
  circle's `u` IS the normal direction — is ported verbatim rather than "corrected", because a
  divergence there would move every circle both hosts draw.
- `:593` `GeometryBuckets::edge_points` now returns that polyline, so it reaches
  `entity_points`, `entity_wire_segments`, `wire_points`, `face_points`, `all_edge_segments` and
  therefore BOTH the painted overlay and §1's pick lane in one change.

### Law

`a_curved_edge_samples_its_whole_polyline_where_a_straight_one_stays_its_endpoints` — the four
families, their sample counts, the closed-curve floor, and the `points` override — and
`a_curved_edges_pick_target_carries_its_tessellation`, which swaps one box edge for an arc and
asserts its pick target carries 33 points and 32 wire segments while its straight sibling keeps 2.

---

## 3. Per-instance opacity (locked dimming)

React dims a locked entity by `WORLD_LOCKED_OPACITY_SCALE = 0.35`
(`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:178`, `:4020`, `:4271`); the Rust engine's `target_style` locked arm
already computed it (`🧲️picking/🦀️.rs:1160`) and could not reach a COMMITTED mesh, because the
instance lane had no opacity field (W2f §5.5).

- `🌍️world/🦀️.rs:1025` — `World3dSceneInstanceEntry.opacity: Option<f64>`.
- `:11497` — applied to the authored colour's alpha at bridge-publish time, so it survives the
  snapshot page (which carries a colour, not a style) and composes with whatever `MESH_STYLE_PAINT`
  row the instance later resolves to.
- `…/🎭️modes/✏️edit/🦀️.rs:123` — `cad` publishes `opacity` (and `disabled`) for `object.locked`.

Law: `the_instance_lane_carries_locked_opacity_and_the_three_unreachable_style_rows`.

---

## 4. The mesh style rows

**Verified by grep first, and the table was already complete.** W9b landed all seven rows of
`MESH_STYLE_PAINT` with React's values and React's priority ladder (`🌍️world/🦀️.rs:7648`
`mesh_style_paint`, `:7665` `resolve_mesh_style`) — `celebrated`'s conic triad included. What was
still missing is what W9b's own §7.3 named: **no producer** — `World3dSceneInstanceEntry` carried
only `provisional`, so three of the seven rows were reachable by the table and by nothing else.

- `🌍️world/🦀️.rs:1010`-`:1020` — `highlighted`, `disabled`, `celebrating` on the instance lane,
  matching React's `WorldInstanceRecord` (`🌐️World3dHost/🟦️.tsx:180`, `:183`) and
  `isWorldInstanceCelebrating`.
- `:1734` — retained as `highlighted_instance_ids` / `disabled_instance_ids` /
  `celebrating_instance_ids` beside the existing `provisional_instance_ids`.
- `:11846` — the draw lane now fills the WHOLE `MeshStyleState` instead of three of its six fields,
  so the ladder `disabled → provisional → celebrated → selected → highlighted → hovered → neutral`
  actually runs.

`WORLD3D_SHADER` was **not touched**, so the four-backend shader-contract twin did not need to grow
(see §7).

---

## 5. Gumball config + vertex markers

### 5a — `hoveredComponent`: already done, not re-done

`state.hovered_component_id` / `_object_id` / `_mode` are read from `selection_json.hoveredComponent`
(`🌍️world/🦀️.rs:11335`) and consumed by the hover paint (`instance_hovered_component_id` `:9196`,
`instance_hovered_component_matches` `:9770`) and by the component pick/hover cursors. Left alone.

### 5b — `gumballConfig`: the config was published and ignored

`cad` already publishes it — `…/🎭️modes/✏️edit/🦀️.rs:263` writes
`gumballConfig {moveAxes, movePlanes, rotate, scaleAxes, scalePlanes, scaleUniform}` for the
dislocate utility — and **nothing on this target read it**. The handle gate was a `transform_mode`
string match that admitted the three move axes and the three move planes UNCONDITIONALLY, so a
`rotate`-mode gumball still offered translation where React offers only the rings.

- `🌍️world/🦀️.rs:11293` `World3dSceneGumballConfigRecord` — every group is `Option<bool>`, because
  React's `resolveGumballConfig` reads each as `config?.group !== false`: an absent key means
  ENABLED and only an explicit `false` hides a group.
- `:11318` `for_transform_mode` — React's `gumballConfigForTransformMode` (`🌐️World3dHost/🟦️.tsx:2683`),
  matched on the PREFIX so this host's own verb spellings (`rotateSelection`, `scaleSelection`,
  the default `translate`) land in React's three arms.
- `:11342` `admits` — React's `gumballHandleEnabled` = group flag ∩ `GUMBALL_PLANE_HANDLES`
  (`🧱️elements/🎬️Scene/🟦️.tsx:441`, `:489`, `:506`).
- `:11372` `world3d_gumball_config` — authored config first, mode fallback second.
- `:5311` (the production pick cursor) and `:9967`/`:9983`/`:10002` (the paint) now share that ONE
  gate, so a handle can never be drawn without being pickable or picked without being drawn.

### 5c — vertex markers were world-unit crosses

The markers were `VERTEX_BASE_SCALE * 0.15` = **0.0075 world units** at any distance — sub-pixel on
any scene larger than a few metres, the same defect React shipped and fixed on lowpoly
(`pointsMaterial` size in world units → `sizeAttenuation={false}` pixels).

- `🌍️world/🦀️.rs:9165` — React's `WORLD_VERTEX_DOT_PX = 6` / `WORLD_VERTEX_MARK_PX = 11`
  (`🌐️World3dHost/🟦️.tsx:2499`, used at `:3062` and `:3097`-`:3107`).
- `:9171` `world_units_per_pixel` — the inverse of `sizeAttenuation`, perspective and parallel.
- `:9183` `vertex_marker_half_extent`, used by `append_component_overlays` (`:9315`, which now takes
  the camera and viewport) and by §1's pick-target hover paint.

Laws: `the_gumball_offers_the_authored_config_then_the_transform_mode_fallback_intersected_with_the_plane`
and `vertex_markers_hold_their_pixel_size_at_every_camera_distance`.

---

## 6. Verified by grep first — what was already done, and one thing nobody had noticed

| claim from the hand-offs | grep verdict | action |
| --- | --- | --- |
| `hoveredComponent` JSON reader missing | **present** — `🌍️world/🦀️.rs:11335` parses it, `:9196`/`:9770` paint from it | none |
| the seven-row `MESH_STYLE_PAINT` table missing rows | **complete since W9b** (`:7648`) — only the PRODUCER was missing | §4 |
| `celebrated`'s conic SPIN | **still absent by design** — no per-instance flag in `WORLD3D_SHADER`; the row paints React's own solid fallback | not done, §7 |
| `oklab_mix` for the `disabled` row | **present** (`ui_styling::color::oklab_mix`) | none |
| the engagement-preview lane | **present since W2f** (`:9333`) — it paints, it never hit-tested | §1 |

**Found while doing §5b and not in any hand-off:** `selection_json.transformMode` had **no reader at
all** on this target. `state.transform_mode` was initialised to `"translate"`
(`🌍️world/🦀️.rs:1908`) and never assigned again, so the rotate/scale arms of the old gumball gate
(`matches!(state.transform_mode.as_str(), "rotate" | "rotateSelection")`) were dead code: the rings
and the scale handles could be picked only by a state no wire could produce. `:11682` reads it now,
which is what makes the mode fallback in §5b reachable at all.

---

## 7. What this packet did NOT do

1. **No wasm build and no activation** — the packet's own instruction. W14d owns the puzzle3d live
   loop and picks this source up; §8 says what to look for.
2. **`WORLD3D_SHADER` untouched**, so the four-backend shader-contract twin was NOT grown. The
   `celebrated` conic spin still needs a flag bit in the instance `flags: vec4<f32>` plus the MSL and
   HLSL mirrors (W9b §7.2) — unchanged by this packet.
3. **No React reader for the `pickTargets` lane.** W2f's hand-off asked for readers in BOTH
   `🌐️World3dHost` and `♾️infinite/🌍️world`; this packet did the wgpu half and the shared wire. React's
   shell path has no sub-object pick affordance either (W2f §I4.2 — "the first on EITHER target"), so
   the lane is currently produced and consumed by one renderer. §8 carries it.
4. **The screen-space marquee over pick targets** — a rubber band still selects instances only.
5. **No probe run, no browser, no screenshot.**

---

## 8. Hand-offs for W14d's live check

1. **The lane only appears while a `cad` engagement session accepts a selection.** On puzzle3d it is
   never published (puzzle3d has no `SpatialPickTarget` producer), so the expected live verdict there
   is "no regression", not "sub-object picking works". To see it, open a CAD pane, start a
   construction interaction that waits for a selection, and read `pickTargetsJson` off the surface.
2. **What to look for in `dumpChrome`/the action log:** a click on a corner of a CAD solid should
   publish `interactionSelect` with `targets` = `[{"granularity":"vertex","id":"<kernel id>"}]` —
   NOT `{"granularity":"object","id":"<instance id>"}`. A move over one should publish
   `interactionHover` with the same granularity, and exactly once per target (the alias dedupe).
3. **The gumball gate changed shape.** Any window whose selection JSON carries no `gumballConfig`
   and no `transformMode` now gets React's move-only fallback (move axes + move planes, no rings, no
   scale handles) where this target previously ALSO admitted nothing else — but a window that
   publishes `transformMode: "rotate"` now loses its translation handles, which is React's behaviour
   and a visible change. `cad`'s dislocate utility publishes an explicit config and is unaffected.
4. **Vertex markers will look bigger** on any scene larger than a few metres — they were sub-pixel.
   If a pane looks noisy under the `vertex` granularity, that is this change, and the knob is
   `WORLD_VERTEX_DOT_PX` / `WORLD_VERTEX_MARK_PX`.
5. **Locked CAD objects now paint dimmed and muted** (`disabled` row at 0.45 × the 0.35 opacity
   scale). If a pane reads "too dark", check `object.locked` before blaming the style table.
6. **One new wire lane.** A renderer that reassembles scene lanes by name gets `pickTargets` for
   free; anything that enumerates them positionally must be re-generated. The TS mirror, the neutral
   fixture and the Interpreter size validator are already updated here.

---

## 10. Files

**Shared wire**
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` — `pick_targets_json` + the `PickTargets` lane
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json` — the neutral declaration
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` — the TypeScript mirror (type + `WORLD3D_SCENE_LANES`)
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs` — literal
- `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` + `…/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs` — size validator

**wgpu world**
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` — items 1, 3, 4, 5
- `…/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — six laws

**CAD (producer + curve sampling)**
- `✏️s/🔌️plugins/📐️cad/…/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️.rs` — `CadEdgeCurve` parameters
- `…/✏️editor/⚙️engine/🧲️picking/🦀️.rs` — the `edgeSamplePoints` ladder + `edge_points`
- `…/✏️editor/⚙️engine/🧲️picking/🧪️tests/🔬️unit/🦀️.rs` — two curved-edge laws
- `…/✏️editor/🎭️modes/✏️edit/🦀️.rs` — the `pickTargets` producer + locked `disabled`/`opacity`
- `…/🧬️schema/💡️inferences/🧪️tests/🔬️construct-query-unit/🦀️.rs` — literal

---

## 9. Gates

Every command in the foreground, `-j 4`, one cargo at a time, stdout captured under
`🗑️generated/w14g-*.txt`. Machine load averaged 80-105 from peer builds throughout, so each gate
took tens of minutes.

