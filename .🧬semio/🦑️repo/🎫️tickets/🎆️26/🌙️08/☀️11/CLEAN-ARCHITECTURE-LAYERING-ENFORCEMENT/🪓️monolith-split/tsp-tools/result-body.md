# RESULT: TEST-SWEEP-PLUGINS (`tsp`)

Executor: Sonnet, foreground, no sub-agents, no ticket tool, no modifying git command. Run 2026-10-17 22:15 to 2026-10-18 00:40. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/tsp-tools/`. Logs: `F/🗑️generated/tsp-*`.

## 0. Verdict

DONE for everything that is not environment or an open peer decision. In the bun-run suites (513 files) the failing tests fell from 369 to 76 on the last full run, and 14 of those 76 (layout media export) were fixed again afterwards (a peer carve of the layout editor moved the Rust under the law while the run was going), so 62 remain. Passing tests rose from 5,676 to 7,401 (suites that used to die at module load now run, for example norm wire twins: 15 failures hid 2,720 tests). Files passing: 441 of 557 before (110 failing, 4 no-test helpers, 2 crashes) -> 483 of 513 after (26 failing, 4 no-test helpers). The 44 files that import `vitest` are no longer run under bun (they need the vitest runtime and were failing there for missing `vi` APIs); they run through their 14 vitest configs (section 5).

Every remaining failure has a named cause in section 4. None of them can be fixed here without cargo, a build/install, an owner decision or a restructure that peers are doing right now.

## 1. Method

- Universe: 572 tracked files under `✏️s/**` that import `bun:test`, `vitest` or `node:test` (`tsp-tools/runlist.txt`). The `.test.ts` naming is almost unused; taxonomy tests are `🧪️tests/<case>/🟦️.ts`. 15 of them are vitest config files (`vitest-configs.txt`), 44 are vitest tests.
- Runner `tsp-tools/run.py`: one `bun test ./<file>` per file from the repo root, `nice -n 10`, retry once on a crash, resumable, per-file JSON (`🗑️generated/tsp-baseline`, `-b`, `-r1`, `-r2`, `-final-bun`). Baseline = `tsp-baseline` + `tsp-baseline-b` (the 22 files that the first parallel run lost to rc -9 under swap pressure were re-run alone).
- Final bun run: `TSP_TIMEOUT=120000` (bun `--timeout`) and `SEMIO_TEST_ARTIFACT_DIR` set to a `tsp-artifacts` folder (energy needs it); everything else identical.
- Vitest: `tsp-tools/vt.py` builds the explicit `SEMIO_VITEST_POLICY` that the owner scripts pass, then runs `node vitest.mjs run --config <cfg>`.
- Machine state: load average 50 to 140, swap 11 of 12 GB. Several heavy suites were SIGKILLed or hit the 5 s default timeout in the first run; those are marked "load" below and pass when run alone.
- Per-file baseline list: `tsp-tools/baseline-summary.txt`. Current state: `tsp-tools/current.json`.

## 2. Before / after

| | before | after |
|---|---:|---:|
| bun-run test files | 557 (441 pass, 110 fail, 4 no-test, 2 crash) | 513 (483 pass, 26 fail, 4 no-test) |
| tests passing | 5,676 | 7,401 |
| tests failing | 386 (369 in the 513 retained files, 17 in vitest files run under bun) | 76 (62 after the layout fix) |
| vitest configs passing | not measured | 7 of 14 pass (fem, layout, remodel, puzzle x2, gis-service-composition, plus wfc moved to bun); the rest are environment (section 5) |

### Failing tests before -> after, by cause (bun-run files)

| cause | before | after |
|---|---:|---:|
| reader path stale after fixtures moved into `🧪️tests/🧫️fixtures` | about 150 | 0 |
| directory URL or concatenated path lost its trailing slash | about 40 | 0 |
| asset or fixture folder renamed (emoji or name) | 3 | 0 |
| sqlite capability moved to its `🚪️io/🪶️sqlite` owner | 7 | 0 |
| schema or generated shape moved on, test/codec/fixture stale (diff deltas, sparse deltas, native roles, jpg, dictionary change) | 24 | 0 |
| Rust source-text law followed a carve (crate names, `&mut self`, owners, new files) | 40 | 0 (14 reappeared from a peer carve and were re-fixed) |
| executor `nx:run-commands` -> `repo:script` | 4 | 0 |
| real data bug (German label reused English `end`) | 16 | 0 |
| removed by design (patch-snapshot leaves, layout doc oracle slot) | 9 | 0 (test halves deleted) |
| load / timeout / swap (pass alone) | about 25 | 3 |
| environment: unlinked workspace package, unbuilt dist, undeclared `@xmldom/xmldom` | 15 | 8 |
| needs cargo output or a built binary or wasm (pinned meshes, `SEMIO_FEM_NEUTRAL_RESULTS`, Nx envelope) | 20 | 22 |
| peer in-flight rewrite, law not yet adjudicated (semio native prefix) | 22 | 22 |
| structural placement law with real violations (14 fixture files) | 3 | 3 (4 tests) |
| other open: pdf stream-roles Rust twin never written (1), tsv async-closure parser (1), bim (r9a, 1) | 3 | 3 |

(The two "environment" and "cargo" rows count the same suites before and after; the cause rows that dropped to 0 are the ones fixed.)

## 3. Fixes (all re-run green, all with `bun test <file>` unless noted)

Reader paths and slashes (the test, not the data, was stale):
- procedural: `📚️examples/🧪️tests/🧩️geometry` fixture and budget constants (8 example suites), `⏱️phased-job/🧪️tests/🧰️support` `loadFixture` reads `<subset>/🧪️tests/🧫️fixtures/<category path>` (new `loadModuleFixture` for the module-local `brep-interchange`), mesh-support oracle, generate-interactions, process3d timber-replay.
- shooting asset folder `🌲️hexagonal-cut`, vdi3805 `🌶️nonconforming`; shooting graphql/proto facet `/`.
- trinity rewriting snapshot (5 URL roots, 5 concatenations, capture root, typed fixture schema lists gained `artifact-reference` and the jack diff, graphql roots gained `artifact-reference`, sparse deltas apply onto `before` with `null` = unchanged), trinity binary64-transport, trinity-graph-child composition (typed fixture root, `🧬️mutations/`, `🧪️tests/🧫️fixtures` roots).
- block io suites x2 (`../🧫️fixtures`), block5d sqlite (root `/`, Block3d catalog uses the stdio kit dialect now enforced by the parser), puzzle 3d/5d sqlite (root `/`, `Puzzle5dScale` lives in `document-records`/`document-model`, `Puzzle5dSelection<'a>` is a view not a record), puzzle2d canonical (families under `🧬️mutations`), puzzle2d close (family paths, `pub fn`, `testkit::`).
- stdio: step history inputs (fixtures under `🧪️tests`, mutation codecs under `📝️text`/`💾️binary`, generic construction), binary history, ifc2x3, pdf14, bmp, dwg schema, dwg metadata (Rust now split into object files), pdf retirement (`📐️model`), norm wire twins, norm oracle-source-ownership control path (and the Python host scratch dir now lives under `SEMIO_TEST_ARTIFACT_DIR`, the host refuses a workDir outside the workspace).
- spatial-kernel mesh-session, cad layering fixture paths.

Schema-first (the schema/generator moved, the consumer was stale):
- animate: diff source patch fields are `{value}`; equation: sparse expression diff `{kinds,nextLabel}`; architect: program JSON delta codec rewritten to `removed/inserted/moved/modified` (`🚪️io/📝️text/🔺️diff/🔣️json`), contract test and native-roles oracle follow; jpg compact patch row follows the `{schema,image}` snapshot; layout dictionary-change fixture (hub composition, now `layout.dictionary-change/v2`) follows `LayoutDataFieldsDelta {presence, entries}`.
- Vendor vocabulary: `x-semio-collection-catalog` registered in `🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json` (the architect snapshot schema carries it).
- Data bug: 16 en1993 `insert-*` leaves printed the English word `end` in the German label; now `Ende` (`🔩️en1993/.../🧬️mutations/*insert*/🦀️.rs`, one label line each, not compiled here).

Source-text laws that followed carves: procedural geometry registry (crate names from `COMPUTES` paths), flow tessellate, draw interaction topology and svg export, layout media export (producer and adopter now in `🎥️media-export-job-and-factories` and `🕹️layout-play-app`; source schema, fixture, TS type and the `🔣️schema-catalog.json` sha256 updated twice), stdio retained-native refusal (6 producers, `OneItemOwners` delegation, fem3d `📬️store-preparation`), trinity member admission (ingress, composition), note action cohort (command enum, `mutation_apply_preparation_factory`), catalog browser-host (nested `withTestBrowserHostV1`).

Executor `repo:script`: wfc bitmap x2, energy capability. The wfc package: its five wfc3d tests still imported `vitest` (the other wfc suites were converted to `bun:test` on 2026-10-01) and its vitest config listed `bun:test` files; the five tests now import `bun:test`, `📜️script.ts test` runs `bun test` over the same two globs (as layout does), the stale vitest config was deleted and its `project.json` input now names the artifacts.

Removed by design (evidence in the file history):
- stdio compact patch test: the `nativePilots` half that asserted a `🩹️patch-snapshot` leaf schema, descriptor and protocol record per artifact. Commit 251fa724c4a deleted the `🩹️patch-snapshot` and `📸️set-snapshot` leaves of every stdio artifact; the patch algebra half (prepare/apply/inverse/size) stays and the fixture rows lost their dead fields.
- layout `🧬️schema/🧪️tests/🔣️transport`: the doc-contract oracle needs the Drawing child codec, which `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing` binds (commit 36fbe9da786 deleted the layout-owned `🪪️document` module). The test now covers the owner-only facets and the unbound-slot refusal; the composition test keeps the full oracle (it passes).

Declared dependency: `@xmldom/xmldom` (used by the draw svg tests and the stdio docx generator) was in no `package.json` and not installed. I added it to `✏️s/package.json` `devDependencies` and copied the cached 0.9.10 package into `node_modules/@xmldom/xmldom` (untracked). With it the 7 draw svg files pass (including the fill-rule test). `✏️s/bun.lock` still needs a `bun install` to record it.

Other test-runner changes in this slice: `x-semio` vocabulary and `package.json` above; no test deleted outside the two items above.

## 4. Remaining failures (bun-run files), by cause

Environment: workspace package not linked or not built (`bun install` / `build` needed): writer sqlite (`@semio-tech/writer-writer`), gis terrain sqlite, gis gismap sqlite (no dist), space home sqlite, stdio las public, stdio dwg public, stdio pdf font, wfc bitmap "unlinked public artifact" (no dist). 8 tests.

Needs cargo output, a built binary, wasm, or the Nx invocation envelope: fem surface x2 (`SEMIO_FEM_NEUTRAL_RESULTS` from a Rust run), stdio dev command-ownership x3 (cargo metadata), catalog preparation x1 and native-prerequisites x5 (Nx namespace), procedural brep x3 files (4 tests; the committed pinned tessellations are not in the fixtures, refreshed only by `support::writing()` in the Rust twin), geometry-service x7 (its fixture is the 3-byte placeholder `[]` since 251fa724c4a; the Rust twin reads the same placeholder). 22 tests.

Peer in-flight, law not adjudicated: stdio semio base `🫙️prefix` x22. The native decode in every semio subset was rewritten (`decode(payload, control, native_control)`, `binary`/`document`, `native::Owned::new` x23 in brep); the law still asserts `fn binary_into(`, `fn document_into(`, `workspace::decode(` and no `Owned::new(`. The Rust never contained the old names during the history I can see (since 2026-10-10), so the law has been red for a week. It needs the owner of the native prefix work to restate the law.

Structural law with real violations: `🧪️tests/📁fixture-placement` x4. 14 files under `🧫️fixtures` are not beneath a `🧪️tests` owner or sit under `🧬️schema`: flow tessellate `🔣️.json`; procedural, fem (x3), layout, raster, stdio part21, jpg and zip TS fixture entries under `🧪️testing/🧫️fixtures` (public exports of the tfe entries work, `package.json` `./testing/fixtures/*`); trinity jack `rich-read`, `query`; stdio gif diff (x2). Moving them needs importer and package-export rewrites that conflict with the tfe fixture-entry design; left for the coordinator.

Open, single items: stdio pdf `stream-roles` (the TS test asserts a Rust fn `pdf_stream_roles_multi_object_reorder_delete_and_path_refusal` that has never existed in git history), stdio tsv (the independent tree-sitter Rust parser in `tree-sitter-wasms` does not know async closures `async|a,b|` that today's `🧪️tests/🦀️.rs` uses), bim (r9a skip).

Load / timeout (pass when run alone): procedural `set-widget-input` (explicit 5 s test timeout), native-ownership (5 s), norm `oracle-source-ownership` (its plan builder walks the repo, 165 s under load against a 30 s test timeout; the tmpdir bug that hid behind it is fixed).

## 5. Vitest (14 configs, `tsp-tools/vt.py`, last run `🗑️generated/tsp-vitest-fin*`)

Pass: fem (7 files), layout renderer-contract (1), remodel (4 files, 1,373 tests), puzzle (12 files) and puzzle renderer-contract (3), gis service-composition (3 files). wfc left vitest (above).
Fail, environment: gismap presentation (1 file: `@semio-tech/framework-renderer-react` resolves `exports` `./🟦️.tsx` to `/🟦️.tsx` under vitest/Node 24), cad extensions x3 (`@semio-tech/cad-js-module-spatial-shape` not installed), trinity jack lsp (`self` is not defined: worker module under node), gis services x2 (config uses Bun-only `import.meta.dir`; the owner script runs it through its own runtime), catalog (4 tests: strip-only TypeScript parameter property, product mode env, generator preview command, source audit cancelled).

## 6. Python oracles

198 `🧪️tests/**/🐍️.py` adapters are loaded by the repo test host `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py` (it injects `semio_repo_test`); there is no standalone entry. They run through `bun ./📜️script.ts test oracle <level> --owner <path>` of the test platform, which needs the Nx invocation envelope; the only TS test that drives the host directly is norm `oracle-source-ownership` (161 EN 1991 scenarios), whose host crash on the scratch dir is fixed and whose run is timing-limited (section 4). I did not run the other adapters. Installed oracle libraries: ifcopenshell 0.8.4, numpy 2.0.2, shapely, sympy, jsonschema (so the bim IFC oracle imports); python 3.9.6 is the system interpreter, the host chose a uv 3.14 interpreter.

## 7. Rules and cleanup

- No `[DEBUG]` log added. Pre-existing `[DEBUG]` console lines remain in the draw `scene/📋️prepare` and `scene/🎨️paint` tests (not mine).
- Not run: any Rust, any Python adapter, the 8 environment suites, `bun install`.
- Files outside `✏️s/**` touched: `🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json` (one keyword), `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` (one sha256, layout source schema), `🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🧪️tests/🪶️sqlite/🧫️fixtures/🧾️dictionary/🔺️diff/🔣️.json`, repo `node_modules/@xmldom/xmldom` (untracked copy).
