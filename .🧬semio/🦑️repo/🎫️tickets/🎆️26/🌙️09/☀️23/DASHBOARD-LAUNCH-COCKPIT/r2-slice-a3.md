# R2 Slice A-3: Dashboard Terminal View

## Round 3 (newest): V-1 findings F3, F4, F5

* F3 fixed: the view's connect thread called `supervisor::start_detached`, the CLI helper that `println!`s `workspace dashboard daemon ready at pid N` onto the terminal the TUI draws on. It now calls the silent `spawn_daemon` and reports failures as a status line. Test `nothing_the_view_does_prints_into_the_terminal_it_draws_on` scans the view sources for `println!`, `eprintln!`, `print!(` and `start_detached`.
* F4 fixed: `Ctrl+B s` now sorts the restored sessions by start time, raises and focuses the newest task window (input mode on) before asking for the replay, so no extra Tab is needed. Test `restoring_all_task_views_raises_and_focuses_the_most_recent_one`.
* F5 (stale cells after incremental paint): the view cannot cause it by painting outside the engine (it never writes to the terminal except `present(render_due patch)`), and every scene mutation goes through `node_mut`/`set_visible`, which mark dirty; the F3 text was the one foreign writer and is gone. Whether the engine diff still leaves cells behind is T-B's: re-run `the_incremental_screen_equals_a_full_repaint` after this change.
* Unit result: `cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` = 72 passed, 0 failed. PTY journeys (`bun ./📜️script.ts test journeys` in the package, private build dir): **NOT RE-RUN**. The debug `semio` cannot be built since `semio-framework-ui` stopped compiling at `⌨️tui/🚇️pty/🦀️.rs:716:90` (`partition(is_script)` closure takes `&&str`, A-2's in-flight edit, unchanged for over 30 minutes while a retry loop ran). Re-run F3/F4 scenarios (`detaching_leaves_the_task_running…`, `two_views_of_one_workspace…`, `the_incremental_screen_equals_a_full_repaint`) once that line is fixed.

## Round 2c (newest)

Verified: `cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` = **64 passed, 0 failed** (private build dir); `bun test 🧪️tests/⌨️controls/🟦️.ts` = 4 pass. One edit outside my files, disclosed: `🎮️registry/🦀️.rs:1449` called a renamed method (`launch_json` -> `launch_plan_json`), which broke the whole crate for over an hour; I renamed the call only.

Wiring: width mode from `Capabilities` via `Tui::set_width_mode` (Cluster, or Scalar without full Unicode); task status on tabs through `Shell::set_status` after every remount (tab labels no longer carry a glyph; the chrome draws glyph and role colour and animates the spinner on `tick`); Tasks list rows carry `ListState::statuses` (glyph, colour, spinner by T-D).

Runtime-audit items, checked against the current code:

| Id | State | Evidence and test |
| --- | --- | --- |
| P1-6 | fixed | Overview rows come from the live session map (a later `Ctrl+B h` lists tasks); selection is kept by `RowAction::Session(id)`, not by index; `close_window` focuses the previous neighbour; `q` is plain typing. Tests `p1_6_the_tasks_list_keeps_its_selection…`, `p1_6_closing_a_window…`, `q_typed_into_a_search_never_quits…` |
| P1-7 | fixed | no `render_full` in the view sources (asserted by test); loop uses `render_due`/`render`; list, launcher, footer writes happen only when content differs, so unchanged frames leave `Tui::dirty()` false. Test `p1_7_an_unchanged_screen…` |
| P1-8 | fixed | Home/End/PageUp/PageDown (keymap `first`, `last`, `page-*`), caret and filter row by `Tree`, Ctrl+U and new `delete-word` (Alt+Backspace), paste, match count and a preview of the command line in the caption. Test `p1_8_the_launcher_filter…` |
| P1-11 | fixed | all view strings in the catalogue (source scan test); footer hints are fitted whole beside the status (`fit_hints`, controls first, short `hint_activate`), none is cut at 80 columns in en and de, armed and idle. Tests `p1_11_*` |
| P2-1 | fixed | journal: torn last line ignored and cut off by the next writer; lock wait bounded (2 s) with a message naming the file; a journal that would exceed 1 MiB is compacted into one snapshot event plus the new one; `load_lenient` never fails startup on a broken journal and the dashboard shows it. Tests `p2_1_*` (4) |
| P2-6 | fixed (view side) | Esc followed at once by the prefix arrives as the Alt chord; when no binding owns it the view sends Esc to the program and arms the prefix; prefix twice sends it literally (existing test); armed prefix expires after 3 s (`expire_prefix`, loop wait bounded by `prefix_wait`). The byte-level escape timeout stays in T-A's parser. Test `p2_6_*` |
| P2-7 | fixed | limit checked before `set_body`; a failed send keeps the start in `pending` for the reconnect (no orphan, notice); reconnect: one in-flight attempt, backoff 0.5 s doubling to 8 s, reset on success. Tests `p2_7_*` (2) |

## Round 2b (newest)

Verified: `cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` = **51 passed, 0 failed** (private `CARGO_BUILD_BUILD_DIR=…/build-fleet-a3`, because the shared build dir was locked by about 12 concurrent cargos); `bun test 🧪️tests/⌨️controls/🟦️.ts` = 4 pass.

1. Launcher uses T-D's `Tree`: the model holds the commands as a flat pre-order `TreeItem` forest (`tree_items`, `rebuild_tree(old)` keeps open folders, filter and selection across registry updates); the Tree widget owns browse keys/mouse/filter/scroll; the model only reacts to `Activated(item)` and owns form + confirmation. A command's label carries its id so the tree filter finds ids. The window has tree, caption, input and form list; tree and list swap visibility by stage and the engine focus follows (`follow_stage`). The 18 journeys run unchanged.
2. Status: `Dashboard::status_of` maps sessions to T-D `theme::Status` (waiting, running, success, failure, fault, warning, info for ready); tabs and the Tasks list use `Status::glyph(theme glyph set)`. **Colour role is not applied**: `WindowStackTabState` and `ListState` rows carry no role (request T-B: `status: Option<Status>` on stack tabs; T-D: per-row role on `ListState`). Spinner animation needs the same hook.
3. Capabilities: `apply_capabilities(unicode_full)` sets `Theme::set_glyphs` (ASCII glyphs when the backend is below full Unicode), covered by a test. **`CellBuffer::set_width_mode` cannot be reached**: `Tui` keeps its buffers private (request T-B: `Tui::set_width_mode`).
4. A-2 protocol: `ReplayStart` resets only that session's terminal (RIS via `feed_replay`) and notes truncation; per-session `ReplayComplete` ignored, `ReplayComplete{None}` ends restoring and re-sends sizes; paged `Sessions` applied as they come; `Pending`/`Interrupted` shown (glyphs/rows); `Error{code}` is localized (11 codes, en and de) into the notice and never written into a terminal; `skew()` banner localized (incompatible protocol refuses the connection and retries after 5 s).
5. Waker: `Connection::set_notifier` is wired to the backend `Waker` at connect, so PTY output repaints at once; the loop waits at most 80 ms (or the engine deadline), the 16 ms live cap is gone.
6. Keymap gained `prefix.search-output` (`/`), which calls `begin_search()` on the focused terminal; en/de text, test.
7. A compound opens one tab per member with its own TaskLabel (test: hub and quiz tabs); resize and input paths were already view-complete (rect-driven `fit` + `Resize`, `Connection::input`).

New tests: tab per compound member, localized errors, replay clears only its terminal, ASCII glyphs, search action.

## Round 2a

Owner files: `🖥️terminal/**`, `⚙️preferences/**`, `🧬️schema/⚙️preferences/**`, fixtures `⌨️controls`, `⌨️keymap`, `⚙️preferences`, `🔎️launcher`, tests `⌨️controls`, `⚙️preferences`. Dashboard root `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`.

## 1. What exists now

| Area | State | Where |
| --- | --- | --- |
| Declarative keymap | prefix key + bindings as data (scopes `prefix`, `window`, `view`), defaults as data, overrides fold per binding over local/workspace preference events, collisions/unknown/typing keys are ignored and named, never silent | `D/⚙️preferences/⌨️keymap/{🔣️.json,🦀️.rs}`, schema `D/🧬️schema/⚙️preferences/⌨️keymap/🔣️.json` and `…/⚙️preferences/🔣️.json` |
| Preference events | `Change` gained `prefix` and `bindings`; CLI `--prefix`, `--bindings JSON`, env `SEMIO_DASHBOARD_PREFIX`; `Preferences::{keymap,locale,launch_defaults}`; `bind(CommandSpec)` removed | `D/⚙️preferences/🦀️.rs` |
| Locale catalogue | one `ui_locale::app_labels!` block (about 125 labels, en and de, compile-checked, placeholders checked equal in both languages); `Locale` comes from the preference, no view-local enum, no inline pairs | `D/🖥️terminal/🌐️labels/🦀️.rs` |
| Launcher | pure state machine over the registry: tree filed by verb, incremental all-words search (selects first command), select then activate, parameter form (choice/text/flag, defaults, pre-selection from preferences, extra args, extra env), live `resolve` preview with `requires`, compound members and the exact command, confirmation for `mutating`, mouse click select-then-activate | `D/🖥️terminal/🚀️launcher/🦀️.rs` |
| Starts | `registry.resolve(id, &Request)` -> `SpawnGroup::from_launch(&launch, &[])` -> one `ClientMsg::SpawnGroup`; the launcher window becomes the primary task terminal; services already live are not requested again; queued while disconnected (cancellable with the stop key) | `D/🖥️terminal/📡️sessions/🦀️.rs` |
| Tabs and titles | tab text = status glyph + `registry::tab_texts` (duplicates ` ·2`), written into the layout node `title` (T-B contract: the chrome reads tab labels from there); long title (`window_title` + exit code) shown in the footer for the focused task; no string matching anywhere | `D/🖥️terminal/🪟️windows/🦀️.rs` |
| Focus | the engine's focus is the only source (`focused_index` walks from `tui.focus()`); click on a window moves the keyboard there; overlay focus keeps the last window | `🪟️windows` |
| Resize | every frame: `tui.layout()`, `terminal.fit(rect)`, `ClientMsg::Resize` only for changed sizes (so split, zoom, close, new tab, drag, terminal resize and replay are all covered); start size comes from the widget rect (no `-4`) | `Dashboard::sync_sizes` |
| Input | `Dashboard::handle(&Event)` is the only entry (testable without a terminal). Terminal with keyboard: only the prefix key is reserved (`prefix prefix` sends it); keys encoded by T-C `TerminalState::on_key` (passthrough set per frame); replies/Copy signals of `feed` forwarded; `feed_replay` while restoring; `RIS` (not `TerminalState::new`) before a replay; paste via `Event::Paste`; wheel, selection and copy through the engine; `q` no longer quits | `D/🖥️terminal/⌨️controls/🦀️.rs` |
| Windows | prefix actions: new task, tasks, settings, keyboard help, split down/right, grow/shrink, zoom, close, next/previous, detach, shutdown (waits for the daemon, 5 s patience), restart/stop/kill, copy, toggle input, appearance, language; chrome signals (close/maximize/new tab with tab index, tab activate/move, splitter drag) applied; right-button context menu per window/tab/terminal with keys from the keymap | `⌨️controls` |
| Footer, help, usage | generated from keymap + catalogue (`footer_hints`, help window rows, `help_text`); armed prefix lists every prefix action | `⌨️controls`, `📋️panes`, `🦀️.rs` |

## 2. Tests (all run)

`cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` (CARGO_TARGET_DIR `target-fleet-a3`): see section 5 for the last verified result.

* `⚙️preferences/⌨️keymap/🧪️tests`: spellings, defaults vs shared vectors, customization cases, reachability.
* `🖥️terminal/🌐️labels/🧪️tests`: completeness and placeholder parity en/de, every action has a text.
* `🖥️terminal/🚀️launcher/🧪️tests`: 18 journeys from `🧫️fixtures/🔎️launcher/🔣️.json` (workspace facts + pinned entry list + steps + expectations), registry update keeps search/selection, German verbs.
* `🖥️terminal/🧪️tests`: view journeys with synthetic events and an offline link (no daemon): launcher flow and resolved spawn with label, parameters/args/env, confirmation, required service, terminal passthrough bytes, prefix actions, unbound key, footer in en/de, customized keymap, help, tab/title from labels, restore without stealing focus, resize propagation, click focus, shutdown waits, cancel pending start, copy, incremental `render()`, context menu, usage text.
* `⚙️preferences/🧪️tests`: journal replay of keymap events (layer merge per binding), rejected changes, arguments, launch pre-selection.
* TS (third-party oracles): `D/🧪️tests/⌨️controls/🟦️.ts` (bun): Ajv validates keymap + preference vectors, independent TypeScript key canonicalizer and keymap builder reproduce all vectors, Node `readline` decodes terminal bytes into the same canonical key names as `spec_of`. Result: 4 pass. The old `🧪️tests/🌀️control-plane/🟦️.ts` preference/launcher/controls tests still pass with the extended fixture.
* Features: `🧪️tests/⌨️controls/🥒️.feature`, `🧪️tests/⚙️preferences/🥒️.feature` extended.

## 3. Decisions and notes

* Verbs/tree: playgrounds are one entry with a `renderer` parameter (A-1 API), not one row per renderer; the preference renderer pre-selects it. `launch_defaults` also pre-selects `language`, `terminology`, `appearance` for commands that offer them (the old `bind` locked them too); this makes them appear in the task label parameters.
* Status colour role: only the glyph is done; no status roles exist in `Theme` yet (T-D).
* The long window title is not drawn by the T-B chrome (only tab labels); it lives in the footer for the focused task.
* Exit code of a task: footer title only (and Tasks list); no line is written into the pane.

## 4. Open requests to other slices

* A-2: a notifier on `Connection` (waker) so PTY output repaints without the 16 ms live slice; the view polls `receive(ZERO)` once per loop (`LIVE_SLICE` 16 ms while a task is live, 80 ms idle).
* T-D: a `Tree` element; the launcher draws its tree through `ListState` (windowed slice of pre-rendered lines), swapping is local to `paint_launcher`/`screen`.
* A-1: nothing blocking; `Request.env` and `SpawnGroup::from_launch` are used as published.
* L-2/docs: the dashboard README key table should be replaced by `semio --help` output (generated from the keymap); I did not touch `README.md`.

## 5. Verification log

* Verified (run, 2026-10-08): `cargo check -p semio-framework-repo-dashboard` clean for my files; `cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` = 41 passed, 0 failed (after the context menu, incremental render, and tab/title contract changes); `bun test ./🧪️tests/⌨️controls/🟦️.ts` = 4 pass; control-plane TS preference/launcher/control tests = 3 pass; keymap module in a scratch crate = 4 pass.
* Re-verified after T-B/T-D landed and the coordinator relay (T-B §6) was integrated: `cargo test -p semio-framework-repo-dashboard --lib -- terminal:: preferences::` = 46 passed, 0 failed. This now includes the `SpawnGroup::from_launch` start path, `RESET` through `feed_replay`, `connection.input()`/`skew()`, `TabMoved` -> `reorder_stack_tab`, `SplitterDragged` -> `resize_split`, the context menu (`open_menu`, `Activated`/`Dismissed`), keymap preference tests, and the new chrome-labels test.
* T-B relay integrated: `WindowState.labels` (`ChromeLabels`, six en/de texts) and `new_tab = true` are set per locale, written only when different; the view no longer writes `WindowState.focused` (engine owns it; the view reads `tui.focus()`); the main loop drives `tui.tick(now)`, `deadline_ms()` as the backend wait bound (capped by 16 ms while a task is live, 80 ms idle), `render_due(now)` and `present(&patch, tui.cursor())`; the remount hack is gone (titles live on layout nodes).
* bun: `🧪️tests/⌨️controls/🟦️.ts` 4 pass; control-plane preference/launcher/control tests 3 pass.
* Not done: waker wiring (needs a notifier on `Connection`), a `Tree` widget (T-D has none yet), status colour roles, `begin_search` prefix action, focus-in/out forwarding to the child (T-B/T-C `on_focus` is engine-side).
