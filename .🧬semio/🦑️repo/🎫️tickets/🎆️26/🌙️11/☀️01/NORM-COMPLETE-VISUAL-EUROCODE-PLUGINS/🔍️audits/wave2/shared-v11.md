# Shared v11 (wave 2)

Nothing compiled (no cargo, no rustc). "- ready for rebuild: shared v11" is in `🗑️generated/integrate/latest.md`. Paths are under `✏️s/🔌️plugins/📕️norm/` unless noted; `APP` = `📇️registry/🧬️contract/🖥️app-surface`.

## 1. Check <-> entity link, wired
- Check row click (router): the row's `setSelectedCheckIndex` action decodes to the editor command; `dispatch_norm_command` now fills `Emit.effects` from the new `NormRetainedEditor::check_focus_effects(command, doc)` (default none). Each of the ten `en19xx` editors implements it with one `match` arm calling `live_check_focus_effects::<Self>(doc, payload.index, checks::INTERACTION_DOMAIN)` (`APP/🫧️live-lanes/🎛️editor-wiring`): report + scene -> `focus_of_check` -> `focus_effect(domain, &focus)` (`APP/🔗️check-entity-link`), a `ReplayShellCommand` of the framework `interactionSelect` (replace, pick) on the owner entity address. Framing is the existing lane: `ViewCheck` is set by the same click and the viewport frames the selection when a check is set. A document-level check or an undrawn owner emits no selection. `focus_outcome` (ShellKeyOutcome) is removed in favour of `focus_effect`.
- Viewport pick -> checks window: `NormSelection` gets `picked` (base ids of the picked entities, not the check subject, so keyboard stepping never narrows). `ChecksViewState.owners` (not part of the wire form) narrows `plan_checks` to those owners, and falls back to the whole list when no check belongs to a pick (an outliner row of a table without checks). `LiveView::render_checks(windows, locale, controller, domain)` builds the state with `checks_view` (selected row = governing check of the pick, now compared by base id, plus pending progress); the ten editors' `render_body` call it instead of `checks::render` (which stays for its unit tests).
- Tests: `APP/🔗️check-entity-link/🧪️tests` (effect shape, global check, both directions), `APP/🪄️selection-mirror/🧪️tests/🔬️unit` (pick narrows, governing row, stepping does not narrow, no-match keeps the list), and in the `norm_live_acceptance!` macro (stamped into all ten editors): `a_click_on_a_check_row_routes_to_the_selection_of_its_owner_entity` (action `setSelectedCheckIndex` + index -> `command_from_action` -> `check_focus_effects` -> `interactionSelect` for the default document's first drawn-owner check).

## 2. `interaction_topology`
- `APP/🧰️entity-editing/🧭️interaction-topology` (mounted in the inspection crate): `domain_topology(tree, scene, report)` lists, once each, every scene entity address (granularity of its kind: the nodes Select all picks), every outliner row address (granularity = collection) and every check address (`check`); `norm_interaction_topology(domain, ..)`, `check_address`. `#n` part ids need no node (the lowering dispatches `interactionId` = the base address).
- `live_interaction_topology::<A: NormLiveEditor>(doc, domain)` in the editor wiring; all ten editors add `fn interaction_topology` to their `ArtifactEditor` impl (`checks::INTERACTION_DOMAIN`). `viewport_interaction_definition` now declares `HierarchyProvider::Topology` (was `Flat`), so the framework `selectAll` verb and the stale-selection pruning work from the topology.
- Touch menu: `render_touch_menu(entries, controller, locale)` (domain argument removed) raises the framework `selectAll` verb with no payload; the 512-byte fallback to `norm.shortcut.select-all`, `select_all_action` and `INTERACTION_SELECT_VERB` are deleted. Tests: `the_touch_menu_select_all_raises_the_payload_free_select_all_verb_for_any_scene_size` (3 vs 5000 entities, identical menu), `a_large_scene_is_selected_whole_where_the_action_argument_would_have_overflowed` (2000 members) and the per-editor topology law in the macro.
- Behaviour note: the framework `selectAll` selects the nodes of the active granularity (first declared kind) in every domain; the chord (`select_all_outcome`, in-process `ShellKeyOutcome::Select`, no 512-byte limit) still selects every kind and respects the outliner filter, because `ShellKeyOutcome::Action` only reaches window-owned app actions (`dispatch_app_keybinding`), not framework verbs. Moving the chord onto the verb needs a framework `ShellKeyOutcome` variant for a framework verb (not changed here).
- Found while doing this: the panels crate (`🧾️panels/📦️packages/🦀️rust/Cargo.toml`) lists `serde_json` only under dev-dependencies, while v10's touch-menu used it in non-test code; that use is gone with the fallback. The link module uses `semio_framework_pack_json` instead.

## 3. Owner-decision lines (`📌️important/requests.md`, marked "shared v11")
- 500 slope circle: `[done]` (circleCentreX/Z/Radius exist, handles and oracle use them).
- 938 umbrellas: `[done]` (`umbrella-fix.md`: the contract umbrella lists the 26 crates; scanner check 13 must read it).
- 1298 traffic groups: `[done]` (typed `trafficGroups` since en1990 v8; Table A2.4 values stay unverified in the register).
- 585 `Outcome.flags`: now `[open]` (xi static and dynamic rows are in the registry; next shared wave).
- 1527 en1994 leaves: now `[open]` for `beams[].steel.rM` only (`reinforcementProduct` done for columns and beams in en1994 v10).
- Unchanged decisions: 770 (green build + goldens), 791 (golden re-compare), 967 (en1996 generator profile), 1019 (`Remedy.unit_scale`, right after the first green build).

## Files touched in artifact trees (mechanical, uniform, ten times)
`🗿️artifacts/<en1990..en1999>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: `fn interaction_topology` in the `ArtifactEditor` impl, `fn check_focus_effects` in the `NormRetainedEditor` impl, and the `checks::BODY_CHECKS` arm of `render_body` now `live.render_checks(..)`.

## Unverified / risks
- Nothing compiled. Expect: the `where A::Snapshot: ToValue` bound at the ten call sites, `Emit.effects` field name/type, `DslValue::object(vec![..])`, `semio_framework::SELECT_ALL_ACTION_ID` re-export.
- The retained publication contract of `setSelectedCheckIndex` lists only the `WindowConfig` lane; if the job refuses an emitted `ReplayShellCommand` effect, add the effect lane there.
- A pick of an annotation/handle id is not a topology node and is pruned on revalidation; the global badge is deliberately never selected.
- Narrowing has no off switch beyond clearing the selection (Escape / empty click); `setChecksView` is bound by no artifact today.
