# Shared v9 (wave 2)

Nothing compiled (no cargo, no rustc). "- ready for rebuild: shared v9" is in `🗑️generated/integrate/latest.md`.

## 1. Select all (ephemeral local lane, view state)
- Framework write API: `ShellKeyOutcome::Select { domain_id, targets: Vec<ShellSelectTarget{granularity, id}>, merge }` plus `ShellKeyOutcome::select(domain, targets)` (replace) in `🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/📱️shell-hooks` (rs, ts, proto twins, package re-export, wire test). The wgpu shell (`⌨️keyboard`) dispatches it as the framework verb `interactionSelect` (`method: pick`): a selection write on the interaction lane, never a document edit, never an undo step.
- Norm side (`host-routes`): `publish_selectable_scene(domain, scene)` (called by `render_viewport_panel`), `publish_visible_rows(domain, tree, filter)` (called by `render_inspector_value` when the outliner has a domain), `select_all_targets`, `select_all_outcome(scope)`. `NormShellHooks::handle_key` answers `HostRoute::SelectAll` with it, so Ctrl+A in `norm-<variant>-viewport` selects every scene entity of its domain, narrowed to the rows the outliner filter admits; no domain or empty scene -> `Ignored`.
- Tests (entity-editing `select_all`): chord -> outcome -> addresses (scene order, granularity per kind), filter narrows and clearing widens, empty/no-domain selects nothing; feature line added to the entity-editing `.feature`.
- Not done: the touch-menu "Select all" entry still raises `norm.shortcut.select-all` (`render_touch_menu` has no domain and has no non-test caller yet); it should render `interactionSelect` with `select_all_targets(domain)`.

## 2. Tree-item drag and drop (wgpu shell)
Finding: the shell already had the tree drag session (`TreeDragState`, hover target `drop_target_id`, Before/After/Inside insertion marker in `render_tree_drag_overlay_node`), only the commit onto a tree row was missing; no `NodeFlags::DROP_TARGET` sync was needed.
- `reconcile::tree_item_drop_action(document, key)` finds the row's `Trigger::Drop` binding in the published document (bounded by node count).
- `retained_ui_engine::{tree_item_drop_action, apply_tree_row_drop}`; `apply_drop_committed` and the new function share `queue_drop_action` (payload `application/x-semio-*` JSON merged into the action args, `windowId` stamped).
- `finish_tree_drag` now ends with the tree-row drop (hit window via `retained_hit_window`), so the outliner row drop emits `moveEntity{path, over, id}` -> `MoveItem` (the outliner already bound it). Keyboard, touch and screen reader keep the move up/down items.
- Test: `a_tree_row_declares_its_drop_action_through_its_drop_binding` (document records with and without a Drop binding). The synthetic pointer sequence through the whole shell was not written (needs the shell fixtures and a build); the existing outliner/entity-editing tests cover the action -> `MoveItem` half.

## 3. Triage of `→ shared` lines
24 open lines in `requests.md` now carry a status and a "shared v9" reason. Done: 1017 (informational), 1137 (`global_reading`/`part_reading` exist and en1994 uses them). Obsolete: 971 (`EntityRef::new` takes any path), 1018, 1103 (en1995 has `mut-panel-layers`), 1304. Deferred (need a build or an oracle run, no code missing): 416, 547, 744, 625, 775, 831, 1376. Owner decisions with a recommendation: 500, 585, 770, 791, 938 (do not mint umbrellas for library crates, relax check 13), 967, 1019, 1298, 1527. The two framework lines (select all, tree drops) are `[done]`; the v9 meta line is closed.

## 4. Risks of shared-v8, verified by reading
- `bind_on` on `TreeItemBuilder` with `Trigger::Drop`: ok. `bind_on<B: HasBase>` calls `try_on_with`, `TreeItemBuilder: HasBase`; `Trigger` comes from the prelude; `drag_data` takes `UiFixedMap<UiText>`.
- `UiFixedMap::try_push(key: UiText, value: V) -> Result<(), (UiText, V)>`: matches the outliner call (`map_err`), keys are pushed in one call so the sorted-order rule holds.
- `DslRecord` of `MoveEntity { Option<u32>, Option<String>, Option<f64> }`: precedents with the same field types exist (`PasteEntities.at`, `ViewportAct.text/number`, `EvaluateUnits.units_json`).
- `Self::command_from_action` in the macro: the macro is invoked inside the `impl` (all editors, line `norm_command_from_action!` indented in the impl), the signature matches, the borrow of `args` ends before the recursive call.
- `static NO_VIEW: ViewOptions { BTreeMap::new() }`: `BTreeMap::new` is const, the value types are plain data (Sync).
- `HOST_CONTEXT` global: the new tests use distinct domains and the pure functions, not the global.
- Not verifiable by reading: vdi3805 goldens and oracle JSON schemas (need regen after a build); the en1990 test `the_typed_view_carries...` compares scenes of group 0 and group 1 and needs the traffic lanes to differ (artifact scope, first build decides).
- No certain errors found, no fixes needed in this list.
- New residual risks in v9 code: `TreeItemProps`/`ActionBinding` field visibility in the new reconcile test (constructed from another crate); `Option::is_none_or` (fine on the pinned nightly); `ShellSelectTarget` re-export list in `📦️packages/🦀️rust/🦀️.rs` (two lines edited); `decode_scoped_identity` error type `Debug` in the test.
