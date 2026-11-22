# Wave 3: TextEditor touch and a11y, suggestion submenu, plugin recovery panel, Board2d Tab

Source: items 2, 6, 8, 9 of `📓️opus55-reaudit-os-elements.md`. Each item was verified against the tree first. Build status is at the end.

Abbreviations: `EL` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `TW` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`, `SH` = `EL/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging`.

## 1. TextEditor touch selection and aria-busy / aria-invalid

Verified. React (`EL/✏️TextEditor/👆️touch-selection/🟦️.ts`, `🟦️.tsx:48-114`) has no selection handles: a touch long press (500 ms, 10 px slop) or a second tap within 350 ms and 10 px selects the word under the finger (`selectSpanAtScreen`); a drag extends through the ordinary pointer path. wgpu had none of it, and its mouse double click did not select a word either (`click_count` was read only for tables).

- Recognizer: `TW/🧊️renderer/🛎️frame-actions/🎮️interaction-state/👆️text-touch/🦀️.rs`, a port of `createTouchSelectionV1` with the same constants, per-host state, a clock poll (`poll_text_touch`, called from `poll_world_touch`) and a wake deadline merged into `wake_deadline` so an idle app sleeps until the long press is due. Three unit tests.
- Wiring: `interaction-state/🦀️.rs` feeds `SurfaceKind::TextEditor` touch down, up and move in `handle_pointer_button` and `handle_pointer_move`, and cancel in `handle_pointer_cancel`. Coordinates are made window-local with the retained body rect, so slop and selection share a frame.
- Selection: `text_editor_select_word_at` in `EL/🗣️Interpreter/🎯️targets/🧊️wgpu/🖱️scene-interaction/🦀️.rs` (scene under a window-local point, then `text_editor_select_span_into`).
- Mouse double click: the same file now selects the word on a primary `PointerUp` with `click_count >= 2` (React `detail >= 2`).
- Handles: not in React, so none were added.
- a11y: `EL/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/📤️editor-accessibility-frame-stats-and-exports/🦀️.rs` now publishes `busy` (explicit draft pending, React outer `aria-busy`) and `invalid` (explicit draft refusal, React `aria-invalid`) on the textbox node. React's `aria-busy={!editorReady}` has no counterpart because a node is only published once the host is mounted.

## 2. suggestion-submenu ownership, search and rows

Verified OPEN. wgpu had no consumer of the guest's `suggestionMenu` record at all (only the vortex open request existed).

- New `SH/🕹️input/🎣️suggestion-submenu/🦀️.rs` (registered in `TW/🕹️shell-input/🦀️.rs`): `suggestion_submenu_target`, `with_suggestion_submenu`, `set_submenu_children`, `suggestion_menu_owns_window`, `suggestion_rows` (pending row, empty row, `acceptSuggestion` rows with the active one checked), `SuggestionSearch`. Three unit tests.
- `open_context_menu` (`SH/🖐️pointer/⌨️keyboard-and-palette/🔁️deferred-menus/🦀️.rs`) releases a previous search, finds the target, turns the row into a submenu over the pending row and dispatches `openVortexSuggestions {fullId, x, y, windowId, submenu: true}`. `sync_suggestion_submenu` (start of `drain_deferred_actions`, and the settle pump stays armed while a search is open) swaps in the live candidates and dispatches `closeVortexSuggestions {fullId}` once the menu is gone.
- World record source: `WorldSessionState` gains `suggestion_menu_json`, `brush_candidate_index`, `suggestion_source`, filled from `interaction_json` once per change in `sync_world3d_state`.
- Vocab: `host.checkingPlacement`, `host.noPlacement` (en, de).
- Hover preview: each candidate row carries `hoverSuggestion` (dispatched when the pointer or keyboard moves onto it) and its trace key; `ToolRunTraceLayer.focus` (`tool-run-trace/🦀️.rs`) draws that one record alone, opaque and outlined, for the world (`WorldTraceState`) and the board (`board2d_set_suggestion_focus`), and the focus is dropped with the menu.
- Floating popup: an owned, non-submenu record opens a context menu at the record's x,y with the candidate rows (`suggestion_popup_request`), releases with `closeVortexSuggestions` (world) or `closeHandleSuggestions` (board), and is not reopened while the dismissed record still reads open (`suggestion_popup_suppressed`).
- Board2d: `Board2dSurface.suggestion_menu_json` is plumbed from the engine registration (`EngineSurfaceKindDetail::Board2d`), so a board's "suggest" row becomes the same submenu with `nodeLabel · handleLabel` rows, `handleId` args and the hovered-index check.
- Title and select: the popup starts with a labelled separator row (`surfaceContextMenu.placementSuggestions`, en/de), a heading no arrow key reaches, and, like React's `closeOnSelect={false}`, a click or Enter on a row keeps it open (`shell-hit` and `context_menu_handle_key` skip the close for a popup); it closes once the guest's record closes. No known differences remain.

## 3. ChromePanels plugin recovery panel

Verified OPEN: `ui.plugins.recovery.*` had no wgpu key and nothing built the panel (the shared `ui_recovery_panel` has no wgpu caller and carries a third verb React's panel lacks).

- Keys `plugins.recovery.{title,crashed,quarantined,restartApp,disablePlugin}` in `TW/📖️shell-vocab/🧵️task-manager-and-route-vocab/🦀️.rs`, en and de, text copied from the React catalog.
- `build_plugin_recovery_ui` beside the route-not-found page (`SH/📂️doc-open/🧭️route-not-found/🦀️.rs`): title, copy, Restart (`reloadPlugin`) and Disable (`quarantinePlugin`), both with `pluginId`. Panel id `os.plugin-recovery` (docs identity), registered as a leaf, published like the route page and painted by the same overlay card (`🗨️view-overlays`).
- State `OpeningState.plugin_recovery`: set when `reloadPlugin` fails (React `crashed`), cleared when it succeeds; `quarantinePlugin` (React `recovery.disablePlugin`) sets `quarantined` and uninstalls unless it is the session's own program.
- The panel now replaces the canvas: an opaque fill over the area between navbar and footer, with the content laid out edge to edge and the navbar left live (`🗨️view-overlays/💬️chrome-dialog-and-tooltips`). The session's own plugin cannot fail a reload natively (it is never dropped), so in practice the panel appears for non-primary reload failures and after a disable.

## 4. Board2d Tab

Verified PARTIAL. React's Tab capture listener cycles the brush candidate (`brushCycleCandidate`) when the brush is armed and the pointer is over the pane, or, when not armed, while this window owns the open suggestion menu. wgpu required both the pointer and the brush.

- `board2d_key_into` (`EL/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs`) takes the window id and applies React's gate; Escape keeps the pointer gate. `board2d_suggestion_menu_owner` parses the menu record (unit test); the cache field `BoardSyncCache.suggestion_menu_json` follows the surface retirement ladder.
- Caller `board2d_key` in `interaction-state/🦀️.rs` passes `Board2dSurface.window_id`.

## Build status

NOT COMPILED, by the coordinator's build policy: the queued cargo check was cancelled after more than 35 minutes in a cold queue, and integration lane E compiles everything. All claims are source-level.

Crates touched (check these): `semio-renderer-wgpu-shell-input`, `-shell-pointer`, `-shell-doc-open`, `-shell-docs`, `-shell-vocab`, `-shell-state`, `-engine-board`, `-engine-attach` and the EngineCanvas registry crate, `-ui-engine`, `-ui-engine-dump`, `-frame-actions`, `-scene-text-vfs` (`Board2dSurface` field), `semio-os-infinite-world-model`, and the world host crate (`sync_world3d_state`). Also the shell tests in `EL/🐚️Shell/🧪️tests/🧲️engine-surface-retention` (updated for the new `Board2d` variant field).

New unit tests to run: `text_touch` (frame-actions, 3), `suggestion_submenu` (shell-input, 5), `suggestion_menu_owner_tests` in `🎲️board2d/🦀️.rs` (engine-board, 1).

Likeliest compile risks: `semio_framework::DslValue::from(serde_json::Value)` in shell-input, `AdmittedSurfaceMap::iter()` shape in `suggestion_popup_request`, `if/else` closures coerced to `fn(f32, f32) -> Touch` in `interaction-state`, and the duplicate `tool-run-trace-layer` file next to `tool-run-trace` (only the latter was given `focus`).
