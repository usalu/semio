IN WORK

# 🧪️ Linux Lane s6-suites: Typecheck, Full Suites and Desktop Smoke of Staffage on g1-m04

State written 2026-11-25 12:35. `R` = `/home/deck/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket. Scripts: `U/🧰️linux/s6/`. Logs: `U/🗑️generated/linux-s6/` (prefix `s6-`). Only runs of this session are counted.

## 1. Typecheck

Two projects cover the two packages: `S/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json` (React target, its app and all 42 case files; 1494 files, 770 MB, 9 s) and `S/🌐️site/📦️packages/🟦️typescript/tsconfig.json` (site, builder, recovery, site cases, and through the app import the React target again; 2115 files, 1.35 GB, 16 to 53 s). Neither covers the other: the React project has no site file, the site project has no React case file.

| # | Command (from `R`) | Exit | Result |
| --- | --- | --- | --- |
| 1 | `repo ./📜️script.ts typecheck` in the React package | 1 | 0.1 s: `Original parent Script process invocation required` (the router only runs under `repo run`) |
| 2 | `timeout 300 repo run @teaching/staffage-react:typecheck --verbose` | 124 | graph slice read, then waits to grow its reservation from 512 MB to 8191 MB, which this host cannot grant beside the Rust build; killed by the 5 minute limit in phase `door.script` |
| 3 | `🧰️linux/s6/typecheck.sh react` (tsc 5.9.3, `--noEmit -p tsconfig.json`, under `repo capacity run --memory=3000`) | 2 | 3 errors (below) |
| 4 | `🧰️linux/s6/typecheck.sh site` | 2 | 3 errors (below) |
| 5 | `typecheck.sh react` after the repairs | 2 | 2 errors, both in the engine file |
| 6 | `typecheck.sh site` after the repairs | 2 | 2 errors, both in the engine file |

Errors and what was done:

| Where | Error | Cause | Done |
| --- | --- | --- | --- |
| `S/🎯️targets/⚛️react/🧪️tests/🥞️level-warming/🟦️.tsx` (32,52) | TS2554 Expected 2-3 arguments, but got 1 | s1's new case built `new ArchiveStore(memoryStorage())` without the view list the constructor requires | repaired: the store gets `["dark-line"]`, as the other cases do |
| `S/🌐️site/🧪️tests/🧵️flow-worker/🟦️.ts` (26,123) | TS2339 Property 'resolveId' does not exist on a member of the plugin union | s1's new case read `resolveId` on every page plugin; one of them (the serve-only plugin) has none | repaired: the case narrows with `"resolveId" in plugin` |
| `S/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts` (254,101) and (254,108) | TS7006 Parameter 'value' / 'i' implicitly has an 'any' type | not s1 or s5: the line is in `loftShapeIssues`, an uncommitted addition to the engine written at 12:15:09 today by another lane (after `vector(section.at, 3)` the centre is narrowed so far that `map` has no typed callback) | not touched: engine (`🔨️modules`) is outside this lane's scope |

So the typecheck of both projects is **red by one engine line that is not mine to edit**; the site and the React target themselves have no error left.

## 2. Suites

### React target (`S/🎯️targets/⚛️react/🧪️tests`, 42 case files)

Command per file, from `R` (driver `🧰️linux/s6/suite.sh react <case folder>...`, which calls `🧰️linux/s1/react-test.sh`): `SEMIO_BUILD_MODE=prototype SEMIO_TEST_LEVEL=quick repo capacity run --memory=2500 -- timeout 1500 bun --cwd <React package> node_modules/vitest/vitest.mjs run --config S/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts /<case folder>/ --reporter=default --reporter=json`. Every run selected exactly one file. Counts are read from Vitest's JSON report (`s6-react-<case>.json`), log `s6-react-<case>.log`.

| Case file | Exit | Passed | Failed | Skipped | Seconds |
| --- | --- | --- | --- | --- | --- |
| `active-chrome` | 0 | 2 | 0 | 0 | 5 |
| `address-roundtrip` | 0 | 18 | 0 | 0 | 3 |
| `air-controls` | 0 | 16 | 0 | 0 | 7 |
| `asset-findings` | 0 | 6 | 0 | 0 | 3 |
| `catalog-payload` | 0 | 6 | 0 | 0 | 3 |
| `clock-determinism` | 0 | 8 | 0 | 0 | 3 |
| `clock-frame-budget` | 0 | 6 | 0 | 0 | 5 |
| `collections-favorites` | 0 | 4 | 0 | 0 | 3 |
| `compare-state` | 0 | 6 | 0 | 0 | 2 |
| `custom-element` | 0 | 3 | 0 | 0 | 3 |
| `detail-sections` | 0 | 10 | 0 | 0 | 5 |
| `downloads` | 0 | 8 | 0 | 0 | 3 |
| `drawing-transport` | 0 | 4 | 0 | 0 | 2 |
| `favorites-persistence` | 0 | 7 | 0 | 0 | 3 |
| `figure-render` | 0 | 5 | 0 | 0 | 4 |
| `flow` | 0 | 9 | 0 | 0 | 10 |
| `flow-downloads` | 0 | 6 | 0 | 0 | 21 |
| `flow-style` | 0 | 2 | 0 | 0 | 2 |
| `grid-scroll` | 0 | 16 | 0 | 0 | 21 |
| `language-selection` | 0 | 10 | 0 | 0 | 5 |
| `level-warming` | 0 | 3 | 0 | 0 | 3 |
| `model-controls` | 0 | 16 | 0 | 0 | 8 |
| `model-downloads` | 0 | 16 | 0 | 0 | 166 |
| `model-lateral-rest` | 0 | 3 | 0 | 0 | 6 |
| `model-view` | 0 | 26 | 0 | 0 | 12 |
| `model-viewer-page` | 0 | 7 | 0 | 0 | 9 |
| `mount-archive` | 0 | 1 | 0 | 0 | 4 |
| `playground-compose` | 0 | 6 | 0 | 0 | 3 |
| `playground-drag` | 0 | 3 | 0 | 0 | 4 |
| `playground-tabs` | 0 | 14 | 0 | 0 | 15 |
| `playground-timeline` | 0 | 19 | 0 | 0 | 4 |
| `range-span` | 0 | 10 | 0 | 0 | 4 |
| `search-filter-sort` | 0 | 14 | 0 | 0 | 2 |
| `settings-focus` | 0 | 2 | 0 | 0 | 5 |
| `shared-borders` | 0 | 8 | 0 | 0 | 3 |
| `state-budget` | 0 | 3 | 0 | 0 | 5 |
| `structure-columns` | 0 | 2 | 0 | 0 | 4 |
| `svg-templates` | 0 | 3 | 0 | 0 | 1 |
| `text-reach` | 0 | 4 | 0 | 0 | 2 |
| `viewpoint-pages` | 0 | 12 | 0 | 0 | 6 |
| `viewpoints` | 0 | 35 | 0 | 0 | 7 |
| **41 files** | all 0 | 359 | 0 | 0 | |

`catalog-loading`, alone, with the runner's own budget variable (`SEMIO_TEST_BUDGET_MS`, read by `testLevelBudgetMs`; the case's limit in the file was not edited): `SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=2700000 TEST_TIMEOUT_S=3300 suite.sh react 📚️catalog-loading`.

| Case file | Exit | Passed | Failed | Skipped | Seconds |
| --- | --- | --- | --- | --- | --- |
| `catalog-loading` | 1 | 16 | 1 | 0 | 1081 |

- The failing case is "loads the authored archive as one ready row per manifest entry with an empty manifest check". It ran to its end in **1075 s**: with its own limit of 300 s (`testLevelBudgetMs("quick")`) it cannot end on this host, whatever the result. That part is a limit of this host (the check of 375 assets on a swapping 12 core machine), not a defect.
- It then failed on content: `expected { ready: 374, invalid: 1 } to deeply equal { ready: 375 }`. The invalid asset is `hiker-with-trekking-poles`, with six findings `extent-front-depth` / `extent-plan-depth` (front elevation and plan at 1:200, 1:100, 1:50: lateral extent 0.477 to 0.478 m against the declared 0.4989 m).
- Reproduced apart from the React target: `repo capacity run --memory=1500 -- timeout 300 bun 🧰️linux/s6/asset-findings.ts 👤️humans/hiker-with-trekking-poles`, exit 0, the same 6 findings in 12 s from the engine's `validateAsset` alone (log `s6-asset-findings-hiker.log`).
- **Not caused by s1 or s5, and not a host limit: catalog and engine state.** The asset document was last written 2026-11-24 19:32 and passed the release builds of 11:18 and 11:59 today (a release build rejects an invalid asset). Since then the engine was edited by another lane: `S/🧬️schema/🟦️.ts` 12:12, `S/🔨️modules/🗂️catalog/🟦️.ts`, `🧊️volume/🟦️.ts`, `🧊️volumes/🧱️rules/🟦️.ts` 12:15, `S/🔨️modules/🖨️representation/🪞️copies/🟦️.ts` 12:15, `S/🔨️modules/🧮️evaluation/🟦️.ts` 12:34 (the `loft` shape work, uncommitted). Which of these edits moved the lateral extent of this asset was not traced. Not touched: catalog and engine are outside this lane's scope.

Sum of the React target: 42 files, 375 passed, 1 failed, 0 skipped.

### Site (`S/🌐️site/🧪️tests`, 6 case files)

Command per file, from `R` (driver `suite.sh site <case folder>...`, which calls `🧰️linux/s1/site-test.sh`): `SEMIO_BUILD_MODE=prototype SEMIO_TEST_LEVEL=long repo capacity run --memory=2500 -- timeout 1200 bun --cwd <site package> node_modules/vitest/vitest.mjs run --config S/🌐️site/🧪️tests/🎚️config/🟦️.ts --configLoader native /<case folder>/ --reporter=default --reporter=json`. Level `long`, so that the gated cases run.

| Case file | Exit | Passed | Failed | Skipped | Seconds |
| --- | --- | --- | --- | --- | --- |
| `catalog-release` | 0 | 2 | 0 | 0 | 25 |
| `host-document` | 0 | 5 | 0 | 0 | 2 |
| `stale-chunk` | 0 | 7 | 0 | 0 | 2 |
| `flow-worker` | 0 | 2 | 0 | 0 | 2 |

Running: release build, then `label-fit`, the smoke and `model-split`.

2026-11-25 19:15: this lane was taken over on the Mac (the Linux worker g1-m04 stopped at the usage limit around 13:10). Report: `site-mac.md`.
