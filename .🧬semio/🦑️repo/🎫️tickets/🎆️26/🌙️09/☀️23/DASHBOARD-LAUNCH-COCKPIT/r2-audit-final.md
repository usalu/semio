# Round 2 Final Audit: Dashboard As The Only Control Plane

Date 2026-10-08. Read-only audit by a fresh agent. No git-modifying command, no daemon start or stop, no `run` without `--dry-run`, no edits to source or `AGENTS.md`. Generated output (command results, dumps) is in `🗑️generated/` (`a1-*`, `a3-*`, `a5-*`, `a6-*`). Reports read first: `r2-fleet-brief.md`, `r2-plan.md`, the newest-on-top sections of all `r2-slice-*.md`. Slice reports are treated as claims; each item below was checked against files and commands.

Paths: `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. Binary under test: `⚡️cache/cargo/target-fleet-v1/debug/semio.exe` (built 08:25).

## Summary

| # | Item | Result |
| --- | --- | --- |
| 1 | No launch files, no readers | FAIL (3 launch files still in the git index as unstaged deletions; working tree is clean) |
| 2 | Root scripts, removed-script references, server-start docs | FAIL (root scripts PASS; docs still show `bun ./📜️script.ts dev mcp stdio os` as a runnable command) |
| 3 | `semio` CLI checks | PASS |
| 4 | USAGE vs README verb table vs dispatch | FAIL (`dev` still a forwarded route; `--help` documented wrong; flag drift) |
| 5 | MCP client and devcontainer derivation checks | PASS |
| 6 | Docstring and rules spot check | FAIL on uniqueness only (emoji-first, no `[DEBUG]`, no comments in bodies all PASS) |
| 7 | AGENTS.md contradictions (report only) | FAIL (lines 50-51, owner action) |
| 8 | Other ways to start processes outside the dashboard | FAIL (direct `script.ts dev|serve`, Nx targets, package `dev` scripts, `semio dev` route) |

## 1. No launch files exist, and no code reads them

Result: FAIL. Working tree: PASS. Index: FAIL.

- Filesystem: `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.claude/launch.json` do not exist. `ls -la .vscode .claude` shows only `extensions.json`, `mcp.json`, `settings.json` and `settings.json`, `settings.local.json`. A `find` for `launch.json`, `*launch.seed*`, `tasks.json` outside `node_modules` and `.🧬semio` returned nothing.
- `git ls-files` still lists all three files. `git status` shows them as ` D` (deleted in the working tree, deletion not staged). They stay FAIL until the deletion is staged or committed.
- `git grep -n -i -E 'launch\.json|launch\.seed' -- . ':!.🧬semio' ':!.cursor/plans'` gives 3 hits, all classified:
  - `AGENTS.md:50`: "All devs are using `launch.json` and never use the cli." Contradicts the goal. See item 7.
  - `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154-155`: two ledger rows naming the removed files. Kept on purpose as hash-pinned deletion evidence (`r2-slice-l1.md:11,45`). Accepted exception. Note: the reason text at line 152 says those rows "leave the catalog", which the rows themselves contradict.
- Name-based sweep (`🧩️launch`, `launch.jsonc`, `.claude/launch`, `vscode/launch`) finds no further hits.
- Code readers: none. The only code mention is the assertion that the file is absent, `D/⌨️usage/🧪️tests/🔬️unit/🦀️.rs:10`. The README line `D/README.md:12` states that none exists.
- `.cursor/plans` (excluded by the brief) contains launch mentions; not classified here.

Action for owner: stage the three deletions (`git add -A` of those paths) when committing.

## 2. Root scripts, removed-script references, server-start docs

Result: FAIL (docs). Root package scripts: PASS.

- `package.json:29-34`: exactly `nx` (30), `setup` (31), `dashboard` (32), `dashboard:install` (33). PASS.
- `git grep -n -o -E 'bun run [A-Za-z0-9:_./@-]+'` (outside `.🧬semio`, `.cursor/plans`) found only:
  - `dashboard`, `setup`, `dashboard:install`: all exist in `package.json`.
  - `test`, `lint`, `format`, `generate`: only inside frozen document fixtures (`✏️s/…/📖️readme.md:716`, `…/🏷️.xml:2`). Document content, not dev guidance.
  - `dev`: only as a process command string in the cleanup fixture and its test (`🧫️fixtures/🧼️workspace-cleanup-stray-processes/🔣️.json:8`, `🧪️tests/…/🟦️.ts:23`).
  - `dev:block:2d`: inside a ticket fixture JSON.
  - `dashboard:start|status|stop|preferences`: no remaining hits.
- `bun nx run …` in docs: only builds, `workspace:setup`, and CI (`.github/workflows/architecture-quiz.yml`). No doc tells devs to start servers with `bun nx run`.
- FAIL, `bun ./📜️script.ts dev` documented as a runnable command:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md:11`: `bun ./📜️script.ts dev mcp stdio os   # what .mcp.json launches`.
  - same file `:13`: a flags example, and `:133`.
  - root `README.md:686`: the same command in the MCP servers row.
  - Classification: stdio MCP client launch documented as a manual command. Counted as FAIL until the doc points to the dashboard or states that only clients run it.
- Comment only: `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts:4` mentions `script.ts dev storybook-static` (test server).
- Not a dev guide: `.codex/plans/declarative-actions.md:52` `bun nx run @spatial/js-core:test` (plan document, test target).

## 3. `semio` CLI

Result: PASS. Exit codes and outputs in `🗑️generated/a3-*.txt`.

- `semio --help`: printed the dashboard keyboard help, not the usage text. This is recorded under item 4.
- `commands --check --root /c/git/semio`: `26573 commands, 0 problems`, exit 0 (`a3-commands-check.txt`). The run exceeded 120 s.
- `commands build mit bestand --root /c/git/semio`: six build targets listed (`@semio-tech/mit-bestand-…:build` variants), exit 0 (`a3-build-mit.txt`).
- `run playground:s --param renderer=react --dry-run`: exit 0. One process, `bun nx run @semio-tech/framework-os-dev:dev-s-react-dev`, `ready.port` 6070, `stop: independent`, `S_DATA_DIR` set (`a3-run-playground.txt`).
- `run @teaching/architecture-quiz:dev --param stack=beside --dry-run`: exit 0. One process, `bun nx run @teaching/architecture-quiz:dev`, `ready.port` 6063, `PROCTOR_PORT` 8793 (`a3-run-quiz.txt`).

Not run: the cargo test suite of the dashboard (UNVERIFIED in this audit).

## 4. USAGE vs README verb table vs dispatch

Result: FAIL.

Agreed (PASS):
- Native verbs: `NATIVE_VERBS` at `📦️packages/🦀️rust/🦀️.rs:55` (dashboard, preferences, daemon, catalog, command-tree, plugin, commands, run, tasks, logs, stop, restart, kill, open; `repo-view` internal). USAGE at `⌨️usage/🦀️.rs:5` lists all of them. README verb table `README.md:104-115` lists all of them.
- Dispatch `📦️packages/🦀️rust/🦀️.rs:64-78`: every native verb is matched; anything else goes to `root_delegation::run`.

Disagreements (FAIL):
- `dev` is still a forwarded route. `📜️root-delegation/🦀️.rs:12` `ROOT_VERBS` contains `dev` and `stdio`, forwarded to `bun` at `:24-25`. USAGE `⌨️usage/🦀️.rs:5` lists "forwarded to the root script: … dev (storybook, mcp)". README `:124` says "The retired `semio dev <variant>` is `semio run playground:<variant>`", and `:132-134` lists route verbs as forwarded. The README calls `dev` retired while the binary still forwards it. The root script still registers `DevScript` (`📜️script.ts:15202`, implemented at `:248-264`, starts storybook and mcp).
- `semio --help` is documented wrongly. README `:98`: "`semio --help` prints the usage". Actual: `--help` is flag-first, so it goes to the dashboard (`📦️packages/🦀️rust/🦀️.rs:83-88`), which prints the keyboard help (`a3-help.txt` line 1: `semio [dashboard] [--root PATH] …`). A unit test asserts that behaviour (`D/🧪️tests/🔬️unit/🦀️.rs:55`). The README is wrong, not the code.
- Flag drift between README and USAGE (USAGE omits):
  - `run --timeout S` (README `:105`; parsed at `🧭️cli/🦀️.rs:305`).
  - `stop|restart|kill --group` (README `:108`).
  - `catalog --json` (README `:113`; parsed at `📇️playground-catalog/🦀️.rs:10`).
  - `dashboard --language L` (README `:111`).

## 5. MCP client files and devcontainer derivation

Result: PASS.

- `bun ./📜️script.ts agents check`: `[agents check] every client file matches the root declaration`, exit 0 (`a5-agents-check.txt`).
- `bun ./📜️script.ts generate config --check`: `[generate config] derived configuration is current`, exit 0 (`a5-generate-config-check.txt`).
- Not verified: that the checks fail on drift (read-only audit; no files changed).

## 6. Docstring and rules spot check

Result: FAIL on uniqueness. Emoji-first, no `[DEBUG]`, and no comments in bodies all PASS.

Sample: 10 random changed `.rs` files (`🗑️generated/a6-sample10.txt`). The full set of 50 changed `.rs` files under `D` and `⌨️tui` was then checked (`a6-paths.txt`, `a6-all-issues.txt`). Six changed files are deleted in the working tree and were not evaluated.

- `[DEBUG]`: 0 hits in all 50 files.
- Non-doc `//` comments: 0 in all 50 files, apart from `// #region` and `// #endregion` markers. No block comments.
- Emoji-first: every doc block start carries an emoji. The `#️⃣` keycap counts as an emoji. Lines flagged by a naive check were paragraph or `@see` continuations (for example `🔌️backend/🦀️.rs:1128`, `🌀️daemon/📼️replay/🦀️.rs:5`).
- FAIL, uniqueness: a leading emoji repeats within a file, in 5 of 50 files:
  - `📝️text/🦀️.rs`: 7 repeated leading emoji.
  - `✉️ipc/🦀️.rs`: 3.
  - `🚇️pty/🦀️.rs`: 2. `🦇` at lines 331, 639, 1005, 1034; `🏁` at 309, 328, 999 (several are Unix and Windows variants of one API).
  - `🎮️registry/🦀️.rs`: 2.
  - `🔲️cell/🦀️.rs`: 1.
- The rule is ambiguous (per file or per crate). Counted as FAIL per file.

## 7. AGENTS.md lines that contradict the goal (report only)

Result: FAIL. Owner action. Agents may not edit `AGENTS.md`.

- `AGENTS.md:50`: "All devs are using `launch.json` and never use the cli."
- `AGENTS.md:51`: "You MUST register all executable commands there by following the existing order, grouping and naming." ("there" = `launch.json`.)
- No other tracked instruction file contradicts the goal: `.agents/**`, `.codex/**`, `.cursor/**`, `.github/**`, and the nested `AGENTS.md` files were scanned for `bun run`, `bun nx run`, `script.ts dev|serve|start`, `launch`, `vscode`.
- Plan §3.7 already flags `AGENTS.md:50-51` as owner action.

## 8. Other ways to start processes outside the dashboard

Result: FAIL. Verified direct paths:

a. `semio dev <route>` forwarded to the root script. `📜️root-delegation/🦀️.rs:12,24-25`. Root `DevScript` is still registered (`📜️script.ts:15202`) and starts `storybook` (`:310`), `storybook-static` (`:13853`) and `mcp`.

b. Nx targets that start long-running processes, runnable with `bun nx run <project>:<target>` or `bun run dev` in the project:
  - root `📋️project.json:1231-1524`: `bun ./📜️script.ts dev storybook …` (many variants), `dev mcp`, `dev mcp repo`.
  - `🌎️hub/📦️packages/🦀️rust/📋️project.json:1148-1297`: `dev postgres`, `dev neo4j`, `dev secure-suite|native|mcp|admin`.
  - `♻️mit-bestand/🧺️demonstrator/📋️project.json:84,175,208`: `bun ./🔨️modules/🧩️runtime/📜️script.ts serve` and `serve-test`. The runtime script registers `serve` (`📜️script.ts:81`).
  - `🏢️semio-tech/🎡️play/📋️project.json:185,214,282`: the same `serve` pattern (`🧩️runtime/📜️script.ts:75`).
  - `🧰️…/🧩️vscode/📦️packages/🟦️typescript/📋️project.json:17`: `bun ./📜️script.ts dev`.
  - generated playground targets `serve-<variant>-react-<profile>` and `dev-<variant>-react-<profile>` in `📚️library/🟨️.mjs:1206-1223`.

c. Package-level `"dev": "bun nx run …"` scripts, startable with `bun run dev` in the package directory (5 files): `♻️mit-bestand/🧺️demonstrator/package.json:20`, `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json:9`, `🏢️semio-tech/🎡️play/package.json:20`, `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/package.json:12`, `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/package.json:9`.

d. MCP client launches. `.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json` and `.codex/config.toml` start stdio servers through `bun ./📜️script.ts dev mcp stdio <kind>` (`📜️script.ts:17296`; the README at `🌉️mcp/README.md:11` says what `.mcp.json` launches). These are started by clients, not by devs. Counted as a start path outside the dashboard; owner decision whether MCP clients are in scope.

e. Devcontainer `postStartCommand` and `postAttachCommand` (`.devcontainer/devcontainer.json:41-42`) run `setup devcontainer start|attach`. `setup devcontainer gitkraken` starts GitKraken Desktop and Xvfb on demand (`.devcontainer/README.md:33`). GUI helpers, not dev servers. Noted, not counted.

f. UNVERIFIED: `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:855,864` asserts generated targets run `bun ./📜️script.ts serve <variant> react <profile>`. The root script registers no `serve` route (`📜️script.ts:15197-15233`). Whether the generator still emits that command was not confirmed.

Confirmed PASS paths: `.vscode/settings.json:115` (LaTeX Workshop recipe) runs `bun run --cwd … dashboard run …`. The VS Code extension contributes repo-domain commands only and starts no servers. `.vscode/` has no tasks or launch files. `.cursor/hooks.json`, `.claude/settings.json` and `.github/hooks/repo.json` have empty `hooks`.

## Not verified in this audit

- Cargo test suites (dashboard usage drift test, daemon, TUI). The slice reports' test claims were not re-run.
- That `agents check` and `generate config --check` fail on drift.
- Whether the `serve` targets in item 8f are live.
- `.cursor/plans` (excluded by the brief).
- `Monorepo.sln` (plan §3.7 flags it as kept, not checked).
