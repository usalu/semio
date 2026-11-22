# Wave 4 item 2: text-selection context menu (wgpu twin of React)

Path shorthand: `UI` = `🧰️framework/🔨️modules/🖱️ui/`; `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`; `SH` = `S🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/`; `INT` = `S🗣️Interpreter/🎯️targets/🧊️wgpu/`.

## What React does
`TextSelectionContextMenuHost` (`UI🧱️elements/🖱️ContextMenu/🟦️.tsx`) listens to `contextmenu` in the capture phase. When the pointer is on a non-collapsed DOM text selection it suppresses the native menu, reads the selection at open time (an open menu steals focus and collapses it) and opens `buildTextSelectionContextMenuItems` (`📝️text-selection/🟦️.ts`): Cut (editable only, disabled without selection, `mod+x`), Copy (disabled without selection, `mod+c`), Paste (editable only, `mod+v`), separator, Select all (`mod+a`). Labels `ui.contextMenu.cut/copy/paste/selectAll`. Verbs run Clipboard API + input/execCommand.

## Changes
- Schema (new, shared): `UI🧬️schema/📝️text-selection-menu/🔣️.json` holds rows (id, verb, icon, chord, editableOnly, needsSelection, labelKey, en/de label), action controller `framework`, prefix `textSelection.`.
- React: `buildTextSelectionContextMenuItems` now builds from that JSON (same ids, order, flags as before).
- TS law: `UI🧱️elements/🖱️ContextMenu/🧪️tests/📝️text-selection/🟦️.ts` (3 tests, ran with `repo test-file`: 3 pass): row order, readable gating, JSON en/de equal `configurationEn/De` contextMenu labels.
- Rust builder: `text_selection_menu_items(origin, editable, has_selection, is_de, chord)` and `text_selection_verb_of` in `SH🕹️input/🔖️identity-and-types/🦀️.rs` (reads the same JSON via include_str). Rows are `framework` actions carrying the press point (`x`,`y`) so keyboard and ordinal activation (menu already closed) still find the surface.
- Open: `open_context_menu` (`SH🖐️pointer/⌨️keyboard-and-palette/🔁️deferred-menus/🦀️.rs`) first asks `text_selection_at(x,y)`; a non-empty selection in the text editor under the point opens this menu instead of the guest menu (React's capture-phase host wins the same way). Import added in `🖐️shell-pointer/🦀️.rs`.
- Claim: `claim_text_selection_action` / `text_selection_at` in `SH🕹️input/⌨️keyboard-and-palette/🦀️.rs`; called before the existing text-editor claim in `SH🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs` (mouse) and in the Activate arm of `SH🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs` (keyboard, ordinals).
- Engine intents (no chord synthesis): `INT✍️text-and-table-focus/🦀️.rs` gained `TextSelectionVerb`, `text_editor_selection_at`, `queue_text_selection_verb_at`, `drain_text_selection_verbs`. Copy/Cut share one body with the Ctrl+C/X chords (`copy_or_cut_text_editor`, extracted from `apply_focused_text_editor_clipboard_key`); Select all uses `text_editor_set_selection_into`; Paste uses the native clipboard job lane (`queue_text_editor_clipboard_read`), the testkit mock, or on wasm `clipboard_read_text` then `apply_text_editor_text_at` on the next step. The queue is drained at the top of `drive_scene_interaction_step` (`INT🖱️scene-interaction/🦀️.rs`, which also re-exports the three items).

## Laws added
`SH../🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs` (mounted in the `🍰️shell-tests-input-3` bin): `the_text_selection_menu_offers_the_shared_schemas_rows`, `the_text_selection_menu_gates_rows_by_editability_and_selection`, `a_text_selection_row_names_its_verb_and_carries_the_press_point`.

## Lane E: compile and run (nothing compiled here)
- `cargo check --tests`: `semio-renderer-wgpu-ui-engine-focus`, `semio-renderer-wgpu-ui-engine`, `semio-renderer-wgpu-shell-input`, `semio-renderer-wgpu-shell-pointer`, and the bin `semio-renderer-wgpu-shell-tests-input-3`; wasm32 check of the focus crate (cfg'd `PENDING_TEXT_SELECTION_PASTES`, `paste_into_text_editor`).
- Test: the three laws above (`context_menu` filter in shell-tests-input-3); existing `🔬️wgpu-ui-command-wiring` laws in `semio-renderer-wgpu-ui-engine-tests` must stay green (chord body was refactored).
- TS: `repo test-file` on the new test (done) and the existing `🧪️owned-locale-detector-retirement/🧲️contextmenu-and-more` ContextMenu tests.
- Compile risks: `engine.scene_at(..).node/.kind` field names (copied from `surface_context_menu_target`), `DslValue::as_f64` returning `Option<f64>`, `retained_context_menu_window` return type destructure.

## Known gaps
- Only text-editor scenes are covered. Retained single-line `Input` fields keep their key-chord clipboard but have no selection-menu entry (the retained engine exposes no selection-at-point reader); read-only text has no wgpu "readable selection" model, so `editable` is always true.
- Menu Paste on wasm applies on the next scene step after the browser grants the clipboard read; not exercised in a browser.
- Cut/Paste have no history-typing-run interplay beyond what `text_editor_replace_selection_into` already does.
