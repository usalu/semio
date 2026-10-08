# Round 2 Slice S-1: Stragglers And Zero-Touch Invocation

Date 2026-10-08. Findings handled: sweep #2, #4, #7-#13, #18, #21, #22 and coordinator decisions A-H. `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`.

## A. Zero-touch invocation

- `semio …` = `bun run dashboard …` is stated once where the CLI is introduced: root `README.md` ("four golden paths", plus the sketchpad getting-started) and `D/README.md` ("The Command Line"). Agent-facing prose (`README.md` agents paragraph, `D/README.md` agents paragraph, quiz, pets, proctor, hub READMEs) now says `bun run dashboard run …`. `.agents/skills/**` and the MCP/hook files contain no `semio <verb>` text (verified with `git grep`).
- Devcontainer: `.devcontainer/Dockerfile` writes `/usr/local/bin/semio` (`exec bun run --cwd /workspaces/semio dashboard "$@"`) before `USER vscode`. Asserted in `testContainerRuntimeBootstrap` (`📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts`); that law passes. The image itself was not built (UNVERIFIED), `bun run --cwd <dir> dashboard <args>` forwarding was checked on Windows with a probe package.

## B. Stale binary

- `D/📦️installation/🟦️.ts`: the record is now `{version:2, platform, path, sources:{roots, digest}}`. `digest` = `<files>:<sha1>` over name, size and mtime (no file is read) of every file below the source roots, plus `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`. Roots = the module directory of the dashboard crate (`<module>/📦️packages/🦀️rust` owns its siblings) and of every transitive path dependency (`[dependencies]`, `[build-dependencies]`, `workspace = true` through `[workspace.dependencies]`; dev-dependencies excluded). Skipped: `target node_modules dist build .git 🗑️generated 🧪️tests 🧫️fixtures coverage temp …` and `.ts .tsx .py .log .md …`. Roots are recorded at install time, so startup parses no manifest.
- Measured on this checkout: 33 roots, 825 files, digest 17-30 ms (concurrent async stat; the synchronous variant took 60 ms on 341 files, hence async). Cap was ~100 ms.
- `launchDashboard` (bootstrap fast path of `bun run dashboard …`) and `dashboardExecutable` (Nx path, `📦️packages/🦀️rust/📜️script.ts`) compare first. Stale or invalid record: prints `[dashboard] Rebuilding because …; Ctrl+C cancels`, runs `bun <package>/📜️script.ts build` with inherited output; a non-zero exit throws, no outdated executable runs. `buildAndInstallDashboard` captures the sources before the build (edits during the build stay detectable) and passes an `AbortSignal` bound to SIGINT/SIGTERM to cargo.
- Exception, added on my own judgement: `mayRunStale` (`tasks logs stop kill open`, `daemon stop|status`) skips the gate so a broken tree cannot keep a developer from stopping the daemon. Documented in `D/README.md`.
- Tests: `D/🧪️tests/🧊️execution/🟦️.ts` (4 pass, 58 expects) with fixture sections `mayRunStale` and `staleness` in `D/🧫️fixtures/🧊️execution/🔣️.json` (fake crate tree: 13 mutations incl. mtime-only; roots equal `dep local mod`) and a record round trip (stale on edit and on added file with `n -> m files` text, fresh on build-output edit). Run: `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test ./<D>/🧪️tests/🧊️execution/🟦️.ts`. Scenario added to `🥒️.feature`.
- Consequence to know: the installed record on disk is still version 1, so the next `bun run dashboard …` (except the tolerant verbs, which then report "Invalid dashboard installation record") rebuilds a release binary once. UNVERIFIED end to end: my debug cargo build (`target-fleet-s1`) hung at `Compiling serde` for 25 minutes together with other fleet cargo processes alive since 04:25 (shared build-dir lock), so I killed it and never ran the rebuild path for real.

## C. `start` verb dissolved

- `workspace:start` ran `bun nx run @semio-tech/framework-os-dev:local-hub`, a target that does not exist (`git grep`), detached and invisible to the daemon. The hub owner already is the dashboard command `os-hub:local-hub-owner` (continuous).
- Removed: `StartScript` region and `.register("start")` in `📜️script.ts`, target `start` in `📋️project.json`, `"start"` in `D/📜️root-delegation/🦀️.rs` `ROOT_VERBS` (sets compared by hand with the `.register(` list: equal; the Rust test itself was not run). `workspace:start` is now "unknown command" in `semio run`.
- Added `metadata.semio.dashboard.ready {port 8787, path /admin}` to `os-hub:local-hub-owner` (`🌎️hub/📦️packages/🦀️rust/📋️project.json`). Dry run resolves it with ready 8787; `semio commands --check` (a2 debug binary): 20409 commands, 0 problems.
- README: `workspace:start` removed from the root command list; the hub owner is named as the dashboard command.

## D. `clean stray-processes`

- `📚️library/🧼️workspace-cleanup/🧟️stray-processes/🟦️.ts`: new `dashboardPids(rows, daemonPids)` protects the recorded daemon (`daemon.pid` below `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard`, read each pass), every row whose command is the installed `…/tools/dashboard-cli/<sha256>/semio` or `semio daemon serve`, all descendants and the dev-tool launcher ancestors. `planStrayProcessRemovals(rows, selfPid, daemonPids = [])`, `cleanKillStrayProcesses(dry, dashboardCacheDirectory)`, `listDevLeftoverRows(..., daemonPids)`. Note the old marker `/\/semio\//` matched the daemon itself. `cleanKillStrayProcesses` returns `[]` on Windows, unchanged.
- Tests: 3 new fixture cases in `🧪️fixtures/🧼️workspace-cleanup-stray-processes/🔣️.json` (recorded pid with a task tree, command-only protection of daemon and view with launcher, debug-build daemon) plus pid-file and audit tests: 7 pass. Without the daemon pid the first case plans 4 kills instead of 1.
- `.agents/skills/clean/SKILL.md` item 6 says so.

## E. Finding #4, #18

- New `agentClientLauncher(tools, root, server, runtime)` in `📚️library/🤖️agent-clients/🟦️.ts` (declared words of the server up to the first flag, entry script absolute). `🧑‍💻dev/🔌️vite-plugins/🟦️.ts` `semioAgentCredentialInstallVitePlugin` derives the receipt launcher from the root `tools` declaration (`semio` server); the literal is gone, derived before the credential file is written. Why up to the first flag: `--folder` and `--hub` are mutually exclusive and the credential flow adds `--hub --space --credential-file`.
- Fixture `launchers` + 3 tests in `📚️library/🧪️tests/🤖️agent-client-configuration` (22 pass). Plugin transpiles; its HTTP route was not exercised. #18 reworded to "editor-settings rewrite".

## F. Finding #13 (VS Code extension)

- Removed `runRepoCommand` (terminal starter, zero callers). The probe `<root>/repo/cli/cli` is replaced by exported `resolveRepoBinaryPath(root, env)`: `REPO_CLI_BIN`, else `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-repo[.exe]` (the `semio-repo` bin of `⌨️cli`, target-dir from `.cargo/config.toml`, same location `⌨️cli` `RepoScript` execs). `hasRepoAccess()` is false until that binary is built (it is not built here). Mocha test added in `🧪️tests/🧩️extension/🟦️.ts` (needs the VS Code host: not run); assertion messages updated. `bun ./📜️script.ts build` of `@semio-tech/repo-vscode`: both bundles built.
- Other extension spawns (`execShell` of section list, `ticket close`) call the repo CLI, not servers; left.

## G. `.vscode/settings.json` LaTeX recipe

Tool and recipe renamed `semio-dashboard`, `command: bun`, `args: ["run","--cwd","%WORKSPACE_FOLDER%","dashboard","run","@semio-tech/mit-bestand-bericht:build"]` (`--cwd` because LaTeX Workshop runs in the document directory, whose nearest `package.json` has no `dashboard` script). `bun ./📜️script.ts generate config` rewrote `.devcontainer/devcontainer.json`; `--check`: current. Dry run of the id resolves.

## H. Docs

- Root `README.md`: `npm run setup` -> `bun run setup` (2x); sketchpad getting-started is now `bun run dashboard run playground:s --param renderer=react --detach --wait-ready` (port 6070). The old two-command advice and the claim that `dev` watches the repo were stale: the Nx plugin (`📚️library/🟨️.mjs:1207`) makes `dev-s-react-dev` = cached `activate-s-react-dev` then the same `serve`. `activate-s-react-dev` is NOT a registry command (`unknown command`), so the README names it only as the prerequisite. Studio block uses `bun run dashboard run os-hub:dev[-secure-suite] --detach --wait-ready`; `script:<name>` dropped from the id list.
- `🌎️hub/README.md` (dev, secure-suite, postgres), quiz README table (dashboard id column, `steady=true`, `stack=beside` with its ports), pets READMEs, proctor README line 282.
- Verification (a2 debug binary copy, `semio run <id> --dry-run --json`): resolve ok: `os-hub:dev`, `os-hub:dev-secure-suite`, `os-hub:dev-postgres`, `os-hub:local-hub-owner`, `@semio-tech/pets-react:dev`, quiz and proctor `dev`, `@semio-tech/mit-bestand-bericht:build`, `workspace:lint`, `workspace:dev`, `playground:s`. Two corrections found: the CLI only accepts `key=value`, so the M-2 spelling `--param steady` is wrong and is `--param steady=true` (flag values are `true|false`); the pets parameter is `stories-port`, not `port` (pre-existing README error in two files, fixed).

## Open for others

- `workspace:dev` dry run reports `longRunning:false` although README says `dev` stays live (not mine).
- `D/README.md` verb table still lacks `dashboard`, `preferences`, `catalog`, `command-tree`, `plugin registry`, `workflow` (finding #6, A-1).
- Cargo is jammed on this machine (build processes since 04:25 holding the shared build-dir); the first rebuild after this change will wait on it.
- `.codex/plans/declarative-actions.md` (#20) untouched.

## Files

Edited: `D/📦️installation/🟦️.ts`, `D/📦️packages/🦀️rust/📜️script.ts`, `D/📜️root-delegation/🦀️.rs`, `D/README.md`, `D/🧪️tests/🧊️execution/{🟦️.ts,🥒️.feature}`, `D/🧫️fixtures/🧊️execution/🔣️.json`, `📜️script.ts`, `📋️project.json`, `🌎️hub/📦️packages/🦀️rust/📋️project.json`, `README.md`, `🌎️hub/README.md`, `.devcontainer/Dockerfile`, `.devcontainer/devcontainer.json` (derived), `.vscode/settings.json`, `.agents/skills/clean/SKILL.md`, `📚️library/🧼️workspace-cleanup/🧟️stray-processes/🟦️.ts`, `…/🎮️command/🟦️.ts`, `🦑️repo/🧪️tests/🧼️workspace-cleanup-stray-processes/🟦️.ts`, `🦑️repo/🧫️fixtures/🧼️workspace-cleanup-stray-processes/🔣️.json`, `📚️library/🤖️agent-clients/🟦️.ts`, its test and fixture, runtime-bootstrap test, `🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `💻️client/🧩️vscode/{🟦️.ts,🧪️tests/🧩️extension/🟦️.ts}`, quiz, pets (2), proctor READMEs.
