# Result CENSUS4-SMALL-2 (prefix `c4s2`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Tools: `F/c4s2-tools/` (descendant of `c4p48-tools` for Rust and `c4p51-tools` for TS). Logs: `F/🗑️generated/c4s2-*`. Sandboxes: `/Users/ueli/.cache/c4s2/`.

## 0. Verdict

**DONE** for all nine packages (14 files, none skipped). Rust packages 62, 64 and 65 are verified statically (real compile queued, section 5); TS packages 54, 59, 60, 61, 63 and 66 are verified by type-check, lossless and export checks, taxonomy, and, where the suite can run in this checkout, by identical test sets before and after. 14 files of 1,012 to 2,211 lines became roots of 18 to 335 lines plus 82 new children (largest child 656 lines, longest repo-relative path 174 bytes). No compatibility barrel was added; every importer and every source-text reader found by `git grep` was re-pointed at the owner.

| Pkg | Area | Files | Verdict |
| --- | --- | --- | --- |
| 62 | product/server Rust | gateway unit tests 1,512 → 73 + 6 (max 470) | DONE (static) |
| 64 | apps/trinity Rust | graph editor tests 1,283 → 150 + 6 (max 303); 35 exact cargo test names re-pointed | DONE (static) |
| 65 | product/pets Rust | climbing unit tests 1,228 → 174 + 6 (max 245) | DONE (static) |
| 54 | os/other TS | vite-plugins 1,446 → 83 + 12 (max 292); two-human 1,012 → 249 + 4 (max 307); ticket-owned-browser-host-staging 1,744 → 24 + 8 (max 440); host rust script 1,182 → 80 + 5 (max 394) | DONE; dev vitest set identical (222 rows) |
| 59 | teaching TS | pet-walk 1,278 → 310 + 3 (max 642); capacity 1,119 → 335 + 5 (max 220) | DONE; Playwright list identical (58 tests) |
| 60 | os/infinite TS | projection 1,164 → 173 + 5 (max 323); chunkkey 1,114 → 38 + 5 (max 365) | DONE; vitest 103/103 identical |
| 61 | apps/cad TS | brepjs 2,211 → 103 + 8 (max 656) | DONE (the suite cannot load in this checkout, identical failure before and after); tsc diagnostics ⊆ baseline |
| 63 | framework/tests TS | docklayoutstore 1,376 → 18 + 4 (max 610) | DONE; vitest 306/306 identical |
| 66 | product/server TS | server twin 1,226 → 46 + 5 (max 566) | DONE; vitest 18/18 identical |

## 1. Found state (fresh slice)

Re-measured at 18:33: all 14 files over 1,000 lines, none skipped; line counts differ from census 4 only for the trinity file (1,276 → 1,283), the dock layout store (1,396 → 1,376) and the Rust packages script (1,183 → 1,182). No file of this slice appears in a running slice's claim in `📓️FLEET.md`. Concurrent TS slices (tt5, c4p50, c4ts) touched none of them; every installer or splitter run was preceded by `cmp` of the live file against its snapshot.

## 2. Package 62: gateway unit tests (Rust, `semio-framework-server-tests`)

File: `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs` (1,512 lines, SHA256 `a7f2efb3aa8b5af0`, snapshot `F/c4s2-tools/orig/gwu.rs`). Owner: the companion crate `semio-framework-server-tests` (lib root `🧪️tests/🥣️companion/🦀️.rs`, `#![cfg(test)]`, mounts the file as `gateway_unit::tests`).

Splitter `F/c4s2-tools/c4s2.py` (descendant of `c4p48.py`), spec `specs/gwu.py`. Root keeps the imports, the shared helpers (`state`, `server`, `actor`, `event`, `grant`, `scratch`, `loopback`, `caller`, `address`) and the `use tower::ServiceExt;` of the edge region: 73 lines. Six children next to it:

| child folder | lines | content |
| --- | ---: | --- |
| `👥️presence-fanout-and-kick` | 82 | colour leases, fanout lanes, kick |
| `🗂️static-cors-credential-and-blob` | 117 | static hosting, CORS, credentials, blob writes |
| `🔁️event-stream-and-server` | 234 | replay/live seam, relay, server builder, document socket identity |
| `🛰️presence-rooms-and-budget` | 470 | room readers, pages, per-address budget, state frame |
| `🧵️presence-sessions` | 271 | socket sessions, rosters, watched rooms |
| `🛂️edge-and-throttle` | 308 | client addressing, throttle, body limits, allowances |

Checks (all run, logs `F/🗑️generated/c4s2-*gwu*`):

| Check | Result |
| --- | --- |
| lossless (`c4s2.py lossless gwu`, live: `live.py gwu`) | missing 0; extras only docstrings, `use super::*;`, `#[path]`/`mod`/`use` wiring and region markers; live bytes equal to the stage |
| regions | 12/12 before, 14/14 after (PresenceStream and Edge repeat a pair where a region spans the root and a child) |
| structure (`check.py`) | `CHECKS OK`: 6 mounts once each, docstring emoji = folder emoji, unique, longest path 151 bytes, no `[DEBUG]` |
| private declarations (`defscan.py`) | 81 scanned, 0 cross-file leftovers; 12 items widened to `pub(super)` (`F/🗑️generated` stage tsv) |
| rustfmt | 0 errors over the 7 staged files (a deliberate syntax error gives 1) |
| test names (`testnames.py`) | 30 test fns before and after, identical set (their module path gains one level) |
| taxonomy / RGI | `dirs 6 unresolved []`, `bad=0` |
| scoped patched differential (`oracle.py gwu all`, test config) | stage run and live run (`C4S2_LIVE=1`): errors 533 / 533, 0 only-orig, 0 only-new, warnings 0 / 0 (first run had 4 `unused import … ::*` warnings for children nobody else uses: set `reexport=False`) |
| stub-layer typeck oracle (`posrun.py` pre on the unsplit tree, post on the live tree, `posdiff.py`) | 538 rows each, `POSDIFF IDENTICAL`; 0 privacy, 0 trait-scope rows |
| census own closure (test cfg, `census-own.ts`) | before 7 files / 168,691 bytes / 3,173 lines; after 13 / 170,403 / 3,222; canonical `census.ts` for `semio-framework-server` lib 14 files / 165,448 bytes / 3,440 lines (ratio 0.860, unchanged by this slice) |
| source-text readers | none (`git grep -F` for the path and `gateway_unit`: only the companion's `#[path]` mount) |

No new crate, test target or launch route. Real check queued with the other Rust packages (section on the compile queue).

## 3. Package 64: trinity graph editor tests (Rust, `semio-s-dev-trinity-jack`)

File: `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/✏️editor/🦀️.rs` (re-measured 1,283 lines; census said 1,276 because the TRINITY-API-DRIFT slice edited it at 17:27; SHA256 `78a50fd448567483`, snapshot `orig/tri.rs`). Owner: `semio-s-dev-trinity-jack` (own `[workspace]`, lib root `🪆️graph/🦀️.rs`, file mounted as `tests::editor_laws`). The file was last written at 17:27, nothing running touches it (installer refuses a live file that differs from the snapshot).

Root keeps the imports, the harness (`JackTestApp`, `new_app`, `meta`, `query_windows`, `settle`, the manifest helper, the round-trip wire test): 150 lines. Children:

| child folder | lines | content |
| --- | ---: | --- |
| `🔌️live-load-envelope` | 109 | envelope wires, admission, live load pump (3 helpers widened) |
| `🖼️scene-rendering-and-text-edit` | 248 | scene helpers, rendering, query runs, selection, text edits (5 widened) |
| `🔭️lod-catalogue-and-examples` | 227 | LOD, catalogue, inspection, example loading, deletion, export, ports |
| `🔎️query-ownership-and-examples` | 303 | query ownership, shared document query, window kind actions, shipped queries (1 widened) |
| `🩹️rail-verb-laws` | 211 | rail verbs (region `RailVerbLaws`) |
| `🤖️agent-lane-and-archive-laws` | 77 | agent lane (region `AgentLaneLaws`), three `*_law!` macro invocations, foreign carriers; `super::` paths rebased to `super::super::` |

**Source-text reader found and fixed.** `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/📜️script.ts` lists 35 exact cargo test names `tests::editor_laws::<fn>` for `semio-s-dev-trinity-jack` (`runExactCargoLaws`); all 35 re-pointed to `tests::editor_laws::<child>::<fn>` by `c4s2-tools/repoint-tests.py` (35 rewritten, 0 missing, each mapped through the live children). Without it the exact-name run would have found no test.

Checks: lossless OK (missing 0, regions 2/2 → 2/2), `check.py` OK (longest path 124 bytes), `defscan.py` 37 private declarations 0 leftovers, rustfmt 0 errors, `testnames.py` 35 test fns identical, RGI bad=0, taxonomy unresolved [], pub API sets equal. Scoped patched differential: stage run 1 showed 45 new E0425 (widened helpers not reachable from siblings: the four children with cross-sibling items now `use child::*`), run 2 and the live run (`C4S2_LIVE=1`): lib 45/45 and test 527/527 errors, 0 only-orig, 0 only-new, warnings 0/0. Stub-layer typeck oracle (`posrun.py` pre/post, `posdiff.py`): 1,298 rows each, `POSDIFF IDENTICAL`. Census own closure (test cfg): before 19 files / 204,484 bytes / 2,261 lines, after 25 / 206,288 / 2,309 (limits 32 / 262,144 / 4,000; lib unchanged 7 / 26,410).

## 4. Package 65: pets climbing unit tests (Rust, `semio-framework-pets-traversal`)

File: `🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🦀️.rs` (1,228 lines, SHA256 `9f4d5c6e1dd43c01`, snapshot `orig/pet.rs`), mounted as `climbing::tests` by the traversal crate (`🧭️traversal/🦀️.rs` mounts `../🧗️climbing/🦀️.rs`). Root keeps the `//!` header, imports, constants (including `SOURCE = include_str!("../../🦀️.rs")`, so no include path is rebased) and the 25 shared helpers: 174 lines. Six children, all `reexport=False` (no sibling uses another child's items, 0 widened):

| child folder | lines | content |
| --- | ---: | --- |
| `🧱️wall-and-hoist-laws` | 142 | stage cut, line of sight, hoist, wall holds/crowns/takes/surveys |
| `🫳️grip-climb-slide-and-mantle` | 126 | grip, climbing speed and clip, sliding, mantle |
| `🪜️ladder-laws` | 189 | ladder lean, rise, passes, generated stages, climb, carry, toppling |
| `🪝️grapple-shot-and-haul` | 245 | shot, perches, ropes, hook flight, haul, swing, landing |
| `🗺️routes-and-wall-lines` | 189 | routes, lunges, pitches, ways of wall lines |
| `📜️committed-vectors-and-source` | 199 | constants, committed wall/ladder/grapple vectors, exact operations of the design |

Checks: lossless OK (no regions in the file), `check.py` OK (longest path 147 bytes), `defscan.py` 50 private declarations 0 leftovers, rustfmt 0 errors, `testnames.py` 48 test fns identical, RGI/taxonomy ok, pub API equal. Scoped patched differential (stage and live): lib 19/19, test 27/27 errors, 0 only-orig, 0 only-new, warnings 0/0 (this crate resolves its sibling modules, so the differential reaches below the unresolved externals). Stub-layer oracle: lib 583 rows and test 1,990 rows, `POSDIFF IDENTICAL`. Census own closure (test cfg): before 5 files / 189,599 bytes / 3,151 lines, after 11 / 191,001 / 3,193; lib unchanged 3 / 76,343 / 1,355. Readers: `git grep` for the path, `climbing::tests::` and `traversal::climbing`: only `pub use` / `use` of `climbing::Toss` items, no text reader and no test-name list.

## 5. Compile queue (Rust packages 62, 64, 65; not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 18:44), so no `bun repo cargo` call was made. One batched call under the compile mutex of `📓️WAVE5-BRIEF.md`:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-server-tests -p semio-framework-pets-traversal --tests --keep-going --message-format=short
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-dev-trinity-jack --tests --keep-going --message-format=short --manifest-path '✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml'
```

If a build shows a missing `pub(super)`, widen that single definition line in the child (never with a qualified path).

## 6. Package 54: os/other TS (4 files)

Tools (`F/c4s2-tools/`): `gsplit.cjs` (c4p51 module splitter, now rebasing relative `import()` and import-type specifiers inside moved text), the new `descsplit.cjs` (splits a `registerTests1(vitest, dependencies, source)` suite: `describe` groups become `registerXTests` children, declarations several groups use go to a kit factory, the parent keeps an orchestrator), `lossless.cjs`, `cycles.cjs`, `exports.cjs` (every exported name of the snapshot is exported by exactly one new file), `absbase.cjs` + `tsc.sh` (type-checks a pre-split snapshot from outside the repo with absolute specifiers for a baseline), `contracts.ts` (re-runs the re-pointed contract fixtures outside `bun:test`), specs in `specs/{vp,th,tos,hs}.json`, snapshots in `snap/` (hashes in `orig/SHA256`).

| # | File (under `🧰️framework/🛍️products/💻️os/`) | Before | Root after | Children (largest) |
| --- | --- | ---: | ---: | --- |
| 1 | `🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` | 1,446 | 88 (descriptor route guard, service worker scope, local hub session plugin) | 12: `🧭️repo-root` 7, `🗃️sqlite-constructor` 20, `📇️bootstrap-folder-mirror` 167, `🗄️backbone-storage` 123, `🔀️stream-mux-server` 81, `🚧️production-test-boundary` 17, `🧱️backbone-vite-plugin` 170, `🚀️activation-vite-plugin` 242, `🫧️blob-vite-plugin` 93, `🔖️source-freshness` 292, `🛰️agent-bridge-rendezvous` 90, `🔐️agent-credential-install` 111 |
| 2 | `🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts` | 1,012 | 249 (the `runTwoHuman` entry and CLI) | 4: `🔑️sessions` 172, `🏠️space-creation-and-edits` 272, `📋️run-types` 35, `🧭️journeys` 307 |
| 3 | `🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` | 1,744 | 24 (orchestrator) | 8: `🧰️test-kit` 106 (the 49 owner imports, `testDependencies`, `createTestKit` with the three shared helpers), `🌉️fixtures-and-pixels` 249, `🛂️routes-and-bridge` 274, `🧬️jco-rewrites` 240, `🚚️vendor-transport` 440, `🏗️profile-and-catalog-build` 152, `🗃️handles-and-output` 125, `📡️poll-and-smoke` 220 |
| 4 | `🖥️host/📦️packages/🦀️rust/📜️script.ts` | 1,182 | 80 (router, `CheckScript`, `TestScript`, `WasmScript`) | 5 under `🎛️commands/` (the repo's own precedent for the os root script): `🎞️media-persistence-and-retirement` 175, `🚪️member-open` 167, `🪪️member-history-identity` 263, `📚️member-history-dictionary` 391, `🏭️member-factory-identity` 155 |

Sources read as text and re-pointed (each found by `git grep` for the path, all re-pointed at once):
- the `os-dev-composition-ownership` fixture: `ownerImporterPath` and consumer `browser-host-owner-importer` now name the test kit (it holds the 49 owner imports, as the old file did; 44 of them match the fixture's 47 owners, the same count as before the split); `os-host-hash` names `🪪️member-history-identity` and a new row `os-host-dictionary-hash` names `📚️member-history-dictionary`, because both children use `blake3Hex`; the Nx `inputs` of `test-os-dev-composition-ownership` and `test-tool-configuration-ownership` in `🦑️repo/…/📚️library/📦️packages/🟦️typescript/📋️project.json` follow;
- the `tool-configuration-ownership` fixture: the consumer holding the vite config path is the `🚚️vendor-transport` child;
- the `os-source-topology` fixture: anchor `semioBackboneVitePlugin` lives in `🧱️backbone-vite-plugin`; `windows-command-paths`: `protectOwnerOnly(` lives in `🔐️agent-credential-install`;
- staging root: `🧪️fixtures/🔌️staging-root.json` `sourceFiles`, `📋️project.json` `stagingRootSources` (the law compares the two sets) and `🧫️fixtures/⚙️config-graph.json` `require` list the 12 vite-plugin children next to the parent;
- readers: the canonical bootstrap mirror test reads the `📇️bootstrap-folder-mirror` child; the ticket-owned vendor child reads `semioProductionTestBoundaryVitePlugin` from `🚧️production-test-boundary`; the plugin store test `authored-extension-installation-identity` imports `devStreamMuxServer` from `🔀️stream-mux-server`; the test kit spreads a second namespace (`External16`, `backboneDbHandleFor` from `🗄️backbone-storage`);
- importers rewritten by the splitter (named imports re-pointed per owner, no barrel): the dev vite builder, the wgpu server and its config, the play builder and activation, the package entry `📦️packages/🟦️typescript/🔌️vite-plugins/🟦️.ts` (an existing `export … from` public entry, now one line per child), the hot-update and config tests, the canonical mirror test, the studio test.
- Fixed on the way: an original region mismatch (`//#region 🔖️SourceFreshnessVitePlugins` closed by `//#endregion SourceFreshnessVitePlugins`, one line); `REPO_ROOT`'s `import.meta.url` path gained one `../` because it now lives one folder deeper; two unused type aliases of the ticket-owned suite (`DeployedRegistryEntryV1`, `PluginSourceEvent`) were dropped; the parent header of the vite plugins was rewritten for what it now owns.

Checks actually run:

| Check | Result |
| --- | --- |
| dev vitest (`vitest.sh`, config `🧪️tests/🎚️config/🟦️.ts`, level long), before vs after all four edits | 19 files, 222 tests (183 passed, 37 failed, 2 skipped) before and after; the sorted (status, file, title) sets are **identical** (`diff` exit 0, 222 rows). The 37 failures are pre-existing (20 in the ticket-owned suite, local-hub, hot-update, staging-root, config file) |
| plugin store vitest (`authored-extension-installation-identity`) | 9 of 9 passed after, including the 3 `extension store route precedence` laws that use the moved `devStreamMuxServer` |
| lossless (`lossless.cjs`) | vite-plugins, two-human, host script: 0 lines lost except the intended edits listed above; ticket-owned: 13 lost lines = the split destructure, 8 rebased type aliases and dynamic imports (their rebased copies are the added lines) |
| exports (`exports.cjs`) | all 42 / 21 / 0 exported names of the three module snapshots exist in exactly one new file |
| tsc (`tsc.sh`, project options of `🧰️framework/🛍️products/💻️os/tsconfig.json`, 47 root files) | before (snapshots of vite-plugins, two-human, host script via `absbase.cjs`): 0 diagnostics; after: 3 diagnostics, all in the ticket-owned children (`pluginOutRoot` TS2339, a TS2554 arity and TS7016 for `whatwg-url`), which the baseline of the ticket-owned snapshot also reports (5 there: 2 more came from destructured names that the split no longer destructures) |
| import cycles (`cycles.cjs`, 33 files) | 0 strongly connected components above size 1 |
| runtime import | `bun -e import(…)` of the two-human parent (2 exports), studio test (14), vite-plugins parent (4), activation child (2) and the package entry (34 exports) loads; the host script reaches its final `receiveScriptProcessInvocation` (a launcher envelope is required, so it was not run further) |
| taxonomy / RGI | `tax.ts` 30 of 30 new directories resolve (`crate-module-children`, `members-of-tests`); `rgi.mjs` bad=0; longest repo-relative path 170 bytes |
| contract fixtures (`contracts.ts`) | the re-pointed rows pass (importer, `os-host-hash`, `os-host-dictionary-hash`, tool-configuration consumer, os-source-topology anchor, windows-command-paths); the closure-of-inputs law reports 4 lacking and 5 extra inputs that are unrelated to this slice (0 of them mine); the library suites themselves do not load in this checkout (`bun test` fails at load: missing `postcss-load-config`, wrong fixture path) |

No new launch route: the children are library modules reached by the existing test entry points.

## 7. Package 59: teaching TS (2 files)

| # | File (under `🎓️teaching/`) | Before | Root after | Children |
| --- | --- | ---: | ---: | --- |
| 1 | `🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts` (Playwright spec, project `pets`) | 1,278 | 310 (header, imports and the 9 home-screen and hand specs) | `🧰️pet-kit` 642 (ensemble, constants, instrumentation, checks; `MENAGERIE`'s `import.meta.url` path gained one `../`), `🧭️cast-and-choices` 194 (5 specs), `🪜️mischief-ladder-and-phone` 150 (3 specs) |
| 2 | `🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts` (bun entry and its own worker script) | 1,119 | 335 (header, the gate, the worker spawns `new Worker(new URL(import.meta.url))`, the `isMainThread` dispatch) | `📊️settings-and-measurements` 147, `🚚️transport` 150, `🎒️hall-learner` 220, `🎭️abusive-script` 202, `🖥️proctor-process` 90 |

The capacity file is loaded as a `Worker` by its own URL, so both `new Worker(new URL(import.meta.url), …)` calls and the role dispatch stay in the parent; the children are pure modules.

Wiring: the Playwright project `pets` in `🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts` lists the two new spec files next to the parent (`testMatch`); the kit is not a spec and is not listed. The capacity run is routed by `🛂️proctor/📦️packages/🦀️rust/📜️script.ts` through the unchanged parent path.

Checks actually run:

| Check | Result |
| --- | --- |
| Playwright discovery (`playwright test --list --config 🎭️e2e/🎚️config/🟦️.ts`, `PLAYWRIGHT_BASE_URL` dummy), before vs after | 58 tests in 14 files before, 58 tests in 16 files after; the sorted (project, title) lists are **identical**; discovery also loads the kit, so its constants and the ensemble read work (the specs themselves need the booted stack and were not run) |
| capacity entry: `bun 🏋️capacity/🟦️.ts --learners x`, before (snapshot via `absbase.cjs`) vs after | the same error `--learners must be a positive whole number, got "x"` |
| capacity worker dispatch (`c4s2-worker-probe.mjs`: a `Worker` on the file with `role: script` and `role: hall`), before vs after | identical: `script -> "undefined" cannot be parsed as a URL`, `hall -> undefined is not an object (evaluating 'orders.learners.map')`, i.e. the parent boots as a worker and reaches `script` and `lecture` in their children |
| lossless | capacity: 0 lines lost, 5 docstrings added, regions 14/14; pet-walk: 1 line changed (`MENAGERIE` path), docstrings and `@see` lines added, no regions |
| exports / cycles | capacity: 21 of 21 exported names in exactly one new file, 6 files 0 cycles; pet-walk: 0 exports, 4 files 0 cycles |
| tsc (root `tsconfig.json` options, root files only) | before (the two snapshots): 0 diagnostics; after (parents, children, config): 0 diagnostics |
| taxonomy / RGI | 8 of 8 new directories resolve (`members-of-tests`), `bad=0`; longest path 119 bytes |

## 8. Package 60: os/infinite TS (2 files)

Under `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/`:

| # | File | Before | Root after | Children |
| --- | --- | ---: | ---: | --- |
| 1 | `📐️projection/🟦️.tsx` | 1,164 | 173 (mode options, template selection ids, the kind and orbit projection switches) | 5 under `📐️projection/`: `🧮️projection-pose` 323, `🔄️projection-matrices` 100, `🧭️projection-gizmo-specs` 141, `🎬️projection-rig` 218 (`.tsx`), `🏷️projection-templates` 188 |
| 2 | `🧪️tests/🧪️chunkkey/🟦️.tsx` (state at 18:56, after the projection re-pointing) | 1,114 | 38 (imports, orchestrator and the dependency object) | 5 suites groups with `descsplit.cjs`, no kit (the 7 local type aliases are copied where used): `🧭️navigation-and-camera-state` 155, `🎛️gizmo-specs-and-view-templates` 268, `📐️projection-poses-and-matrices` 365, `🏷️template-labels-and-lod` 158, `🧱️mesh-reference-and-volume` 176 |

The projection importers were rewritten by the splitter to the owning children: the r3f root `🟦️.tsx` (its two `export … from "./📐️projection/🟦️.tsx"` blocks, an existing explicit re-export surface of the r3f module, now one block per owner), `🧭️gizmo-viewport/🟦️.tsx`, and the chunk key test (namespace members `Dep3.x` became aliased imports). The checked-values test of `🖱️ui` reads `worldProjectionMorphMatrix` from the source text; it now reads `🔄️projection-matrices`. The projection parent header was rewritten for what it keeps. The unused `ModuleUnderTest` namespace import of the chunk key test was dropped by the splitter (nothing used it).

Oracle debt registry: the five `🔮️oracles/🔣️.json` `productionDebt.reachableFrom` lists name the projection file; no new child imports `three` directly (`grep` for `from "three"` over the children: none), so the repository-contract law "only the recorded paths are excused" is unaffected and the lists were left as they are.

Checks actually run:

| Check | Result |
| --- | --- |
| vitest (`vitest.sh`, config `r3f/🧪️tests/🎚️config/🟦️.ts`) before the first edit vs after both splits | 1 file, 103 tests, all passed, identical sorted (status, file, title) sets |
| `bun test` of `🖱️ui/🧪️tests/📥️checked-values/🟦️.ts` (reads the moved function) | 4 pass, 0 fail |
| tsc (root options, 12 root files) | before: 0 diagnostics; after: 0 diagnostics |
| lossless / exports / cycles | projection: 57 of 57 exported names in exactly one new file, code lines lost only the replaced header (2), 8 files 0 cycles; chunk key: lost lines are the split destructure, the 7 rebased type aliases and the tail call that the splitter re-aliased; 6 files 0 cycles |
| taxonomy / RGI | 10 of 10 new directories resolve (`crate-module-children` under `📐️projection`, `members-of-tests` under the test), `bad=0` |

## 9. Package 61: apps/cad TS (1 file)

`✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs/🟦️.ts` (2,211 lines, SHA256 `f2dfe0de09eda6ea`) is the brepjs `SpatialKernel` kept as a vitest differential oracle, with an OpenCascade wasm engine, a dedicated worker entry that loads itself, and an in-source suite hook. Root after: 103 lines (exports of `kernelGeometry`, the dedicated worker entry, `BrepjsTestDependencies` and the `import.meta.vitest` hook; the `🧲️Header` region markers that the splitter drops were restored). Eight children under `🧱️brepjs/`: `🧩️opencascade-and-scratch` 44, `🧱️entity-maps` 269, `🔗️wires-and-selection` 283, `✂️edit-diffs` 378, `🧊️solid-topology-and-fixture` 192, `⚙️wasm-engine` 656 (the `BrepjsWasmEngine` class), `📨️worker-and-kernel` 256, `📦️exchange` 69.

- The worker is the parent: `BrepjsWorkerClient` now lives in `📨️worker-and-kernel`, whose `new Worker(new URL("../🟦️.ts", import.meta.url), { type: "module" })` was re-based by hand; the worker entry that listens for messages stays in the parent and imports the engine.
- The `/// <reference types="vite/client" />` and `vitest/importMeta` lines were repeated in `🧩️opencascade-and-scratch`, the only child that reads `import.meta.env`.
- Importers: the public entry `📦️packages/🟦️typescript/🧪️testing/🔮️spatial-kernel/🧱️brepjs/🟦️.ts` (`export … from` per owner), the semio spatial kernel suite, and the geometry suite (`await import("../🧱️brepjs/🟦️.ts")` of which only `BrepjsKernel` is used, now `📨️worker-and-kernel`). The vitest `includeSource` entry in `📐️cad/🧪️tests/🎚️config/🟦️.ts` stays the parent.

| Check | Result |
| --- | --- |
| vitest (config `📐️cad/🧪️tests/🎚️config/🟦️.ts`, filter `brepjs`), before vs after | the file fails at load in both (`Cannot find package 'brepjs-opencascade/src/brepjs_single.wasm?url'` imported from the cad plugin engine), identical set; so the in-source laws could not run, before or after |
| tsc (root options) | before (snapshot via `absbase.cjs` + the tests file): 12 diagnostics (11 TS2307 missing modules, 1 TS2322 caused by the snapshot copy's own path); after: 11 diagnostics, the same 11 TS2307; none new |
| lossless | 0 lines lost except the re-based worker URL (1 line); regions 28 → 38 (repeated `BrepModelBridge`, `EditTopologyOps`, `BrepjsKernel`, `BrepjsScratch` pairs, header restored); exports: 12 of 12 names in exactly one new file; 9 files 0 cycles |
| taxonomy | 9 of 9 directories resolve (`members-of-tests`; the existing `🧪️tests` folder stays `tests`), `bad=0` |

## 10. Package 63: framework/tests TS (1 file)

`🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` (re-measured 1,376; `registerTests1` of 1,367 lines plus the `if (import.meta.vitest)` hook). The parent keeps the `registerTests1` orchestrator, the `TestSource` type and the in-source hook (the `framework-root-source-topology` law requires the file to contain `if (import.meta.vitest)` and `import("@semio-tech/framework")`): 18 lines. Four children by `descsplit.cjs`: `🗂️state-stores` 248 (dock layout, dock UI state, window pane state, OS shell config), `🧭️playground-actions-and-plugin-sources` 315 (with its `SYNTHETIC_PLUGIN_CATALOG` fixture), `🎰️machines` 610 (the machine fixtures region and the seven machine suites), `⏱️async-helpers` 217.

| Check | Result |
| --- | --- |
| vitest (config `🧪️tests/🎚️config/🟦️.ts`, run from `🧰️framework` so the globs match), before vs after | 6 files, 306 tests passed before and after; sorted (status, file, title) sets identical; the docklayoutstore file contributes 86 tests |
| tsc (root options) | before (snapshot): 0 diagnostics; after (parent + 4 children): 0 diagnostics |
| lossless / cycles / taxonomy | 1 line lost (the 40-name destructure, split per child), 4 docstrings added, 5 files 0 cycles; 4 of 4 directories resolve, `bad=0` |

## 11. Package 66: product/server TS (1 file)

`🧰️framework/🛍️products/🖥️server/🟦️.ts` (1,226 lines, the TypeScript twin of the server wire contract). Root after: 46 lines (the header doc, the imports and the `import.meta.vitest` hook that registers the wire suite). Five children: `🧬️wire-types` 297, `🚚️transport` 107, `🔤️codec` 566 (primitives and all decode/encode functions), `🛣️routes-and-sockets` 137 (frames, route table, error kinds, lanes, `socketRoot`, `encodePathSegment`), `📡️client` 143 (`ServerClient`, `serverCallError`). Two import cycles of the first attempt were removed by moving `serverCallError` into the client and `socketRoot`/`encodePathSegment` into the routes.

Re-pointed: the package entry `📦️packages/🟦️typescript/🟦️.ts` (`export * from` each of the five owners instead of the old parent, still "reexports only"), the Nx `default` input of `📋️project.json` (the five children added), the wire suite (`import type` from the transport owner and its `Pick<typeof import(…)>` dependency type, now an intersection of the five owners), the quiz deploy script and the two capacity children that import from the server file (rewritten by the splitter), and two `@see` lines of the quiz presence and proctor modules.

| Check | Result |
| --- | --- |
| vitest (config `🧪️tests/🎚️config/🟦️.ts`), before vs after | 18 passed before and after, identical sets (the in-source hook in the parent still runs the wire suite against the shared fixture) |
| tsc (root options) | before (snapshot): 7 lines, all from the snapshot copy's own path (a `typeof import` of the copy); after (parent, 5 children, entry, wire suite, 5 importers): 0 diagnostics |
| lossless / exports / cycles / taxonomy | 0 lines lost, regions 28 → 32; all 102 exported names of the old module exist in exactly one child; 7 files 0 cycles; 5 of 5 directories resolve, `bad=0` |

A first apply of this split left the parent without imports because the splitter ignored identifiers written as object shorthand (`{ PRESENCE_PROTOCOL, … }`); it was rolled back (own created children removed, parent restored from the snapshot) and redone after fixing `gsplit.cjs` and `descsplit.cjs`. The earlier applies (vite-plugins, two-human, host script, capacity, pet-walk, projection, brepjs) were checked by tsc, which would have reported any missing name; none was reported.

## 12. Observations and hand-offs

- Pre-existing and not caused by this slice: 37 failing dev vitest cases (20 in the ticket-owned suite), the dev `🧹️config` file that fails at load, `bun test` of the library contract suites (`os-dev-composition-ownership` reads its fixture from a wrong path, `tool-configuration-ownership` needs `postcss-load-config`), the `os-dev-composition-ownership` closure law (4 lacking and 5 extra Nx inputs, none of them mine) and its 44-of-47 owner imports, the brepjs suite that cannot load without `brepjs-opencascade`, and the 8 wiring-budget breaches of plugin crates owned by peers.
- `os-dev-composition-ownership` should probably be re-pointed to `🧪️tests/🧫️fixtures` by its owner; this slice did not touch that test file.
- The `if (import.meta.vitest)` and `Worker`-by-own-URL patterns (capacity, brepjs) require the parent file to stay the runtime entry; the children of both are pure modules.
- Dependency note for the Rust tests: `semio-framework-server-tests` test closure 13 files / 170,403 bytes and `semio-s-dev-trinity-jack` 25 / 206,288 / 2,309 lines (own sources; the canonical census with dependencies is the gate's business and was not re-run because the splits only add files inside the same crates).
- Tools for later slices: `F/c4s2-tools/descsplit.cjs` (registerTests1 suites), `gsplit.cjs` (rebases dynamic imports, handles shorthand properties), `absbase.cjs` + `tsc.sh` (baseline type-check of a snapshot), `contracts.ts`, `repoint-tests.py` (exact cargo test names).

## 13. Files

- Created: the 82 children listed per package above (62: 6 Rust; 64: 6 Rust; 65: 6 Rust; 54: 12 + 4 + 8 + 5; 59: 3 + 5; 60: 5 + 5; 61: 8; 63: 4; 66: 5), each a `<emoji>️<kebab>/🦀️.rs` or `🟦️.ts[x]` next to its root.
- Rewritten roots: the 14 files of the table in section 0.
- Edited outside the packages (readers and importers): `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/📜️script.ts` (35 test names), `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`, the fixtures `os-dev-composition-ownership`, `tool-configuration-ownership` and `os-source-topology`, `windows-command-paths`, dev `🧪️tests/🧫️fixtures/{🔌️staging-root,⚙️config-graph}.json`, dev `📦️packages/🟦️typescript/📋️project.json`, the server `📦️packages/🟦️typescript/{🟦️.ts,📋️project.json}`, `🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts`, the readers and importers named in the package sections (studio, hot-update, canonical mirror, plugin store route precedence, checked-values, r3f root, gizmo viewport, brepjs entry and suites, wire suite, quiz deploy, capacity children, two quiz `@see` lines).
- Tools kept in `F/c4s2-tools/` (specs `specs/*.py`, `specs/*.json`, snapshots `snap/` and `orig/` with `orig/SHA256`); logs kept in `F/🗑️generated/c4s2-*`.
