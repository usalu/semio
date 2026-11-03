# W3-MULTI-EDIT report (2026-11-03)

Static work only (no cargo, no repo run). freeze.txt absent at every edit batch. Shared code in `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`; per artifact only the ten `✏️editor/🦀️.rs` command tables.

## 1. Multi-entity delete (`deleteEntities`)
- Command: `LiveCommand::DeleteEntities { collection, ids }`, tool id `deleteEntities` appended to `NORM_LIVE_TOOL_IDS` (index 10) and `NORM_EDITOR_TOOL_IDS`; publication contracts (artifact lane) in both lists; macro arm in `norm_command_from_action!` and payload module `delete_entities` (`ids_json`) in `norm_live_commands!`; dispositions fixture row + `routesPerApp` 16 / `identities` 160. All ten editors: import, `app_commands!` row, `tools:` list, action definition (EN/DE label), description (EN/DE).
- Reducer: new `entity-editing/🗑️delete` (`delete_edits`): one `NormEdit::RemoveItem` per distinct row, last position first (positions stay valid while the per-kind remove mutations apply in order); resolved by `live_entity_edit` -> `handle_edits` into ONE `Emit` = one undo step, all or nothing (a kind invariant refusing one removal refuses the batch).
- Refusals (`DeleteRefusal`): nothing selected, unknown list, unknown id -> fault codes `norm.entity-delete-empty` / `norm.entity-delete-unknown` with EN+DE notices in `norm_fault_notices`; `message(german)` gives the localized reason.
- References: identical to single `removeItem` (per-kind remove mutation decides; dangling references and placement rows are not touched separately). No extra cascade invented.
- Delete key: `ShortcutAction::Delete`, chord `Delete` (Global), route `Entity("deleteEntities")` via `host_entity_call` (selection collection+ids), declared automatically as window action `norm.shortcut.delete` (EN "Delete selected entities" / DE "Ausgewählte Bauteile löschen"). Touch menu: new "Delete"/"Löschen" entry (enabled with a selection; 5 entries). Context menu: Delete row now `deleteEntities` for 1..n rows (shortcut label "Delete"), no longer disabled for several rows.

## 2. Bulk edit from the menu
- Row `bulk-edit` (2+ rows): label "Bulk edit (n selected, k fields, m mixed)" / DE, enabled when the rows share an editable field, else disabled with a localized reason. It raises the declared window action `norm.shortcut.focus-properties` (the Alt+3 "Focus properties" action). The Properties window already renders the bulk inspector (`render_selection_inspector`: shared value, "Mixed"/"Gemischt" for disagreeing fields, one `setFields` bundle = one undo step) for exactly the published multi-selection.
- `is_focus_shortcut` + macro arm: focus actions are accepted by the app as a view action that changes no lane (before: they ended in `norm.unhandled-action`).

## Tests (test-first, written with the code)
- Unit (`entity-editing/🧪️tests`): `batch_delete::*` (descending order + dedupe, one emit / all-or-nothing, refusals with notices and EN/DE, Delete chord + `host_entity_call` + declared action), updated touch menu (5 entries, routes), host routes, context menu (delete = `DeleteEntities` for 1 and 2 rows, bulk-edit row focus action). `command-args` tests: round trip + parse of `deleteEntities`. Shell-hooks test: `Delete` key -> `norm.shortcut.delete`.
- Stamped law (`norm_live_acceptance!`, all ten editors): `every_registered_entity_kind_deletes_a_multi_selection_as_one_atomic_batch_that_undoes_and_redoes` — per kind: duplicate to get two rows, `DeleteEntities` emits exactly 2 mutations in one emission, applying them removes exactly the selected rows, the inverses (computed per step, applied in reverse) restore the original document value, redo deletes the same rows again, an unknown id refuses the whole batch. The context-menu law now asserts `deleteEntities` and runs the menu's delete for real.

## Open / compile-unverified
- Window focus itself is the shell's job: no framework verb focuses a window by kind, so the menu row relies on the declared `norm.shortcut.focus-properties` action (same path as Alt+3); the app accepts it, the Properties window shows the bulk inspector anyway for the multi-selection.
- Unverified symbols: `protocol::ColdMutation::{diff,inverse}` and `protocol::apply_diff_cold(raised.diff(), ..)` from inside the stamped test module (copied from `mutation_bridge`), `Mut` alias use, `DeleteRefusal` re-export through `entity_editing`, `looks_english` not used. A failing kind is named in every panic message.

"- ready for rebuild: multi-edit" appended to all ten `build-en19xx/latest.md`.
