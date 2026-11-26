# Audit MA1: How the `repo dashboard` TUI Is Built

Read-only audit. No code, git, ticket or process changes. `repo dashboard` was not run.

Abbreviations: `REPO` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo`, `DASH` = `REPO/🔨️modules/🎛️dashboard`, `TUI` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/⌨️tui`.

## 1. Stack and code location

- Language: Rust, fully native. No ratatui, crossterm, bubbletea or ink. The terminal engine is in-house: `TUI/⚙️engine`, `TUI/🧱️cell-model`, `TUI/📺️terminal` (`features = ["native"]`), `TUI/📟️vt` (own VT parser/screen), `TUI/🚇️pty`, `TUI/🧩️widgets`.
- Entry point: `DASH/🚪️entrypoint/🦀️.rs` builds the `semio` binary. The library root is `DASH/🦀️.rs`. The view is `DASH/🖥️terminal/🦀️.rs`, with `run_with(root, parsed)` at about line 90 and `run_view` at about line 55.
- Crates (`DASH/*/📦️packages/🦀️rust/Cargo.toml`): `semio-framework-repo-dashboard` (bin `semio`), `-terminal`, `-daemon`, `-inventory`, `-registry`, `-launch`, `-labels`, `-preferences`, `-core`, `-cli`, `-wire`, `-repo-domain`.
- Terminal crate depends on `semio-ui-tui-{core,engine,frame,terminal,vt,widgets}`, `ui_locale`, `ui_styling`. No third-party TUI crate.
- Build/test entry: `DASH/📦️packages/🦀️rust/📋️project.json` (`@semio-tech/repo-dashboard-rs`), `repo ./📜️script.ts build|test`. The TS owned route is `DASH/🛤️task-routes/🟦️.ts`. The launch planner and its door route are documented in the `INSTANT-DASHBOARD-CONTROL-PLANE` ticket.
- The `repo` operations catalogue is `REPO/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json`. The grep for a `dashboard` key there found no entry. The dashboard command is routed through the door route, not a catalogue key.

## 2. Views, tabs and panels: registration

- Window kinds are one enum in `DASH/🖥️terminal/🪟️windows/🦀️.rs:49`:
  `pub enum Kind { Files, Document, Overview, Settings, Diagnostics, KeymapEditor, Help, Launcher, Output }`
- Placement enum at `windows/🦀️.rs:53`: `pub enum Place<'a> { Preference, Split(Axis), Stack(&'a str) }`. Windows are split or stacked by `Place`.
- Body variants are matched in `DASH/🖥️terminal/📋️panes/🦀️.rs` (`Body::Diagnostics` at about line 116). Overview rows come from `RowAction` at `panes/🦀️.rs:15` (`None, Files, NewTask, Settings, Help, Refresh, CancelDiscovery, Session(String), OpenReady(..), Diagnostic(usize), ...`). Adding a view means: add a `Kind` variant, a `Body` arm in `panes`, a `RowAction` entry, a label set, and a keymap action.
- Keymap is declarative: `DASH/⚙️preferences/⌨️keymap/` with `Scope::{Prefix, Window, View}`.
- Existing views that are relevant to fleets: `Overview` (task rows with status glyph), `Diagnostics` (list of failures), `Output` (per-session terminal). None shows aggregate metrics, workers or a fleet. `DASH/🖥️terminal/🪟️windows/🦀️.rs` has no metric types.
- Session model (`DASH/🌀️daemon/✉️ipc/🦀️.rs:232`, `SessionInfo`): `session_id, command, status, pid, code, started_ms, ended_ms, ready_url, title, group`. There are no CPU, memory, throughput or queue fields.

## 3. Data sources and refresh

- Daemon connection: `DASH/📎️connection/🦀️.rs`. One thread owns the transport. It writes queued client messages and reads `ServerMsg`. The first message is a hello. Queue cap is `QUEUED_BYTES = 16 MiB`. A slow view stops reading, so it cannot starve others.
- Wire protocol: `DASH/🌀️daemon/✉️ipc/🦀️.rs:90`, `enum ServerMsg { Attached, SessionChanged, SessionRemoved, Sessions{..}, ReplayStart, ReplayComplete, Error{code}, Pong, Shutdown }`. `ErrorCode` has 12 kinds. Schema lives under `DASH/🧬️schema/🌀️daemon/🔣️.json`. Transport is local IPC (`DASH/🌀️daemon/🚚️transport`, `🧵️wire`). No MCP, no HTTP, no sockets-as-API beyond the daemon endpoint.
- Inventory (command registry): `DASH/📚️inventory/🦀️.rs`. A background job reads workspace sources once. Snapshot is versioned (`SNAPSHOT_VERSION = 7`, `SNAPSHOT_LIMIT = 64 MiB`), keyed by file fingerprints. The dashboard starts it at `run_with`: `dash.inventory = Some(dashboard_inventory::start(&root, false))`.
- Refresh loop: `run_view` in `🖥️terminal/🦀️.rs` runs `dash.poll_daemon`, `poll_background`, `poll_source_suggestions`, `poll_documents`, then `term.wait(...)` with `IDLE_SLICE = 80 ms`, shortened by tui deadlines and prefix timeouts. It re-frames only when something changed (`polled.changed || background || ... || ticked`). Then `tui.render_due(now)` emits an ANSI patch. There is no timer-driven polling of metrics. Adding a fleet view means feeding a new source into `poll_*`, or using `tui.tick` for animation.

## 4. Charting primitives available

- Widget state enum: `TUI/🪀️widget/🦀️.rs:85` `WidgetState::{Label, List, Select, Tabs, Log, Input, Editor, Divider, Chip, Table, Terminal, Wizard, Tree, Scrollable, Progress, Toggle, Menu, Dialog, Palette, Tooltip}`. The widget modules are in `TUI/🧩️widgets/🦀️.rs` (`pub mod` list at lines 4-61).
- Progress: `ProgressState { label, value: Option<f32>, frame }` at `TUI/🧱️elements/📶️Progress/🎯️targets/⌨️tui/🦀️.rs:14` (determinate bar or indeterminate spinner). This is the only numeric visual.
- Table: `TableState`, `TableColumn`, `TableRow`, `TableAlign` (`TUI/🧩️widgets/table`). Usable for fleet tables.
- Not present: sparkline, gauge, bar chart, line/time-series chart, histogram. A grep for `sparkline|gauge|braille|bar_chart` in `TUI` and `DASH` found only comments and `Icons`. Any multi-resolution chart needs new primitives (for example a Unicode block-character sparkline using `▁▂▃▄▅▆▇█` in a `TUI` cell painter).

## 5. Internationalisation

- Labels: `DASH/🖥️terminal/🌐️labels/🦀️.rs`. A `dashboard_labels!` macro generates a compile-checked catalogue. Missing German text is a compile error. Identifiers stay verbatim.
- Shared locale machinery: `TUI/../🌐️locale` (`ui_locale::{AppLabels, Locale, Terminology}`). Ordering rule: en first, de second.
- Language choice: `DASH/🖥️terminal/🌐️language-choice/` (interactive first-run picker), declared in `DASH/⚙️preferences/🌐️language/🔣️.json`. Language comes from `SEMIO_LANGUAGE` (exported by `repo`), then `LANGUAGE`, then `LC_ALL`/`LC_MESSAGES`/`LANG`. No default is assumed. New fleet labels must be added to the macro catalogue in both languages.

## 6. Headless testing

- Unit tests in `DASH/🖥️terminal/🧪️tests/🔬️unit/` (for example `window-focus-and-sessions/🦀️.rs`). They build a `Dashboard`, drive it with events, call `tui.render_full().0` and assert on rendered text (for example `assert!(rendered.contains("exit 0"))`).
- Laws: `DASH/🖥️terminal/🧪️tests/🏛️laws/🦀️.rs`. Test internals are exported only under `#[cfg(feature = "testkit")] pub mod testing` (`DASH/🖥️terminal/🦀️.rs` end). `run_view` is re-exported for a fake backend.
- Engine/TUI tests: `TUI/🧪️tests/` (`🎞️render-equivalence`, `📏️layout`, `🔬️backend-native-unix-unit`, `🔬️engine`, `🔬️chrome`, ...).
- Commands: `REPO/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json` lists targets `test`, `test ci-source`, `test dispatch`, `test language`, `test source-freshness`, `test authority`, `test execution`, `build`. Run with `repo ./📜️script.ts test` from `DASH/📦️packages/🦀️rust`. Terminal crate tests need feature `testkit` (not executed in this audit, so exact cargo invocation is unverified).
- Test fixtures: `DASH/🧪️tests/🧫️fixtures/` (for example `🧭️journeys/🏗️workspace`, `🎮️registry/🏗️workspace.json`).

## 7. Control plane and instant dashboard

- Control plane is the workspace daemon (`DASH/🌀️daemon`): owns processes, PTYs, replay (64 KiB per session), reconnect, lock, journal (`📜️journal`), and lifecycle events. Views attach; daemon restart replays projections. Limits: 128 sessions, 16 views, 16 MiB per frame (from the `DASHBOARD-LAUNCH-COCKPIT` audit notes).
- Instant start: ticket `INSTANT-DASHBOARD-CONTROL-PLANE` (`26/11/03`). It adds a native door route `dashboard` so `repo dashboard` does not build the full graph. Installation record and digest govern rebuilds. This is startup latency, not metrics.
- Ticket summaries:
  - `DASHBOARD-TUI-WORKFORCE` (`26/08/12`): built the VT emulator (`TUI/📟️vt`, tests `vt_*`), PTY, widget clipboard, daemon client, registry, and a workforce scheduler with agent runners (`cursor-agent`, `claude -p`, `codex exec`). The workforce scheduler and `semio workflow status|run` are in that ticket's notes (`w5-workforce-agents.md`). I did not locate their source files.
  - `DASHBOARD-LAUNCH-COCKPIT` (`26/09/23`): runtime audit found the keyboard dead on macOS (`poll()` on `/dev/tty`), discovery of `.vscode/launch.json`, and verb filing. Audit captures live under `🗑️generated/runtime-audit`, which was not read.
  - `INSTANT-DASHBOARD-CONTROL-PLANE` (`26/11/03`): door route and in-process rebuild, described in section 7 above.
- No fleet, worker or multi-resolution metric concept exists in the dashboard today. Closest analogues: `SessionInfo` status and timing, `Diagnostics` list, `Progress` element.

## 8. Implications for fleet-monitoring views

1. Add a `Kind::Fleet` (or per-resolution kinds) in `windows/🦀️.rs`, a `Body` arm in `panes/🦀️.rs`, and labels in both `en` and `de` in `labels/🦀️.rs`.
2. Data: prefer a new `ServerMsg`/`ClientMsg` pair in `daemon/✉️ipc` with schema in `DASH/🧬️schema`, or a separate file/job fed by `dashboard_inventory`-style background thread. Keep it off the view thread.
3. Charts: no sparkline or gauge exists. Build a small `TUI` widget (cell painter with block glyphs) under `TUI/🧩️widgets`, plus a `Progress` reuse for gauges. Multi-resolution means a ring buffer per resolution, rendered at the chosen one.
4. Tests: render-to-text in `DASH/🖥️terminal/🧪️tests/🔬️unit`, plus a `TUI` render-equivalence test. Add a language-agnostic fixture under `DASH/🧪️tests/🧫️fixtures` per CLAUDE.md.

## Unverified

- Exact cargo test invocation and whether `testkit` tests pass today.
- Location of the workforce scheduler source (`w5`) and of the `semio workflow` command.
- Contents of the `runtime-audit` capture folder.
