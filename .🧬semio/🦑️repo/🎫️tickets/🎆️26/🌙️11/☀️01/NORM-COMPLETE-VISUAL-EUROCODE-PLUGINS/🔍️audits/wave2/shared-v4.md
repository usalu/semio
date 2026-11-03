# Shared v4 (wave 2)

Nothing compiled or run (no cargo). "ready for rebuild: shared v4" appended to `🗑️generated/integrate/latest.md` and the ten `build-en19xx/latest.md`. Paths below are relative to `✏️s/🔌️plugins/📕️norm` unless marked framework.

## Done
- Keyboard: `ShortcutAction::{NextCombination, PreviousCombination}` bound in the Viewport scope to `.` and `,`; `{NextGoverningCheck, PreviousGoverningCheck}` Global `Alt+G` / `Alt+Shift+G` (not `[` `]`: those need AltGr on German layouts). Routed unchanged through `dispatch_shortcut` and the shell key hook as `norm.shortcut.<kebab>`; the viewport window declares the actions (EN/DE labels) from the same table. Helpers `focus_order::{step_index, step_governing}` (wrapping; governing walks utilisation descending, not-applicable last) and `lane_runtime::step_combination_event(count, item, forward) -> LaneEvent::ViewItem`. Tests: accessibility `combination_and_governing_check_stepping_are_bound_and_wrap`, live-lanes `the_combination_stepping_shortcuts_become_view_item_events_that_wrap`. The host action handler that reads the combination count is still the live editor's (declared, not executed by a norm handler: same state as `next-check`).
- Remove-row remedy: `RemedyBound::Remove` + `Remedy::remove(target, action)` (compliance check-results; proto, graphql, ts, json twins), `edit_rules::{RemedyEdit, remedy_resolution}`; `handle_apply_remedy_with_option` raises `NormEdit::RemoveItem` (id-removal rules apply), `apply_remedy_edit` removes from the tree, `remedy_edit`/`previewRemedy` refuse a removal (`norm.remedy-removes-row`), the chip shows the action text with symbol. Test `a_remove_remedy_deletes_the_row_its_target_selects_and_is_no_value_write`. en1990 can now emit `Remedy::remove(subject, "Reset to NA")` on `factorOverrides[id=x]`.
- Angular zero direction: `HandleKind::angular_zero_direction` (+X projected perpendicular to the axis, +Z for an X axis), `angular_s` (signed right-handed angle), `angular_point` (inverse) in `🎬️structural-scene/🧬️schema/🕹️handles`, documented there; `lane-overlays::drag_dimension` now draws the arc end point instead of a line along the axis; law helper `test_support::assert_angular_handle_laws`; unit tests with hand-derived values and a round trip. en1995 can drop its private `angle_of`.
- Migration wording: `typescript-package-commands` lines 36 and 78 and `retained-disposition-oracle` line 60 reworded (no test matched the text). `admission == "migrated"` is fixture data, untouched.
- EN 1997 gate texts: 14 `RULES` in `⚖️compliance/🗣️reasons` (flow net, base width, zero net pressure, prop depth, Kranz, observation wells, t2>t1, gamma_sat, exit gradient, net cone resistance, confidence slope, profile pointer/series, creep segments), found by scanning every `gate(...)`/`Err("..")` literal of the EN 1997 catalogue against the table; test `the_en1997_gate_texts_have_a_rule_and_a_german_text`.
- Stored LayoutPrefs in the shell (framework, domain-neutral): `PluginShellHooks::prefs_key`, `ShellHookDriver::{prefs_to_load, supply_prefs, prefs_text}` (request carries the text), wgpu `plan_dock_windows` reads `stored_field_get(key)` once per opened app; `NormShellHooks::prefs_key` = `norm.visual.layout`. Test `the_stored_layout_preferences_are_read_once_per_app_and_ride_on_every_request`. Writing prefs back needs a layout-edit UI (not built).
- Small: en1998 field-meta doc comment (`site.sApR`).

## Closed without code (see requests.md)
Obsolete: the three compliance size requests (facade is 3186 active lines after the v3 split; the `SceneKit` kit-crate cut of CRATE-BUDGET C concerns artifact scene crates, not shared, and shared crates are within budget), stray ndp table. Verified already done: `view.activeCombination` routing, `evaluate` removal (`Migrated` classified framework-owned, used by every composition), `IdRemoval` with parent selectors, `Rows::row` takes any collection tag, `dyn_enum_close!` through the core re-export, design-case mounts kept.

## Still open (with reason)
- Unit-picker persistence: viewport-config crate is not green (no build or witness report).
- `Velocity` quantity kind: needs vdi3805 `VdiQuantityKind` and twins in one pass.
- FE torsion constants, oracle-harness adapters, `evaluate_case_file`, `scene_law_rows`: numerical and third-party work not possible without running code.
- A11y renderer wiring of `check_row_node/handle_node/diagram_nodes`, help overlay, NDP annex section; Outcome-level NDP flag.

## Risks
Unverified by rustc: exhaustive `RemedyBound` matches outside the files I changed (only `matches!` uses were found by grep), borrow shape in the wgpu `plan_dock_windows` (`self.docs.session` vs `self.dock`), `remedy.action.resolve` return type in the chip, `remove_value_at_path`/`PathSegment` visibility via `use super::*` in edit-rules, `TOLERANCE` in scope for `HandleKind` impl (already used in that file).

## Handover
Shared v5 owns the shared crates from here. No background edits were running: the only background tasks were two read-only greps (the second was stopped). No further shared edits after this note.
