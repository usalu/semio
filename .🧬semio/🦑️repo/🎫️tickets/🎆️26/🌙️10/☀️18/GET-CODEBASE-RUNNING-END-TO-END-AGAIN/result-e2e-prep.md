# result-e2e-prep: end-to-end runs for play, demonstrator and course (2026-10-19, ~13:45)

Executor `e2e-prep`. Static only: no activation, serve, Playwright, cargo, `show projects`, download. Inputs read: `brief-1019.md`, `audit-products-1019.md`, `result-bzl-man.md`, `result-demo.md`, `result-play.md`, codex ticket 19 (`runbook-final.md`, `readiness-status.md`; not touched). `result-qe2e.md` does not exist.

## 1. Findings that matter before the first run

| # | Finding | Status |
|---|---|---|
| F1 | **Stale imports broke the e2e scripts.** `play` and `demonstrator` `🔨️modules/🧪️e2e/📜️script.ts` (`repoCacheDirectory`), both root `📜️script.ts` (`runVitest`) and the demonstrator Vite config (`repoCacheDirectory`) imported from the retired path `🧰️framework/📦️packages/🟦️typescript/🛍️products/🦑️repo/…`. `bun build` said "Could not resolve", so `test-e2e` would have died at load. | FIXED (5 files, one specifier each, re-pointed to `🧰️framework/🛍️products/🦑️repo/…`) |
| F2 | The same class sits in the **activation chain**: `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts` (3 specifiers; reached by play/demo through the puzzle `:wasm` dependency). | FIXED (3 specifiers) |
| F3 | 66 further unresolved specifiers of the same class remain in 62 files: ~40 hub `🦀️rust/📜️script.ts`, `✏️s/🧑‍💻dev/**`, `🌎️hub/🧩️compositions/🗄️stdio/**`, `♻️mit-bestand/🎤️präsentation/**`, and `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts` (codex-owned; `@teaching/proctor:build/test` cannot load until fixed). None is in the explicit target closure of the three products (see section 3), so I left them. One fail-closed command repairs all: `bun e2e-tools/stale-imports.ts --apply` (rewrites a specifier only if the new target is an existing file; "0 stay unresolved" in the dry run). | OPEN, coordinator decides (suggest running it now) |
| F4 | Playwright browsers: expected by `playwright-core@1.64.1` (`browsers.json`): chromium 1248, chromium-headless-shell 1248, ffmpeg 1013. Repo cache `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` holds 1234/1234/1011 only. All test scripts pin `PLAYWRIGHT_BROWSERS_PATH` to exactly this directory. | NOT INSTALLED (no download made) |
| F5 | `test-e2e` of play and demonstrator is self-contained: it depends on `serve-e2e` (continuous, own port-0 server, session file under `dist/services/e2e`) which depends on `prepare-e2e` (`activate-prototype` + `workspace:deps-browsers`). The dev serve on 6033/6029 is **not** needed by the tests; `run-e2e.sh` still starts it (as asked) as a smoke and runs it concurrently with the test server (extra load; `--no-serve` drops it). | by design |
| F6 | Course has **no** `activate-prototype`/`serve`. `test-e2e` boots its own throw-away stacks (fresh OS ports, own proctor data, dev site via `bun 📜️script.ts dev-site`). Dev topology needs the proctor built by cargo (minutes; shared cargo store) and installs Chromium itself via `ensureBrowser` (this is a download hidden inside the gate, so run `workspace:deps-browsers` first). Dashboard default mode is `release`; `--topology dev` accepts prototype or release. The rehearsal topology needs `SEMIO_BUILD_MODE=release` and the release-site build. | by design |
| F7 | Argument shape for the course gate (verified in `selectBuildTasksV1` and `endToEndPlan`): everything after the first `--` is forwarded; the gate needs `--topology dev --` (second `--` starts Playwright arguments). `bun repo run @teaching/architecture-course:test-e2e -- --topology dev --` | used by the script |
| F8 | `--tool=` goes before `run`: `bun repo [--tool=nx|bazel] run …` (from `bun repo --help`). | used by the script |

## 2. Static checks of specs and configs against current source

Method: `e2e-tools/check-selectors.ts <product>` extracts every `data-*` hook, `data-slot` value, `[id=…]`, `.semio-*` class and fixture key from the specs and requires an occurrence in non-test source (tracked and untracked `ts/tsx/rs/json/css/html`, 49,079 files, tests/e2e/node_modules/dist excluded). Plus `bun build --target=bun --packages=external` on every entry (resolves all relative imports), plus targeted reads.

| Product | Tokens | Found | Not found (all explained) |
|---|---|---|---|
| play (`🧪️tests/🎭️acceptance` + 2 fixtures) | 58 | 47 | 4 `${…}` template placeholders; 4 `domPaintCases` ids (fixture-internal); `[data-host-snapshot-json]` (CSS form of a found attribute); `procedural-play-inspector.value.input` and `.output.volume` are composed by `format!("{prefix}.value.input")` / `"{prefix}.output.{port}"` in the generation3d inspection panel (`prefix = "procedural-play-inspector"`, verified in the Rust source) |
| demonstrator | 38 | 35 | `data-fixture-json` (only named in a history comment), `demonstrator-pane-card-open-chip` (composed: `OverviewCardOpenChip` renders `data-slot={`${slot}-open-chip`}` with `slot="demonstrator-pane-card"`), `${elementId}` placeholder |
| course (`🎭️e2e/🚶️learner` + 16 spec dirs) | 81 | 81 | none |

Further checks:
- Play: catalog `🔨️modules/🧩️runtime/🔣️.json` = 18 groups, 149 panes, 122 with `exampleLabel`; overview card aria-label `Open {{label}}: {{tagline}}` (`⚛️play-card.tsx`) matches the spec regex; `{{apps}} apps` count label, `play-language-choice` with `English`/`Deutsch` buttons, hash deep link (`LayeredOverview`: `location.hash`/`hashchange`, `data-layered-pane={pane.id}`) all present. Config: `testDir 🧪️tests`, `testMatch 🎭️acceptance/🟦️.ts`, base URL from `PLAYWRIGHT_BASE_URL` set by `test-e2e` from the service session; `@semio-tech/repo-lib/testing` exports `playwrightTestTimeoutMs`; `pngjs` and `@playwright/test` installed. `SEMIO_RENDERER` is already gone from the site script (prod-fix).
- Demonstrator: `brandPaneIds()` regex on `🪧️brand.ts` yields exactly the 8 `PANE_CASES` ids (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik); all 17 window kind ids used by `PANE_CASES` occur in plugin sources/descriptors; `Förderhinweis`, `Weiter`, `introduction-info-box` present; port 6029 consistent (project.json, Vite config `MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029`, pipeline.json). Spec header path comment was stale (named a deleted file); corrected.
- Course: all 16 `testMatch` spec directories exist with `🟦️.ts`; `COURSE.quizzes` (5) all resolve; `/#physics` is the quiz `physics` of topic `energy` (matches the hierarchy test `quizzes -> energy -> physics`); ports are not hard-coded (OS-selected per stack; dev port 6061 from `package.json`, proctor 8791); `bun build` of the course router, e2e gate, stack, deploy, learner, config and Vite config: 0 errors. `@teaching/proctor` `bootstrap/🟦️.ts` builds clean.
- Entry closure: `e2e-tools/target-script-closure.py` walks `dependsOn` of every target of the three products over all `📋️project.json` (142 tasks, 89 inferred `framework-os-dev` tasks that have no script, 0 unresolved projects) and found 20 command scripts. `bun build` of all 20: clean after F1/F2.
- Not verifiable statically: whether the 149 + 6 play panes and the 8 demonstrator panes actually paint (needs the run), the `output.volume` port name in the generation3d analysis flow, GPU availability (`--use-angle=metal` on macOS is the default; `PLAY_E2E_GPU=swiftshader` / `DEMONSTRATOR_E2E_GPU=swiftshader` exist, the map pane cannot be graded on swiftshader).

Expect long runs: the play suite has 149 per-pane tests plus 6 generation3d (3 flows x 2 locales), the overview and the language test (157 tests, workers 1, up to 240 s each).

## 3. Changes (files)

Product files:
- `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📜️script.ts`: `repoCacheDirectory` import re-pointed.
- `🏢️semio-tech/🎡️play/📜️script.ts`: `runVitest` import re-pointed.
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📜️script.ts`: `repoCacheDirectory` import re-pointed.
- `♻️mit-bestand/🧺️demonstrator/📜️script.ts`: `runVitest` import re-pointed.
- `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts`: `repoCacheDirectory` import re-pointed (serve and e2e Vite config).
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`: header path comment only.
- `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`: 3 imports re-pointed (admission, vitest, wasm policy).

No Playwright config, spec logic, `project.json`, `package.json`, `.vscode` or ticket-19 file was edited.

Ticket tools (`C/e2e-tools/`, inputs, not permanent tooling):
- `run-e2e.sh`: the runner (section 4).
- `check-selectors.ts`: the selector audit (section 2).
- `stale-imports.ts`: detects and (with `--apply`, optional path prefixes) repairs the F1/F3 class.
- `target-script-closure.py`: lists the command scripts in a product's target closure.

## 4. `C/e2e-tools/run-e2e.sh <product>`

`run-e2e.sh <play|demonstrator|course> [--tool=nx|bazel] [options] [-- <extra>]`, bash 3.2 compatible (macOS), logs under `C/🗑️generated/e2e-<product>-<stamp>-{browsers,activate,serve,warm-proctor,test-e2e}.log`, `SEMIO_TICKET_DIR=C` exported so Playwright reports and the course run directory land inside the ticket's generated folder.

Chain, play (6033) and demonstrator (6029):
1. preflight: `node_modules/playwright/cli.js` exists (dependency sync green), free disk >= 15 GB (`E2E_MIN_FREE_GB`), serve port free, Playwright browsers for the revisions in `playwright-core/browsers.json` present in the repo tool cache.
2. `bun repo [--tool] run <pkg>:activate-prototype`
3. `bun repo [--tool] run <pkg>:serve` started detached in its own session (`perl setsid`), log `…-serve.log`; polled every 3 s until `http://127.0.0.1:<port>/` answers (limit `E2E_SERVE_TIMEOUT`, default 1800 s); aborts early if the serve process dies.
4. `bun repo [--tool] run <pkg>:test-e2e`
5. stops the serve process group (TERM, then KILL after 30 s), also on Ctrl+C/TERM/any failure, and warns if the port still answers.
Course: `bun repo [--tool] run @teaching/architecture-course:test-e2e -- --topology dev [--keep] [--proctor <exe>] -- [playwright args]`; no activate and no serve by default (`--serve` adds a detached `course:dev` on 6061; `--warm-proctor` runs `@teaching/proctor:build` first, which needs F3 for the proctor script).

Options: `--check` (preflight only), `--install-browsers`, `--skip-activate`, `--no-serve`, `--serve`, `--topology=dev|rehearsal|both`, `--proctor=`, `--release` (exports `SEMIO_BUILD_MODE=release`), `--keep`, `--warm-proctor`. Exit codes: test-e2e's own code on test failure; 3 preflight, 4 browsers missing, 5/6 serve died/timeout, 10 activate/warm failure. A summary table with per-step exit and seconds is printed at the end and on every abort.

Verification of the script itself: `bash -n` clean; `--help`; `play --check` on the real tree (reports the three MISSING browser directories and exits 4, which is the intended gate); a full dry run with a stub `bun` on PATH (stub answers activate, serve via `python3 -m http.server`, test-e2e): activate -> serve ready after 3 s -> test -> serve process group stopped and port closed (exit 0); failing test (exit 3) propagated and serve stopped; course call produced `… test-e2e -- --topology dev --keep -- --grep foo`. The real chain was not run.

Browser install (not run, needs the go-ahead because it downloads about 300 MB): `bun repo run workspace:deps-browsers` (executes `playwright install chromium` with `PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`; bazel: `bun repo --tool=bazel run workspace:deps-browsers`). Required afterwards: `chromium-1248`, `chromium_headless_shell-1248`, `ffmpeg-1013`, each with `INSTALLATION_COMPLETE`. `run-e2e.sh <product> --check` verifies exactly that; `--install-browsers` runs the command for you. The revisions are read from the installed `playwright-core`, so they follow the dependency sync (re-check after it finishes; 1248 is for 1.64.1).

## 5. Run order for the coordinator (after the sync is green)

1. `bun e2e-tools/stale-imports.ts` (dry run, expect 0 unresolved), then `--apply` (optional, F3).
2. `bun repo run workspace:deps-browsers` (go-ahead for the download), then `e2e-tools/run-e2e.sh <p> --check`.
3. `e2e-tools/run-e2e.sh demonstrator` (smallest: 8 panes), then `play`, then `course --topology=dev --warm-proctor` last (cargo build; serialize it against other cargo users).
4. Add `--tool=bazel` for the Bazel pass; the runner is tool-neutral.

## 6. `project.json` changes needed (bzl-man owns them; none applied)

None blocks the run. Recommended:
- `@teaching/architecture-course:test-e2e`: add `"workspace:deps-browsers"` to `dependsOn`, as play/demonstrator `prepare-e2e` do, so the Chromium download is owned by the build graph instead of the gate's inner `ensureBrowser`. The dashboard `execution-mode` parameter pins `SEMIO_BUILD_MODE=release` (needed only for the rehearsal topology); for `--topology dev` it is unnecessary.
- Optional: play/demonstrator `test-e2e` could declare `dependsOn` order notes in metadata so Bazel treats `serve-e2e` as a service, if the adapter does not already (unverified, Bazel never executed).

## 7. Open items

- F3: 66 stale imports in 62 files (command above); proctor script is codex-owned.
- A language-agnostic law that fails when any target command script (from `target-script-closure.py`) has an unresolved relative import would have caught F1/F2; not added (permanent test belongs to the repo test owner).
- Browsers 1248/1013 not installed; Docker daemon still down (course image steps unaffected for the dev topology).
- `.vscode/` retirement and `package.json` nx-vs-repo decisions are unchanged (user decisions).
- Selector audit is textual (attribute/value occurs somewhere in source), not a render proof; first run is the real test.
