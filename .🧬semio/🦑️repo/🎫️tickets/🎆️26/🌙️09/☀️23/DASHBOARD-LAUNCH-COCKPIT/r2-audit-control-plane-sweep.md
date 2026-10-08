# R2 Control-Plane Sweep (read-only)

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`, audit date 2026-10-08. Read-only: no daemon, builds, tests, dashboard installs or git-modifying commands. Method: `rg`/`grep` and reads through Bash.

- Scope: whole working tree except `.🧬semio/`, `node_modules`, `.cursor/plans/`, target dirs. `temp/` was included. `.venv` (third-party), `storybook-static`, `test-results` and `.nx` were skimmed only.
- Line numbers are as read during the audit. The tree was changing while I worked (`📋️project.json` moved 10 lines), so re-check before editing.
- Raw search dumps: `T/🗑️generated/r2-sweep-*.txt` and `r2-md-launch.txt`.
- Items marked UNVERIFIED were not run, only read.

## Verdict

The launch-file layer is gone. No `launch.json`, `launch.seed*`, `tasks.json`, `*.code-workspace`, `.idea/runConfigurations`, Procfile, Makefile/justfile or dev compose service exists in the tree. No code reads the removed files. Root `package.json` scripts are exactly `nx`, `setup`, `dashboard`, `dashboard:install`.

What still lets a developer or agent start a process outside the dashboard:

1. Root-script `start` and `dev` routes, which start processes outside the daemon (rows 2 and 3).
2. Docs that teach `bun nx run … :dev` or `bun ./📜️script.ts serve` as the way to start a server (rows 7–11).
3. Verb drift between the USAGE text, the dispatch and the README (row 6).
4. Dead editor-extension process code (row 13).
5. `AGENTS.md` lines 50–51, which contradict the goal (row 1, owner action).

## Findings

| # | file:line | finding | severity | suggested fix |
| --- | --- | --- | --- | --- |
| 1 | `AGENTS.md:50-51` | "All devs are using `launch.json` and never use the cli." and "register all executable commands there". Contradicts the goal. `CLAUDE.md`, `GEMINI.md` and `.github/copilot-instructions.md` are links to AGENTS.md (`📜️script.ts:194-199`, `:270`), so agents read the same lines through them. | blocker (owner action; agents may not edit) | Owner replaces both lines with: the dashboard (`bun run dashboard`) and `semio run <id>` are the only control plane, and no launch file exists. |
| 2 | `📋️project.json:1203-1209` (`workspace:start`, command at `:1207`) → `📜️script.ts:310` → `📚️library/🟦️.ts:1548-1556` | Root `start` verb runs `spawnDaemon("bun", ["nx","run","@semio-tech/framework-os-dev:local-hub"], {stdio:"ignore"})`. The `detached` flag is set on non-Windows. The child is outside the daemon: it does not appear in `semio tasks` and `semio stop/kill` cannot reach it. `README.md:879` names `workspace:start` a dashboard command. | blocker | Delete the detached spawn. Model the hub as a registry service (`requires`) or as a daemon-run target, and keep `S_LOCAL_ONLY` as a declared parameter. |
| 3 | `🧰️…/🎛️dashboard/📜️root-delegation/🦀️.rs:9` (ROOT_VERBS contains `start`, `dev`), `:15-21` (forwards to `bun ./📜️script.ts`); `📜️script.ts:325-341` (`DevScript`: `s`, `storybook`, `storybook-static`, `mcp`, playground apps) | `semio start` / `semio dev …` forward to the root script. Invoked directly, `dev s|storybook|<playground>` runs dev servers in the foreground, outside the daemon and outside `semio run`. `README.md:107` documents route forwarding as normal behaviour. The registry commands `workspace:dev` (`📋️project.json:1210`) reach the same code, but only when they run as daemon tasks. | blocker | Owner decision. Drop `start` and the server branches of `dev`, since the Nx targets already exist as dashboard commands. Keep only MCP stdio (row 16). Remove `start`/`dev` from ROOT_VERBS once done. |
| 4 | `🧰️…/🧑‍💻dev/🔌️vite-plugins/🟦️.ts:1411` | Second writer of the `dev mcp stdio os` launcher, in the agent-credential install plugin. | info | Derive the launcher text from the same `📋️project.json` `tools` declaration as the MCP client files. |
| 5 | `🌊️workflow/🦀️.rs:253-254` (`Command::new("cursor-agent").arg("-p")`, also `:267`); dispatched at `📦️packages/🦀️rust/🦀️.rs:66`; nx target `📦️packages/🦀️rust/📋️project.json:132-137` | `semio workflow run` starts coding-agent processes directly, outside the daemon. It is absent from USAGE and from the README verb table. | should-fix | Either route the runners through the daemon as registry commands, or remove the verb. Document it if it stays. |
| 6 | `⌨️usage/🦀️.rs:5` (USAGE) vs `📦️packages/🦀️rust/🦀️.rs:62-74` (dispatch) vs `🎛️dashboard/README.md:86-94` (verb table) | Dispatch has `dashboard`, `preferences`, `repo-view` (`:64`), `workflow` (`:66`), `command-tree` (`:68`), `plugin registry` (`:69`), `catalog`. USAGE lacks `workflow` and `repo-view`. The README table lacks `dashboard`, `preferences`, `catalog`, `command-tree`, `plugin registry`, `workflow`. USAGE lacks `stop\|restart\|kill --group` (`🧭️cli/🦀️.rs:346`) and `daemon serve` (`🌀️daemon/🦀️.rs:52`, internal). `repo-view` has no documentation; repo-wide it appears only in dispatch and tests (UNVERIFIED that the TUI is its only caller). | should-fix | Make the README table, USAGE and dispatch match. Document or remove `workflow`. Document `repo-view` as internal, or delete it if nothing calls it. |
| 7 | `README.md:148-151` (Getting started, browser shell) | Starts the shell with `bun nx run …:activate-s-react-dev` and `S_OS_PORT=6060 bun ./📜️script.ts serve s react dev`. The second line runs the package script `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` directly, outside `semio run`. | should-fix | Keep activation as `semio run @semio-tech/framework-os-dev:activate-s-react-dev`. Start with `semio run playground:s --param renderer=react --detach --wait-ready`. |
| 8 | `README.md:170-171` | "`bun nx run os-hub:dev-secure-suite`" and "`bun nx run os-hub:dev`". `README.md:666` already gives `semio run os-hub:dev --detach --wait-ready`, so the README contradicts itself. | should-fix | Use `semio run os-hub:dev-secure-suite --detach --wait-ready` and `semio run os-hub:dev …`. |
| 9 | `🌎️hub/README.md:114-115` ("As the dev loop"), `:213-214` (`bun nx run os-hub:dev-postgres`) | Dev servers started with `bun nx run`. | should-fix | Same `semio run` form. Keep the `(dashboard commands …)` note. |
| 10 | `🎓️teaching/🏛️architecture/❓️quiz/README.md:49-56` (command table) | Commands listed as `bun nx run @teaching/architecture-quiz:dev`, `:dev-site`, `:test-e2e`, `@teaching/proctor:dev`. `:44` already says `semio run <command id>`. `🎓️teaching/🛂️proctor/README.md:282-287` has the same table, but `:282` states these are dashboard commands. That one is acceptable. | should-fix (quiz README); info (proctor README) | Make the quiz table column `semio run <id>` and keep `bun nx run` only as the id. |
| 11 | `🧰️…/🐾️pets/README.md:62`; `🎓️…/🐾️pets/README.md:358-361` | `:62` lists `bun nx run @semio-tech/pets-react:dev` as the stories gallery command. `:358-361` already offers `semio run`. | should-fix (`:62`) | Use `semio run @semio-tech/pets-react:dev --detach --wait-ready`. |
| 12 | `.vscode/settings.json:111-116`; copy `.devcontainer/devcontainer.json:275-277` | LaTeX Workshop recipe `semio-nx` runs `bun nx run @semio-tech/mit-bestand-bericht:build` directly. This is a build, not a server, but it is an editor starting a process outside the dashboard. | should-fix | Canonical: `"command": "semio", "args": ["run", "@semio-tech/mit-bestand-bericht:build"]`. Owner decides how `semio` reaches PATH. Regenerate the devcontainer copy with `workspace:generate-config` (`📜️script.ts:25966` says the copy is derived). |
| 13 | `💻️client/🧩️vscode/🟦️.ts:1658-1664` (`getRepoBinaryPath` probes `<root>/repo/cli/cli`); `:1686-1690` (`hasRepoAccess`); `:1734-1751` (`runRepoCommand`: creates a terminal, sends the binary and args) | No `repo/` folder exists at the root, so `hasRepoAccess()` is always false and `runRepoCommandJson` (analyze, `:2398`) returns null silently. `runRepoCommand` has zero callers. It is a terminal-based process starter. | should-fix | Delete `runRepoCommand`. Remove the `repo/cli` probe. Route analyze through a real CLI, or delete it. |
| 14 | `🧰️…/🎛️dashboard/🧫️fixtures/🗺️coverage/🔣️.json:2` (1073 rows: 1023 `origin:"vscode"`, 46 `"claude"`, 4 `"compound"`); only consumer `🧪️tests/🗺️coverage/🔮️oracle/🟦️.ts` | The fixture is "frozen over the retained rows of the removed .vscode/launch.json … and .claude/launch.json". Nothing imports the oracle; it appears only in an `@see` at `:7`. Orphaned migration artefact. | should-fix | Delete fixture and oracle (dependents audit L-1), or re-derive expectations from `semio commands --json` if a coverage check is still wanted. |
| 15 | `git status` (unstaged): `D .claude/launch.json`, `D .vscode/launch.json`, `D .vscode/🧩️launch.seed.jsonc` | Files are deleted in the worktree, but the index still tracks them. | should-fix | Coordinator stages the removal (`git add -u` on those paths). This audit ran no git-modifying command. |
| 16 | `.mcp.json:1-30`; `.vscode/mcp.json:1-31`; `.cursor/mcp.json:1-29`; `.codex/config.toml:2-3`, `:8-9` | Clients launch `bun ./📜️script.ts dev mcp stdio <client>` directly. These files are derived from `📋️project.json:184-265` (`tools[].mcp.clients`) and drift-gated (`📜️script.ts:17397-17425`), so C-1a derivation is in place. Stdio MCP is spawned by the client itself, so `semio run --detach` cannot replace it. | info (owner should confirm the exception) | Keep. State the stdio exception in the dashboard README (`🎛️dashboard/README.md:12`). |
| 17 | `.claude/settings.local.json:2-11` (git-ignored, `.gitignore:3`) | `enabledMcpjsonServers` lists `neo4j-semio`, `neo4j-elements`, `neo4j-coda`, `neo4j-reuse`, `neo4j-metabolism`, `neo4j-extra`. No `.mcp.json` defines them. `enableAllProjectMcpServers: true`. | info (local file) | Owner prunes locally. No repo change. |
| 18 | `🧑‍💻dev/🔌️vite-plugins/🟦️.ts:1000` ("launch-config rewrite"), `:1009` (`.vscode` in unwatched list) | Stale wording. `.vscode` still exists, so the entry stays. | info | Reword to "editor settings rewrite". |
| 19 | `📜️script.ts:315` (log: "every `dev s` row signs in…"); `🎛️dashboard/🧭️cli/🦀️.rs:244` and `🎮️registry/🦀️.rs:1423` (`launch_json`, which produces the `--dry-run` plan JSON) | Launch vocabulary is left over. No launch.json file is produced. | info | Rename `launch_json` → `launch_plan_json`. Reword the log line. |
| 20 | `.codex/plans/declarative-actions.md:52` | `bun nx run @spatial/js-core:test`, `@spatial/js-kernel-brepjs:test`. No `@spatial/*` project exists in the tree (grep empty). Stale plan. | info | Delete the plan, or leave it as history. Owner's choice. |
| 21 | `README.md:131`, `:145`, `:842` | `npm run setup`. It works, because the root `setup` script exists, but AGENTS requires bun. | should-fix (AGENTS rule, small) | `bun run setup`. |
| 22 | `.agents/skills/clean/SKILL.md:39` (`clean stray-processes`) | Kills processes by executable name (`bun`, `nx`, `vite`, `cursor-agent`, …). It may take down daemon-supervised tasks. UNVERIFIED whether it spares the daemon and its task tree. | should-fix (UNVERIFIED) | Exclude the daemon PID tree and `.🧬semio/…` tasks before killing. |
| 23 | `.devcontainer/devcontainer.json:41-42` (`postStartCommand`, `postAttachCommand` → `setup devcontainer start\|attach`, `📜️script.ts:234-244`); lifecycle `🔁️lifecycle/🟦️.ts:320-325` spawns Xvfb and GitKraken detached | Container infrastructure and GUI tools, not dev servers. | info | None. |
| 24 | `.devcontainer/docker-compose.yml:15` (`sleep infinity`); `🌎️hub/compose.yaml`; `🎓️…/❓️quiz/🚀️deploy/compose.yaml` | Infra and production stacks. No dev services. | info | None. |
| 25 | `.gitignore:592` (`!.vscode/*.code-workspace`) | Negation for a file type that does not exist. | info | Drop on the next gitignore edit. |

Items 2, 3 and 13 are the only ones that start a process the dashboard cannot see or stop.

## Checked and clean

- Item 1: no `launch.json`, `launch.seed*`, `tasks.json`, `*.code-workspace`, `.idea/runConfigurations`, Procfile, Makefile or justfile with dev targets, or dev compose services in the tree. The only Makefile is a third-party template under `.venv/` (excluded).
- Item 2: outside `.cursor/plans/`, no `preview_start`, `launch row|entry|configuration`, `Run panel` or `.vscode/launch` mentions. `bun run dev:*` appears only in `.cursor/plans/` and one test string (`🧼️workspace-cleanup-stray-processes/🟦️.ts:22`). The dashboard README (`:14-15`, `:40`) is correct.
- Item 3: no code reads `.vscode/launch.json`, the seed or `.claude/launch.json`. The `scripts[...]` lookups in `📚️library` tests target package manifests, not the root.
- Item 5: `.agents/skills/*/SKILL.md` start no servers. `.github/hooks/repo.json` and `.cursor/hooks.json` are `{}`. `.claude/settings.json` has `"hooks": {}`.
- Item 6: `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` and `.github/copilot-instructions.md` are the only files with the launch wording; the last three are links that `setup git` creates. `temp/` is clean. The USAGE test (`⌨️usage/🧪️tests/🔬️unit/🦀️.rs:10`) asserts USAGE has no "launch".
- `script:` command kind: intentionally dropped. `🎮️registry/🧪️tests/🔬️unit/🦀️.rs:38` now treats `script` as an ordinary project name.

## Not run (UNVERIFIED)

- `semio --help` against USAGE, `semio commands --check`, and the MCP drift gate (`bun ./📜️script.ts agents check`). Run these before closing.
- The coverage oracle and `repo-view` callers outside the dashboard crate.
- Behaviour of `clean stray-processes` (row 22).
