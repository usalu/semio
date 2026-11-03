# Shared v8 (wave 2)

Nothing compiled (no cargo, no rustc). Everything below is by reading; "ready for rebuild: shared v8" is in integrate/latest.md.

## 1. EDITOR-UX host and UI (all in app-surface)
| Piece | Where | Test (input -> events -> state) |
|---|---|---|
| Host key handler | `NormShellHooks::handle_key` routes through `shortcut_route`: undo/redo -> `Ignored` (the shell's reserved edit chords run the store history), select all -> `Ignored` (no plugin write API for the interaction selection, request), view steps and entity chords -> `norm.shortcut.*` actions. The command bridge (`norm_command_from_action!`) turns `norm.shortcut.copy/duplicate/paste` into `copyEntities/duplicateEntities/pasteEntities` over the `HostContext` (`publish_host_selection` is called by the inspector render, `remember_clip` by the copy command) | shell-hooks tests (chords), entity-editing tests `the_key_handler_turns_copy_paste_and_duplicate_chords...`, `..._refused_with_a_reason`, `undo_and_redo_chords_are_left...` |
| Long-press menu | `render_touch_menu` (tree group, enabled entries raise the shortcut action id, disabled ones say "Not available"/"Nicht verfügbar"), `touch_menu_nodes` + `publish_touch_menu_access` (roles `menu`/`menuitem`, set positions, window role `menu`) | `the_touch_menu_renders_actions...` |
| Multi-select properties | `render_bulk_inspector`, `render_selection_inspector` (wired into `render_inspector_value` for >1 selected rows of one collection); `FieldView.bulk` makes every control raise `setFields{collection, ids, field}`; `bulk_writes_arg` expands it to one write per row (one undo step); mixed shows "Mixed"/"Gemischt" and an empty control | properties test `a_multi_selection_shows...`, entity-editing test `a_bulk_set_writes...` |
| Outliner reorder | `OutlinerState.reorderable` (+ `with_reorderable(&NormEditRules)`), draggable rows (`ROW_DRAG_MIME`, payload `{id}`), per-row `Trigger::Drop` -> `moveEntity{path, over}`, move up/down items (first has no up, last no down); new tool `moveEntity` = `LiveCommand::MoveEntity{path, id, MoveTarget::{To, Over, Step}}` -> one `NormEdit::MoveItem` through the family `MoveItemRule` | outliner tests (affordances, EN/DE), command-args tests (parse), entity-editing test `moving_a_row_by_step_position_or_drop...` (tree order) |
`moveEntity` is wired through the macro, `NORM_LIVE_TOOL_IDS` (append), publication contracts, `NORM_EDITOR_TOOL_IDS`, the dispositions fixture (15 routes, 150 identities) and the ten editors (import, command row, tools list, action definition, description). en1992 outliner uses `with_reorderable(&EDIT_RULES)`.
Moved down into the panels crate because the hook lives there: `entity-editing/{host-routes, bulk-edit}` (`Shared` -> `SharedValue`). a11y: `ROLES` +menu/menuitem (test list updated).

## 2. Compile review: certain errors fixed
- `edit-rules`: `move_item_edit` sat inside `impl NormEditRules` (no `self`) while its test and docs call it as a free function; moved out, `move_item_over_edit` / `move_item_step_edit` added.
- `Validate for Quantise`: non-exhaustive after `Decades`; arm added.
- `CARGO_MANIFEST_DIR` of the units oracle test pointed one level too high after the core split (`../../../`).
- `RemedyBound` matches without wildcard (`remedy_resolution`, en1995 `flipped`); `LiveCommand` match in `live_entity_edit` exhaustive.
- Crate layering: the shell hook (panels) must not reach `entity_editing` (live-editor); resolved by moving host routes down.
- Checked and found consistent: all `NormEditRules` literals carry `move_item`; `Hatch` matches (mesh, parts, canvas) exhaustive; `Part.ink` literals; `CaseForm` matches (23 variants) in `nominal`; NDP table crates (Cargo paths, workspace members, Cargo.lock entries, `c`/`d` arity, const join); brace balance of all 3347 changed `.rs` files (one false positive outside the shared crates).
NOT read line by line (no capacity): viewport3d lowering/colour, lane-stores/lane-runtime internals, case-edits, check-results, incremental-evaluation rows, ndp JSON, the artifact-side changes of other agents.

## 3. Velocity
`QuantityKind::Velocity` (compliance core + scene kind), `Quantity::velocity_m_s`, unit family mapping `F::Velocity`, `format_quantity` `m/s`, overlay unit; twins updated (compliance ts/json/graphql, scene graphql). vdi3805 mirrored minimally (enum, both `From` arms, sqlite, sql CHECK, admission, ts, schema json, fixture) so the workspace stays exhaustive.

## 4. Coordinator extras
RemedyBound::Insert (see requests "Shared v8"), typed `ViewOptions` in `KitDoc` (ten artifacts, en1990 codec removed), view lane threaded into live scenes (editor and viewer closures take `&ViewOptions`), wildcard sweep on own enums in the shared crates (`edit-rules`, `host-routes`, `live_entity_edit`, `ViewOptions` accessors); the remaining `_` arms on `CaseForm` (`forms/ground`, `forms/joints` `build`) and `Geometry`/`SolidShape` are by design.

## Risks (unverified by rustc)
`bind_on` on `TreeItemBuilder` with `Trigger::Drop`; `ui::UiFixedMap::try_push` signature; `DslRecord` derive of `MoveEntity { Option<u32>, Option<String>, Option<f64> }`; `Self::command_from_action` recursion inside the macro; static `HOST_CONTEXT` shared by parallel tests (tests avoid the global); `static NO_VIEW: ViewOptions` with a `BTreeMap`; vdi3805 goldens and the oracle JSON schemas need a regen; the en1990 test `the_typed_view_carries...` compares scenes per traffic group and may need the group to change the drawn entities.
