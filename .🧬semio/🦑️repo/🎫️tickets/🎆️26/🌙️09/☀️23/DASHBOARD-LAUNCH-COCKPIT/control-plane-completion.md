# Dashboard Control Plane Completion

## Reported

Build, test and publish tasks were not showing, e.g. the mit-bestand Zwischenbericht build.

## Findings (measured on the real workspace, macOS)

1. **Keyboard dead on macOS.** `NativeTerminal::poll` used `poll()` on `/dev/tty`, which macOS does not
   support. No key ever reached the dashboard: Enter on "New task" did nothing, so the launcher and every
   task were unreachable. Reproduced through a pseudo-terminal (`drive-dashboard.py`).
2. **`.vscode/launch.json` was never read.** All 4,569 configurations (every one `node-terminal`) and the
   4 compounds — the registry developers actually use — were absent.
3. **Targets were filed under their raw name.** Discovery produced 5,413 top-level entries
   (`build-zwischenbericht`, `activate-cad-react-dev`, …) instead of verbs, and they only appeared after the
   ~46 s workspace walk.
4. **Not zero-touch.** No installation record existed; `bun run dashboard` failed with "not installed".
5. **`Ctrl+B Q` lost the shutdown.** The view exited before its writer thread flushed, so the daemon survived.

## Changes

- `🖱️ui/⌨️tui/🦀️.rs`: terminal input waits with `select()`.
- `🎛️dashboard/🌳️command-tree/🦀️.rs`: verb filing (`VERBS`, `task` fallback) for targets, inferred targets
  and scripts; launch configurations and compounds as a discovery source available in the seed (first
  frame); JSONC tolerance; `${workspaceFolder}`, `${env:…}`, defaulted `${input:…}`; direct exec for plain
  argument lists, platform shell otherwise; `CommandLeaf::Compound`.
- `🎛️dashboard/🖥️terminal/🦀️.rs`: compound activation; `Ctrl+B Q` waits for the daemon's confirmation.
- `🎛️dashboard/📚️inventory/🦀️.rs`: snapshot version 2 (tree shape changed).
- `🎛️dashboard/🧬️schema/🔣️.json`: `CompoundLeaf`.
- `⌨️cli`: first start builds and installs when no installation record exists; the bootstrap fast path is
  taken only when installed.
- Tests: `🧫️fixtures/🚀️launch-configurations` (+ feature), Rust unit test, independent `jsonc-parser`
  projection, native pseudo-terminal launcher test.

## Evidence

- Real workspace `command-tree --dump-tree`: 43,913 commands under 24 verbs + repo domain
  (build 1,153 · test 8,211 · publish 26 · gate 2,314 · dev 12,165 · task 1,467 …), 4 compounds.
- Search `build mit bestand zwischenbericht` → launch configuration, workspace script and Nx target.
- Real TUI in a pseudo-terminal on the real workspace: launcher opens, the search lists the Zwischenbericht
  build 10 s into discovery (seed).
- Fixture workspace through the real TUI: launch configuration ran with its environment through `sh -c`
  to `exit 0`; compound started both members.
- `cargo test -p semio-framework-repo-dashboard --lib`: 53 passed, 0 failed (before the native test was
  added); `actual_cli_launcher_receives_keys_and_runs_a_launch_configuration` (ignored by default, needs
  `SEMIO_TEST_CLI`) passed against the debug binary; control-plane TypeScript suite 7 passed.

## Not covered

- Launch configurations needing an input without a default (56 inputs, mostly `projectTarget.*` pick
  lists) are not offered; the same project targets are reachable as Nx targets.
- `serverReadyAction` (opening the browser) is not performed.
- Full discovery still takes ~46 s under the current machine load; scripts, launch configurations and
  playground entries are available immediately and the result is cached for the next start.
