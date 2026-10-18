# Audit: semio-tech play (port 6033)

Read-only audit, 2026-10-18. Nothing was built, served, activated or modified. Commands run: `ls`, `cat`, `grep`, `lsof` (read-only), and read-only Python parsing of `.vscode/launch.json`, `project.json` files and `package.json` names.

## Headline

- The play dev chain is **stale in its memory notes**: the target `activate-dev` no longer exists, the receipt directory moved from `dist/♻️activation/dev` to `dist/♻️activation/prototype`, and the lane list is now 35 hard-coded lanes, not 26 or 28.
- **Nothing is staged right now**: `:6033` has no listener (`lsof` empty), `🏢️semio-tech/🎡️play/dist` contains only `reports/`, and `.cache/b/p/m/browser` does not exist. `serve` calls `readPlayActivation` first (`🔨️modules/🧩️runtime/📜️script.ts:44`), so it cannot serve until `activate-prototype` has run.
- The ticket scripts that the memory notes point to (`describe-serial-then-activate.sh`, `describe-all-then-activate.sh`, `describe-plugins.sh`) still call `bun nx run …:activate-dev` and `@semio-tech/<p>-plugin:describe`. Both are stale. The `-plugin` projects and the `describe` target do not exist.
- Launch entries, play project targets, e2e scripts, the Vite config, the build jobs and all ticket-owned launch scripts resolve to files that exist. Each entry was checked for path existence only; none was executed.

## 1. Launch entries for play

Source: `.vscode/launch.json`. `.vscode/🧩️launch.seed.jsonc` mirrors 9 of the 12 core play entries; the three `prepare-dev`, `prepare-e2e` and `serve-e2e` entries exist only in `launch.json`.

| Launch name (launch.json line) | Command | Resolves to play target |
|---|---|---|
| `🛠️dev🏢️semio-tech🎡️play` (88057) | `bun repo run @semio-tech/semio-tech-play:dev`, env `SEMIO_TECH_PLAY_PORT=6033`, `SEMIO_RENDERER=react`; `serverReadyAction` on `http://…:6033` | `dev` (depends on `activate-prototype`) |
| `♻️activate🏢️semio-tech🎡️play` (88079) | `bun repo run @semio-tech/semio-tech-play:activate-prototype --configuration=prototype` | `activate-prototype` |
| `🖥️serve🏢️semio-tech🎡️play` (88094) | `bun repo run @semio-tech/semio-tech-play:serve` | `serve` (depends on `activate-prototype`) |
| `📦️prepare-dev🎡️play` (169501) | `bun repo run @semio-tech/semio-tech-play:prepare-prototype --configuration=prototype` | `prepare-prototype` |
| `📦️prepare-e2e🎡️play` (169516) | `bun repo run @semio-tech/semio-tech-play:prepare-e2e` | `prepare-e2e` |
| `🛠️serve-e2e🎡️play` (169531) | `bun repo run @semio-tech/semio-tech-play:serve-e2e` | `serve-e2e` |
| `📦️build🏢️semio-tech🎡️play` (80715) | `bun repo run @semio-tech/semio-tech-play:build` | `build` (release jobs) |
| `📦️build🏢️semio-tech🎡️play🆕️fresh` (80760) | `… :build --skip-cache --skip-remote-cache --parallel=8 --output=stream`, `SEMIO_BUILD_MODE=release`, `SEMIO_NATIVE_BUILD_GENERATION=fresh` | `build` |
| `📦️build🏢️semio-tech🎡️play📦️release` (in the `📦️release` group, ~80947) | `… :build --skip-cache --skip-remote-cache --parallel=4 --output-style=stream`, `SEMIO_BUILD_MODE=ship` | `build` |
| `⚖️gate…🎭️e2e` (80745) | `… :test-e2e` | `test-e2e` (needs `serve-e2e` → `prepare-e2e` → `activate-prototype`, `workspace:deps-browsers`) |
| `⚖️test-user-language…🟦️` (80730) | `… :test --exclude-dependencies -- long --run 🪧️brand.ts --testNamePattern="PLAY_PANES"` | `test` |
| `⚖️gate…📦️release` (80839) | `… :test-release --configuration=release` | `test-release` |
| `⚖️gate…🚀️release-verify` (~80854) | `… :release-verify` | `release-verify` |
| `⚖️gate…🛣️routing` (~80869) | `… :test-release-routing` | `test-release-routing` |
| `⚖️test-enduser-workflows🧊️generation3d🎭️play` (122223) | `… :test-e2e-enduser --skip-cache --parallel=1 --configuration=prototype`, `SEMIO_TICKET_DIR=…/PROCEDURAL-FEATURE-COMPLETE` | `test-e2e-enduser` |
| `⚖️gate…prefinal-task-graph` | `… :build --graph=play-prefinal-current-task-graph.json`, `SEMIO_BUILD_MODE=ship` | `build` |
| `⚖️gate…🆕️fresh-contract` | `… :test --skip-cache --skip-remote-cache --exclude-dependencies -- ./🔨️modules/📦️site/🆕️fresh-build/🟦️.ts` | `test` |
| `🔎️diagnostic…` (≈9 entries) | `bun repo exec --projects=@semio-tech/semio-tech-play … bun "${workspaceFolder}/.🧬semio/…/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/<probe>/📜️script.ts"` | ticket probes (see section 3) |
| Other `⚖️gate…` entries named after play (framework tests, ticket oracles, `bun repo exec --projects=…`) | `bun repo exec …` with `bun test` / ticket oracles | framework or ticket checks; not play-owned targets |

There are 84 distinct launch commands that mention play, its port, its ticket or its serve chain. 45 of them are play-directed (`semio-tech-play:` target or play ticket script). The table lists the ones that matter for running. The 45 play-directed rows were written to `/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/play-launch.txt`.

## 2. Build chain

**Play project** `🏢️semio-tech/🎡️play/📋️project.json`:
- `prepare-prototype` (line 10): `dependsOn` 35 `@semio-tech/framework-os-dev:prepare-<lane>-react-prototype` targets, plus `bun ./🔨️modules/🧩️runtime/📜️script.ts prepare`, `SEMIO_BUILD_MODE=prototype`.
- `activate-prototype` (line 67): `dependsOn` `prepare-prototype` plus the 35 `activate-<lane>-react-prototype` targets. Runs `… script.ts activate`. Output `{projectRoot}/dist/♻️activation/prototype`.
- `dev` (line 127): `dependsOn` `activate-prototype`; `SEMIO_TECH_PLAY_PORT=6033`, `SEMIO_BUILD_DYNAMIC_OUTPUT=false`, `SEMIO_BUILD_TERMINAL=false`, `NODE_OPTIONS=""`, `VSCODE_INSPECTOR_OPTIONS=""`; `continuous: true`, `cache: false`.
- `serve` (line 158): same serve command, `dependsOn` `activate-prototype`.
- `build` (line 182): `bun ./🔨️modules/📦️site/📜️script.ts build` with 13 release jobs (see below).
- `test` (line 459), `test-release-routing` (501), `test-release` (511), `release-verify` (550), plus `prepare-e2e`, `serve-e2e`, `test-e2e`, `test-e2e-enduser` between them.

**Lane targets.** The `@semio-tech/framework-os-dev:{prepare,activate}-<lane>-react-prototype` targets are **not in any `project.json`** (HEAD and working tree both have 53 targets and none of them is a lane target). The repo graph inference generates them per playground row:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` lines 1444–1461: `serve-`, `dev-`, `activate-`, `prepare-${variant}-react-${profile}`, plus `prepare-…-native-…` and `…-wgpu-…`.
- Input rows: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts` (156 variants). All 35 lanes that play lists are present there (checked).
- Per-lane command: `bun ./📜️script.ts activate <variant> react prototype` (line 1451 area), which writes `.cache/b/p/m/browser/prototype/runtime/react/<variant>` (`outputs`).
- Resolution of `@semio-tech/framework-os-dev:activate-<lane>-react-prototype` through `bun repo run` was **not executed**. Treat it as unverified.

**Play runtime script** `🔨️modules/🧩️runtime/📜️script.ts` (routes: `prepare`, `activate`, `serve`, `prepare-test`, `serve-test`):
- `PreparationScript` (lines 18–29): checks every plugin and extension module dir in the plugin-modules root has `.semio-artifact.json` with `🌉️bridge.js` and `🔣️.json`.
- `ActivationScript` (lines 32–38): `readPlayActivation` only; prints counts.
- `ServeScript` (lines 41–52): `readPlayActivation`, then `serveVite` with config `🏗️builder/🌐️vite/🟦️.ts`, `host = 0.0.0.0` if `DEVCONTAINER=true` else `127.0.0.1`, `port = SEMIO_TECH_PLAY_PORT ?? 6033`, `strictPort`, renderer `react`, engines `play`.
- `PrepareTestScript` / `ServeTestScript` (lines 55–78): isolated E2E listener on port 0, sets `SEMIO_TECH_PLAY_FROZEN=true` (no file watching).

**Union receipt** `🔨️modules/🧩️runtime/♻️activation/🟦️.ts`:
- `PLAY_UNION_RECEIPT_DIRECTORY = "🏢️semio-tech/🎡️play/dist/♻️activation/prototype"` (line 22).
- `playActivationLanes()` (lines 29–43): greedy cover over `playRuntimeComponentIds()`, excluding the `s` host lane. The lane count is computed at runtime; the 35-lane hard list in `project.json` is a separate source of truth.
- `mergePlayActivationReceipts` (line 148): throws `Stale play activation lane` when two lanes disagree on a component's `artifactSha256` (line ~139) and the installed artifact matches neither.
- `publishPlayUnionReceipt` (line 179) writes the merged receipt to the `dist` directory above.

**Vite config** `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`: `server.port = SEMIO_TECH_PLAY_PORT ?? 6033`, `strictPort: true` (line 73); `watch` is disabled when `SEMIO_TECH_PLAY_FROZEN === "true"` (line 76); `cacheDir` via `viteCacheDirectory(repoRoot, …)`.

**Release jobs** (`build.metadata.semio.release.jobs`, project.json ~line 182–458). Every job root and its `📜️script.ts` exist:
- `describe-tool` (`🧰️…/🔌️plugin/🖨️describe/📦️packages/🦀️rust`, `build`), `font-assets` (`🖼️canvas/🔤️fonts/…/🦀️rust`, `build`), `fonts` (`♾️infinite/📦️packages/🦀️rust`, `fonts`), `support` (`🔌️plugin/📦️packages/🟦️typescript`, `support`; package `@semio-tech/framework-plugin-web`), `canvas-runtime-assets`, `flow-wasm`, `surface-wasm`, `editor-wasm`, `graph-wasm`, `puzzle-wasm` (`🌎️hub/🧩️compositions/🧩️puzzle/…/🦀️rust`, `wasm`, `wasm32-unknown-unknown`, `workerProtocol: exclusive`), `map-tiles`, `bundle`, `pages` (site script in `🔨️modules/📦️site/`).
- Prerequisites: `workspace:deps-wasm-opt` (root target exists), `@semio-tech/plugin-registry:generate` (exists), `@semio-tech/assets:build` (exists).
- `components:complete` (listed in `pages.after`) is **not defined** in the root `project.json` or the graph inference. Unverified.

**Wasm plugin builds for the dev lanes** run inside the lane targets through the framework-os-dev activation (`canonicalWasmBuildVitePlugin`, `playgroundFlowWasmDevStubPlugin`) and the wasm32 policies. They are not listed in play's `project.json`.

**Staging directories**:
- Play's own output: `🏢️semio-tech/🎡️play/dist/` (receipt under `♻️activation/prototype`, plus `reports/`). Currently only `dist/reports` exists.
- Browser products: `repositoryCachePartitionRootDataV1(workspace,"products")/browser/<prototype|release>/{modules,sessions,runtime}` (`🧰️…/♻️activation/📂️paths/🟦️.ts`, line 8). The inference outputs name `.cache/b/p/m/browser/<profile>/runtime/react/<variant>`. **`.cache/b/p/m/browser` does not exist now.** The cache holds `products.json`, `storage-events.jsonl`, `bazel/`, `next/`, `nx-hook/`, `vite/` and `tmp/`.
- Fleet state: `.🧬semio/🦑️repo/⚡️cache/play-fleet/` exists, but `…/play-fleet/activation/` does not.

**Ports**: play serve/dev `6033` (127.0.0.1, or 0.0.0.0 with `DEVCONTAINER=true`); E2E listener on port 0 (random); the demonstrator playground's own react/wgpu ports (6107 / 6207) are not play.

**Environment variables**:
- `SEMIO_TECH_PLAY_PORT` (default 6033); `SEMIO_TECH_PLAY_FROZEN` (`"true"` disables Vite watch; forced by `serve-test`; defaulted to `true` in `🔁️serve-supervisor.sh`).
- `SEMIO_BUILD_MODE` (project.json defaults `prototype`; launch build entries set `ship` or `release`); `SEMIO_NATIVE_BUILD_GENERATION`; `SEMIO_BUILD_DYNAMIC_OUTPUT`, `SEMIO_BUILD_TERMINAL`, `NODE_OPTIONS`, `VSCODE_INSPECTOR_OPTIONS` (in `dev`).
- `DEVCONTAINER=true` (serve binds 0.0.0.0).
- `SEMIO_RENDERER=react` is set on the `dev` launch entry but play code hardcodes `renderer: "react"`; it is a no-op.
- `PLAYWRIGHT_BASE_URL` (required by the E2E config); `PLAY_E2E_GPU=swiftshader` (software GPU); `SEMIO_TICKET_DIR` (report location for E2E and enduser runs).
- `SEMIO_BUILD_TOOL` / `--tool=nx|bazel`: `🧰️…/🦑️repo/…/🏗️build/🧭️invocation/🟦️.ts:121–132`. The default is `nx` (`🔣️policy.json` `"defaultTool": "nx"`), so `bun repo run` still executes Nx unless `--tool=bazel` is given.
- Ticket-level only: `PLAY_MUTEX_STAMP`, `PLAY_CHAIN_STEPS`, `PLAY_ACTIVATE_FIRST`, `SEMIO_TECH_PLAY_SUPERVISOR_LOG`, `PLAYWRIGHT_BROWSERS_PATH`.

## 3. Stale references

Existence was checked with `ls` or with a Python read of the `project.json` files.

| # | Reference (where) | Status | Current equivalent |
|---|---|---|---|
| 1 | Memory `project-semio-tech-play-boot-recipe`: `bun nx run @semio-tech/semio-tech-play:activate-dev` | **Stale**. No `activate-dev` target in play `project.json`; the target is `activate-prototype` (line 67). The interface is `bun repo run …`. | `bun repo run @semio-tech/semio-tech-play:activate-prototype --configuration=prototype` |
| 2 | Memory: receipt in `dist/♻️activation/dev` | **Stale**. Code uses `dist/♻️activation/prototype` (`♻️activation/🟦️.ts:22`). | `…/dist/♻️activation/prototype` |
| 3 | Memory: "26 pane lanes" / "28 lanes"; "all ~58 apps"; play-runtime notes "69 panes" | **Stale counts**. `project.json` lists 35 lanes; the catalog `🔨️modules/🧩️runtime/🔣️.json` now has 149 panes across 18 groups. The runtime greedy cover (`♻️activation/🟦️.ts:29–43`) decides the actual lanes. | Count from `playActivationLanes()` after a run |
| 4 | `📜️describe-serial-then-activate.sh` (memory-cited, ticket 26/09/19) lines 29, 30, 41 | **Stale**. Uses `bun nx run …:activate-dev` and `bun nx run "@semio-tech/$p-plugin:describe"`. `@semio-tech/<p>-plugin` projects do not exist (animate is `@semio-tech/animate-js`, architect `@semio-tech/architect-js`, math `@semio-tech/mathematical-js`). The only `describe` target is `@semio-tech/os-plugin-describe-rs:describe`; the graph inference uses `@semio-tech/os-plugin-describe-rs:build` (inference lines 1237 and 1367). | Lane targets are `activate-<variant>-react-prototype`; the describe prerequisite is `@semio-tech/os-plugin-describe-rs:build` |
| 5 | `📜️describe-all-then-activate.sh` (lines 6, 8, 10–12) | **Stale**. `bun nx show projects`, `bun nx run-many -t describe`, `bun nx run …:activate-dev`. | As above |
| 6 | `📜️describe-plugins.sh` (line 9) | **Stale**. `@semio-tech/$p-plugin:describe`. | As above |
| 7 | `📜️describe-serial-then-activate.sh` line 9 | Hard-coded `/Users/ueli/Documents/semio` and absolute ticket paths. Works here only. | — |
| 8 | Memory: `📜️wasm-build-mutex-play-stamped.sh`, `🔁️serve-supervisor.sh`, `📜️daemonize.py`, `📓️play-runtime.md`, `📜️describe-serial-then-activate.sh`, `🧪️probe-*.ts` | **Exist** in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/`. | — |
| 9 | Memory: `📜️wasm-build-mutex.sh` in the OS-HUB ticket `…/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/` | **Exists**. Uses `/tmp/semio-wasm-build.{lock,queue}`. | — |
| 10 | Memory: `🔁️serve-supervisor.sh` logs under `🗑️generated` | Still the design (`🔁️serve-supervisor.sh` line 6). The sweep risk in the memory note is not checked here. | Use `SEMIO_TECH_PLAY_SUPERVISOR_LOG` outside `🗑️generated` |
| 11 | `🔨️modules/🧪️e2e/🎚️config/🟦️.ts:20`: `throw new Error("Run the play test-e2e target through Nx")`; header line 4 "Nx prepares…" | **Stale wording**. Behaviour is fine. | Say `bun repo run @semio-tech/semio-tech-play:test-e2e` |
| 12 | `🔨️modules/🧪️e2e/📜️script.ts:13, 40`; `🔨️modules/🧩️runtime/📜️script.ts:17, 31, 40` | **Stale wording** ("owned by Nx", "Nx preparation graph"). | Neutral wording |
| 13 | Play `package.json` scripts call `bun --cwd "../.." repo run …` | Consistent with the neutral interface. **Conflicts with `AGENTS.md`**, which says `package.json` must call `nx`. The `repo` CLI itself defaults to `nx`. | Decide which rule wins; not changed |
| 14 | `components:complete` (play `project.json` `build.pages.after`) | **Unresolved**. No definition in root `project.json` or the graph inference. | Unverified |
| 15 | `🔎️arguments/📂️path with spaces/📜️script.ts` (launch `🔣️arguments`) | **Exists** (checked with `ls`). | — |
| 16 | All other launch-referenced ticket scripts under `BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/` (`📜️script.ts`, `🔍️*/📜️script.ts`, `🔭️routing`, `feature-audit`, `acceptance-oracle`, `🔍️publication`) | **Exist**. | — |
| 17 | Launch `🛠️dev…` sets `SEMIO_RENDERER=react` | Not read by play code (no-op). | — |
| 18 | `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` | Both hold play entries; the seed lacks `prepare-dev`, `prepare-e2e`, `serve-e2e`. Git status shows `launch.json` as `MM`. | Keep the two in sync |

Other stale data: the play memory notes all use the old `nx` interface. `bun nx` still works in principle because the root `package.json` keeps `nx` 23.2.0 and `defaultTool` is `nx`. The memory note on the external sweep (`project-external-sweep-breaks-nx-tooling`) is about `bun nx` tooling, not play. It remains relevant.

## 4. Existing probes and E2E checks

**Play unit and contract suites** (`bun repo run @semio-tech/semio-tech-play:test`, `./📜️script.ts test` → vitest with `🧪️tests/🎚️config/🟦️.ts`):
- In-source vitest blocks in `🔨️modules/🧩️runtime/🟦️.ts` (lines ~108–118, `registerTests1` for `playpanecoverage` and `playpanedefaults`) and `♻️activation/🟦️.ts` (`playactivation` suite).
- Suites under `🧪️tests/`: `playgrid` (PLAY_PANES), `playpages`, `playrelease`, `playmaptiles`, `playfreshbuild`, `playdevmounts`, `playpanecoverage`, `playpanedefaults`, `playactivation`, `release-job-order`.
- `playgrid` contains a `describe("PLAY_PANES")` block (line 8). The `PLAY_PANES` name pattern in the `test-user-language` launch entry therefore matches.

**Acceptance E2E** (`bun repo run @semio-tech/semio-tech-play:test-e2e`):
- Flow: `serve-e2e` (`serve-test`, port 0, frozen) → `prepare-e2e` → `activate-prototype`. The `test` route runs Playwright on `🧪️tests/🎭️acceptance/🟦️.ts` through `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`, which uses `PLAYWRIGHT_BASE_URL`, one worker, chromium with `--use-angle=metal` on macOS.
- `🧪️tests/🎭️acceptance/🟦️.ts` (~38 KB): `BOOT_SETTLE_MS = 2_000` (line 56); `collectErrors` treats `/^input #\d+ .* refused: /` as refused input (line 85); asset-path `text/html` answers are failures (line 113); the paint witness is polled (line 68–69). Boot tests are generated per pane (`boots ${pane.label} (${pane.variant})`, line 457). It also has `edit`, `analyze` and `cancel` for the 3D workflow.
- Per the memory, this suite is the all-app boot gate. A pane is "good" only when the shell beacon reports ready, the 2 s settle passes, there are no page or console errors, no refused inputs, no HTML answers for media, and the pane is visibly painted.

**Enduser** (`test-e2e-enduser`): same server, `--grep "3d (en-US|de-DE)|language.*choose"`, report `enduser-2026-10-13/browser`.

**Release** (`test-release`): `withPublishedPlayPagesV1` builds and serves the published pages, then runs Playwright against them. `test-release-routing` runs `🧪️tests/🧪️playrelease/🟦️.ts`.

**Ticket probes** (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/`): `🧪️probe-boot.ts`, `🧪️probe-lanes.ts`, `🧪️probe-closure.ts`, `🧪️probe-console.mjs`, `🧪️fleet-wait.sh`, `🧪️test-binary-watchdog.sh`. (`probe-batch.mjs`, cited in `📓️play-runtime.md`, is not in the ticket folder.) Ticket 26/09/23 diagnostics (`🔍️*/📜️script.ts`, `acceptance-oracle`, `feature-audit`, `🔭️routing`, `🔍️publication`) are wired through `launch.json`.

**Manual recipe from the memory** (`project-semio-tech-play-boot-recipe`, 2026-09-22): run the suite directly against a live server with `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6033 PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright bun node_modules/playwright/cli.js test --config 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts --output <dir>`. All pieces exist: `node_modules/playwright/cli.js`, the ms-playwright cache directory, and the config file. Not run.

**Judging "runs end to end"**, from the code and the memory:
1. `activate-prototype` finishes with one union receipt and no `Stale play activation lane` error (and no lane drift warning that names a lane as stale).
2. `serve` answers `200` on `127.0.0.1:6033`. Judge the restart by the supervisor's events file, not by `curl`, because a stale server still answers.
3. The acceptance suite passes with zero page errors, console errors, refused inputs, HTML-for-media answers and resource errors, after the 2 s settle, for every generated pane.
4. For release: `build` jobs complete and `test-release` passes against the published pages.

## Open items

- Nothing was run, so the lane target resolution through `bun repo run` and the `activate-<lane>-react-prototype` inference are unverified.
- `components:complete` has no definition found.
- The `🧪️playactivation` unit test that pins `project.json` dependsOn to the computed lanes was not read.
- The state of the sweep and `/tmp` mutex queue was not checked.
