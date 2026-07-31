# w1c-paint-parity — final report

Scope: `ui/wgpu/rs/lib.rs`'s `paint` region only (`region-claims.json`'s `w1c-paint-parity` claim). `widgets` was read-only reference. One additive test was also added to `engine`'s test submodule, per the brief's explicit exception. All other regions (`arena`, `tree`, `reconcile`, `flex`, `text`, `events`, `shell`, `engine` proper, `widgets`, `component`, `theme`, `re-exports`, `host`, `cursor`, `scene_slots`, `Cargo.toml`) were left untouched.

## Per-fidelity-gap summary

**Button disabled/loading** — Loading was already implemented before this pass (`draw.push_loading_border`); re-routed through a new shared `paint_loading_border` helper. Disabled was unimplemented (`UiButtonNode.disabled` read nowhere); `widgets::render_button`/`WidgetNode::Button` has no `disabled` field at all, so nothing to port — independent fix using a `dim()` closure (`Rgba::with_alpha`, halves alpha) on border/bg/icon/text, mirroring `ui/js/react/index.tsx`'s `disabled:opacity-50` convention; disabled buttons no longer honor `HOVERED`.

**Slider unit-label** — `UiSliderNode.unit` was never read; `WidgetNode::Slider` has no `unit` field either. Ground truth: `framework/renderer/react/ui-interpreter.tsx`'s `case "slider"` (`{value} {unit}`, muted small, trailing). Ported as a right-aligned muted-small readout inside the slider's own bounds (React claims extra flex-row width; `paint` can't — that's `flex`, out of scope).

**NumberStepper** — two fixes: (1) nested center-value border, closing the exact 14-vs-19-instance divergence `golden_number_stepper_known_gap`'s doc comment documented (confirmed precisely by a mid-task coordinator note from the `w0-engine-facade` agent); ported `widgets::render_number_stepper`'s nested `push_control_border` around the center segment. (2) "Mixed" display: `uniform:false` now shows `UI_INSPECTOR_MIXED_PLACEHOLDER` ("Mixed") in `theme.text_muted`, per `ui-interpreter.tsx`'s `mixed: !control.uniform` — `widgets` ignores `uniform` entirely (both its format branches are identical), so this isn't a widgets port either. The brief's "single-border visual style" phrasing is superseded by the coordinator's more precise nested-border finding.

**Vec3 mixed-state** — `value: None` was unwrapped to `[0.0;3]` (editable-looking zeros). `widgets::render_vec3` has the same bug. Ground truth: `ui-interpreter.tsx`'s vec3 case (`mixed = tuple == null` → `value=""`, `placeholder="—"`, `disabled=true`). Ported as em-dash text in `theme.text_muted` + half-alpha-dimmed border per axis.

**Select open popup** — still gapped, confirmed non-actionable from `paint` alone: `tree::WidgetState` is still an empty marker (no open/closed slot anywhere), `NodeFlags::HAS_POPUP` exists but nothing sets it, and `reconcile::children_of` doesn't expand `Select`. No "is this open" signal exists in the retained model yet. Left closed-rest-state-only with an updated doc comment.

**Tree hover/drag/collapsed-state extras** — split static from live-state-dependent. Individual tree rows have no `NodeId`/`NodeFlags` (same root cause as Select's gap — `reconcile` doesn't expand `Tree` into per-item children), so hover/drag remain gapped. Ported everything static: ancestor guide lines (`paint_tree_guides`, adapted from `widgets::tree_draw_guides` for `paint`'s depth-starts-at-1 convention), selected/highlighted text now use `active_foreground` (was always `text_element`), section-header folder icon + collapsed-aware color, item `description` text, always-visible (`reveal_on_hover: false`) `actions` icons, and a new `paint_control` adapter that renders a row's inline `control: Option<UiControlNode>` (previously never painted at all).

**Field description/required/error** — `paint_field` only drew the bare label; `widgets`' `WidgetNode::Field` doesn't carry these fields either (explicit `..` in its match arm), so this is an independent port from `ui/js/react/index.tsx`'s `Field` component: label + `*` required marker (`theme.error`), description (muted-small), error (`theme.error`) below the label. Positioned relative to `bounds.y` only since `reconcile`/`flex` don't yet reserve the child's layout slot below this text (pre-existing, documented `flex` gap, out of scope).

**Section/Stack/Tree/Button loading-border shared helper** — added `paint_loading_border(draw, bounds, color, theme)`, wrapping the pre-existing `draw::DrawList::push_loading_border`. Wired into `Stack` and `Tree` (both had unused `loading` fields); `Section`/`Button`/`TreeItem` now route through the same helper.

**Correction to the brief's animation-clock assumption**: the brief assumed no animation-clock scaffolding exists. That's stale — `draw::UiInstance::loading_border`'s own pre-existing doc comment describes a real time-varying spinning/pulsing ring driven by `UI_SHADER`'s `kind==6` branch and `render_frame`'s `time_seconds` param. `draw`/`shaders`/`gpu` already have this wired; `paint` just needed to call the existing primitive from more places. Flagged rather than building a redundant fake clock.

**Image/ComponentScene/ExternalSlot** — untouched as instructed; verified no regression.

## Files touched

- `ui/wgpu/rs/lib.rs`, `paint` region (lines ~10891-11785): imports; `paint_node`; `paint_button`; `paint_select` (doc-only); `paint_vec3`; `paint_slider` (gained `atlas` param); `paint_number_stepper`; `paint_field`; `paint_tree_widget`/`paint_tree_item` (gained `is_last_at_level` param); new `paint_loading_border`, `paint_tree_guides`, `paint_control`; 9 new additive tests under a new `//#region 🔖FidelityFixes` subregion in `paint`'s own test module.
- `ui/wgpu/rs/lib.rs`, `engine` region's test submodule (nominally owned by `w0-engine-facade`, flagged per exception): added one new additive test, `golden_number_stepper` (`uniform:true`, full `assert_equivalent`), right after the untouched pre-existing `golden_number_stepper_known_gap`.

## Build/test verification

`cargo check -p ui_wgpu --features engine`: clean, 0 errors (a transient error from a concurrent `w1d-events-overlay` edit to `events` was observed mid-task and cleared on its own — unrelated to `paint`).

`cargo test -p ui_wgpu --features engine`: **110 passed, 1 failed**.

Golden-harness table (`engine::tests::golden_*`, all `ok` except as noted):
stack, text, button, separator, input, select, toggle, vec3, key_value, slider, number_stepper_known_gap (pre-existing, untouched), **number_stepper (new, full equivalence)**, ring, icon_select, tree (unaffected — its fixture leaves description/actions/control/loading `None` and passes `icons: None`), field_known_gap (pre-existing), section_known_gap (pre-existing), image_known_gap, component_scene_known_gap, external_slot_known_gap — all `ok`.

`paint::tests`: 13/13 `ok` (4 pre-existing + 9 new: `painting_a_disabled_button_dims_its_border_alpha`, `painting_a_loading_section_emits_a_loading_border_instance`, `painting_a_loading_stack_emits_a_loading_border_instance`, `painting_a_loading_tree_emits_a_loading_border_instance`, `painting_a_mixed_vec3_shows_fewer_glyphs_than_a_zeroed_one`, `painting_a_uniform_number_stepper_nests_a_border_around_its_center_value`, `painting_a_mixed_number_stepper_shows_the_mixed_placeholder_in_muted_color`, `painting_a_slider_with_a_unit_emits_extra_glyphs_for_the_readout`, `painting_a_field_with_description_required_and_error_emits_extra_glyphs`, `painting_a_tree_item_with_description_emits_more_than_a_bare_item`).

**The one failure**: `component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json` — a golden-JSON fixture mismatch for `board2d`/map-style fields on `UiComponentSceneNode`, entirely inside `component` (out of my scope, untouched by me). Looks like fixture drift from concurrent work (an in-flight `GENERALIZE-RASTER-SURFACE-KIND-TO-PAINT2D` ticket touches related surfaces). Flagging for whoever owns `component`; did not investigate further.

## Still gapped (honest accounting)

- Select's open popup — no data source exists yet; needs `reconcile`'s Select-child expansion (owned by `w1a-reconcile`, not landed as of this task's finish) plus somewhere to persist open/closed state.
- Tree row hover/drag-indicator — needs per-row `NodeId`/`NodeFlags`, which needs `reconcile`'s Tree-item expansion into real retained children (same root cause as Select's gap, not landed).
- Image/ComponentScene/ExternalSlot — intentionally untouched.
- Tree item action labels — only icons ported for always-visible actions (labels are hover-revealed in `widgets`, which needs hover state that doesn't exist yet).
