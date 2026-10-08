# Round 2 Plan: Finish The Only Control Plane

Coordinator document for round 2. `fleet-plan.md` §1–§6 stays the binding target design; this file
records what round 2 found and who finishes what. Rules: `r2-fleet-brief.md` plus fleet-plan §6.

## 1. State Found (2026-10-08 audits)

| Area | Audit | State |
| --- | --- | --- |
| TUI interaction D01–D37 | `r2-audit-tui-interaction.md` | 1 fixed, 3 partial, 33 open; §4.1 types exist but are stubs |
| TUI chrome R01–R25 | `r2-audit-tui-chrome.md` | 0 fixed, 1 partial; labels dropped, keymap hard-coded, inline locale pairs |
| Daemon §3 + runtime P0–P2 | `r2-audit-daemon.md` | 0/12 §3 bullets done; `✉️ipc`, `🚚️transport`, `📼️replay` committed but not compiled; old inline ipc live |
| Launch dependents | `r2-audit-launch-dependents.md` | generator gone; 3 launch files orphaned; root gate, 13 tests/fixtures, ~25 docs remain |
| Registry + CLI | `r2-audit-registry.md` | pending |
| Config surfaces | `r2-audit-config-surfaces.md` | pending |
| Build | coordinator | `semio-framework-ui --features tui-terminal` failed on Windows (ConPTY `Send`), fixed by coordinator; dashboard `--all-targets` fails in command-tree unit tests |

## 2. Slices (round 2)

| Slice | Owner files | Delivers |
| --- | --- | --- |
| T-A terminal I/O | TUI `🔡️ansi`, `🔌️backend`, `🪟️windows`, `🏃️host` | fleet-plan §4.2 T-A; D01 D02 D06 D20 D22 D26 D27 D36, R01 R12; native resize (SIGWINCH + Windows console input buffer), waker wiring API, cursor emission in `present`, capability detection, UTF-8 paste with cap, focus reporting, `?1003h` motion, OSC 52 copy |
| T-B engine and windows | TUI `📡️event`, `🎬️scene`, `📏️layout`, `⚙️engine`, `🖥️chrome`, widget plumbing in `🪀️widget` (not the terminal widget), elements Window Tabs Navbar Footer Chip Divider Label, new overlay elements | §4.2 T-B; D07 D08 D09 D10 D19 D25 D32 D37, R02 R03 R06 R07 R08 R13 R14 R16 R17 R19 R23 |
| T-C embedded terminal | TUI `📟️vt`, terminal widget part of `🪀️widget` | §4.2 T-C; D03(terminal) D04 D05 D13 D14 D15 D16 D35, R04 R09 R15 |
| T-D text and lists | TUI text/cell/theme, elements List Wizard Table Log Input Select, new Tree Scrollable Progress Toggle | §4.2 T-D; D11 D12 D21 D28 D29 D33 D34, R05 R10 R11 R18 R20 R21 R22 R24 |
| A-1 registry + CLI | `🎮️registry`, `🌳️command-tree`, `📚️inventory`, `📇️playground-catalog`, `📜️root-delegation`, `🚪️entrypoint`, `⌨️usage`, `🧬️schema/🎮️registry`, `🧬️schema/🔣️.json` | registry is the only command source; command-tree becomes a projection of registry entries (or is dissolved into it); broken command-tree tests; §2.4 CLI verbs complete; ticket commands; README registry sections |
| A-2 daemon | `🌀️daemon/**`, `📎️connection`, `🧬️schema/🌀️daemon`, TUI `🚇️pty` | §3 complete; wire `✉️ipc`/`🚚️transport`/`📼️replay` and delete the inline copy; runtime audit P0–P2 |
| A-3 view | `🖥️terminal`, `⚙️preferences`, dashboard locale strings | the application on the §4 API: declarative customizable keymap, locale catalogue (en, de), TaskLabel tabs and titles, launcher on registry `Launch`, parameter form, resize propagation, single focus source, waker wiring |
| L-1 launch removal | root `📜️script.ts` interactivity regions, tests/fixtures B2–B13 of the dependents audit, Nx `inputs`, vite watch globs, `.gitignore:591`, devcontainer port law, the three launch files | dependents audit §4 steps 1–4 and 6 |
| L-2 docs | READMEs and comments of dependents audit group C | every doc points at the dashboard (`bun run dashboard`, `semio run <id>`) |
| C-1a agent-client canon | `.mcp.json`, `.vscode/mcp.json`, `.cursor/*`, `.codex/config.toml`, `.github/hooks/**`, `.claude/settings.json`, MCP/hook regions of root `📜️script.ts` | all MCP client files derived from root `metadata.semio.dashboard.tools` by one writer + drift check in the policy gate; working `semio` gateway folder; dead hook blocks and duplicates gone |
| C-1b editor/container/root canon | root `package.json`, `.vscode/settings.json`, `.vscode/extensions.json`, `.devcontainer/**` (not ports), `.github/dependabot.yml`, nextest copies, `🚚️migration.json`, settings policy (Go/Rust) | bootstrap-only root scripts; devcontainer editor settings derived from `.vscode`; broken paths fixed; dependabot real; lifecycle calls a real binary |
| V-1 battle tests | new suites | after A-1/A-2: every declared command resolves (`m1a-coverage.ts` expectations against `semio commands --json` / `semio run --dry-run`), PTY journeys on Windows ConPTY and Unix, multi-view, load |

## 3. Cross-Slice Interfaces (binding)

1. `TaskLabel` and `Ready` live in `🌀️daemon/✉️ipc` (A-2 moves them; `🎮️registry` imports them). Until A-2
   lands the move, A-1 and A-3 use `crate::registry::{TaskLabel, Ready}`; A-2 rewrites those imports.
2. `registry::Launch` is the only input of a start. A-2 provides
   `daemon::ipc::SpawnGroup::from_launch(&Launch, client_env) -> SpawnGroup` (one `SessionCommand` per
   `LaunchProcess` with `command_id`, `label`, `ready`, `group`); A-3 and the CLI `run` call it.
3. The view never constructs commands from strings; `command_tree::CommandSpec` stops being a start input.
4. TUI slices keep §4.1 signatures; additions are allowed, renames are not. A-3 consumes them as they land.
5. Root `package.json` keeps only `nx`, `setup`, `dashboard`, `dashboard:install`; the registry drops the
   root-script source and the `script:` id kind (amends fleet-plan §2.1/§2.3).
6. `.vscode/settings.json` and `.vscode/extensions.json` are the canonical editor settings; the devcontainer
   copies are derived. MCP client files and hook files are derived from one declaration each.
7. Kept as declared, flagged to the owner: `gemini --yolo` and `kiro-cli chat --trust-all-tools` tools,
   `Monorepo.sln`. `AGENTS.md` lines 50–51 contradict the goal; agents may not edit it (owner action).
8. Agents and editors start servers only through `semio run <id> [--param k=v] --detach --wait-ready`,
   which prints the ready URL; browser previews attach to that URL. No editor or agent launch file exists.
