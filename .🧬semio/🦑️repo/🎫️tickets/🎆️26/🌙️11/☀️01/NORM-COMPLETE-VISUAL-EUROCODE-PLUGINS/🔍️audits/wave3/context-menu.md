# W3-CONTEXT-MENU report (2026-11-03)

Static work only (no cargo, no repo run). freeze.txt absent at every edit batch.

## Finding: the premise was half wrong
The wgpu shell already has the secondary-click trigger: `button == 2` in `staging/🍱️view-chrome/🖱️pointer-input` calls `open_context_menu(x, y, hit)`, which asks the plugin (`ArtifactEditor::context_menu`, `ContextMenuRequest` with `surface.hits`/`selection`) and falls back to the window actions when the plugin answers nothing. The norm editors never overrode `context_menu`, so right-click only ever showed the generic window-action menu. What really lacked:
1. a keyboard trigger (context-menu key, Shift+F10): `KeyAction` had no such key, `key_action_from_dispatch("ContextMenu")` returned `None`, the winit host mapped only F11;
2. tree rows (outliner, checks list) were not reported as hits;
3. the norm menu content.

## Framework (domain-neutral)
- `KeyAction::ContextMenu` (input crate); `key_action_from_dispatch("ContextMenu")`; winit host maps `NamedKey::ContextMenu` and F10; the three exhaustive matches updated (`keybinding_capture_key_token`, `key_matches_chord`, `ui_event_from_key_action`, which now hands a retained scene `KeyDown "ContextMenu"`).
- `is_context_menu_chord(action, modifiers)` (context-menu key with any modifiers, Shift+F10 alone) and `PointerCtx::keyboard_context_menu_anchor` (centre of the keyboard-focused control, else the active window body, else the dock canvas), wired in `handle_keyboard_async` before the shell chord gate; skipped while a text field is edited or a dock drag runs; opens the same `open_context_menu` as the pointer, so the Escape/arrow/Enter keyboard navigation of the open shell menu applies.
- `open_context_menu` reports the row under the pointer or the focus as hit `{domain: "tree" (CONTEXT_MENU_TREE_HIT_DOMAIN), id: tree item id}`.
- Tests: `the_context_menu_key_and_shift_f10_are_the_keyboard_secondary_click_and_nothing_else_is` (wgpu-shell-input), `normalized_host_maps_the_context_menu_key_and_f10_for_the_keyboard_secondary_click` (wgpu-winit-app-p3c).
- The touch long-press is untouched (`map_gesture`, `touch_menu`; its tests are unchanged).

## Norm (shared, `app-surface`)
- New `🧰️entity-editing/🍽️context-menu/🦀️.rs` (mounted in the inspection crate beside host-routes): pure `context_menu_items(tree, probe, host_context, focus, german)`. Rows: focus check, duplicate, copy, paste, delete, bulk-edit hint (only for 2+ rows), select all; shortcuts shown (mod+d/c/v/a). Duplicate/copy/paste dispatch `host_entity_call`, i.e. exactly the tools of the chords and the touch menu; select all raises the framework `selectAll` verb with the norm granularities; delete is `removeItem {path, index}` for exactly one row (several rows: disabled with the reason); focus check is `setSelectedCheckIndex` (governing check of an entity via the new shared `governing_check_index`, or the checks row's own index). A hit inside the selection keeps the selection, outside it acts on the hit row alone. Disabled rows stay in place with a localized reason (aria-disabled/described by the framework). EN and DE for every row; the language comes from the view state, no default.
- Hit forms understood: 3D pick by plain or scoped id (`member|b3`), outliner row `norm-outliner.<path>.select`, checks row `norm-checks-check-<n>[.detail]`.
- Outside the viewport with nothing hit and nothing selected the app answers no rows, so the shell keeps offering the window's own actions.
- Wiring: `live_context_menu::<A>` (editor-wiring) and macro `norm_context_menu!()` (placed in each `impl ArtifactEditor` after `norm_command_from_action!`) in all ten editors en1990..en1999.
- Tests: 7 unit tests (`entity-editing/tests`, module `context_menu`: outliner row, 3D pick plain/scoped, selection widening, checks rows, empty/disabled reasons, EN/DE, parity with the touch menu tools) and the stamped editor-level law `a_secondary_click_on_an_entity_row_opens_a_menu_whose_rows_run_the_real_editor_commands_in_both_languages` in `norm_live_acceptance!` (every entity kind of every editor).

## Open / honest limits
- Bulk edit itself stays in the Properties window (no argument-free action opens it); the menu shows a disabled hint row with the count. Multi-row delete does not exist (no `deleteEntities` command; adding one means a schema/command change in all ten editors).
- A secondary press does not select the row first; the menu acts on the hit row explicitly instead, which makes it correct without selection.
- The host context (selection, clip) is the process-global one of host-routes; the stamped law reads it, so it assumes no concurrent test publishes a selection of the same collection.
- Compile-unverified: `crate::protocol::wire::decode_scoped_identity` in the inspection crate, `tree_hit_item_id` visibility in the deferred-menus file, `is_none_or`, `Rect` fields in the anchor, the `pub use keyboard_handling::is_context_menu_chord` path used by the wgpu-shell-input test. A failure names the item.

"- ready for rebuild: context menu" appended to all ten `build-en19xx/latest.md`.
