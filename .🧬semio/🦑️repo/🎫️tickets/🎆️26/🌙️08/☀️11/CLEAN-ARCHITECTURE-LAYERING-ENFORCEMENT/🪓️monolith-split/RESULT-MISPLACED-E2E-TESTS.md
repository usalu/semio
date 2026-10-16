# Result: Misplaced E2E Tests (slice MISPLACED-E2E-TESTS, prefix `met`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Run of 2026-10-16. Tools: `met-tools/` in this folder. Logs and raw outputs: `🗑️generated/met-*` (kept).

Path aliases used below:

- `OS` = `🧰️framework/🛍️products/💻️os`
- `H` = `OS/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel` (the framework time-travel harness)
- `S` = `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel` (the puzzle scenario)
- `GIS` = `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition`

## Verdict

Both targets are done and verified. Nothing is committed, nothing was run against a live dev serve (none was listening on 6012, 6112 or 6033).

| Target | Before | After |
|---|---|---|
| 1. `GIS/🧪️tests/🟦️.ts`, 7,388 lines, 129 tests | 149 tests in the gis route (129 gis file + 20 framework worker file), **64 pass, 85 fail** | same 149 titles, **149 pass, 0 fail**; 25 files (+ 2 support, 1 aggregator) 24 test files of at most 968 lines plus 3 support files; 4 test files (29 tests) remain under gis |
| 2. `H/🟦️.ts`, 6,765 lines, time-travel gate | 9 type errors (8 in the file), a runtime ReferenceError and a runtime TypeError in the gate | harness (19 modules, at most 645 lines) in the framework, scenario (9 modules, at most 892 lines) in the puzzle plugin; 0 type errors in the tree; a real headless Chromium run through the harness passed |

## Baseline (recorded before the first edit)

### How each file runs

- Gis tests: in-source vitest suites (`import.meta.vitest`) registered by `GIS/👷️worker/🟦️.ts` through `registerTests1`. Config `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts` (`includeSource: [framework worker, gis worker]`). Registered routes: `bun repo run @semio-tech/gis-gismap-js:inference-service-receiving-check` (launch entry, `--testNamePattern "gis map inference port"`) and the package script `inference-check --browser`. The framework worker `OS/🔨️modules/🏪️store/👷️worker/🟦️.ts` runs under `bun repo run @semio-tech/framework-os:test` (config `OS/🧪️tests/🎚️config/🟦️.ts`). Command used here (the repo's own vitest, no gate bypass): `node node_modules/vitest/vitest.mjs run --config <config> --reporter=json`.
- Time-travel gate: `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` target `time-travel` -> `bun ./📜️script.ts verify time-travel` -> `H/../✅️verification/🟦️.ts` -> `runTimeTravelCli`. Launch entries in both JSONs: `⚖️gate⏪️time-travel⚛️react`, `⚖️gate⏪️time-travel🧊️wgpu` (puzzle2d, `--serve` :6012 and :6112) and `🧪️test📜️history🌐️universal⚛️react` / `…🧊️wgpu` (any declared variant).

### Pass/fail sets

| Run | Result |
|---|---|
| Gis vitest config | 149 tests, 64 pass, 85 fail |
| Gis `tsc` (os tsconfig, gis worker closure) | 99 error lines (70 in the gis test file, 2 in the gis worker, 27 in the foreign `📃️UiDocumentStore/🧪️tests/🧪️typedwire`) |
| Time-travel `tsc` (file + `✅️verification`) | 9 errors: 8 in the file (3 undefined names, 5 stale API uses), 1 foreign (`readPackageName`) |
| `H/🚀️boot/🧪️tests` and `H/🪞️inventory/🧪️tests` (dev vitest config) | both suites fail to load: `loadCatalogTaxonomy is not a function` (foreign) |
| `bun test` of `🌎️hub/…/🌍️gis/🧪️tests/🧱️io-boundary/🟦️.ts` | Bun 1.3.14 segfault (foreign) |
| Time-travel against a dev serve | not runnable here |

Causes of the 85 gis failures: 61 `ENOENT` (fixtures moved by the 2026-10-15 fixture-placement sweep, commit `0c3e78944af`, from `…/🧫️fixtures/…` to `…/🧪️tests/🧫️fixtures/…`; this file was never re-pointed), 10 stale lease data (`profile: "wasm-release"`; production admits `release`/`prototype`), 11 `STACK_TRACE_ERROR` in two describes (same missing fixtures), 1 stale `identityActorConfig` call and expectation (it now takes `actorIdentityGrant` as second argument), 2 more fixture misses.

## Target 1: gis service-composition tests

### Finding

The file was one function (`registerTests1`) holding 129 tests, registered from the gis worker. By subject under test, 100 of them exercise the **browser backbone worker** (`OS/…/🏪️store/👷️worker`), the Directory lane, identity config or the worker's wire handling, not the gis variant. None tests hub *code* (the hub is a server; these tests drive the worker against fake hub sockets), so no test went under `🌎️hub`. Replication is only a dependency of the worker tests (envelopes, frames), not their subject, so none went under `📡️replication`. The owners are the os worker suites (`OS/🧪️tests`), the Directory module and the os config module. 29 tests are genuinely gis (24 in the inference-port file, 5 more that use the gis inference driver's seams, gis types or gis fixtures) and stay under the gis variant.

### Layout (27 files; every test keeps its title and its describe nesting)

| File | Lines | Tests |
|---|---|---|
| `OS/🔨️modules/📇️directory/🧪️tests/🔑️session-broker-port/🟦️.ts` | 341 | 4 |
| `OS/🧪️tests/🏗️space-artifact-creation/🟦️.ts` | 216 | 7 |
| `OS/🔨️modules/📇️directory/🧪️tests/📄️event-page-bootstrap/🟦️.ts` | 129 | 4 |
| `OS/🔨️modules/📇️directory/🧪️tests/🎟️hub-session-capability/🟦️.ts` | 159 | 4 |
| `OS/🧪️tests/🚨️document-actor-recovery-worker/🟦️.ts` | 97 | 3 |
| `OS/🧪️tests/🌉️wire-bridge/🟦️.ts` | 135 | 7 |
| `OS/🧪️tests/🧱️artifact-bootstrap-restore/🟦️.ts` | 537 | 10 |
| `OS/🎚️config/🧪️tests/🪪️identity-config-facet/🟦️.ts` | 94 | 3 |
| `OS/🎚️config/🧪️tests/🔧️config-mutation-parity/🟦️.ts` | 60 | 3 |
| `OS/🔨️modules/📇️directory/🧪️tests/🏘️space-administration/🟦️.ts` | 497 | 10 |
| `OS/🔨️modules/📇️directory/🧪️tests/🛤️directory-lane/🟦️.ts` | 224 | 5 |
| `OS/🧪️tests/📂️document-open/🟦️.ts` | 384 | 7 |
| `OS/🧪️tests/🎯️execution-target-lease/🟦️.ts` | 525 | 6 |
| `OS/🧪️tests/🎭️browser-actor-reservation/🟦️.ts` | 475 | 2 |
| `OS/🧪️tests/📨️document-backbone-batches/🟦️.ts` | 577 | 7 |
| `OS/🧪️tests/👀️folder-watch/🟦️.ts` | 188 | 4 |
| `OS/🧪️tests/📴️hub-reconnect-outbox/🟦️.ts` | 296 | 6 |
| `OS/🧪️tests/👥️scope-safe-presence/🟦️.ts` | 59 | 1 |
| `OS/🧪️tests/📍️checkpoint-pair-decoder/🟦️.ts` | 99 | 3 |
| `OS/🧪️tests/🪜️document-catch-up/🟦️.ts` | 153 | 3 |
| `OS/🧪️tests/🧭️actor-command-windows/🟦️.ts` | 42 | 1 |
| `GIS/🧪️tests/💡️inference-port/🟦️.ts` | 968 | 24 |
| `GIS/🧪️tests/🗺️document-open/🟦️.ts` | 963 | 3 |
| `GIS/🧪️tests/🔁️peer-rebootstrap/🟦️.ts` | 130 | 1 |
| `GIS/🧪️tests/🛤️directory-lane/🟦️.ts` | 101 | 1 |
| `OS/🧪️tests/🧰️worker-law-fakes/🟦️.ts` (shared fakes and fixtures) | 385 | 0 |
| `GIS/🧪️tests/🧰️dependencies/🟦️.ts` (gis dependency types) | 9 | 0 |
| `OS/🧪️tests/📚️worker-law-suites/🟦️.ts` (aggregator `registerWorkerLawTests`, `workerLawSourceV1`) | 57 | 0 |

The old `GIS/🧪️tests/🟦️.ts` is removed (it was byte-identical to the snapshot `met-tools/gis-original.ts.txt` when removed).

How they run:

- The framework worker's in-source block (`OS/…/👷️worker/🟦️.ts`, +2 lines) now also calls `registerWorkerLawTests`, so `framework-os:test` and the gis route both run the 120 framework-worker tests (20 old + 100 relocated) in the framework worker file.
- `GIS/👷️worker/🟦️.ts` calls the four gis registrars with the merged gis dependency bag (`driver.test`, models, merged `testSeams`) and the OS-rebased source.
- The old file-level `beforeEach` (capability and seam reset) moved into `workerLawFakesV1(...).resetWorkerLaw`. Every relocated describe registers it as its own scoped `beforeEach`; the root-level tests call it first. It no longer leaks into the framework's unrelated 20 tests.
- New launch entry `⚖️test-worker-law-suites🖥️os🟦️` (`bun repo run @semio-tech/framework-os:test -- quick ../../🔨️modules/🏪️store/👷️worker/🟦️.ts`) in both launch JSONs, next to `⚖️test-source-cancellation-target🔌️plugin🟦️`, order `900.0365751`. The existing gis entry (`inference-service-receiving-check`, pattern `gis map inference port`) is unchanged and still selects the inference describe.

### Tests that stay in the gis variant (and why)

- `retains the preceding inference job when a successor opening is refused` and the whole `gis map inference port` describe (23 tests; gis inference worker seams `inferencePort`, driver functions).
- `backbone-worker directory lane > backbone worker owns one full scoped stream and retires it terminally on 4401` (sets the gis driver's `inferenceApprovalUndoOwner`).
- `artifact bootstrap atomic restore > browser document peers refetch the same exact pair after scoped rebootstrap` (reads the gis map peer-rebootstrap fixture).
- `backbone-worker offline resilience >` the three tests that need gis types or gis titles: `browser execution target lease verifies GIS wasm bytes before plan exchange`, `browser document actor transfers one verified cold pair only after lifecycle ACK and exact page receipts`, `browser GIS viewer exposes localized renderer-unavailable after verified lease`.

The describe wrappers (`backbone-worker offline resilience`, `artifact bootstrap atomic restore`, `backbone-worker directory lane`) are repeated in the files that split them, so every full name is unchanged.

### What changed beyond relocation (all listed, each re-proved)

1. Fixture paths re-pointed to the swept locations (`…/🧫️fixtures/…` -> `…/🧪️tests/🧫️fixtures/…`; the two administration fixtures to `📇️directory/🧪️tests/🧫️fixtures/🏛️administration/…`): 85 failures cleared.
2. Stale test data: `profile: "wasm-release"` -> `"release"` (3 places); `identityActorConfig` test updated to the current `(actor, actorIdentityGrant, dataDir?)` signature (the grant literal is the one `backbone-parity` uses).
3. Concrete plugin data neutralized in relocated framework tests (they are plain test data, proved by the tests passing): kind `s.gis.gismap` -> `s.demo.demo`, labels `GIS Map`/`GIS-Karte` -> `Demo Map`/`Demo-Karte`, `gis.map.operation` -> `demo.map.operation`, `s.cad.cad` -> `s.demo.demo`, `pluginId: "cad"` -> `"demo"`, `s.note.note@1` -> `s.demo.demo@1`, `noteShellCommand` -> `demoShellCommand`, `semio:gis` -> `semio:demo`.
4. Guards that read the old file by path now read the gis test directories: `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧱️io-boundary/🟦️.ts` (test `GIS receiving configuration selects its owner and all fixture URLs resolve`, run on its own with `bun test ./… -t "fixture URLs resolve"`: pass) and `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts` (now loops over the gis test directories, floor `> 8`; the receiving part is replayed by `met-tools/guard-check.ts`: 15 `source.url` readers and 17 `import.meta.url` readers, 0 missing).
5. Fixture reads keep the `source` convention: `workerLawSourceV1(documentWorkerTestSourceV1)` rebases the worker's URL onto `OS/🟦️.ts` exactly as the old `source` line did.

### Verification

| Check | Command | Result |
|---|---|---|
| Stage A: pure relocation, same fixture paths | gis vitest run (see Baseline) before fixing anything | 149 titles, **identical status per title** to the baseline (64 pass, 85 fail) |
| Final gis route | `node node_modules/vitest/vitest.mjs run --config <gis config> --reporter=json` (`met-final-gis.json`) | 149 tests, **149 pass**; titles identical; 85 fail->pass, **0 pass->fail** |
| Final os route, worker file | same with `--config OS/🧪️tests/🎚️config/🟦️.ts worker` | 120 tests, 120 pass (the relocated suites run in the framework route too) |
| Title and body proof | `met-tools/body-proof.ts` (AST, every `it` body with relative paths resolved, declared transformations applied) | 129 tests, 129 distinct titles, 129 present; exactly 3 body differences, all declared above (identity test; 2 administration fixture paths) |
| Region markers | `grep` | every marker of the old file present, one pair per file holding its content |
| Type-check | `tsc -p met-tools/tsconfig.gis-after.json` (same closure as the baseline) | 97 error lines vs 99: same errors, 2 fewer (the identity test); no new error |
| Path length | `python3` over the 57 new paths of both targets | longest 138 bytes (limit 240) |

## Target 2: time-travel gate

### Finding

The file is the headless end-to-end gate of non-destructive history editing: a harness (CLI, report, console capture, browser, the framework's own `framework.history.*` panel, band, editor, ARIA oracle, replay and reprojection probes), plus the plugin-neutral universal journey (step 24) and console digest (step 9), plus the **puzzle 2d journey** (steps 1 to 23: nodes, drags, rotate and scale, handles, `?plugin=puzzle2d`, ports 6012 and 6112, `data-board-*`, `dumpBoard2d`). It used 28 module-level `let`s as shared state and hard-coded the plugin in `PLUGIN_VARIANT`, `DEFAULT_SERVES`, `OVERVIEW`, `routeUrl`, `wgpuBootShape` ("2d-" windows), the editor's `dx`/`dy`, a benign-console regexp and `timeTravelServeVariantV1` (`admit("puzzle2d")`).

### Design

- **Harness** (`H`, framework, neutral): `H/🟦️.ts` keeps only `runTimeTravelCli(invocation, repoRoot, defaultOutDir, segments, scenario?)`. Children, each an emoji-prefixed module: `🧠️state` (shared run state, `export let` live bindings, one setter per variable), `🧩️scenario` (the contract `TimeTravelScenarioV1` and `TimeTravelBlockV1`), `🧭️arguments`, `🧾️report`, `🌍️copy` (framework `COPY` en/de), `🎧️console`, `📄️page`, `🧊️wgpu-mirror`, `🧱️chrome`, `📜️history`, `🎗️band`, `🖊️editor`, `♿️aria-oracle`, `🔁️replay-arm`, `🔄️reprojection`, `🧵️history-session` (opening an editor, alternatives, exit), `🌐️universal` (step 24), `📟️console-digest` (step 9), `🎪️run` (one locale pass, `runProbe`).
- **Scenario contract**: `variant`, `defaultServes`, `bootWindowPrefix`, `editorNumbers`, `paletteQuery`, `stepNumbers`, `order`, `steps`, and hooks `context`, `prepare`, `beforeStep`, `afterSteps`, `diagnostics`, `exploreInventory`. The harness appends the shared steps (`[...scenario.order, 24, 9]`).
- **Scenario** (`S`, puzzle plugin): `S/🟦️.ts` exports `puzzle2dTimeTravelScenario` plus the original 100-line docstring; children `🎲️board` (board vitals, positions, pointer gestures, the wgpu board dump, `PLUGIN_VARIANT`, `DEFAULT_SERVES`, `OVERVIEW`), `🌍️copy` (the board-specific labels merged over the framework copy), `🌄️drag-journey` (steps 1-5), `🔂️reload-check`, `🌿️alternatives` (steps 6-8), `🧮️gaps` (steps 10-15), `🛣️lanes` (steps 16-19), `🕹️interaction` (steps 20-23).
- **Run command, owned by the plugin**: `@semio-tech/puzzle-js:time-travel` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/`: new `TimeTravelScript` in `📜️script.ts`, target `time-travel` in `📋️project.json` with the dashboard renderer parameter and the `playwright` external input, script `time-travel` in `package.json`). Example: `bun repo run @semio-tech/puzzle-js:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de`. The two launch entries `⚖️gate⏪️time-travel⚛️react` and `⚖️gate⏪️time-travel🧊️wgpu` now run it (both JSONs). `@semio-tech/framework-os-dev:time-travel` stays and is universal-only: `--universal --variant <declared variant> --serve <url>`; without `--universal` or a scenario it throws a message that names no plugin.
- `H/🚀️boot` takes the scenario's variant (`scenarioVariant`), its test and fixture no longer name any plugin (catalog variants are resolved from `playgroundCatalog()`; tokens `$scenario`, `$family`, `$familyWgpuPort` in the fixture).

### Behaviour deltas (all deliberate)

1. A universal run (no scenario) no longer runs the board prerequisites, the board-blocked gating, or the (previously empty-context, no-op) reload check, and needs `--serve`. A universal wgpu run against a non-board plugin used to be blocked by missing `dumpBoard2d`; it now runs.
2. The universal boot counts every window (`bootWindowPrefix` defaults to `""`); the puzzle scenario keeps `"2d-"`.
3. The benign-console regexp is now `activate-[a-z0-9]+-react-prototype` (any variant) instead of `activate-puzzle2d-react-prototype`.
4. The editor read returns `numbers: Record<pointer, StepperRead|null>` for the pointers the scenario asks for (`["dx","dy"]`) instead of the fixed `dx`/`dy` fields; the scenario's reads were rewritten accordingly (`ed.numbers.dx`).

### Baseline defects fixed on the way (they stay fixed)

- `universalChangeOneInput` (module level) called `dismissNotices`, `settledControl`, `visibleNoticeCodes`, which were local to `stepU`: a `ReferenceError` the first time the universal journey reached a clean input. They are now module-level helpers in `🌐️universal`.
- `timeTravelServeVariantV1(…, playgroundCatalog)` passed the *function* where the catalog array is expected (`catalog.filter is not a function`, turned into a "blocked" acceptance record). Now `playgroundCatalog()`.
- `activatePlaygroundRuntime(…, "dev", …)` no longer typechecks (profiles are `prototype`/`release`) and `playgroundActivationTargetV1(variant, "dev", …)` in the boot test disagreed with the fixture's `…-prototype` trace: both use `"prototype"`.
- `variant: null` in the acceptance `measured` records (not assignable): `variant ?? "none"`.

### Verification

| Check | Command | Result |
|---|---|---|
| Type-check | `tsc -p met-tools/tsconfig.tt-all.json` (os tsconfig; the 29 new files, `✅️verification`, the boot and inventory tests, the puzzle `📜️script.ts`) | **0 errors** in the time-travel tree, the scenario, the boot and inventory tests and the puzzle script; 1 foreign error left in `✅️verification` (`readPackageName`, as in the baseline). Baseline: 8 errors in the file. |
| Declaration proof | `met-tools/tt-proof.ts` over the 330 top-level declarations of the old file | 299 identical, 28 changed (each reviewed: state writes -> setters, the declared patches, the editor `numbers` reads, `Ctx` -> `object` on shared steps), 3 replaced by the scenario object (`ORDER`, `STEP_NUMBERS`, `STEPS`) |
| Region markers | script over the 20 original region names | all present, balanced in every file |
| Module graph | `bun met-tools/smoke-import.ts` | harness and scenario load without TDZ or cycle faults; scenario has 22 steps, 6 hooks |
| Dry argument parse | `bun met-tools/dry-parse.ts` (`configure` with real flags) | react scenario: serve `:6012`, route `…/?plugin=puzzle2d`, `--only 1,2,reload`, `--folder-at 5`, `--long-history 50`; wgpu default serve `:6112`; universal without scenario: serve from `--serve`, `only` = `{24, 9}`; no scenario and no `--serve` throws |
| Headless run through the harness | `bun met-tools/smoke-run.ts`: `configure` + `runProbe` against a local static page (no repo state written; acceptance record not published) | real Chromium, boot verdict, step 9 console digest and markdown report: **6 verdicts, 6 PASS**, 0 uncaught, 0 hard faults |
| Plugin-name grep | `🗑️generated/met-plugin-grep.txt`: 46 framework files (harness, boot test and fixture, the 14 worker-law suites, directory and config suites, fakes) | **0 hits** for `puzzle*`, `gis*`, `gismap`, `cad`, `draw`, `wfc`, `energy`, `procedural`, `architect`, `writer`, `raster`, `stdio`, `generation3d`, `process3d`; the only matches of the broader word `note` are the harness's own `note()` report function |
| Path length | python over the 57 new paths | longest 138 bytes |
| Boot and inventory vitest | dev config, filter `time-travel` | both suites still fail to load with the baseline's foreign `loadCatalogTaxonomy is not a function` (unchanged); the edits there are verified by type-check only |
| Not done | a run against a real puzzle 2d dev serve (React :6012, wgpu :6112) | not possible here; the steps themselves are verbatim text (299 of 330 declarations identical, the rest declared above) |

## Taxonomy

- Test-case folders are matched by registry name. `met-tools/taxonomy-names-added.txt` lists the 50 new folder names appended to `members-of-tests` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (1,404 -> 1,454 names), installed by a hash-guarded atomic replace (file hash read at `6ab85e57cd98`, unchanged when replaced).
- Validation: the sweep's simulator `sim.ts` on the live taxonomy plus the new directories: unresolved directories 33,540 -> 33,488, **0 ambiguous, 0 previously resolved directory changed kind**; `validateTaxonomy` 0 problems; `loadTaxonomy` ok (758 kinds).
- 4 names are already registered elsewhere (`⏪️time-travel`, `🎲️board`, `📜️history`, `🕹️interaction`); registering them in `members-of-tests` changed the resolved kind of existing directories with the same names (for example `…/🧫️fixtures/🗺️testing-readme-coordinates/📜️history`), and registering them in `crate-module-children` produced 8 ambiguities, so they are left unregistered (2 of those 4 directories, `S` and `S/🎲️board`, stay unresolved like thousands of their siblings; `H/📜️history` and `S/🕹️interaction` resolve through their existing rows).
- The registry as a whole is volatile: the file was rewritten twice by other processes while I worked (an earlier sweep's +261 names were no longer present). Re-run `sim.ts` before relying on the count.

## Foreign issues and hand-offs

- Other dev gates name puzzle2d the same way (same rule, not in this slice): `🧑‍💻dev/🧪️tests/🧫️fixtures/⏯️tool-run-matrix.json` (`s.puzzle.puzzle2d@1/*#editor`) and `…/⏱️interaction-latency.json` (`puzzle2d-drag`).
- `@semio-tech/framework-os-dev:time-travel` (a dashboard target) now needs `--universal …`; its old no-argument dashboard run moved to `@semio-tech/puzzle-js:time-travel`.
- `H/🚀️boot/🧪️tests` and `H/🪞️inventory/🧪️tests` cannot load (`loadCatalogTaxonomy` is not exported where the catalog loader expects it), and `bun test` of the hub io-boundary file's first test segfaults Bun 1.3.14: foreign, unchanged.
- Foreign type errors in the gis closure that I left alone: 55 `ArtifactActorConfig.actorIdentityGrant` / `ShardBudget.maxFrames` / unknown-`corpus` errors, all present in the baseline.
- While I worked another process deleted `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧰️schema/🟦️.ts` and rewrote files under `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/`; I touched none of that apart from `🧱️io-boundary/🟦️.ts`.

## Files

- Created (framework): 14 os worker-law suites, `🧰️worker-law-fakes`, `📚️worker-law-suites`, 5 Directory suites, 2 config suites, 19 time-travel harness modules.
- Created (gis): 4 suites, `🧰️dependencies`.
- Created (puzzle): 9 scenario modules (`S/🟦️.ts` and 8 children).
- Edited: `OS/🔨️modules/🏪️store/👷️worker/🟦️.ts` (+2), `GIS/👷️worker/🟦️.ts`, `H/🟦️.ts` (rewritten), `H/🚀️boot/🟦️.ts`, `H/🚀️boot/🧪️tests/🟦️.ts`, `H/🚀️boot/🧫️fixtures/🔣️.json`, `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧱️io-boundary/🟦️.ts`, `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts`, puzzle `📜️script.ts`, `📋️project.json`, `package.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `🔣️taxonomy.json`.
- Removed: `GIS/🧪️tests/🟦️.ts`.
- Tools and snapshots (kept): `met-tools/` (`split-gis.ts`, `split-tt.ts`, the proofs, snapshots of both original files, tsconfigs). Logs: `🗑️generated/met-*`.

## Commands run (all in the foreground, `perl -e 'alarm N; exec @ARGV' -- …`)

- `node node_modules/vitest/vitest.mjs run --config <gis config>`: baseline, stage A, stage B, final (the final result is above).
- `… --config OS/🧪️tests/🎚️config/🟦️.ts [worker]`: stage B (609 tests in the whole config, 112 fail in files I did not touch), final worker file 120 of 120.
- `… --config 🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts time-travel`: before and after, identical load failures.
- `node node_modules/typescript/bin/tsc -p met-tools/tsconfig.{gis-baseline,gis-after,tt-baseline,tt-all}.json`.
- `bun test ./🌎️hub/…/🧱️io-boundary/🟦️.ts -t "fixture URLs resolve"`: 1 pass.
- `bun met-tools/{body-proof,tt-proof,guard-check,smoke-import,dry-parse,smoke-run}.ts` and `python3 met-tools/{lossless,cmp-titles,cmp-tsc}.py`.
- `bun 🗑️generated/taxsweep/sim.ts` and `validate.ts` (from the taxonomy sweep) on the live and the candidate taxonomy.
