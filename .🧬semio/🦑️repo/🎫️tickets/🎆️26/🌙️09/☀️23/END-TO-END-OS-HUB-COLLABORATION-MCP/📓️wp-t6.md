# WP-T6: Dashboard Extraction, Print Distribution Package, Gallery Measurement, Placement Rows, Contract Delta

Slice: T6 (session 10). Captures: `.tmp-ticket/wp-t6/generated/`. Ticket inputs: `.tmp-ticket/wp-t6/*`.
Inherits: T4 §1.3, §4, §5; T1 §3.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Dashboard crate extraction | **Done.** `cargo check --tests` clean (0 warnings in the crate), unit tests **29/29**, parity case `🌳️command-tree-projection` **2/2** (no-oracle decision, so parity 0/0); `semio` binary smoke-tested | `dashboard-check-1.txt`, `dashboard-test-1.txt`, `parity-dashboard-1.txt`, `cli-check-1.txt`, `semio-bin-1.txt` |
| 2. Print `semio-viz-charts-distribution.sty` | pending | |
| 2b. Print gallery measurement module (coordinator add-on) | pending | |
| 3. Placement rows (vitest 4, registration 4, hub 4, flow 2, stdio 1, depth 6, self-test 2) | pending | |
| 4. Test-platform suite + contract delta | pending | |

## 1. Dashboard extraction

Followed T4 §5, with one correction: the dashboard's own `🌀️daemon` and `🌳️command-tree` already superseded the cli copies, so only six modules moved and the terminal was restored.

- **Moved** from `🦑️repo/🎮️commands/` into `🔨️modules/🎛️dashboard/` (plain `mv`, unit tests travel with them): `🌊️workflow`, `🔌️plugin-registry`, `🛝️playground-development-session` → `🛝️playground-session`, `⌨️cli-usage-presentation` → `⌨️usage`, `📇️playground-catalog-query` → `📇️playground-catalog`, `📜️root-script-delegation` → `📜️root-delegation`. Each gained an emoji `//!` module doc.
- **Deleted** as superseded: `🖥️terminal-dashboard-daemon` (the dashboard `🌀️daemon` already owns the `run` command, with the Windows named pipe), `🌳️command-tree-discovery` (the dashboard `🌳️command-tree` is its superset with repo-domain leaves), the `🎮️commands/🔣️.json` collection and the directory itself.
- **Restored** `🖥️terminal/🦀️.rs` from `bb961413d4^:…/🎮️commands/🎛️terminal-dashboard/🦀️.rs` and ported it to the current tree: `CommandLeaf::Process` spawns into a PTY window as before; `CommandLeaf::Repo` opens an output window titled by `action_key` and feeds it `RepoAction::execute(root)` in process. Common window setup is one `open_output`. Both `[DEBUG]` lines of the old file were dropped.
- **cli** (`⌨️cli/🦀️.rs`, 910 → 33 lines) now owns only the verb dispatch and calls the dashboard modules directly (no re-exports). The root comes from `semio_framework_repo_workspace::find_repo_root` instead of a private copy. The new `command-tree` verb is wired, and the usage text (and its unit test) lists it. The cli's `🧪️tests/🔬️unit` duplicated the dashboard's unit tests over the deleted copies, so it was removed.
- **Taxonomy:** the 9 repo-only names were removed from `members-of-commands.memberNames`, and the stale `🎮️commands/🌳️command-tree-discovery` row was removed from `🧅️layering.json`.
- **Not in scope, still open:** the cli's `repo` binary (`📦️mcp-main.rs`) and its 8 Rust test adapters call `semio_framework_repo_cli::mcp_verb` / `repo_cli`, which have never existed. That is the unwritten Rust port of the repo CLI (T4 §1.4). `cargo check -p semio-framework-repo-cli --lib --bin semio` is clean.
- **Runtime:** `semio` (non-tty) prints the usage and exits 1; `semio command-tree` prints the discovered tree; `semio daemon status` reports "daemon not running"; `semio plugin registry check` reports the two missing generated catalogs (the registry has not been regenerated since the reboot).

## Processes (pids)

## Files changed (T6)
