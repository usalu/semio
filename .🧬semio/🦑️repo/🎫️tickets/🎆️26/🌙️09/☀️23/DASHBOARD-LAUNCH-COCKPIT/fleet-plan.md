# Fleet Plan: Dashboard As The Only Developer Control Plane

Coordinator document. Every execution agent reads the sections named in its brief. Contracts here are
binding; if a contract cannot work, report to the coordinator instead of inventing a variant.

Audits this plan is built on (same folder): `launch-inventory.md`, `launch-dependents.md`,
`tui-interaction-audit.md` (defects `D01`–`D37`), `tui-chrome-parity-audit.md` (defects `R01`–`R25`),
`dashboard-runtime-audit.md`.

## 1. Target State

- `semio` (native Rust) is the single control plane: interactive TUI and a non-interactive command line
  that drive the same workspace daemon. Editors and agents start nothing any other way.
- No file named `launch.json` or `launch.seed.jsonc` exists; nothing generates, reads, gates, tests or
  documents one. `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json`, the generator
  `…/📇️registry/🚀️launch/` and every dependent are removed or re-expressed against the registry.
- Runnable things are declared once, next to their owner, in a schema-first form the dashboard discovers.
- The TUI is a real terminal application: resize, hover, mouse (click, drag, wheel), text selection and
  clipboard, visible cursor, complete key forwarding, windows that are the terminal projection of the
  shared window design, tabs named after their task.
- Battle tested: every declared command resolves; pseudo-terminal end-to-end suites cover the whole flow.

## 2. Canonical Command Registry

### 2.1 Sources (nothing else is a command)

| Source | Maintained by | Discovered from |
| --- | --- | --- |
| Nx targets, their `continuous` flag and `configurations` | each `📋️project.json` | manifest walk + `.nx/workspace-data/project-graph.json` |
| Root workspace scripts that call Nx | root `package.json` | file |
| Playground variants (ports, examples, user slots) | plugin `[[package.metadata.semio.playground]]` | generated playground catalog |
| Dashboard declarations (below) | owner `📋️project.json` → `metadata.semio.dashboard` | same walk / graph |
| Ticket commands | `<ticket folder>/🎮️commands.json`, open tickets only | ticket index |
| Repo-domain actions (tickets, goals, analyze, tree, statutes) | Rust domain crates | in process |

### 2.2 Declarations (JSON Schema `🎛️dashboard/🧬️schema/🎮️registry/🔣️.json`)

Target level, `targets.<name>.metadata.semio.dashboard`:

```jsonc
{
  "verb": "dev",                                   // optional; default = leading word of the target name
  "ready": { "port": 6061, "portEnv": "TEACHING_ARCHITECTURE_QUIZ_PORT", "path": "/admin" },
  "requires": ["os-hub:dev"],                      // services that must be ready before this starts
  "parameters": [ Parameter, … ]                   // target-owned parameters
}
```

Project level, `metadata.semio.dashboard`:

```jsonc
{
  "parameters": [ Parameter, … ],                  // workspace root project only: the global axes
  "tools":     [ { "id": "mcp-inspector", "verb": "dev", "command": ["bun", "x", "…"], "cwd": "", "env": {},
                   "continuous": true, "ready": {…}, "requires": [], "parameters": [] } ],
  "compounds": [ { "id": "s-with-hub", "verb": "dev", "stop": "together",
                   "members": [ { "run": "os-hub:dev" }, { "run": "playground:s", "parameters": { "renderer": "react" } } ] } ],
  "groups":    [ { "id": "stdio-artifacts", "target": "test", "projects": ["@semio-tech/stdio-pdf-rs"] } ]
}
```

`<ticket>/🎮️commands.json` has the project-level shape restricted to `tools` and `compounds`.

```jsonc
Parameter = {
  "id": "test-level",
  "kind": "choice" | "text" | "flag",
  "appliesTo": { "verbs": ["test", "gate"], "playground": false },   // global axes only
  "default": "quick",                                               // absent: not applied unless chosen
  "required": false,
  // choice: each value carries its effect
  "values": [ { "id": "quick", "env": { "SEMIO_TEST_LEVEL": "quick" }, "nxFlags": [], "args": [] } ],
  // flag: effect when switched on
  "env": {}, "nxFlags": ["--excludeTaskDependencies"], "args": [],
  // text: where the typed value goes (exactly one)
  "valueEnv": "PRINT_NATIVE_GRAMMAR_PHASE", "valueFlag": "--handle", "valuePositional": true
}
```

Effect semantics: `env` is added to the process environment, `nxFlags` go before `--`, `args` after `--`.
Tokens in `command`, `cwd`, `env` values and `args`: `{workspace}` is the workspace root, `{port}` the
resolved ready port, `{<parameter id>}` the chosen value of a parameter. Paths are workspace-relative.
`ready` means: the task is ready when its output shows `http://(127.0.0.1|localhost|0.0.0.0):<port>`; the
ready URL is that match plus `path`. `port` may be omitted when `portEnv` names a parameter or environment
value. Members of a compound start in order, each after the previous member is ready when it declares
`ready`; `stop: "together"` stops all when one stops. `requires` starts (or reuses) the named commands and
waits for their readiness first. Member and `requires` references: `<project>:<target>[:configuration]`,
`playground:<variant>`, `tool:<project>/<id>`.

Global axes (root project): `cache` (use | skip-local | skip-all), `test-level` (quick | long |
exhaustive, canonical spelling `SEMIO_TEST_LEVEL`), `dependencies` (flag `--excludeTaskDependencies`),
`build-mode` (ship = `SEMIO_BUILD_MODE=ship` + skip-all), `nextest-output`, `cargo-jobs`, `build-budget`.
Playground parameters are built in: `renderer` (react | wgpu-wasm | wgpu-native), `example`, `user-slot`,
`app-role`. Every command also accepts free extra arguments (typed at launch, appended after `--`), which
replaces all stored test-filter / test-file / test-scope rows. Never stored: `NX_DAEMON`,
`NX_ISOLATE_PLUGINS`, `NX_CACHE_PROJECT_GRAPH`, `NX_TUI`, `FORCE_COLOR`, `NX_PLUGIN_NO_TIMEOUTS`, literal
tokens, absolute paths.

### 2.2.1 Amendments (coordinator decisions on `m1a-owner-declarations.md` §9; additive, binding)

1. `requires` entries and compound members share one shape: a reference string or
   `{ "run": "<ref>", "parameters": {…}, "env": {…} }`. `env` on a member is how a compound states a
   relationship between its members (for example the hub URL a shell joins).
2. No fixed `env` in a target declaration: environment that always applies belongs in the Nx target's own
   `options.env`. The dashboard block only describes what Nx cannot.
3. Groups name `targets: [..]` (one or more); the single-target `target` key does not exist. `projects`
   passes Nx patterns (`*`, `!name`) through unchanged.
4. `appliesTo.verbs` selects every command kind with that verb, tools included; `nxFlags` effects are
   ignored for commands that do not run Nx.
5. A launch accepts free extra environment (`semio run … --env KEY=value`, and a field in the launcher)
   next to free extra arguments.
6. `ready.printed: true` takes the whole printed URL (up to whitespace) as the ready URL instead of
   `match + path`.
7. Confirmed readings: `{workspace}` and parameter tokens expand in parameter defaults; `ready` with
   `portEnv` and no `port` reads the port from the resolved environment; group `projects` are Nx patterns.
8. Playground facts that today exist only in the launch seed (data directory per user slot, hub join,
   local-only, example lock, viewer role suffix) move into the playground registry metadata of their plugin
   with a reader in the catalog; they are not dashboard declarations (slice P-1, after L-S2 and A-1).

### 2.3 Identity And Labels

- Command id: Nx form `<project>:<target>[:configuration]`, `playground:<variant>`, `tool:<project>/<id>`,
  `compound:<project>/<id>`, `group:<project>/<id>`, `script:<name>`, `ticket:<id>/<tool id>`,
  `repo:<action key>`. Parameters are `key=value` pairs.
- Verb: declared `verb`, else the leading word of the target name if in the verb list, else `task`.
  Long-running = Nx `continuous: true`, `tools[].continuous`, playground dev, or a declared `ready`.
- `TaskLabel { verb, owner: [String], subject, qualifier, parameters: [(String, String)], members: u16 }`
  is computed by the registry at launch and travels with the session (daemon schema). Tab text and window
  title are pure functions of it (rule in `tui-chrome-parity-audit.md` §4.3: `verb subject [qualifier]`,
  24 cells, middle elision, status as glyph and colour role, ` ·2` for duplicates).

### 2.4 Non-Interactive Surface (same daemon, same tasks as the TUI)

`semio commands [words…] [--json]`, `semio run <id> [--param k=v]… [--detach] [--wait-ready] [-- args…]`,
`semio tasks [--json]`, `semio logs <task> [--follow]`, `semio stop|restart|kill <task>`,
`semio open <task>`, `semio daemon start|status|stop`. Exit code of an attached `run` is the task's.

## 3. Daemon Contract

Schema-first in `🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json`, then Rust.

- `TaskLabel` and `Ready` are wire types: their single home is `daemon::ipc` (daemon schema); the registry
  produces them.
- `SessionCommand` gains `command_id`, `label: TaskLabel`, `group: Option<String>`, `ready: Option<Ready>`.
- `SessionInfo` gains `started_ms`, `ended_ms`, `ready_url`, `title` (child OSC title), `group`.
- Ready detection runs in the daemon; views and `--wait-ready` consume `SessionChanged`.
- Compounds and `requires` are orchestrated by the daemon (`SpawnGroup`), including stop-together.
- Output: bounded in-memory ring per session for replay plus a per-session log file under the dashboard
  cache for scrollback and `semio logs`; replay restores the full visible screen and scrollback.
- Flow control instead of disconnect on backlog; PTY writes are queued and retried; child gets
  `TERM=xterm-256color`, `COLORTERM=truecolor`; the child environment is the requesting client's, not the
  daemon's; exit reports signal deaths as `128 + signal`.
- Views and daemon exchange a build id and protocol version on attach; a mismatch is shown, never silent.

## 4. TUI Framework Contract (`🧰️framework/🔨️modules/🖱️ui/⌨️tui/`)

One file per module after the split (`w0-tui-module-split.md` has the folder names).

### 4.1 Shared Types (landed before the parallel wave; do not redefine)

```rust
// event
pub enum MouseButton { Left, Middle, Right }
pub enum MouseKind { Down(MouseButton), Up(MouseButton), Drag(MouseButton), Move, Scroll { dx: i16, dy: i16 } }
pub struct MouseEvent { pub kind: MouseKind, pub pos: Pos, pub mods: u8, pub clicks: u8 }
pub enum Event { Key(KeyEvent), Mouse(MouseEvent), Paste(String), Resize(Size), FocusGained, FocusLost, Wake }

// widget
pub enum CursorShape { Block, Underline, Bar }
pub struct CursorSpec { pub pos: Pos, pub shape: CursorShape, pub blink: bool }
impl WidgetState {
    pub fn on_key(&mut self, event: &KeyEvent) -> Option<WidgetSignal>;
    pub fn on_mouse(&mut self, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal>;
    pub fn on_paste(&mut self, text: &str) -> Option<WidgetSignal>;
    pub fn set_hover(&mut self, rect: Rect, pos: Option<Pos>) -> bool;
    pub fn cursor(&self, rect: Rect) -> Option<CursorSpec>;
    pub fn interactive(&self) -> bool;
    pub fn tick(&mut self, now_ms: u64) -> bool;
}
pub enum WidgetSignal {
    Activated(usize), SelectionChanged(usize), ValueChanged(String), Toggled(bool), TabChanged(usize),
    NavigateBack, Hovered(Option<usize>), ContextMenu { pos: Pos, item: Option<usize> },
    Copy(String), OpenUrl(String), TerminalInput(Vec<u8>),
    WindowClose(usize), WindowMaximize, WindowNewTab, WindowTabActivated(usize), WindowFocus,
    TabMoved { from: usize, to: usize }, SplitterDragged { path: Vec<usize>, delta: i16 },
}

// engine
impl Tui {
    pub fn dispatch(&mut self, event: &Event) -> Vec<(NodeId, WidgetSignal)>;
    pub fn layout(&mut self);
    pub fn render(&mut self) -> AnsiPatch;
    pub fn cursor(&self) -> Option<CursorSpec>;
    pub fn hovered(&self) -> Option<NodeId>;
    pub fn capture(&mut self, node: Option<NodeId>);
    pub fn tick(&mut self, now_ms: u64) -> bool;
}

// backend
pub struct Capabilities { pub color: ColorDepth, pub synchronized_output: bool, pub unicode: UnicodeLevel }
pub trait TerminalBackend {
    fn enter(&mut self) -> io::Result<()>;
    fn leave(&mut self) -> io::Result<()>;
    fn size(&self) -> io::Result<Size>;
    fn capabilities(&self) -> Capabilities;
    fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()>;
    fn wait(&mut self, deadline: Option<Instant>) -> io::Result<Vec<Event>>;
    fn waker(&self) -> Waker;
    fn copy(&mut self, text: &str) -> io::Result<()>;
}
```

`TerminalInput` carries the bytes the embedded terminal encoded for its child (keys, mouse, paste, focus);
the host forwards them to the PTY. Passthrough mode reserves only the prefix key.

### 4.2 Slices And Ownership

| Slice | Owns | Delivers (audit ids) |
| --- | --- | --- |
| T-A terminal I/O | `ansi`, `backend`, `🪟️windows`, `host` | mode table, complete parser, resize, wake, panic and signal restore, paste decoding, colour depth quantisation, synchronized output, cursor emission, clipboard, (D01 D02 D06 D20 D22 D26 D27 D36, R01 R12) |
| T-B engine and windows | `event`, `scene`, `layout`, `engine`, `chrome`, `widget` plumbing, elements Window Tabs Navbar Footer Chip Divider Label and new overlay elements | dirty fix and incremental render, mouse pipeline (hover, capture, clicks, drag, wheel), focus tree, cursor API, overlay layer (menu, dialog, tooltip, palette), window anatomy, tab strip, splitter drag, zoom, titles on layout nodes (D07 D08 D09 D10 D19 D25 D32 D37, R02 R03 R06 R07 R08 R13 R14 R16 R17 R19 R23) |
| T-C embedded terminal | `vt`, terminal part of `widget` | key/mouse/paste/focus encoders by child mode, responses, missing sequences, absolute scrollback with anchoring, reflow, selection, cursor, theme colours, scroll bar data, no search trap (D03 D04 D05 D13 D14 D15 D16 D35, R04 R09 R15) |
| T-D text and lists | `text`, `cell`, `theme`, elements List Wizard Table Log Input Select and new Tree, Scrollable, Progress, Toggle | grapheme width model, wide cells, ellipsis, status roles and glyphs, list model with incremental filter for 50k rows, viewport, hover row, select-then-activate (D11 D12 D21 D28 D29 D33 D34, R05 R10 R11 R18 R20 R21 R22 R24) |

## 5. Dashboard Application Slices

| Slice | Owns | Delivers |
| --- | --- | --- |
| A-1 registry | `🎛️dashboard/🌳️command-tree`, `📚️inventory`, new `🎮️registry`, schemas, CLI verbs `commands` | §2: declarations reader, parameters, labels, ids, cache, no launch file reading |
| A-2 daemon | `🎛️dashboard/🌀️daemon`, `📎️connection`, daemon schema, UI module `🚇️pty`, CLI verbs `run tasks logs stop restart kill open` | §3, `dashboard-runtime-audit.md` P0-1 P0-4 P0-6 P1-3 P1-4 P1-10 P2-3, D17 D23 D31 |
| A-3 view | `🎛️dashboard/🖥️terminal`, `⚙️preferences`, locale strings | the application on the §4 API: keymap, focus, launcher (tree + search + parameters), tasks, settings, tab names, selection, context menus, resize propagation |
| M-1 declarations | every `📋️project.json` that gains `metadata.semio.dashboard`, ticket `🎮️commands.json` files | hand-migrated presets, ready ports, compounds, groups, axes (per `launch-inventory.md` §11) |
| L-1 launch removal | generator module, registry `📜️script.ts`, root `📜️script.ts` gates, tests, fixtures, docs, `.vscode`, `.claude` | per `launch-dependents.md` |
| V-1 battle tests | new end-to-end suites | every command resolves across the monorepo; PTY journeys; multi-view; load |

## 6. Rules For Every Execution Agent

1. Compile-atomic: after each edit batch run `cargo check -p <crate> --message-format=short` for the
   crates you touched and their direct consumers (`semio-framework-ui --features tui-terminal`,
   `semio-framework-repo-dashboard`). Never leave a crate broken while you do something else.
2. Builds and tests in the foreground, one at a time. `cargo test` and binary builds use
   `CARGO_TARGET_DIR=<repo>/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-<slice>`; `cargo check` uses the
   shared directories. No `--release` builds, no dashboard install, no daemon start/stop on the real
   workspace (a developer is using the running one). The coordinator owns release build and cutover.
3. Stay in your files. A peer's transient break outside them: wait and retry, then report; do not fix it.
4. Test-driven and schema-first: a `🥒️.feature` per feature with a fixture under `🧫️fixtures`, a Rust
   adapter, and an independent oracle (existing third-party library) producing the same output.
5. No git-modifying commands, no worktrees, no `AGENTS.md` edits, no ticket open/close, no sub-agents.
   No compatibility layers, deprecations or migration scripts in the codebase.
6. Reports go into this ticket folder; say "WRITTEN BUT UNVERIFIED" where that is the truth.
