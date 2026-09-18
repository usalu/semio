# W1c — shell keyboard shortcuts + command palette parity (wgpu ⇄ React)

Packet W1c of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Sources of truth read for this packet:

- React shortcut table: `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163-190` (`SHELL_KEYBINDINGS`).
- React shell keyboard handlers: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8562-8674` (`handleAppKeydown`) and `:9378-9407` (`handleCommandKeydown`), both gated per shell by `useShellKeydown`.
- React window verbs: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1815-1843`, plus `🏛️ShellHost/🟦️.tsx:10561-10598` (`onWindowOpenInNewWindow`).
- React palette: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🟦️.tsx:26-95` (`UISearch`) + `🏛️ShellHost/🟦️.tsx:10104-10190` (`searchItems`), ranked by `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🔎️fuzzy-ranking/🟦️.ts`.
- wgpu side: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, plus one additive method on `…/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`.

## 1. What changed, in one line each

| # | Change | File / region |
|---|---|---|
| 1 | One declarative `SHELL_SHORTCUT_ROWS` table transcribing React's `SHELL_KEYBINDINGS`, with `ShellShortcut`, `shell_shortcut_for*`, `ShellEditVerb`, `shell_edit_verb_for` | Shell wgpu, new region `⌨️ShellShortcutTable` |
| 2 | `is_reserved_shell_chord` is now derived FROM that table (it was an open-coded `match` that listed five chars) | Shell wgpu, `⌨️WindowScope` |
| 2b | All eight anchors are reachable because **W1h's 8-anchor model landed mid-packet**: `toggle_panel_anchor` delegates to their `toggle_anchor`/`anchor_open`/`dock_tabs`, so the chords follow that model instead of second-guessing it | Shell wgpu, `⌨️ShellShortcutDispatch` + `🧭️PanelAnchorAccessors` (W1h) |
| 3 | `handle_keyboard`'s six open-coded `meta && Char(..)` arms replaced by one table dispatch | Shell wgpu, `handle_keyboard` |
| 4 | New sync/async dispatch halves + panel-anchor / window-verb implementations | Shell wgpu, new region `⌨️ShellShortcutDispatch` |
| 5 | `mod+z` / `mod+shift+z` / `mod+y` dispatched after the app-keybinding loop | Shell wgpu, `handle_keyboard_async` |
| 6 | Escape now also disarms the active mode-level TOOL when no utility is active | Shell wgpu, `handle_keyboard_async` |
| 7 | `DockState::split_stack_with_instance` (instance-aware split; the public split derives the kind from the id) | Dock wgpu |
| 8 | `rank_fuzzy_items` — a branch-for-branch port of `🔎️fuzzy-ranking/🟦️.ts`, replacing the hand-rolled `fuzzy_match_score` | Shell wgpu, new region `🔎️FuzzyRankingParity` |
| 9 | Palette rows: React's declaration order, React's field weights, React's `…` suffix, chords as the description line, `goHome` added, arg-carrying commands of EVERY owner listed and executable | Shell wgpu, `build_search_items` / `command_search_items` / `activate_search_item` |
| 10 | `ShellState.expanded_command_id` — the twin of React's `SET_COMMAND_EXPANDED` | Shell wgpu, `ShellState` |

## 2. The shortcut table (shortcut → React handler → wgpu handler)

`✅` = already correct before this packet, `🆕` = added/fixed here.

| Chord | React handler | wgpu handler | |
|---|---|---|---|
| `mod+p` | `SHELL_KEYBINDINGS["ui.search.toggle"]` → `useControlKeybinding` on the search toggle | `ShellShortcut::ToggleSearch` → `apply_sync_shell_shortcut` | ✅ (now table-driven) |
| `mod+f` | `ui.find.toggle` | `ShellShortcut::ToggleFind` | ✅ |
| `mod+shift+f` | `os.toggleFullscreen` (control keybinding) | `ShellShortcut::ToggleFullscreen` → `apply_os_command("os.toggleFullscreen")` | 🆕 |
| `f11`, `ctrl+meta+f` | `handleCommandKeydown` over `resolvedCommands[].keybindings` | `fullscreen_chord` arm in `handle_keyboard_async` | ✅ |
| `mod+[` | `ui.nav.back` | `ShellShortcut::NavBack` | ✅ |
| `mod+]` | `ui.nav.forward` | `ShellShortcut::NavForward` | ✅ |
| `mod+up` | `ui.nav.up` | `ShellShortcut::NavUp` | ✅ |
| `ctrl+b`, `meta+b` | `ui.shell.panelAnchor.topLeft` | `TogglePanelAnchor(TopLeft)` → `toggle_anchor(TopLeft)` | ✅ chord / 🆕 anchor-scoped |
| `ctrl+m`, `meta+m` | `ui.shell.panelAnchor.topMiddle` | `TogglePanelAnchor(TopMiddle)` | 🆕 |
| `ctrl+shift+b`, `meta+shift+b` | `ui.shell.panelAnchor.topRight` | `TogglePanelAnchor(TopRight)` | ✅ chord / 🆕 anchor-scoped |
| `ctrl+shift+m`, `meta+shift+m` | `ui.shell.panelAnchor.rightMiddle` | `TogglePanelAnchor(RightMiddle)` | 🆕 |
| `ctrl+alt+shift+b`, `meta+alt+shift+b` | `ui.shell.panelAnchor.bottomRight` | `TogglePanelAnchor(BottomRight)` | 🆕 |
| `ctrl+alt+m`, `meta+alt+m` | `ui.shell.panelAnchor.bottomMiddle` | `TogglePanelAnchor(BottomMiddle)` — the **Command** panel's own anchor | 🆕 |
| `ctrl+alt+b`, `meta+alt+b` | `ui.shell.panelAnchor.bottomLeft` | `TogglePanelAnchor(BottomLeft)` | 🆕 |
| `ctrl+alt+shift+m`, `meta+alt+shift+m` | `ui.shell.panelAnchor.leftMiddle` | `TogglePanelAnchor(LeftMiddle)` | 🆕 |
| `mod+shift+w` | `ui.window.close` → `Canvas`'s `closeWindow(activeWindowId)` | `CloseWindow` → `dock.close_window_in_stack` (+ `persist_dock_layout`), same path the `dock.tab.*.close` control takes | 🆕 |
| `mod+shift+enter` | `ui.window.focus` → `toggleMaximize(stackPath)`, enabled only when `canMaximize` | `FocusWindow` → `dock.set_stack_active` + `dock.toggle_maximize`, refused when the mode has ≤1 window | 🆕 |
| `mod+shift+n` | `ui.window.newWindow` → `onWindowOpenInNewWindow`: a new INSTANCE `{kindId}-{n}` split right | `NewWindow` → `dock.split_stack_with_instance(path, "{kind}-{n}", kind, Right)` | 🆕 |
| `mod+alt+arrowright` / `arrowleft` | `ui.shell.mode.next` / `.previous` | `ModeStep(±1)` → `apply_mode_step` | ✅ |
| `mod+alt+e` / `mod+alt+v` | `playground.navbar.roles.editor` / `.viewer` | `SurfaceRole(..)` → `switch_to_session_role` | ✅ |
| `mod+z` | `handleAppKeydown` tail → `onAction({controllerId: app.controllerId, action: "undo"})` | `ShellEditVerb::Undo` → same `dispatch_action` funnel, after the app-keybinding loop | 🆕 |
| `mod+shift+z` | idem, `"redo"` | `ShellEditVerb::Redo` | 🆕 |
| `mod+y` | idem, `"redo"` (the second React alias) | `ShellEditVerb::Redo` | 🆕 |
| `escape` (utility armed) | `handleAppKeydown` → `setActiveUtility` with an empty id | idle-Escape arm → `active_utility_by_window.remove` | ✅ |
| `escape` (tool armed) | `handleAppKeydown` → `setActiveTool` with an empty id | idle-Escape arm → `dispatch_action("setActiveTool", {toolId: ""})` | 🆕 |
| `escape` / `enter,arrowright` / `arrowleft` | `ui.introduction.skip` / `.next` / `.back` | chrome-tour arms at the top of `handle_keyboard` | ✅ |
| `escape` / `enter` | `ui.dialog.cancel` / `.submit` | sync-card, check-in-card and palette arms | ✅ |
| app-declared chords | `handleAppKeydown`'s `session.app.keybindings` loop with `resolveKeybindingTargetWindowV1` + staged-form intent | `match_app_keybinding` → `dispatch_app_keybinding` (same window-scope resolution, same P4 intent) | ✅ |
| command chords | `handleCommandKeydown` over `resolvedCommands` | `resolved_commands()` scan in `handle_keyboard_async` | ✅ |
| `Tab` / `Shift+Tab` | the DOM's own focus order | `cycle_active_window` (cross-window) + `interpreter::dispatch_ui_event` (within content) | wgpu-only, no DOM to inherit |

Ordering inside `handle_keyboard_async` now reads, top to bottom: context menu → palette/find `Enter` → focused-content `Enter`/`Escape` commit → **async shell table rows** (role, mode, fullscreen) → `f11`/`ctrl+meta+f` → command-keybinding scan → content-focus routing → Escape (utility, then tool) → app keybindings → **`mod+z`/`mod+shift+z`/`mod+y`** → `handle_keyboard` (**sync shell table rows**, tour, sync card, palette keys, `Tab`, focused input). The two edit chords sit after the app loop on purpose: that is where React puts them, and it is what lets an app shadow `mod+z` with its own binding.

### Gating rules mirrored

- **Not while typing.** Every row is behind `content_is_editing` (React: `isEditableTarget` / react-hotkeys-hook's form-tag default). The shell's own overlay query fields are chrome, not content, so `mod+p` still closes the palette it opened.
- **Not into an open overlay.** The nav, panel-anchor and window rows additionally refuse while the palette/find overlay is open; React gets this for free because the palette is a focus-trapping dialog with a focused input.
- **An empty anchor is a no-op.** `toggle_panel_anchor` refuses an anchor `dock_tabs` gives no tab, so a chord never opens an empty panel over the canvas.
- **Idle-only for guest round-trips.** Role, mode, fullscreen, the edit chords and app keybindings require `idle` (no focus, no overlay, no sync card, no dock drag), unchanged from before.
- **Reserved beats app.** `is_reserved_shell_chord` is now literally "is this a row of the table (plus `f11`/`ctrl+meta+f`)", so the fourteen chords added here are reserved automatically — React's `reservedShellChordsV1(SHELL_KEYBINDINGS, overrides)` rule. The edit chords are deliberately NOT reserved.

## 3. Command palette

### 3.1 The premise in the audit was stale

`📓️audit-runtime-boot-input.md` row #9 records the palette's own comment: *"there is no `ArtifactApp::handle_command` RPC wired on the plugin bridge yet (only `handle_action` exists)"*. That is no longer true, and had not been true for a while: `ProgramBridgeEntry::handle_command` exists on **both** backends (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:551-557`, JS at `:818`, wasm at `:267`) and `ShellState::dispatch_command` has always driven it. Arg-carrying Plugin/App/Mode commands were **invisible, not unexecutable** — the palette skipped listing them on a premise about the transport.

### 3.2 What the palette now lists

`command_search_items` covers every `in_palette` command of every owner, in three shapes:

| Command shape | React | wgpu now |
|---|---|---|
| zero args | fires `onCommand(address)` | `os-command:<id>` (os, local funnel) or `command:<invocation>` → `dispatch_command` |
| exactly one `Select` arg | one row, `…` suffix, opens the form | one row **per option** ("Set Appearance: Light"), firing with that one argument bound |
| anything else (multi-arg, or a non-`Select` first arg) | one row, `…` suffix, opens the command panel expanded | one row, `…` suffix, `command-form:<addressKey>` → right panel, Settings kind, Commands tab, `expanded_command_id` set |

The per-option expansion is kept (it is strictly more direct than React's form hop and is how every os command already worked), but it is now **bounded to a single-argument command**: expanding one argument of a multi-argument command would silent-fire the other arguments' declared defaults, which is exactly what P4 forbids. Those commands take the `…` redirect instead.

### 3.3 Row inventory vs React

| Row family | React | wgpu now |
|---|---|---|
| `panel.*` | ✅ `setActivePanelTab` | ✅ same |
| `window.*` | ✅ `SET_ACTIVE_WINDOW_ID` | ✅ same |
| `command.*` | ✅ all in-palette commands | ✅ all in-palette commands (see 3.2) |
| `spawn.<pluginId>` (host mode) | ✅ `spawnApp` over `spacePrograms` | ❌ absent — the wgpu shell has no `spacePrograms`/`spawnApp` plumbing at all (not a palette gap; a host-mode gap) |
| `studio.undo` / `studio.redo` | ✅ | ✅ (now localized labels, not raw action ids) |
| `studio.home` | ✅ `goHome` | 🆕 added |
| `studio.commitCheckpoint` | ❌ | wgpu superset, kept |
| `keybinding.*` | ❌ (React reaches app verbs via the window Actions rail) | wgpu superset, kept — but now obeys React's own P3/P4 rule: an arg-carrying verb redirects to its staged form (`action-panel:`) instead of silent-firing defaults, and shows the verb's resolved label + chord instead of the raw action id |

Row order is now React's: panels → windows → commands → keybindings → host-app verbs. Order is load-bearing: it is the ranker's tie-break and it is what a cold (empty-query) palette shows.

### 3.4 Ranking

Before: `fuzzy_match_score` — a hand-rolled subsequence scorer, **no threshold** (a query matching nothing still returned the whole palette in id order), no per-field weights, no token splitting, no typo tolerance, and `group` matched at the same weight as `label`.

Now: `rank_fuzzy_items`, a branch-for-branch port of `🔎️fuzzy-ranking/🟦️.ts` — normalize, per-token scoring over weighted fields, prefix (`0.04 + …`), substring (`0.1 + …`), gapped subsequence (`0.2 + …`) and bounded Damerau–Levenshtein (`0.18 + …`), lower-is-better, ties on declaration order, threshold `0.4`, limit `20`, fields `label ×2 / description ×1 / category ×0.5`. Both palette surfaces use it (`filtered_search_items` and `filtered_find_items`), exactly as React's `UISearch` and `UIFind` both call `rankFuzzyItems` with the same options object.

One documented divergence, carried in the port's docstring: JavaScript's `String.normalize("NFKD")` has no std equivalent and this codebase takes no external runtime dependency, so combining marks already present in the input are stripped but pre-composed characters are not decomposed first. Every ASCII query ranks identically.

## 4. Tests

New file `🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` (**15 tests**), wired as `mod shell_shortcuts_palette_tests` beside the existing Shell test modules:

| Region | Test | Pins |
|---|---|---|
| ⌨️ChordTable | `the_shortcut_table_transcribes_every_react_shell_keybinding_row` | all 21 rows, control id + chords, verbatim |
| | `every_row_dispatches_its_verb_and_outranks_app_keybindings` | 29 chords each resolve to a verb AND are reserved |
| | `the_role_and_mode_helpers_agree_with_the_table` | the two hand-written chord helpers vs the table |
| | `the_edit_chords_are_shadowable_and_carry_the_react_redo_alias` | `mod+z`/`mod+shift+z`/`mod+y`, and that they are NOT reserved |
| | `a_focused_content_field_swallows_every_shell_chord` | the not-while-typing gate; a bare key is never a chord |
| 🧭️PanelAndWindowChords | `each_panel_anchor_chord_toggles_its_own_anchor` | each of the eight anchors toggles its OWN anchor, incl. the bottom-middle Command anchor; an empty anchor is a witnessed no-op |
| | `the_overlay_chords_toggle_through_the_same_table` | `mod+f` opens AND closes through the table |
| | `the_window_chords_close_maximize_and_open_a_new_instance` | close / maximize / `{kind}-2` instance with the right kind |
| 🔎️FuzzyRanking | `the_ranker_answers_the_same_branches_as_the_typescript_twin` | exact < prefix < substring < subsequence, typo inside threshold, no-match |
| | `the_palette_ranking_bounds_and_orders_the_way_react_does` | empty query keeps order, no-match is empty, limit 20 |
| | `a_label_hit_outranks_the_same_hit_in_a_lower_weighted_field` | the `×2 / ×0.5` weighting |
| 🎛️CommandPalette | `the_palette_lists_and_executes_every_in_palette_command` | zero-arg fires, single-select expands per option with one bound arg, multi/non-select gets `…` + `command-form:` |
| | `os_commands_keep_their_local_per_option_rows` | os rows stay on the local funnel; chords render as the description |
| | `picking_an_arg_carrying_command_opens_its_form` | the `SET_PANEL_VISIBLE`+`SET_PANEL_PATH`+`SET_COMMAND_EXPANDED` trio |
| | `the_palette_rows_follow_the_react_declaration_order` | panels → windows → commands → keybindings → studio |

Three tests in `🔬️wgpu-command-registry/🦀️.rs` that asserted the deleted `fuzzy_match_score` were rewritten against the ported ranker (same laws, lower-is-better): `fuzzy_ranking_finds_a_scattered_subsequence_and_rejects_non_matches`, `fuzzy_ranking_puts_a_contiguous_prefix_above_a_scattered_match`, `fuzzy_ranking_is_case_insensitive`.

The pre-existing `every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings` (`🔬️wgpu-shell-chrome-parity/🦀️.rs`), `the_alt_axis_never_swallows_a_neighbouring_chord`, and `🔬️wgpu-shell-input`'s `the_palette_chord_toggles_and_never_types_itself_into_the_query` / `escape_closes_the_palette_rather_than_committing_its_query_field` were kept passing on purpose: `shell_role_chord`/`shell_mode_step_chord` stay the source of truth for their own rows (the table calls them), and `mod+p` stays answerable through the synchronous `handle_keyboard` entry point those tests drive.

### Verification

See §6.

## 5. Remaining gaps (deliberately not closed here)

1. **`expanded_command_id` has no reader yet.** `command-form:` now lands exactly where React lands — W1h's bottom-middle **Command** anchor (`FRAMEWORK_CATEGORY_COMMAND_ID`), via `reveal_dock_tab`, with `expanded_command_id` set. That is React's `SET_PANEL_VISIBLE` + `SET_PANEL_PATH` + `SET_COMMAND_EXPANDED` trio in full. What is still missing is the third write's *consumer*: the Command dock's body must read `ShellState::expanded_command_id` and open that command's staged form (with Execute dispatching `dispatch_command` with the staged arguments). **This belongs to W1h's command-dock packet** — until it lands, a multi-argument plugin command is reachable and correctly addressed but its form is not yet painted.
2. **Per-anchor chord ↔ anchor content.** The chords address anchors, not contents, so an anchor a session assigns no tab to is a witnessed no-op. With the current `default_dock` an app that declares no Display-group or Settings-group panel tab simply has nothing at those anchors — correct, but worth knowing when probing.
3. **`spawn.<pluginId>` palette rows.** Absent because the wgpu shell has no `spacePrograms`/`spawnApp` host-mode plumbing at all — a host-mode packet, not a palette one.
4. **Os command argument ids differ.** React's `buildOsCommands` names its select args `appearance`/`themeId`/`layout`/`locale`/`terminology`/`driver`; the wgpu mirror names every one of them `value`. Harmless today (both sides resolve os commands locally, and `apply_os_command` reads the option value positionally), but it means an os `CommandInvocation` is not wire-identical across renderers. Also, React declares `os.introduceApp` / `os.playTutorial` / `os.recordTutorial` / `os.setLayout` / `open-artifact-with-*`; wgpu's `build_os_commands` declares none of them and adds none here.
5. **Keybinding overrides.** React merges user overrides over `SHELL_KEYBINDINGS` (`composeControlKeybindings` / `reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides)`). The wgpu table is static; `reserved_shell_chords_v1` (the override-aware twin) exists but nothing feeds it user overrides yet, so a rebound shell chord is not honoured on wgpu.
6. **No browser probe.** Everything here is pinned by unit tests against real `ShellState`/`DockState` fixtures; a live `?renderer=wgpu` probe pressing `⌘⇧W`/`⌘Z` on the canvas was not run (no serve was started for this packet).

## 6. Verification

All run in the foreground of this packet's session, on the live tree (which several peer workers were editing throughout — see the note below).

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **0 errors** | `🗑️generated/w1c-native-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | **0 errors** | `🗑️generated/w1c-wasm-check.txt` |
| `cargo test … --lib shell_shortcuts_palette` | **15 passed, 0 failed** | `🗑️generated/w1c-tests.txt` |
| `cargo test … --lib fuzzy_ranking` | **3 passed, 0 failed** (the three rewritten command-registry ranking laws) | `🗑️generated/w1c-tests-fuzzy_ranking.txt` |
| `cargo test … --lib chord` | **11 passed, 0 failed** (incl. the pre-existing chrome-parity chord laws) | `🗑️generated/w1c-tests-chord.txt` |
| `cargo test … --lib shell_input` | **19 passed, 0 failed** (incl. the `mod+p` palette-chord regression) | `🗑️generated/w1c-tests-shell_input.txt` |
| `cargo test … --lib command_registry` | 32 passed, **1 failed** — `directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss` ("rebootstrap resets the frontier"), a Directory-lane test in another worker's region, untouched by this packet | `🗑️generated/w1c-tests-command_registry.txt` |

Two things this packet had to work around, worth recording for the ticket:

- **Peer churn in `semio-framework-ui`.** Six `cargo test` attempts died on `could not compile semio-framework-ui (lib)` with a *different* error family each time (`taffy` `Send` bounds, `register_input_meta` arity, `UiSurfaceToken::new`, `FlexTree::solve`, `LayoutJobStage::MeasureLayout`) — a live refactor in that crate, never this packet's code. The `--lib` check of this crate was green throughout.
- **One real cross-worker contract change.** `DockState::close_window_in_stack` was rewritten mid-packet (W1i) to delegate to a new `close_window`, which — correctly, matching React's `closeWindow` — no longer refuses to close a mode's last window. `mod+shift+w`'s test and the `close_active_window` docstring were corrected to that contract rather than the old guard.
- **Fleet build contention.** At peak, 23 concurrent `cargo` processes were parked with zero `rustc` running for ~20 minutes, and three test attempts were `SIGKILL`ed (exit 137) under memory pressure. Dropping to `-j 3` is what let the runs above finish.
