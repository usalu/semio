# 🔎 Audit: three products against current source (2026-10-19, 12:50 CEST)

Auditor `aud-prod`, read-only, static only. No builds, no `bun repo run`, no git mutation. Read-only git (`ls-tree`, `log`, `show --stat`, `grep`) and `bun repo --help` only. HEAD = `bd9e4e02bd0` (auto-commit 12:21:52). The old reports (`audit-teaching-course.md`, `audit-semio-tech-play.md`, `audit-mit-bestand-demonstrator.md`, `audit-other-products.md`, `result-play.md`, `result-demo.md`, `result-osplay.md`) were re-checked line by line against the current tree.

## 0. Cross-product blockers (read first)

| # | Finding | Evidence |
|---|---|---|
| B1 | `.vscode/` is gone from disk and from HEAD. The 12:21 auto-commit deleted `launch.json` (203 KB), `🧩️launch.seed.jsonc` (129 KB), `settings.json`, `mcp.json`, `extensions.json`. AGENTS.md mandates `launch.json` for every executable command. Every launch row of all three products is therefore gone. | `git show --stat bd9e4e02bd0 -- .vscode` (5 files, 332,543 deletions). `ls .vscode` → no such directory. `git ls-tree -r HEAD` has no `.vscode/` entry. |
| B2 | The course ticket (19) says its 16 course launch entries were "restored", but they are not on disk. Its own claim conflicts with the tree. | `PREPARE-…/readiness-status.md` (Developer command registration row) vs. B1. |
| B3 | Playwright chromium 1248 and ffmpeg 1013 are not installed anywhere. Only 1234 and 1011 exist, in the user cache and in the repo cache. All three `test-e2e` chains need a download first. | Section 4. |
| B4 | Docker daemon is not running. The CLI 29.5.3 exists. Course image build, image check, stack check and publish cannot run. | `docker info`: "failed to connect to the docker API at unix:///Users/ueli/.docker/run/docker.sock". `/var/run/docker.sock` missing. |
| B5 | A dependency sync is in progress (`bun repo dependencies lock --all`, pid 34479, started about 12:49). Nothing may run until it finishes. | `ps`. |

## 1. Target lists and chains (current source)

### 1a. Course: `@teaching/architecture-course` (renamed from `@teaching/architecture-quiz`)

File: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`. Script router: `…/📦️packages/🟦️typescript/📜️script.ts:132-134`.

| Target | Command (`bun ./📜️script.ts …`) | Deps |
|---|---|---|
| dev | `dev` (runDevStack, `🧱️stack/🟦️.ts`) | assets:build, ui-styling-tokens:generate |
| dev-site | `dev-site` | same |
| build | `build` | same |
| publish | `publish` (release, `dist/release/pages/course`) | same |
| test | `test` (vitest native config) | none |
| test-e2e | `test-e2e` (`🎭️e2e/🟦️.ts`) | assets, tokens |
| typecheck | `typecheck` | framework-rs, ui-contract-rs, ui-rs, framework-actor-rs:typegen, tokens, assets (all exist) |
| check | `check` | none |
| preflight | `preflight` (Linux x64, Debian 12 only) | none |
| ci-browser-prepare | `ci-browser-prepare` | none |
| docker-image-build / -check / -publish | same names | none |
| docker-stack-check / docker-stack-bundle | same names | none |
| deploy-check | `deploy-check` | assets, tokens, typecheck |

Proctor: `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`, targets build, test, dev (ready 8791, `PROCTOR_PORT`), check, rebuild, health, backup, restore, erase, prune, capacity.

Chain:
- Dev: `course:dev` → `runDevStack` → proctor built by `bun repo cargo --store=shared --configuration=… -- build --locked --package …` (`🛂️proctor/🏗️bootstrap/🟦️.ts:263`), proctor on 8791, site `dev-site` on 6061 (`TEACHING_ARCHITECTURE_COURSE_PORT`). Beside-stack: 6063 and 8793 (project.json:106-107).
- Course e2e: `test-e2e` boots throw-away stacks on fresh loopback ports (`🎭️e2e/🟦️.ts:169`, `endToEndPorts`). Topologies `dev` and `rehearsal`, both in parallel unless `--serial`. Playwright config `🎭️e2e/🎚️config/🟦️.ts`: projects boot → desktop, phone, layout → presence → shortage → away → faults → pets. Chromium via `ensureChromium` (`🎭️e2e/🟦️.ts` ~L290-300) into `PLAYWRIGHT_BROWSERS_PATH`.
- Release: `deploy-check` runs build, typecheck and end-to-end in order. `docker-image-build` requires a supplied Linux x64 executable and Docker (`🚀️deploy/🟦️.ts:464-472`).

### 1b. Play: `@semio-tech/semio-tech-play` (port 6033)

File: `🏢️semio-tech/🎡️play/📋️project.json` (19,996 B, mtime 10-18 22:24). Root router registers only `test` (`📜️script.ts`).

| Target | Command | Deps |
|---|---|---|
| prepare-prototype | `🔨️modules/🧩️runtime/📜️script.ts prepare` | 35 `framework-os-dev:prepare-<lane>-react-prototype` + wasm |
| activate-prototype | `…/runtime/📜️script.ts activate` | prepare-prototype + 36 deps |
| dev, serve | `…/runtime/📜️script.ts serve` (port 6033, `project.json:132,146,163`) | activate-prototype |
| build | `🔨️modules/📦️site/📜️script.ts build` | release job graph |
| test | `📜️script.ts test` | none |
| prepare-e2e | `…/runtime/📜️script.ts prepare-test` | activate-prototype, workspace:deps-browsers |
| serve-e2e | `…/runtime/📜️script.ts serve-test` (port 0) | prepare-e2e |
| test-e2e | `🔨️modules/🧪️e2e/📜️script.ts test` | serve-e2e |
| test-e2e-enduser, test-release, test-release-routing, release-verify | e2e/site scripts (all registered) | various |

Playwright: `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`. testDir `🧪️tests`, testMatch `🎭️acceptance/🟦️.ts` (11 `test(` calls), one chromium project, workers 1. Requires `PLAYWRIGHT_BASE_URL`, supplied by serve-e2e.

Chain: `activate-prototype` → `serve` (6033) → `test-e2e` (serve-e2e → prepare-e2e → deps-browsers).

### 1c. MIT-Bestand demonstrator: `@semio-tech/mit-bestand-demonstrator` (port 6029)

File: `♻️mit-bestand/🧺️demonstrator/📋️project.json`. Root router: `📜️script.ts` (`test`, `runtime-catalog-test`).

| Target | Command | Deps |
|---|---|---|
| prepare-prototype | runtime `prepare` | 9 `framework-os-dev:prepare-<variant>-react-prototype` + 4 wasm (project.json:13) |
| prepare-release | runtime `prepare` | 9 `…-react-release` + 4 wasm (project.json:48) |
| activate-prototype | runtime `activate` | prepare-prototype + activate-lanes-prototype (project.json ~L150-160) |
| activate-lanes-prototype | runtime `complete` | 3 `framework-os-dev:activate-*-react-prototype` |
| dev, serve | runtime `serve` (port 6029, project.json:88-89, 198-199) | activate-prototype |
| build | `🔨️modules/📦️site/📜️script.ts build` | prepare-release + 4 wasm (project.json:117) |
| prepare-e2e | runtime `prepare-test` | activate-prototype, deps-browsers |
| serve-e2e | runtime `serve-test` (port 0) | prepare-e2e |
| test-e2e | `🔨️modules/🧪️e2e/📜️script.ts test` | serve-e2e |
| canonical-architecture | `📜️script.ts test long "./🧪️tests/🏛️shell-brand/🟦️.ts"` | none |
| test-runtime-catalog | `📜️script.ts runtime-catalog-test` | none |
| test | `📜️script.ts test` | none |

Playwright: `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`. testMatch `🎭️acceptance/🟦️.ts` (7 `test(` blocks, same as before). One chromium project, retries 2 when CI. Requires `PLAYWRIGHT_BASE_URL`.

Chain: `activate-prototype` → `serve` (6029) → `test-e2e` (serve-e2e → prepare-e2e → deps-browsers).

All `dependsOn` names in play and demo resolve. Inferred targets (`prepare-/activate-/serve-/dev-<variant>-react-<profile>`, `session-<variant>`) come from `🎮️playground-targets/🟨️.mjs` (lines 112, 204-244). Targets referenced by name exist: `@semio-tech/framework-plugin-web:support`, `semio-framework-os-infinite:fonts`, `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`, `@semio-tech/os-plugin-describe-rs:build`, `…-rs:wasm` for surface, editor, node-graph and puzzle.

Root `📋️project.json` targets used by the chains: `deps-browsers` (line 3189), `deps-wasm-opt`, `components`, `build`, `prepare`, `dev-mcp`, `dev-mcp-repo`. **Missing**: `start`, `dev`, `dev-mcp-engine`, `serve`, `activate`.

## 2. Stale items from the old audits

Status legend: FIXED (verified in source), STILL-PRESENT, NOT-A-DEFECT, CHANGED, NEW.

### Demonstrator

| Item | Status | Evidence |
|---|---|---|
| S1 `workspace:dev -- <variant>` launch rows (335) | FIXED by deletion. The rows are gone with `.vscode/launch.json` (B1). No `workspace:dev` root target exists. | root project.json has no `dev` key. |
| S3 `prepare` without profile | FIXED. `prepare` target removed; build depends on `prepare-release`. | demo project.json:117, :155. |
| S4 `🔨️modules/🧩️runtime/📜️script.ts` dist path | FIXED. Uses `builtPluginModulesRootIn(this.repoRoot, profile)`. | runtime `📜️script.ts:13, :22`. Export exists: `framework-os-dev/♻️activation/🟦️.ts:2`. |
| S5 sessions path | FIXED. Uses `playgroundSessionsRootV1` and `playgroundSessionStagedOutputPath`. | runtime `📜️script.ts:13, :22, :29`. |
| `workspace:dev-mcp-engine` (launch row) | STILL-MISSING target. Row gone with B1. No successor. | root project.json has no `dev-mcp-engine`. |
| `workspace:start` (launch "▶️start") | STILL-MISSING target (root has no `start`). Row gone with B1. | root project.json. |
| Dashboard usage text | STILL-PRESENT (doc). Example `semio run workspace:dev -- storybook` names a target that does not exist (`dev-storybook` does). | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🦀️.rs:6`. |
| Oracle `🎛️dashboard/…/🔮️oracle/🟦️.ts` L261 | FIXED (no `workspace:dev ` hit in tracked files). | `git grep`. |

### Play

| Item | Status | Evidence |
|---|---|---|
| `@semio-tech/<p>-plugin:describe` and `activate-dev` / `dist/activation/dev` | FIXED. Prototype route only. Ticket describe scripts deleted (result-play.md). | project.json:71 `dist/♻️activation/prototype`. |
| `components:complete` | NOT-A-DEFECT. In-process completion group, not a target. | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts:406`. Referenced at project.json:435 (pages job `after`). |
| `SEMIO_RENDERER` | STILL-PRESENT as dead env. Set at `🔨️modules/📦️site/📜️script.ts:58` (`SEMIO_RENDERER: "react"`). No code in tracked framework, play or demo reads it. Launch row gone. | `git grep SEMIO_RENDERER` (framework, play, demo). |
| `package.json` `bun --cwd "../.." repo run` vs AGENTS `nx` | STILL-PRESENT (decision). Same in demo. | play and demo `package.json` scripts. |
| 35 lanes hard-coded in project.json | Kept. Pinned by test `🧪️tests/🧪️playactivation`. | no action needed. |

### Course (items 1-5 of `audit-teaching-course.md` 5c)

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Proctor image build broken by `.cargo` rustc ingress in Docker context | FIXED by design, not yet run. Dockerfile is runtime-only (no cargo; receives a finite tar). `.dockerignore` excludes `.cargo`. The repo `.cargo/config.toml:6-7` ingress still applies to host cargo. | `🚀️deploy/Dockerfile:1-3`, `🚀️deploy/Dockerfile.dockerignore`. |
| 2 | Raw `spawn("cargo")` in dev stack | FIXED. Now `bun repo cargo …`. | `🛂️proctor/🏗️bootstrap/🟦️.ts:263`. No raw spawn left. |
| 3 | `bun repo` entry broken | PARTIAL. `bun repo --help` works. `run` not exercised (rule). | command output. |
| 4a | `site.legal` empty | STILL-PRESENT. `deploy-check` warns. | `🚀️deploy/🔣️.json:5` `"legal": {}`; warning `🚀️deploy/🟦️.ts:321-323`. |
| 4b | Bun version split | CHANGED. Pin is now `bun@1.4.2` (CI reads `package.json`). Local bun is 1.3.14. Mismatch persists locally. | root `package.json:132`; `bun --version`. |
| 4c | 17-18 taxonomy errors, pet-walk failures | NOT RE-VERIFIED (needs run). pet-walk dir exists: `🧪️tests/🐕️pet-walk`. | — |
| 5 | Uncommitted `.github/workflows/architecture-quiz.yml`, `.vscode/*`, staged quiz test | FIXED by commit `bd9e4e02bd0`. But that commit deleted `.vscode/` (B1). Workflow renamed to `architecture-course.yml`. | `git log -1 -- .vscode`. |
| new | Owner registry stale: `📋️build-owners.json:10701-10708` names `@teaching/architecture-quiz` with manifest `❓️quiz/…/📋️project.json`, which does not exist. No `@teaching/architecture-course` row. | `command grep` on file. Leftover `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/` holds only `dist/` and `node_modules/`. |
| new | Course `package.json` uses `bun nx run …` (AGENTS-literal). Play and demo use `bun repo run`. | three `package.json` files. Decision needed. |

## 3. Course ticket 19 (`PREPARE-ARCHITEKTUR-UND-TECHNOLOGIE-FOR-END-TO-END-DEPLOYMENT`)

- Status: `open` (ticket.json). All 40 files have mtime 12:29. No course, cargo, rustc, codex-course or Docker process is running at 12:50. Treat it as **open but stalled**.
- Done (per its own reports, not re-run by me): course diagnostics 13 files / 603 tests green; Course React 38 files / 1,032 tests green; source TypeScript zero diagnostics; deployment router, bounded E2E sessions, runtime-only image recipe; workspace cancellation 40 tests; teaching lock parity 119 records; Rust unit and production repairs (source-only, not compiled). The host used was Windows x64.
- Still open per its reports: canonical native admission. Requires Debian 12 x64 with CAP_SYS_ADMIN, loop devices, ext4 and 72 GiB. Zero runners registered. `ARCHITECTURE_COURSE_RUNNER` returns 404. No image built, published or deployed. Rust not compiled.
- Must NOT duplicate: course router, E2E sessions and ports, deploy code, Dockerfile, runbook and README, native runner admission and preflight, Rust test graph, workspace cancellation, teaching lock parity, anything under `🎓️teaching/**` or `🛂️proctor/**`.
- Shared item to coordinate, not to edit alone: B1 (`.vscode/`). It conflicts with ticket 19's launch claim.

## 4. Playwright and Docker

| Item | Value |
|---|---|
| Installed Playwright | 1.64.1 (`playwright`, `playwright-core`, `@playwright/test` in `node_modules/.bun`) |
| Expected (`node_modules/.bun/playwright-core@1.64.1/node_modules/playwright-core/browsers.json`) | chromium 1248, chromium-headless-shell 1248, ffmpeg 1013 |
| User cache `~/Library/Caches/ms-playwright` | chromium-1234, chromium_headless_shell-1234, ffmpeg-1011 (INSTALLATION_COMPLETE) |
| Repo cache `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` (repo default: `🏃️process/🌿️environment/🟦️.ts:35`) | chromium-1234, chromium_headless_shell-1234, ffmpeg-1011 |
| Missing | chromium 1248, headless shell 1248, ffmpeg 1013 (both caches) |
| Install path | play and demo: `workspace:deps-browsers` (root project.json:3189). Course: `ensureChromium` in `🎭️e2e/🟦️.ts`. Download needs your go-ahead under the download rule. |
| Docker CLI | 29.5.3 at `/usr/local/bin/docker`. Context `desktop-linux` socket `/Users/ueli/.docker/run/docker.sock` missing. `/var/run/docker.sock` missing. Daemon **not running**. |

## 5. Prioritized fix list and work packages (disjoint file ownership)

Priority order:

| Pri | Package | Owner / files | Action |
|---|---|---|---|
| P0 | WP-0 launch registry | dashboard or launch owner; `.vscode/**` | Decide: restore `launch.json` and seed from `bd9e4e02bd0^`, or confirm the dashboard metadata replaces it. Then fix AGENTS conflict. Needs your decision. |
| P0 | WP-1 deps sync | repo tooling | Let the running `dependencies lock --all` (pid 34479) finish. Then `dependencies sync`. |
| P0 | WP-2 Playwright 1248 | repo tooling, cache only | After sync: `bun repo run @semio-tech/semio-tech-play:deps-browsers` (or `workspace:deps-browsers`). Needs go-ahead (download). |
| P1 | WP-3 play | `🏢️semio-tech/🎡️play/**` | Remove dead `SEMIO_RENDERER` (`📦️site/📜️script.ts:58`). Decide `package.json` nx vs repo. Chain test: activate-prototype → serve 6033 → test-e2e. |
| P1 | WP-4 demo | `♻️mit-bestand/🧺️demonstrator/**` | No static fix blocking. Chain test: activate-prototype → serve 6029 → test-e2e. Decide `package.json` nx vs repo. |
| P1 | WP-5 course, macOS-possible subset | none (course ticket owns) | Coordinate with ticket 19: `course:dev` and `test-e2e --topology dev` on this Mac. Release topology and `preflight` need the Linux x64 runner. |
| P2 | WP-6 owner registry | `📋️build-owners.json:10701-10708` | Replace quiz row with `@teaching/architecture-course` at `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`. Verify with `bun repo preflight --scope` (not run here). |
| P2 | WP-7 stale leftovers | `🎓️teaching/🏛️architecture/❓️quiz/` | Remove generated `dist/` and `node_modules/` (no tracked files). |
| P2 | WP-8 dashboard usage text | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🦀️.rs:6` | Replace `workspace:dev -- storybook` with `dev-storybook`. Owner: repo dashboard. |
| P2 | WP-9 course decisions | ticket 19 / user | `site.legal` (`🚀️deploy/🔣️.json:5`); bun pin 1.4.2 vs local 1.3.14; Linux x64 runner registration. |

Recommended executor order: WP-0 decision, WP-1, WP-2, then WP-3 and WP-4 in parallel (disjoint trees), WP-6 and WP-8 in parallel, WP-5 only after ticket 19 agrees.

## 6. Limits

- Static reads of tracked source and `.🧬semio` ticket files only. No builds, no `bun repo run`, no Playwright or Docker runs.
- The old test counts (603, 1,032, 7 demo blocks, 35 lanes) are as reported by peers, not re-run.
- Not verified: whether `bun repo preflight --scope @teaching/architecture-course` reads `build-owners.json`; whether `course:dev` runs on macOS arm64; taxonomy and pet-walk status.
- `grep` scans in `🧰️framework` were partial (timed out at 120 s); conclusions for `SEMIO_RENDERER` and `components:complete` rely on `git grep`, which covers tracked files only.
