# Result: TS-SPLITS-3 (`ts3`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool, 2026-10-16 12:50 to about 14:50. `F` = this folder. Tools: `F/ts3-tools/`. Logs, reports, snapshots: `F/🗑️generated/ts3-*` (snapshots of every original in `ts3-orig/`). No `[DEBUG]` log was added. Short names: `GEO` = `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry`, `PRES` = `🧰️framework/🛍️products/🎤️presentation`, `OS` = `🧰️framework/🛍️products/💻️os`, `PETS` = `🧰️framework/🛍️products/🐾️pets`, `RUST` = `OS/📦️packages/🦀️rust`.

## 1. Verdict

All five files are split. Every test suite keeps its exact test-title multiset and its exact failure set, `tsc` shows no new error on any touched closure, every move is lossless, and the dependency-cruiser `no-circular` counts fell or stayed at zero. One requirement cannot be met literally and is explained: the os script's `--help` output matches byte for byte except for the line number of the entry-call stack frame (section 6.3).

| Original | Lines | Now |
|---|---:|---|
| `GEO/🟦️.ts` | 3,552 | parent of 201 lines (refs, records, primitive hashes) plus 14 children (80 to 532 lines) |
| `PRES/🧪️tests/🎞️presentation-react-deck/🟦️.tsx` | 3,567 | deleted. 5 topic test files (276, 1,330, 100, 417, 1,443 lines) plus 1 helper (35 lines) |
| `OS/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` | 3,046 | deleted. 11 topic test files (82 to 689 lines) plus 2 helpers (37, 14 lines) |
| `RUST/📜️script.ts` | 2,921 (2,934 when split) | 123-line router and entry plus 9 modules in `RUST/🎛️commands/` (51 to 889 lines) |
| `PETS/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` | 3,075 | deleted. 8 topic test files (298 to 561 lines) plus 1 helper (139 lines) |

53 new or rewritten source files. Longest path 128 UTF-8 bytes (limit 240). Every new file opens with an emoji docstring, the emoji is unique among its siblings, and every file is region-balanced.

Two of the three test files were not plain vitest files but registrar closures (`registerPresentationReactDeckTests`, `registerTests1..4`) that the *source* modules (`PRES/🎯️targets/⚛️react/🟦️.tsx`, `OS/🟦️.ts`) imported from an `if (import.meta.vitest)` block. Per the brief ("code must never import its own tests") they became plain vitest files that import the code, the in-source blocks left the source modules, and the vitest configs list the new files in `include` (precedent: `RESULT-TS-AUDIT-FIXES.md`).

## 2. Baseline (recorded before the first edit)

Runner for all vitest runs: `F/ts3-tools/vt.sh <label> <config>` (plain `node node_modules/vitest/vitest.mjs run --config <config> --reporter=json`, `SEMIO_TEST_LEVEL=long`). Sets compared as `status<TAB>fullName` rows (`F/ts3-tools/summ.mjs`). Reports: `ts3-base-*` and `ts3-after-*`.

| Target | Importers and routes | Baseline |
|---|---|---|
| `GEO/🟦️.ts` | 43 files mention the path, 33 import it by value or type (cad plugin engine, renderer panes, repl, `🗺️spatial`, `🧠️semio`, `🧮️preview`, dev cad tests, `🫀️core` re-export `export *`), 3 more by dynamic `import()`. Route: dev cad vitest config `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🎚️config/🟦️.ts` plus the scoped `tsc` closure of the importers | cad vitest: 18 files, 12 tests, 10 pass, 2 fail (`🔬️layering`: ENOENT on two fixture paths); 5 files fail to load (`Cannot find package '@semio-tech/cad-js-module-spatial-shape'`, brepjs wasm). `tsc` on 44 files and closure: 42 errors, none in `GEO` |
| deck test | 1 importer: the react target (dynamic import in its in-source block). Route: `PRES/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (`includeSource`) | 13 files, 140 tests, all pass (128 in the react target file) |
| backbone test | 1 importer: `OS/🟦️.ts` (4 in-source blocks). Route: `OS/🧪️tests/🎚️config/🟦️.ts` (`includeSource`); launch entry `⚖️test-browser-actor-operation-completion🖥️os🟦️` | 98 files, 609 tests, 499 pass, 110 fail (107 in `actor-transport`, 3 in `OS/🟦️.ts`: `AppChannelCodec pins APP_CHANNEL_VERSION…`, `DirectoryClient http … pairs every shared workflow DSL fixture with a pack fixture`, one more in the same file) |
| os script | no TS importer; read as text by 6 fixtures and tests (section 4); 74 `router.register` commands | `bun ./📜️script.ts --help` exits 1 with the invocation-envelope error (`ts3-base-help.txt`, 1,754 bytes); `tsc` on the script: 2 errors (`Cannot find name 'cargoPhase'`, twice) |
| stage unit test | none (found by the pets config glob `🔨️modules/*/🧪️tests/🔬️unit/🟦️.ts`) | pets config: 212 files, 1,133 tests, all pass (283 in the stage file) |

## 3. What changed

Method: `F/ts3-tools/msplit.mjs` cuts one module along top-level statements with the TypeScript checker (unit text moves verbatim, `export` is added only where another module needs the name, imports are rebuilt from the identifiers a module really uses, relative `import(...)` specifiers are rebased, regions are balanced per file), `unclose.mjs` turns a registrar closure into a plain vitest module (dependency bag becomes static imports resolved from the host module, `source.url` becomes a locator const), `reimport.mjs` and `repoint.mjs` re-point importers, `rmblock.mjs` removes the in-source blocks and prunes the imports that only they used. The cluster graph of each split is proven acyclic before anything is written (Tarjan).

### 3.1 `GEO` (14 children, listed bottom-up)

| Child | Lines | Holds |
|---|---:|---|
| parent `🟦️.ts` | 201 | entity refs, kernel records, object records, record and primitive hashes, `TypologyPrimitiveKind`, `KernelTopologyKind` |
| `🗺️paths` | 186 | path segments (pure read, write, clear), `AttributeTable`, entity flags |
| `📥️definition-catalog` | 274 | manifests, asset modules and caches, listeners, asset path ownership, pattern and style types |
| `🧊️model` | 245 | `Model`, hydration, `parseModelJson`, model hashes, topology kind resolution |
| `🎨️typology` | 406 | typology specs and styles, construct kits, attribute and property definitions |
| `🪪️selection` | 101 | selection targets, events, specs, accept expansion, delete from selection |
| `🧮️expressions` | 304 | `Expr`, `ExprEnv`, `evalExpr`, `evalGuard`, path targets on an env, `readModelEntityProperty` |
| `🎰️interaction-spec` | 532 | interaction and action specs, parsers, compilers, active selection spec |
| `🔄️transformation-spec` | 233 | derive rules, parsers, transformation catalog |
| `🧭️definition-ownership` | 358 | owner of typology, action, interaction, attribute, property; actions per definition; objects per definition |
| `🧾️primitive-document` | 159 | primitive document, property computers |
| `📊️stats` | 278 | stat definitions and computers |
| `🧷️selection-operations` | 121 | selection operations and definition scope |
| `🔁️transformation-apply` | 201 | transformation appliers, derive runner |
| `🌌️model-space` | 80 | `ModelSpace`, transfer, serialization |

Cycle-breaking moves that the first cluster plan needed: `AttributeTable`-free path targets moved to `🧮️expressions`; the manifest types and the two caches that the catalog and the typology shared moved into the catalog; asset-path ownership moved into the catalog; `listModelObjectsForModelDefinition` moved into `🧭️definition-ownership`; `getActiveSelectionSpec` and `interactionCanConfirmSelection` moved into `🎰️interaction-spec`; `ModelSpace` became its own layer above the appliers. `readModelEntityProperty` moved from `🧊️model` to `🧮️expressions` so that no model child imports `🗺️spatial` (section 5).

25 module-private declarations became exported because another child needs them (they are the "missing" lines of the lossless check: `MODEL_ENTITY_KINDS`, the six record helpers, the nine cache and catalog accessors, `dedupe…` and the three `shipped…Catalog`).

### 3.2 Presentation deck test

Topics under `PRES/🧪️tests/`: `🎬️reveal-morph` (10 tests), `🖼️deck-rendering` (58), `🔗️url-and-json-preview` (5), `📐️interaction-geometry` (27), `🖱️interaction-dom` (28) and the helper `🧰️presentation-deck-helpers` (test source locator, global stylesheet text, stock reveal matcher host). `🖼️deck-rendering` (1,330) and `🖱️interaction-dom` (1,443) stay below 1,500 lines but are one `describe` each; cutting them further would mean re-opening a `describe` with a replicated `beforeEach`, which I did not do. The react target lost its in-source block (and its `🧪️Tests` and `🔖️json` regions); the config lists the 5 topics in `include` and no longer uses `includeSource`.

### 3.3 Backbone envelope io test

The four registrars became four plain modules, each cut into topics under `OS/🧪️tests/`: `🔄️backbone-envelope-retry` (7 tests), `🧵️backbone-uris-and-workflow` (13), `🎁️pack-value-codecs` (49), `🧾️app-channel-codec` (94), `📞️app-channel-client` (23), `🕸️plugin-graph-and-routers` (14), `📇️directory-read-model` (5), `🪪️document-backbone-worker-wire` (9), `📃️directory-client-pages` (3), `🔔️directory-client-stream` (14), `⏱️directory-client-http` (4), and the helpers `🛠️backbone-wire-helpers` (source locator, document archive fixture, app channel value types) and `🧮️directory-client-helpers`. `source.url` (the URL of `OS/🟦️.ts`, used by about 25 fixture reads) is `new URL("../../🟦️.ts", import.meta.url).href` in the helpers. `OS/🟦️.ts` lost its four in-source blocks, the three regions around them and five imports that only they used (`descriptorDigest*`, `fetchWithTimeout`, `parseBrowserActorUiPatch*`). The dependency bags were resolved from the host's own imports (the `Pick<typeof import(...)>` types of the registrars pointed at stale paths).

### 3.4 `RUST/📜️script.ts`

Per AGENTS.md the script stays the single entry: shebang, docstring, imports, `ScriptRouter` with its 74 `register` calls, and the entry call. The logic moved into `RUST/🎛️commands/<module>/🟦️.ts`: `🧱️script-bases` (51: `CheckScript`, `TestScript`, `NativeTestScript`, `exactCargoStageEnvironments`, `SchemaCheck`), `🗄️database-checks` (420), `✍️wal-checks` (889), `📄️document-checks` (588), `🧬️retained-clone-check` (259), `📇️directory-checks` (270), `📸️snapshot-checks` (125), `👥️member-dialect-checks` (108), `🧪️source-and-native-tests` (253). 76 classes and functions became exported. No class uses `import.meta`; the 4 dynamic `import("../../…")` in classes were rebased and resolve under `tsc`.

### 3.5 Stage unit test

Topics under `PETS/🔨️modules/🎪️stage/🧪️tests/`: `🚪️arrival-and-gaze` (38 tests), `🚶️walking-and-encounters` (42), `🎬️frames-and-summoning` (36), `📏️distance-and-turning` (26), `👁️attention-and-gait` (28), `🧠️determinism-and-mind` (48), `🦾️body-and-gear` (46), `🪄️mischief-and-arithmetic` (19) and the kit `🧰️stage-helpers` (fixtures, event builders; keeps the original suite docstring). Single-use helpers moved into their topic (`eyeOf`, `frozen`, `drawnClashes`, `PARTS`). The pets config lists the 8 topics explicitly because the kit must not be collected as a test file. `🔬️unit/🦀️.rs` (the Rust twin) stays.

## 4. Importers, source-text readers, registration

- **Importers of `GEO`** (33 files, 33 declarations rewritten by `reimport.mjs`; list in `ts3-geom-importers.txt`): every import and export declaration now names the child that owns each name. `✏️s/🔌️plugins/📐️cad/⚙️engine/🫀️core/🟦️.ts` re-exported the module with `export *`; it now exports exactly the 130 originally exported names, grouped per child (explicit `export {…}` and `export type {…}`), so the 25 newly exported internals do not leak through the cad core. 3 dynamic imports re-pointed by hand: cad runtime test (split into 5 awaits), repl suggestion test, dev cad script. No barrel remains.
- **Source-text consumers of `GEO`:** `🔣️taxonomy.json` consumer `cad-spatial-kernel-geometry` (pattern and identity) and the fixture `…/🖍️draw-source-scenario/🔣️.json` (`cadConsumerMount`) now name `📥️definition-catalog/🟦️.ts`, the module that holds the model-catalog path references (`🖼️assets/🏗️modelDefinitions`). The comment in `…/✏️editor/⚙️engine/🧬️typology/🦀️.rs` names the two children.
- **Source-text readers of the os script:** the phase corpus `L/🏗️build/🧪️tests/🎟️phase/🔣️.json` (`cargoCallerPaths` and `cargoReceiverPaths`: the script replaced by the three modules that call `runCargoTestsV1`: `🧱️script-bases`, `📸️snapshot-checks`, `🧪️source-and-native-tests`); fixtures and tests of `rust-source-direction` (`mountedRouter` to `📄️document-checks`, `writerRouter` to `✍️wal-checks`, with the literals in the two tests); `os-dev-composition-ownership` (`os-root-hash` path to `✍️wal-checks`); `root-artifact-dependency-source` (consumer of `runNestedCargoPackageAdapter` to `🧪️source-and-native-tests`). `fixture-law-ownership` reads only the router registrations, which stay. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.
- **Nx inputs:** `{workspaceRoot}/…/🎛️commands/**/*.ts` next to the script path in `L/📦️packages/🟦️typescript/📋️project.json` and in the `jco-package-adapter` `inputPatterns` of `🔣️taxonomy.json`.
- **Policy discovery:** the raw-spawn and null-budget rules scan `📜️script.ts` files only, so the moved os code would have escaped them. `L/⚖️laws/🧩️plugin-consistency/🏛️foundation` gained `POLICY_PACKAGE_SCRIPT_OWNER_DIRECTORIES` (one entry, `RUST/🎛️commands`) and `policyPackageScriptModulePaths`; `🚧️authority-bans` adds those modules to `policyDiscoverScriptTsFiles`. The shared directory walker is now one function used by both the root-script and the package-script lists.
- **Vitest configs:** pets (8 stage topics), presentation-react (5 topics, `includeSource` dropped), os (11 topics added to `include`).
- **Launch files (`.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, same edits in both):** the existing entry `⚖️test-browser-actor-operation-completion🖥️os🟦️` now points at `🪪️document-backbone-worker-wire` (it holds the test its `--testNamePattern` names); 11 entries `⚖️test-<slug>🖥️os🟦️` (group `4_gate`, orders `900.03645501` to `900.03645511`, directly after it) for the 11 backbone topics, in the existing naming. The pets and presentation suites have project-level entries only (`@semio-tech/pets:test`), so no entry was added for them. No command of the os script changed, so none was renamed. `launch.json` parses as strict JSON; the seed parses as JSONC.
- **Taxonomy:** 28 test folder names added to the head of `members-of-tests` and 24 module names (the 14 `GEO` children, `🎛️commands` and the 9 script modules) added to the head of `crate-module-children`, both in `semanticDirectoryMemberKinds` (method of `RESULT-TAXONOMY-SWEEP.md`). The sweep's simulator (`F/rtt-tools/taxmatch.mjs`, extended to `F/ts3-tools/taxwalk.mjs` for whole paths) resolves every new folder segment (`members-of-tests` or `crate-module-children`) with no ambiguity. The real `bun ./📜️script.ts verify taxonomy report` was not run (35+ minutes). The walker reports the pre-existing product folder `🎤️presentation` as unresolved; that is not mine.

## 5. Wrong-direction imports inside my files

| Edge | Handling |
|---|---|
| A source module imports its own tests (react target to deck test; `OS/🟦️.ts` to the backbone test, four times) | Removed. The tests import the code. Removing them also removed 1 `no-circular` cycle (section 6.4). |
| `GEO` children import `SpatialKernel` and `SpatialPreviewKernel` from the sibling `🗺️spatial`, which imports `GEO` back (4 cycles before) | The back edge is gone for every child that `🗺️spatial` imports (parent and `🧊️model`; `readModelEntityProperty`, the only model function needing the preview port, moved to `🧮️expressions`). The 7 type-only imports of the ports from `🗺️spatial` remain: the ports belong in `GEO`, but they are written in terms of the diff vocabulary that lives in `🗺️spatial`, so moving them is a separate refactor (hand-off 2). |
| The os backbone test reached into `OS/🟦️.ts`'s bag of imports | The bag names are imported from their owning modules; the `source` URL is a locator, not a dependency. |

## 6. Verification

### 6.1 Tests

| Suite | Before | After | Verdict |
|---|---|---|---|
| presentation-react config | 140 tests, 140 pass | 140 tests, 140 pass, 17 files | rows identical (`diff` empty) |
| pets config | 1,133 pass, 0 fail, 212 files | 1,133 pass, 0 fail, 219 files; stage 283 = 38+42+36+26+28+48+46+19 | rows identical |
| os config | 609 tests, 499 pass, 110 fail | 609 tests, 499 pass, 110 fail, 110 files | rows identical, the same 110 failures (107 `actor-transport`, 3 now in `⏱️directory-client-http` (1) and `🧾️app-channel-codec` (2)) |
| dev cad config (`GEO` importers, in-source `🗺️spatial`) | 12 tests, 10 pass, 2 fail, 5 files fail to load | identical rows and identical per-file results including the file-level load errors | identical |
| `GEO` exports | 130 value exports | 130 value exports, `Function.prototype.toString` (or JSON for data) identical for all 130 (`ts3-tools/geom-diff.ts` loads the original and the 15 modules under bun); 0 name duplicates across modules; 24 internals newly exported | identical |
| Phase corpus test `Cargo package scripts retain mandatory phases across lexical imported aliases` | n/a (not run before) | 1 pass, 373 expects, with the re-pointed corpus | passes |
| os script registrations | 74 `register("name", Class)` pairs | the same 74 pairs, all 9 modules load under bun (9 of 9) | identical |

### 6.2 `tsc`

`F/taf-tools/tsc.sh` with scoped tsconfigs from `F/ts3-tools/mktsc.mjs` (extend `OS/tsconfig.json`).

| Closure | Before | After |
|---|---:|---:|
| `GEO` importers and children (58 files) | 42 errors | 41 errors; the normalised set differs by one error that a peer fixed in `🏃️process/💾️state/📥️recovery` (`TS2307`); none in `GEO`. A `noUnusedLocals` run found one unused import after my last move; fixed |
| deck topics, react target, config | no baseline of the closure | 0 errors |
| os backbone topics, `OS/🟦️.ts`, config (15 files) | no baseline of the closure | 0 errors |
| stage topics and pets config | no baseline of the closure | 0 errors |
| os script and 9 modules | 2 errors (`cargoPhase`, measured on a copy of the original) | 2 errors, the same message, now in `🧪️source-and-native-tests` |

The deck, os-test and stage originals were deleted by the same step that created their replacements, so I did not take a closure baseline for them; zero errors satisfies "identical or smaller".

### 6.3 `--help` of the os script

`bun ./📜️script.ts --help` runs through the router entry and stops with `Original parent Script process invocation required`, exit 1. Before and after the split the output is the same text; the only differing byte range is the entry-call frame `script.ts:2934:7` (before) versus `script.ts:123:7` (after), because the entry call is now on line 123 of a 123-line file. With that frame's line and column normalised the two outputs are identical (`cmp` returns 0), so the CLI cannot be byte-identical in the strict sense. I also tried the sanctioned launcher (`bun repo exec --projects=@semio-tech/framework-os-kernel … bun <script> bogus-command`) to compare real dispatch, but it spent the 110-second cap in `build:cargo-owners` and was cut; the router table (74 registrations), the module load check and the identical entry error are my evidence for CLI parity.

### 6.4 `dependency-cruiser` `no-circular`

`node node_modules/dependency-cruiser/bin/dependency-cruise.mjs … --config F/ts3-tools/nocycle.cjs --include-only <tree> --output-type err` (the repo rule reduced to `no-circular`, `tsPreCompilationDeps`, so type imports and dynamic imports count). "Before" is a sandbox copy of the tree at `HEAD`, "after" a sandbox copy of the working tree, so the comparison is like for like; reports `ts3-cyc-*-before.txt` and `-after.txt`.

| Tree | Cycles before | after |
|---|---:|---:|
| `GEO` and its siblings under `⚙️engine` | 5 | 2 (the remaining two are `🗺️spatial` with its own in-source test and `🧮️preview`; no `GEO` module is in a cycle any more) |
| `PRES` | 1 (react target to its test and back) | 0 |
| `OS/🟦️.ts` and `OS/🧪️tests` | 0 | 0 |
| `PETS` | 0 | 0 |
| `RUST` (script and `🎛️commands`) | 0 | 0 |

### 6.5 Losslessness

`F/ts3-tools/lossless3.mjs` (trimmed non-empty line multiset, import declarations removed; `ts3-lossless-*.txt`):

- `GEO`: 3,215 original lines against 3,239. Only in the original: exactly the 25 declarations that gained `export` (private to exported). Extra in the new files: the 14 child docstrings, the `export` forms of those 25 declarations and the synthetic region markers (below).
- Deck: 3,402 against 3,422. Missing: the registrar signature and docstring, the dependency and vitest destructurings, the four dynamic-import consts (now static imports), the registrar's closing brace. Every test line is present.
- Backbone: 2,860 against 2,910. Missing: 4 registrar signatures, 4 dependency destructurings, 4 vitest destructurings, 6 closing braces and 7 type aliases and the archive const that gained `export`.
- Script: 2,739 against 2,774. Missing: the class and function heads that gained `export`, 3 dynamic imports and 2 relative imports with rebased paths.
- Stage: no `export` change on test lines; every `describe.each` title is in the set of 283 test rows.

Regions: no marker was dropped. `GEO` was already unbalanced (an unclosed `🧱️kernelGeometry`, a closing `🧱️Model` without an opening): the umbrella opening and closing lines stay in the parent, the children needed 6 synthetic markers in total (one each in `🪪️selection`, `🗺️paths`, `🧮️expressions`, `🎰️interaction-spec`, `📥️definition-catalog`, `🧷️selection-operations`), and the parent got 4 balancing markers for its pre-existing imbalance. Test and script files needed none.

## 7. Failure sets

Identical everywhere (section 6.1). Pre-existing failures I did not touch: 107 `actor-transport` and 3 `OS/🟦️.ts` in the os config; the 2 `🔬️layering` fixture-path tests and the 5 unloadable files in the cad config. `workspace-contract` (`bun test`, after all edits, no baseline): 682 pass, 74 fail, 83 skip; none of the 74 names a draw, cad, projection or consumer test, and the `authored Draw source…` tests are skipped in this environment, so the re-pointed `cadConsumerMount` could not be exercised.

## 8. Foreign errors and hand-offs

1. Pre-existing path breakage seen while repairing readers: `rust-source-direction/pool-use-ownership` and `deferred-wake-ownership` fail on stale `neutralFixture` and `mountedFixture` paths; `os-dev-composition-ownership` and `root-artifact-dependency-source` read fixtures from `L/🧫️fixtures` while they live under `L/🧪️tests/🧫️fixtures`. My re-points are in place but those tests cannot reach them.
2. `GEO` still imports `SpatialKernel` and `SpatialPreviewKernel` from `🗺️spatial` (7 type-only edges). Moving the ports into `GEO` needs the diff vocabulary of `🗺️spatial` to move with them.
3. `RUST/🎛️commands/🧪️source-and-native-tests` has two uses of an undefined `cargoPhase` (runtime `ReferenceError`, same two as before the split).
4. The os script split touched a file that peers were editing (modified at 12:51 and again in the session); it was applied under a SHA-1 pin (`c2c3ec91…`) and the snapshot is `ts3-orig/os-rust-script-original.ts.txt`. `OS/🟦️.ts` and the backbone test also differed from `HEAD` by peer edits; I took snapshots right before editing.
5. Real checks not run: `verify taxonomy report`, the launcher dispatch, `workspace-contract` before the split (no baseline), the Draw consumer tests (skipped).

## 9. Commands run (all from `/Users/ueli/Documents/semio`)

- Baselines and after-runs: `F/ts3-tools/vt.sh base-deck|after-deck <presentation-react config>`, `base-os|after-os <os config>`, `base-pets|after-pets <pets config>`, `base-cad|after-cad <dev cad config>`; summaries with `node F/ts3-tools/summ.mjs <json> <rows>`; `diff base.rows after.rows`.
- Type-check: `F/taf-tools/tsc.sh <tsconfig> <log>` for `ts3-tsconfig-{geom-base,geom-after,geom-unused,deck,bb,stage,script-base,script-after}.json`; comparison with `F/ts3-tools/tscnorm.sh`.
- Splits: `node F/ts3-tools/msplit.mjs <spec> [APPLY]` with `geom.spec.json`, `stage.spec.json`, `deck.spec.json`, `bb1..4.spec.json`, `script.spec.json`; `node F/ts3-tools/unclose.mjs <spec> APPLY` (deck, `bb1..4-unclose.spec.json`); `reimport.mjs`, `repoint.mjs`, `rmblock.mjs`, `names-meta.mjs`.
- Equivalence: `bun F/ts3-tools/geom-diff.ts` (needs the original compiled copy; regenerate it from `ts3-orig/geometry.ts.txt` by replacing the `../🗺️spatial/🟦️.ts` specifier with its absolute path), `bun F/ts3-tools/load-all.ts <modules>`, `node F/ts3-tools/lossless3.mjs`.
- Cycles: `dependency-cruise.mjs` as in 6.4 on the sandboxes (deleted afterwards, the reports stay).
- Source-text readers: `bun test ./L/🏗️build/🧪️tests/🎟️phase/🟦️.ts` (3 pass, 12 fail; the 12 need the script invocation envelope that only the launcher issues, so they fail the same way without my change), the five fixture tests (fail before and after at load or first assertion), `bun test ./L/🧪️tests/🔬️workspace-contract/🟦️.ts` (full run).
- `bun ./📜️script.ts --help` in `RUST` (before and after), `bun repo exec …` (cut by the 110 s cap).
- Read-only git only: `git ls-tree`, `git show HEAD:<path>` for baselines, `git diff --stat`, `git status`.

## 10. Files

Created: the 14 `GEO` children; `PRES/🧪️tests/{🎬️reveal-morph,🖼️deck-rendering,🔗️url-and-json-preview,📐️interaction-geometry,🖱️interaction-dom,🧰️presentation-deck-helpers}/🟦️.tsx`; `OS/🧪️tests/{🔄️backbone-envelope-retry,🧵️backbone-uris-and-workflow,🎁️pack-value-codecs,🧾️app-channel-codec,📞️app-channel-client,🕸️plugin-graph-and-routers,📇️directory-read-model,🪪️document-backbone-worker-wire,📃️directory-client-pages,🔔️directory-client-stream,⏱️directory-client-http,🛠️backbone-wire-helpers,🧮️directory-client-helpers}/🟦️.ts`; `RUST/🎛️commands/*/🟦️.ts` (9); `PETS/🔨️modules/🎪️stage/🧪️tests/*/🟦️.ts` (9).

Rewritten: `GEO/🟦️.ts`, `RUST/📜️script.ts`, `OS/🟦️.ts` (4 blocks and 5 imports removed), `PRES/🎯️targets/⚛️react/🟦️.tsx` (in-source block removed), the 33 importers of `GEO` plus `🫀️core` and the 3 dynamic-import files.

Deleted: `PRES/🧪️tests/🎞️presentation-react-deck/🟦️.tsx`, `OS/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`, `PETS/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` (and their emptied folders).

Edited elsewhere: the three vitest configs (pets, presentation-react, os); `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`; `L/🔣️taxonomy.json` (consumer, inputs, 52 names); `L/📦️packages/🟦️typescript/📋️project.json`; `L/🧪️tests/🧫️fixtures/🖍️draw-source-scenario/🔣️.json`; `L/🏗️build/🧪️tests/🎟️phase/🔣️.json`; the four fixture JSONs and two tests of section 4; `L/⚖️laws/🧩️plugin-consistency/{🏛️foundation,🚧️authority-bans}/🟦️.ts`; the Rust comment in `…/✏️editor/⚙️engine/🧬️typology/🦀️.rs`.

Temporary: `F/ts3-tools/` (scripts and specs), `F/🗑️generated/ts3-*` (logs, reports, snapshots in `ts3-orig/`).
