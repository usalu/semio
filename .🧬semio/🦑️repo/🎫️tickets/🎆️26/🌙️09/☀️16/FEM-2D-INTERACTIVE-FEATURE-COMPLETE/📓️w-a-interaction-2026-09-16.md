# 🕹️ Slice A — fem2d viewport interaction, picking and highlight (2026-09-16)

Agent: slice A (Opus). Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/`, crate `semio-s-artifact-fem-2d`, editor gated behind `--features component-app-assembly`.

## 1. Design

### Projection — one chain, inverted exactly

A fem2d layer is authored in **layer space** by `model::screen_2d(x, y) = (x·SCALE_2D + ORIGIN_2D, −y·SCALE_2D + ORIGIN_2D)` (`SCALE_2D = 20`, `ORIGIN_2D = 40`). The React `📐️Canvas2dHost` then puts layer space on screen with `worldToScreenLogical(p) = (p − camera)·zoom + size·0.5` (`📐️Canvas2dHost/🟦️.tsx:38-52`, and the same transform is applied to the canvas context at `:486-487`). Picking therefore runs in **canvas pixels**, not model meters:

- `fem2d_layer_to_canvas(camera, layer_point, width, height)` — the forward twin of `worldToScreenLogical`; every candidate glyph is projected through it and compared against the raw `x`/`y` the host sends.
- `fem2d_canvas_to_model(camera, x, y, width, height)` — `screenToWorldLogical` composed with the inverse of `screen_2d`; used only for the region point-in-polygon test, which is exact in model coordinates.
- `canvas_and_model_projections_round_trip` pins the pair against a non-trivial camera (`x 137, y −42, zoom 2.25`).

Consequence worth knowing: pick radii are **canvas pixels**, so they stay constant on screen under zoom — the same behaviour draw has.

### Pick precedence

`fem2d_hit_test(doc, camera, x, y, width, height) -> Option<(granularity, id)>`, each tier taking the nearest candidate and returning as soon as a tier has one:

| # | granularity | geometry tested | tolerance |
|---|---|---|---|
| 1 | `node` | distance to `screen_2d(node)` | `FEM2D_NODE_PICK_RADIUS_PX = 8` |
| 2 | `support` | distance to its node | `FEM2D_SUPPORT_PICK_RADIUS_PX = 11` |
| 3 | `load` | distance to the arrow glyph segment | `FEM2D_LOAD_PICK_RADIUS_PX = 6` |
| 4 | `element` | distance to the member axis segment | `FEM2D_ELEMENT_PICK_RADIUS_PX = 6` |
| 5 | `region` | point in outline, minus holes (model space) | exact |

The support radius is deliberately *wider* than the node radius: a support glyph is drawn on top of its node, so a click exactly on the joint yields the node and the ring just outside it yields the support — the plan's "when the node is not hit prefer the node", made decidable.

The load glyph is reconstructed by `fem2d_load_glyph`, which reproduces `fem2d_structure_layers`' `vector_layer` calls byte-for-byte (including its `y`-negation and the `±18` / `−12` glyph lengths), so what is picked is exactly what is drawn. `FemLoad::Area` anchors at the region outline centroid, `MemberUdl` at the member midpoint, `Nodal` at its node.

### Shared resolvers (also used by the inspector/tree slices — names are frozen)

- `fem2d_entity_kind(doc, id) -> Option<&'static str>` — all nine granularities, same precedence.
- `fem2d_load_owner(doc, load_id) -> Option<(&str, &FemLoad)>` — the owning **load case id** plus the load; a target id is always the raw load id.
- `fem2d_entity_model_point(doc, id) -> Option<(f64, f64)>` — node position / member midpoint / region centroid / support's node / load's anchor.
- `fem2d_region_centroid(region)`, `fem2d_layer_to_canvas`, `fem2d_canvas_to_model`.
- `fem2d_addressed_window_kind(view, fault_prefix)` and `fem2d_addressed_camera(cfg, view, fault_prefix)` — the addressed window's kind and its persisted camera, for both Canvas2d window kinds. Faults are `<prefix>.window-required|.window-stale|.window-kind`.

### Effects

Selection and hover are framework-owned; the app only reports which ids were hit.

```
interaction_select_effect(targets, merge)
  -> Effect::ReplayShellCommand { action_id: semio_framework::INTERACTION_SELECT_ACTION_ID,
       args: { domainId: "fem2d", targets: <JSON Vec<InteractionTarget>>, merge, method: "pick" } }
interaction_hover_effect(targets)
  -> Effect::ReplayShellCommand { action_id: semio_framework::INTERACTION_HOVER_ACTION_ID,
       args: { domainId: "fem2d", channel: "pointer", targets: <JSON Vec<InteractionTarget>> } }
```

Both are generic over `&[(impl AsRef<str>, impl AsRef<str>)]` so a panel can pass `(String, String)` rows. The `targets` string is decoded host-side by `serde_json::from_str::<Vec<protocol::InteractionTarget>>` (`🔌️plugin/🦀️.rs:594`); `select_and_hover_effects_carry_the_framework_wire_contract` proves the round trip with that exact decoder, not with a string compare.

`selection_merge_mode(shift, ctrl, meta)` → `replace` / `additive` (shift) / `subtractive` (ctrl|meta) / `invertive` (both), identical to draw's.

### Commands

- **`canvasPointerDown`** — `button != 0` returns `Emit::default()` (the host pans on the middle button and the right button is not ours). Otherwise: read the addressed window's camera, hit-test, emit one select effect. A miss emits an **explicit empty batch with merge `replace`** regardless of modifiers, so a background click always clears.
- **`canvasPointerMove`** — always emits one hover batch for whatever is under the cursor (an app cannot read framework-owned hover back at dispatch time, and the framework's hover machine drops an unchanged batch). No allocation beyond the one hit.
- **`canvasPointerUp`** — `Ok(Emit::default())`. The pick commits on press and this editor has no drag gesture.
- **`focusEntity { id }`** — resolves `fem2d_entity_model_point`, maps it through `screen_2d` into layer space (which is what a Canvas2d camera addresses) and delegates to `set_camera::handle_window` with the window's **existing zoom**. Unknown id → `fem2d.focus-entity.unknown-entity`. Works for both window kinds.
- **`removeSelection`** — `handle` now delegates to the new `pub fn remove_ids(doc, ids: &[String])`; an empty list is a no-op. `pub fn resolve_selection_ids(doc, ids) -> Vec<String>` filters a framework selection down to the ids this command can actually delete. See §4 for the wiring the coordinator still owns.

### Highlight painting

`model::fem2d_structure_layers_with(doc, node_color, line_color, support_color, interaction)` = the bare `fem2d_structure_layers` plus `fem2d_highlight_layers`. The bare function is untouched, so the viewer app (`👁️viewer/…/🧱️model/🦀️.rs`) and the story/vector fixtures are unaffected, and an empty `Fem2dInteractionSnapshot` produces byte-identical layers (asserted).

Overlay layers, hovered first then selected on top, with stable ids:

| entity | layer id | shape |
|---|---|---|
| node | `sel-node-<id>` / `hov-node-<id>` | bounds circle, `selected: true` when selected (the host's own amber selected-bounds treatment, `drawBoundsLayer`) |
| element | `sel-el-<id>` / `hov-el-<id>` | `segments` path, `stroke.width` 4 / 2.5 |
| support | `sel-support-<id>` / `hov-support-<id>` | bounds circle, wider than the node ring |
| load | `sel-load-<id>` / `hov-load-<id>` | `segments` path over the arrow glyph |
| region | `sel-region-<id>` / `hov-region-<id>` | closed `segments` path over the outline |

Colours: `SELECTION_COLOR_2D = #facc15`, `HOVER_COLOR_2D = #fde68a`. A `segments` layer is used wherever width matters — it is the only Canvas2d shape that honours `stroke.width` (`📐️Canvas2dHost/🟦️.tsx:229`); plain `line`/`polyline` layers are drawn at a fixed width.

`model::render_with_progress` and all three results renders (`render_static` / `render_modal` / `render_buckling`) now paint through `fem2d_structure_layers_with`. `results::render`'s public signature is unchanged; the three private renders each took one extra `interaction: &Fem2dInteractionSnapshot` parameter (slice E: keep it when you edit them, it is threaded straight from `render`).

## 2. Files

Owned and written:

- `✏️editor/🕹️interaction/🦀️.rs` — hit-testing, projections, resolvers, effects, window-camera helpers (regions `🔖️ScreenSpace`, `🔖️Geometry`, `🔖️HitTest`, `🔖️Effects`, `🔖️WindowCamera`).
- `✏️editor/🕹️interaction/🧪️tests/🔬️unit/🦀️.rs` — new.
- `✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` (new).
- `✏️editor/🎮️commands/🖱️canvas-pointer-move/🦀️.rs`, `🖱️canvas-pointer-up/🦀️.rs`.
- `✏️editor/🎮️commands/🎯️focus-entity/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` (new).
- `✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs` — `handle` → `remove_ids`, plus `resolve_selection_ids`.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs` — region `🕹️SelectionHighlight` and `render_with_progress`.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs` — highlight threading only (no animation code touched).

Touched outside my ownership, minimally, and why (§4).

## 3. Verification

- `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --tests --message-format short` — green (log `🗑️generated/check-fem2d-wa1.txt` and later runs).
- `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --target wasm32-wasip2` — green, exit 0 (log `🗑️generated/check-fem2d-wasm-wa.txt`). No native-only API is used: the whole slice is `f64` arithmetic, `dsl::json` and framework types.
- `cargo nextest run … -E "test(/interaction::tests::/) or test(/canvas_pointer_down::tests::/) or test(/focus_entity::tests::/)"` — **19 tests run, 19 passed** (log `🗑️generated/nextest-fem2d-slice-a-final.txt`).
  - `interaction` (9): projection round trip; node hit + empty-space miss on the demo; member/region/support/load resolution; camera-dependence; `fem2d_entity_kind`/`fem2d_load_owner` over all nine granularities; `fem2d_entity_model_point`; merge modes; the select/hover wire contract (decoded with `serde_json` into `Vec<protocol::InteractionTarget>`); stable `sel-*`/`hov-*` emphasis layers with the bare-layer equality law.
  - `canvas_pointer_down` (7): select effect on `n1`; modifier→merge table; background click clears; buttons 1/2 ignored; the results window picks with its own camera; unaddressed window and doc-scoped route both fault; hover + no-op release.
  - `focus_entity` (3): one window-config row for node/element/support/load/region; the addressed window kind is honoured for both canvases; unknown entity and unaddressed window fault.
- The wider run `🗑️generated/nextest-fem2d-editor-wa-slice-a4.txt` (62 tests, 56 passed) shows the six remaining failures are all the shared `fem2d_app()`/`dispatch` harness in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` — see §4.2. None are in slice-A files and they equally fail pre-existing tests (`add_node`, `set_camera`, `undo_restores_document_after_add_node`).

**Why the command tests call `handle_window` directly.** They were first written against `unit_tests::context::dispatch`. While running, that harness was in flux under a peer and every app-level dispatch test in the crate failed. The command tests were rewritten to build an `ArtifactView::new(&doc, &HistoryView::empty())` + `ConfigView { snapshot: &NoConfig::default(), window: None }` + a hand-built `ViewModel` and call `handle_window` directly. That is strictly more precise (it asserts on the returned `Emit`, including the effect args, instead of on a post-settle `InvocationResult`) and immune to harness churn.

## 4. What the coordinator must wire

### 4.1 `removeSelection` with an empty id list (the delete keybinding)

`ArtifactEditor::handle` already receives `_interaction: &InteractionView<'_>` (`✏️editor/🦀️.rs:915`). Add an arm before the `_ => command.dispatch(doc, cfg)` fallback:

```rust
Fem2dCommand::RemoveSelection(payload) if payload.ids.is_empty() => {
    let ids = interaction.selection(FEM2D_INTERACTION_DOMAIN).ids.clone();
    remove_selection::remove_ids(doc, &remove_selection::resolve_selection_ids(doc.snapshot, &ids))
}
```

`remove_ids` is `pub` for exactly this. The same arm is needed in `fem2d_retained_reduce` (`:203`) if `removeSelection` keeps its retained route — that function has `_interaction: &protocol::InteractionState`, whose `.ids` for domain `"fem2d"` is the same list.

### 4.2 The shared test harness (blocks every slice)

`✏️editor/🧪️tests/🔬️unit/🦀️.rs`'s `context::fem2d_app` must call `PluginApp::bind_instance_id(1)` (the id `meta("local")` stamps), otherwise **every** typed command is refused with `interactive-job.live-instance` before its handler runs. A peer added this and it was gone again at the last run — it needs to land and stay. Separately, an app-owned **retained** route returns an "empty started" `InvocationResult` from `dispatch_typed` (`🔌️plugin/🦀️.rs:27042`); if the crate keeps every command retained, `context::dispatch` has to settle the operation (`artifact_app_laws::settle_registered_typed_operation`) before a test can read `mutations`/`requested_effects`. Two of the six failures above (`renders_fem2d_model_scene`, `fem2d_visual_job_stale_cancel…`) are store-drop/close-witness issues in the same harness, also not slice-A.

Once the harness settles, two end-to-end assertions are worth promoting back:
- `focusEntity "ridge"` then `decode_fixture_scene::<Canvas2dScene>(&render(app, model::BODY_KEY))` ⇒ `(camera_x, camera_y) == screen_2d(4.0, 7.6) == (120.0, −112.0)`, zoom unchanged.
- `focusEntity "r1"` ⇒ `screen_2d(11.0, 2.8)`.

### 4.3 Minimal edit outside my ownership (unavoidable, flagged)

`✏️editor/🦀️.rs` `FEM2D_PUBLICATION_CONTRACTS`: `setResultAnimation` and `resultAnimationTick` declared `lanes: &[WindowConfig, HostOnly]`. The framework refuses a contract where `HostOnly` is not the **only** lane (`🔌️plugin/🦀️.rs:13259`), so `VcsArtifactApp::with_registry` panicked at construction and **no fem2d app-level test could run at all**. Both rows are now `&[WindowConfig]`. This does not restrict effects: lanes gate store publications only (`:26537-26546`); `Effect`s are never lane-checked (`setActiveExample` is `HostOnly` precisely because it publishes to no store lane). Slice E keeps full `Effect::DispatchAction` freedom.

No other file outside slice A was edited. `✏️editor/🦀️.rs`'s enum rows, bridge arms, manifest and render dispatch are otherwise untouched; the crate root `◻️2d/🦀️.rs` was not touched.

## 5. Open issues

1. **Rectangle (marquee) selection is declared but not implemented.** `fem2d_interaction_definition` advertises `SelectionMethod::Rectangle`; only `Pick` has a code path. A marquee needs a drag gesture across pointer-down/move/up, i.e. a window-transient (fem2d currently opts into `NoTransient`). Either implement it or drop `Rectangle` from the spec.
2. **Two area loads on one region are visually identical.** The demo's `l5` (dead) and `l7` (live) both draw at `r1`'s centroid with the same glyph, so a pick there is resolved by load-case order (dead first). Deterministic, but the glyph should be offset per case before this matters to a user.
3. **Hover is emitted on every pointer sample.** Cheap (one hit test, one small effect), and the framework dedupes, but under a fast pointer this is the highest-frequency command in the app. If the reactor shows pressure, the natural fix is a window-transient "last hovered id" guard, which is the same machinery item 1 needs.
4. **Materials, sections, load cases and combinations have no viewport geometry**, so they are selectable only from the artifact tree. `fem2d_entity_kind` resolves them; `fem2d_entity_model_point` returns `None`, so `focusEntity` faults for them — the tree should not offer a focus row action for those four granularities (slice B).
5. **Results-window highlight rides the undeformed backdrop.** Selected entities are painted at their undeformed positions even when the deformed shape is on screen, which is the right anchor for picking but may read oddly at a large deformation scale.
