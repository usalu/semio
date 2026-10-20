# Audit: end-to-end readiness of course, play and demonstrator (2026-10-20)

Auditor `e2e-audit`, read-only, static. Tree checked 2026-10-20. Chain traced: `e2e-tools/run-e2e.sh` (nx default, plus `--tool=bazel`).
Not run: builds, activations, `bun repo run`, `bun repo show projects`, tests, Docker, Playwright. Reads and `git grep`/`git ls-files` only.

## 0. Verdict

| Product | Chain (default flags) | Static verdict | Blocking items |
|---|---|---|---|
| course, `--topology dev` | `test-e2e -- --topology dev --` | Ready (macOS). Proctor cargo build is the unmeasured cost | none found |
| course, `--topology rehearsal|both` | needs original release mode | Ready only with `--release` (by design) | none found |
| play (:6033) | `activate-prototype` → `serve` (smoke) → `test-e2e` | Ready | `--release` breaks it (B1) |
| demonstrator (:6029) | `activate-prototype` → `serve` (smoke) → `test-e2e` | Ready | `--release` breaks it (B1) |
| `--tool=bazel` | same chain, `bun repo --tool=bazel …` | Not verifiable statically | see section 5 |

Only one hard break found in the launcher: `run-e2e.sh --release` (B1). Two risks are real but unverified: the 60 s session wait (R1) and Bazel process teardown and nested cargo (section 5).

## 1. Status of the 10-19 audit items

| Item | 10-19 status | Now |
|---|---|---|
| B1 `.vscode/` launch rows gone | blocking for AGENTS | still gone (`ls .vscode`: no such directory). Does not block the e2e chain; AGENTS conflict stays open |
| B3 Playwright chromium 1248, ffmpeg 1013 missing | blocking | RESOLVED. Repo cache `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/` has `chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013`, all with `INSTALLATION_COMPLETE` (dirs dated 2026-10-20 11:09). `run-e2e.sh` browser check matches these names (`run-e2e.sh` `browser_report`) |
| B4 Docker daemon down | course image steps blocked | RESOLVED (`docker info` ok). Not on the test-e2e path |
| B5 dependency sync running | blocking | node_modules present (`@semio-tech/*` symlinks dated 05:17); `node_modules/playwright/cli.js` present (Playwright 1.64.1) |
| WP-6 owner registry quiz row | stale | RESOLVED. `📋️build-owners.json:10991` names `@teaching/architecture-course`; no quiz row left |
| `SEMIO_RENDERER` dead env | cleanup | still present at `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts:58`; not on the e2e chain |
| bun pin 1.4.2 vs local 1.3.14 | decision | unchanged; not checked here |

## 2. Chains traced

Common preflight in `run-e2e.sh`: `node_modules/playwright/cli.js` (present), `df` free space (≥15 GB), port 6033/6029 free only when `serve=1` (play/demo default), `check_vite_config` (play and demo only; loader `native` matched by `configLoader: "native"` in both runtime scripts, lines 48 / 64, count 2 each), browser check (all three present now).

### 2a. Play `@semio-tech/semio-tech-play`

1. `repo run @semio-tech/semio-tech-play:activate-prototype` (`🏢️semio-tech/🎡️play/📋️project.json:77-136`). `dependsOn`: `prepare-prototype` (line 19) and 35 `@semio-tech/framework-os-dev:activate-<lane>-react-prototype`. Runs `🔨️modules/🧩️runtime/📜️script.ts activate` with `SEMIO_BUILD_MODE=prototype` (fixed by `fixedTargetBuildMode`).
   - Lane check: all 35 `prepare-<lane>` and `activate-<lane>` names resolve to a `playground` row in a live Cargo.toml (`git grep 'variant = "<lane>"'`, one live row each, `.🧬semio` ticket copies excluded). The `framework-os-dev` project exists (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`). Targets are inferred (`🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`), so existence is confirmed by source, not by a generator run.
2. `repo run …:serve` (`project.json:169-191`, `dependsOn activate-prototype`, continuous). Runtime `serve` (`🔨️modules/🧩️runtime/📜️script.ts:40-51`) uses `SEMIO_TECH_PLAY_PORT ?? 6033` (line 48). `run-e2e.sh` polls `http://127.0.0.1:6033/` with `curl -fsS`.
3. `repo run …:test-e2e` (`project.json:561-573`, `dependsOn serve-e2e`, cache false):
   - `serve-e2e` (`project.json:506-523`, continuous, `dependsOn prepare-e2e`) → runtime `serve-test` (port 0, publishes `ready.json` through the service session, `runtime/📜️script.ts:64-77`).
   - `prepare-e2e` (`project.json:492-505`, `dependsOn activate-prototype` and `workspace:deps-browsers`) → runtime `prepare-test` (`runtime/📜️script.ts:54-61`).
   - `workspace:deps-browsers` (root `📋️project.json:3738-3750`) → `⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts:46`: `playwright install chromium` into the repo cache. Already satisfied, so ~0 s.
   - `🔨️modules/🧪️e2e/📜️script.ts test` → `🟦️.ts:TestScript` (`🔨️modules/🧪️e2e/🟦️.ts:14-38`): reads the session (`playE2eInvocationPid` needs `SEMIO_BUILD_INVOCATION_PID`, set by `🏗️build/📜️script.ts:103`), waits with `waitForServiceReady` (60 s, see R1), spawns Playwright with `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`.
   - Playwright config: `testDir` `🧪️tests` (exists), `testMatch` `🎭️acceptance/🟦️.ts` (exists), one `chromium` project, `workers: 1`, `PLAYWRIGHT_BASE_URL` required (set by the gate). Bare import `@semio-tech/repo-lib/testing` resolves (`./testing` export → `🧪️testing/🟦️.ts` → `playwrightTestTimeoutMs` re-exported from `library/🧪️testing/🎚️levels/🟦️.ts`, exists).
- Imports: every relative import in `🔨️modules/🧩️runtime/**`, `🔨️modules/🧪️e2e/**`, `🔨️modules/📦️site/**`, `🏗️builder/**`, root `📜️script.ts` resolves (checked file by file). Bare import `@semio-tech/framework-plugin/browser-bundle/distribution/inventory` is exported (`🔌️plugin/📦️packages/🦀️rust/package.json:29`).

### 2b. Demonstrator `@semio-tech/mit-bestand-demonstrator`

1. `repo run …:activate-prototype` (`♻️mit-bestand/🧺️demonstrator/📋️project.json:155-181`): `dependsOn prepare-prototype` (line 22: 9 `framework-os-dev:prepare-<variant>-react-prototype` + `framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm`) and `activate-lanes-prototype` (line 182: 3 `framework-os-dev:activate-{energy,fem3d,generator}-react-prototype` → runtime `complete`).
   - All 9 variants have live Cargo rows. All four wasm targets exist (`wasm` target present in each project.json). Project names exist: `@semio-tech/framework-surface-rs`, `framework-editor-rs`, `framework-os-node-graph-rs`, `@semio-tech/puzzle-plugin`.
2. `repo run …:serve` (line 207, dashboard port 6029 via `MIT_BESTAND_DEMONSTRATOR_PORT`, runtime fallback `?? 6029` at `runtime/📜️script.ts:64`).
3. `repo run …:test-e2e` (line 264) → `prepare-e2e` (231) → `workspace:deps-browsers`, `serve-e2e` (245) → `test-e2e` runtime `serve-test` / `prepare-test`, e2e `🔨️modules/🧪️e2e/📜️script.ts test`.
   - Config `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`: `testDir` `🧪️tests` and `testMatch` `🎭️acceptance/🟦️.ts` exist (`🧪️tests/🎭️acceptance/🟦️.ts`).
   - Runtime imports `@semio-tech/framework-os-dev/activation` (export at `framework-os-dev/📦️packages/🟦️typescript/package.json:55`); `playgroundSessionStagedOutputPath` (`♻️activation/🟦️.ts:19`) and `playgroundSessionsRootV1` (`:97`) exist.

### 2c. Course `@teaching/architecture-course` (dev topology)

1. `repo run @teaching/architecture-course:test-e2e -- --topology dev --` (`🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json:231-255`, `forwardAllArgs: true`, `dependsOn` `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`; both exist). Router `📜️script.ts:131` → `TestEndToEndScript` (line 71) → `runCourseEndToEnd` (`🎭️e2e/🟦️.ts:333`). `endToEndPlan` (line 85) parses `--topology dev` and the `--` terminator correctly.
2. `courseOriginalBuildModeV1(invocation, "dev")` (line 335) accepts any original mode (`🏗️builder/🎚️mode/🟦️.ts:7-13`).
3. `ensureBrowser` (line 293): chromium 1248 and headless shell already in `PLAYWRIGHT_BROWSERS_PATH` (repo cache via `repoToolCacheEnv`), so no download.
4. `buildProctor` (`🛂️proctor/🏗️bootstrap/🟦️.ts:270`; launch at 259-263): `bun repo cargo --store=shared --configuration=… -- build --locked --package teaching-proctor --bin proctor`. Checked: `🎓️teaching/Cargo.toml` members exist; `🛂️proctor/📦️packages/🦀️rust/Cargo.toml` `[[bin]] path = "../../🏗️bootstrap/🦀️.rs"` exists; `[lib] path = "🦀️.rs"` exists; `🎓️teaching/Cargo.lock` exists (16:17 Oct 19); every `path =` dependency resolves. Toolchain: `cargo 1.99.0-nightly` (the `.cargo/config.toml` flags need nightly), `node v24.15` (needed by the gate's `spawn("node", …)`, `🎭️e2e/🟦️.ts:145, :307`).
5. Dev stack: proctor on a fresh loopback port with `PROCTOR_MODE=development` (line 237); site via `process.execPath [script, "dev-site"]` (line 264) → router `dev-site` → `runViteBunxDev` with `config: ../../🏗️builder/🌐️vite/🟦️.ts` (exists).
6. Playwright config `🎭️e2e/🎚️config/🟦️.ts`: `testDir` = `📚️course/🧪️tests` (exists). All 14 `testMatch` entries exist (`🚀️site-boot`, `🪪️first-visit`, `🥞️layered-home`, `🎯️quiz-runs`, `⛰️challenge-levels`, `🏆️live-leaderboard`, `🗣️both-languages`, `📱️phone`, `📐️adaptive-layout`, `👥️shared-presence`, `🔌️connection-shortage`, `📴️proctor-away`, `🛟️proctor-faults`, `🐕️pet-walk` and its two sub-specs). Imports of `🎭️e2e/**`, `🧱️stack`, `🚀️deploy/🌐️site`, `🛂️proctor/🏗️bootstrap` resolve; named imports resolve (one `export *` re-export checked by hand).
7. Rehearsal (not in the default): `courseOriginalBuildModeV1(…, "release")` requires the original mode to be release (`🏗️builder/🎚️mode/🟦️.ts:9`), then `buildViteArtifact` and `serveStaticSite`. Not run.

## 3. Broken and risky links (file:line, fix)

B1 (blocking for play and demonstrator, only with `--release`). `e2e-tools/run-e2e.sh:207` exports `SEMIO_BUILD_MODE=release` for the whole run, before the `activate` step at line 210. `activate-prototype` is fixed to prototype (`🧰️…/🏗️build/🎚️mode/🟨️.mjs:4-8`: `^activate…-prototype$`), and an inherited release mode then throws `Conflicting build modes: prototype, release` (`🧰️…/🏗️build/🧭️invocation/🟦️.ts:118`). The usage text at line 17 says the flag is for the test-e2e step only.
  Fix: export `SEMIO_BUILD_MODE=release` only on the course `test-e2e` call (course only). For play and demonstrator, reject `--release` with a usage error.

B2 (wrong default for course rehearsal, documented, not a bug). `runCourseEndToEnd` requires release for `rehearsal` and `both` (`🎭️e2e/🟦️.ts:335`). Without `--release` the gate fails at once, before any build. Keep; the run-e2e help already says so.

R1 (risk, cold start). Play and demonstrator `serve-test` publishes `ready.json` only after the Vite dev server starts. The waiter (`🔨️modules/🧪️e2e/🟦️.ts:25` in play, `🔨️modules/🧪️e2e/🟦️.ts:16` in demonstrator, both under `🏢️semio-tech/🎡️play/` and `♻️mit-bestand/🧺️demonstrator/`) calls `waitForServiceReady` with the default 60 s timeout (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session/🟦️.ts:99`). A cold Vite start with ~35 or ~9 wasm lanes can exceed that.
  Fix: pass an explicit timeout (≥ 600 000 ms, or the `E2E_SERVE_TIMEOUT` budget) at both call sites.

R2 (cosmetic). Course test output goes to `<ticket>/🗑️generated/nx/e2e/` even under Bazel, because `ticketOutput` hardcodes `"nx"` (`🎫️output/🟦️.ts:27`). Play and demonstrator write `<ticket>/🗑️generated/play-e2e/…` and `enduser-…`. Fix: drop the `"nx"` segment or make it the tool name. Not blocking.

R3 (risk, teardown). `stop_serve` (`run-e2e.sh:81-95`) kills the process group of `bun repo … run serve`. Under Bazel the server may run outside that group. The script already warns when the port still answers (line 95). Verify on the first Bazel run with `lsof -nP -iTCP:6033 -sTCP:LISTEN`.

R4 (risk, dependency). Root `📋️project.json` target `workspace:deps-browsers` (line 3738) is `cache:false` with no `no-sandbox` (`🔧️tools/🏗️bazel/📜️script.ts:460-470`). Browser writes into `.🧬semio` may fail under Bazel's sandbox. Browsers are already present, so this only matters after a cache wipe.

R5 (risk). The course gate builds the proctor by calling `bun repo cargo` (`🛂️proctor/🏗️bootstrap/🟦️.ts:263`), which is itself a nested `bun repo` call. Under `--tool=bazel` it runs inside a Bazel action (`test-e2e` is `cache:false`). Verify that nested invocation does not block on the Bazel server lock.

R6 (not blocking). The course router still uses `spawn("node", …)` (`🎭️e2e/🟦️.ts:145, :307`, `📜️script.ts:40`). node is present at `/opt/homebrew/bin/node`. Record this as a host requirement.

R7 (not blocking). AGENTS rule conflict, unchanged: `.vscode/` launch rows are gone (`git show bd9e4e02bd0 --stat -- .vscode`, from 10-19).

## 4. Cold duration estimates (per phase)

These are estimates on this Mac (M-series, 10 cores, 32 GB). Measured numbers are marked.

| Phase | Cold | Warm (cache hit) | Basis |
|---|---|---|---|
| run-e2e preflight (all products) | <1 min | <1 min | static; `df`, vite config load |
| play `activate-prototype` (35 lanes, wasm) | about 3 h | minutes | measured by peers, memory `project-external-sweep-breaks-nx-tooling` and `feedback-wasm-mutex-fixed-stamp-slots` (cited in `audit-activation-compile-1019.md:149`) |
| play `serve` boot (Vite, ~20+ wasm plugins) | 1-3 min | 30-60 s | estimate; memory `feedback-flaky-os-dev-preview` |
| play `prepare-e2e`, `serve-e2e` | 1-2 min | <1 min | estimate |
| play `test-e2e` (11 tests, 1 worker, 600 s cap per test) | 15-30 min | same | estimate, not measured |
| demonstrator `activate-prototype` (9 lanes + 4 wasm + 3 lanes) | about 41 min (2026-09-16) | about 11 min | measured, memory `project-demonstrator-build-and-stale-artifacts` (cited in `audit-activation-compile-1019.md:148`) |
| demonstrator `serve` boot | 1-3 min | 30-60 s | estimate |
| demonstrator `test-e2e` (7 test blocks, 8 panes, GPU) | 10-20 min | same | estimate |
| course `workspace` deps-browsers | 0 (present) | 0 | measured now |
| course proctor `cargo build --bin proctor` (teaching-proctor closure) | 10-25 min | 1-3 min | estimate, not measured |
| course dev-site (Vite) start | 1-2 min | 30 s | estimate |
| course `test-e2e --topology dev` (14 projects, 1 worker, pets last) | 20-40 min | same | estimate |
| course `--topology rehearsal` (release Vite build + specs) | 35-60 min | 25-45 min | estimate |

Run-e2e default for play and demonstrator also starts a smoke `serve` on 6033/6029, which adds one more Vite instance and 1-3 min. Pass `--no-serve` to skip it.

## 5. What differs for `--tool=bazel`

Verified by reading `🧰️…/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` (not run):
- `run-e2e.sh` passes `--tool=bazel` as `bun repo --tool=bazel …` on every call, including `workspace:deps-browsers` and the smoke `serve`. This form is supported (help catalog, `🔣️catalog.json:203`). The Bazel version is pinned in `.bazelversion` (8.4.2). The `bazel` binary is not on PATH; provision with `bun repo tools setup all` first.
- The plan is turned into `BUILD.bazel` under `storage.paths.workspace/<namespace>` with one `//:<label>` action per selected task (lines 533-540). Uncached tasks (`cache:false`) get `no-cache` and `no-remote-exec` (`script.ts:460-470`).
- Continuous tasks are launched with `bazel run` before the build (`script.ts:642-650`). Readiness: `serve` (play, demonstrator) uses its `dashboard.ready` port (6033 / 6029, `resolveReadinessV1` falls back to `port` when `portEnv` is unset). `serve-e2e` declares no readiness, so it counts as ready on `[bazel] task-started`. The test then waits on `ready.json` (see R1).
- Environment: each task gets its environment from `runtime.json`, captured at plan time from `scriptInvocationEnvironment(invocation, process.env)` (`script.ts:393-402`). `SEMIO_TICKET_DIR`, `SEMIO_BUILD_INVOCATION_PID` and `PLAYWRIGHT_BROWSERS_PATH` reach the tasks this way. `SEMIO_BUILD_TOOL=bazel` is set on every task (`script.ts:431`).
- Teardown: see R3. Bazel-specific: `bazel run` children may outlive the client.
- The course gate nests `bun repo cargo` inside a Bazel action: see R5.

Not verified (needs a Bazel run): the `workspace:deps-browsers` sandbox behaviour (R4); whether `serve-e2e`'s early "ready" changes the first Playwright attempt; the Bazel cache hit rates for the 35-lane activation.

## 6. Limits

- Static only. No builds, no activations, no `bun repo run`, no tests, no Playwright, no Docker, no `bun repo show projects`.
- Inferred lane targets (`@semio-tech/framework-os-dev:*-react-prototype`, `@semio-tech/plugin-registry:session-*`) are confirmed by the Cargo playground rows, not by running the generator.
- Duration estimates in section 4 are not measured except where marked.
- Git was read-only (`git grep`, `git ls-files`, `git show --stat`).
- Bun is local 1.3.14 against a pin of 1.4.2 (not re-checked).
