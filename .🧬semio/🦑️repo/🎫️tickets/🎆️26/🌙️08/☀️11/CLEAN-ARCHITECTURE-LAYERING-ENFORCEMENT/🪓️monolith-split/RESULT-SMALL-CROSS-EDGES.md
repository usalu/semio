# Result: Small Cross Edges (P8, P9, P10), prefix `sce`

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Run 2026-10-16 05:20 to 06:40. Raw logs, tools and the scoped cruiser outputs are in `🗑️generated/sce-*` and `🗑️generated/sce-tools/` next to this file (nothing deleted).

## 0. Verdict

| Package | Cruiser edges before | after | Status |
| --- | --- | --- | --- |
| P8 `no-cross-plugin-📏️layout-to-🗄️stdio` | 4 (3 prod, 1 test) | **0** | Done for the TypeScript twin. The Rust side (layout crates depend on the stdio drawing crates, `include_str!` of the drawing DDL) is unchanged and is not visible to the cruiser. |
| P9 `no-cross-technology-🌎️hub-to-✏️s` | 16 (2 prod, 14 test) | **0** | Done. |
| P10 `framework-no-implementation` | 3 (test) | **0** | Done. |

Scoped cruiser, the exact command of `DEPENDENCY-DIRECTION-AUDIT-2.md` section 1 with the touched files as arguments instead of the four roots: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <paths> --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (`🗑️generated/sce-cruise.sh`, path lists `sce-cruise-paths-before.txt` / `sce-cruise-paths-after.txt`).
- Before: exit 188, 1468 violations, 932 modules, 3592 dependencies. Rule lines for the three rules: 23 (`hub-to-✏️s` 16, `no-cross-plugin` 4, `framework-no-implementation` 3). Log `sce-cruise-before.txt`.
- After: exit 196, 1476 violations, 985 modules, 3721 dependencies. Rule lines for the three rules: **0** (`/usr/bin/grep -F` over `sce-cruise-after.txt`). Log `sce-cruise-after.txt`.
- The total grew by 8 only because the after scope contains the new files (53 more modules). The per-rule diff (`sce-rules-before.txt`, `sce-rules-after.txt`) shows the growth is in hygiene families that the moved tests now live under: `no-cross-package-relative-@semio-tech/layout-layout-rs` 0 to 23, `…/framework` 48 to 62, `no-circular` 160 to 184 (24 new cycle lines, 22 inside the peer-owned `🎠️kernel` SDK package tree and 2 in actor and plugin-describe modules; a grep for my new or changed files finds none, 17 old lines disappeared). The moved layout tests carry the same framework imports as before, now counted under the composition project. This is P11 hygiene, not one of the three rules.

## 1. Baselines (before any edit) and after

| Check | Before | After |
| --- | --- | --- |
| Vite artifacts tests (`bun test`, P10) | 47 tests, 45 pass, 2 fail (`neutral browser inventory …`, `neutral package inventory …`: both need `SEMIO_TICKET_DIR`, unset in this shell), 1054 expect() | trimmed file 43 tests / 41 pass / same 2 fail / 1018 expect(), plus the new play file 4 pass / 36 expect(): 47 tests, 1054 expect() in total. Failure set identical. |
| `tsc --noEmit --strict` on the touched files (flags in `🗑️generated/sce-tools/tsc-run.sh`) | 10 errors (`sce-tsc-before2.txt`): 4 in the layout sqlite test (TS2542 ×2, TS2769 ×2), 4 in the hub space ownership test (TS2322, `format` typing), 2 in the hub stdio shipped-fleet test (TS2307, wrong fixture path) | 4 errors, the identical 4 layout sqlite ones (now in the composition copy, positions shifted by my inserted lines); the 6 others are gone (`sce-tsc-p8b.txt`, `sce-tsc-p9.txt`). One new unrelated hit in `p9`: `gismap/🧪️tests/🧩️map-create-region-group` line 90 TS2353 `artifactDir`, on a line I did not touch (the file was not in the baseline set). |
| Layout sqlite snapshot suite + document-contract oracle | 18 pass / 1 fail (sqlite; failure `$.dataFields: unknown field`, pre-existing, recorded by `RESULT-PLUGIN-CROSS-EDGES.md`) + oracle passes | composition copy: 19 pass / 1 fail (same failing test; the 19th pass is the oracle wrapper) |
| Stdio shipped-fleet sqlite TS suite | not run before the move; the tsc baseline already showed TS2307 for both dependency tests (import of a fixture from an empty directory) | 5 pass / 0 fail, 45 expect() |
| Space documents (2 tests), GIS declaration roundtrip (1 test), hub io-boundary (2 tests), hub ownership (2 tests) | not run before (moved/trimmed) | 2 / 1 / 2 / 2 pass |
| Stdio composition contribution contract checks | run by the stdio contract package script (`composition-selection` twin) | 25 checks pass from the hub (`runCompositionContributionChecks()` returned 25) |
| Hub local-relay admission tests | n/a | 4 pass |
| Composition project (all 5 files, shared slot) | n/a | 32 pass, 4 fail (3 pre-existing trinity failures from `RESULT-PLUGIN-CROSS-EDGES.md` W2.2, plus the 1 layout failure above) |

## 2. P8: layout to stdio (Semio drawing)

Findings (evidence): layout embeds a `SemioDrawingSnapshot` as `backgroundDrawing`. Its TS twin called three stdio drawing modules: the snapshot parser (schema), the native JSON codec and the sqlite component codec with its DDL. The drawing standard is 467 files and 6 Rust crates over `✉️base` geometry, so moving it to the framework (the "rightful owner" the task names) is the deferred option 1 of `RESULT-PLUGIN-CROSS-EDGES.md` and was not attempted. No stdio SDK exists (`publicApi.sdkPackage` in the taxonomy is `@semio-tech/framework`).

Design (decision): invert the dependency with a neutral framework port, bound by the one place that knows both owners.
- New framework port `🧰️framework/🔨️modules/🚪️io/🔌️ports/🟦️.ts`: `EmbeddedChildCodecV1<Content>` (parse, nativeJson from/to, sqlite schema/project/reconstruct) and `embeddedChildSlotV1(id)` (unbound refusal, idempotent rebind, refused replacement). SDK entry `📦️packages/🟦️typescript/🚪️io/🔌️ports/🟦️.ts` and package export `./io/ports`. Test `🔌️ports/🧪️tests/🟦️.ts` (3 pass), gate `test-io-ports` (script, project target, package.json, both launch JSONs).
- Layout declares the slot: `🧬️schema/🟦️.ts` exports `LAYOUT_BACKGROUND_DRAWING` and `LayoutDrawingContent = object` instead of importing `SemioDrawingSnapshot`. Native JSON `child()` and the sqlite twin use the slot; `LAYOUT_SQLITE_SCHEMA` became the function `layoutSqliteSchema()` because the DDL now comes from the bound codec. Layout imports only `@semio-tech/framework/io/ports`.
- The binding lives in the s-level composition project `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🔗️binding/🟦️.ts` (the only module importing both layout and the stdio drawing). `SEMIO_DRAWING_EMBEDDED_CODEC` is built from the unchanged stdio exports; `bindLayoutBackgroundDrawing()` is idempotent.
- The two layout TS tests that need real drawing content moved there with all relative paths recomputed by script (`sce-tools/p8-move.py`, 22 + 17 specifiers, all resolve; one string constant `base` fixed by hand): `📏️layout-drawing/🪶️sqlite/🟦️.ts` (was `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`) and `📏️layout-drawing/🪪️document/🟦️.ts` (was `🧬️schema/🧪️tests/🪪️document/🟦️.ts`, plus a bun wrapper test). Registered in the composition `📜️script.ts` (launch entry `⚖️s-plugin-composition🏘️s🟦️` already exists).
- Layout Rust package script: `verify layout-document-contract` no longer runs the TS oracle (it runs the type-check and the Rust test as before); `snapshotSqliteTests` no longer lists the moved file.
- Rust, GraphQL, proto and JSON schema files of layout are unchanged. The `@child kind=s.stdio.semio` annotations stay (declared dialect data).
- Layout depends on the framework only for TS: 0 cruiser edges to stdio.

Files: layout `🧬️schema/🟦️.ts`, `🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts`, `🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`, `📦️packages/🦀️rust/📜️script.ts`; deleted the two moved tests; new composition files above; framework port files above.

Not done / residual for P8: the Rust layout crates still depend on the stdio drawing crates (366 cross-plugin Cargo edges overall, option 1). The slot is global state per module instance, so every consumer of the layout sqlite or native JSON twin must call a binding first (today only the composition tests do).

## 3. P9: hub to s (16 edges)

| Edges | Fix |
| --- | --- |
| `🚀️local-relay/🧭️routing` to GIS schema (1, prod) | The relay needs only the hub's own inference cursor bound. `🌎️hub/💡️inference/🧬️schema/🟦️.ts` (hub-owned `hub.inference` contract, which already held `progressMaxCursor: 16`) now exports `INFERENCE_PROGRESS_MAX_CURSOR` and uses it in `INFERENCE_LIMITS`. The relay imports it. Drift guard: the hub package script (a composition root, exempt in the cruiser policy) already compared the fixture to the GIS constant; it now also asserts `INFERENCE_PROGRESS_MAX_CURSOR === GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR`. |
| `🧩️compositions/🗄️stdio/🧩️composition` to the stdio registry contract (1, prod) | The contract (admit, select, resolve native factories and open targets; 126 lines, no imports) has exactly one consumer, the hub generator, and owns no stdio-only logic. It moved with its tests, fixtures and both schemas to `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/📋️contract/**` (5 files, `mv`, text identical). Schema `$id` is now `…/hub/compositions/stdio/composition/contribution/schema.json`; the entry was removed from `🧰️framework/…/🗣️dsl/✨️derive/🗂️schema-document-index.json`. The stdio contract package script lost the `composition-selection` twin; the hub script gained `composition contribution-check`, project target `composition-contribution-check` and a launch entry. Plugin artifacts keep authoring the data (`semio.stdio.composition-contribution/v1`, unchanged in every artifact contribution file). |
| `🧪️tests/🤝️dev-collaboration` to the s dev catalog (2) | The two generated catalog imports were unused except for the host ids. `COLLAB_E2E_REQUIRED_PLUGIN_IDS` became the lazy `collabRequiredPluginIds()` over the framework's `generatePluginRegistry(repoRoot)` hosts (parity checked: same ids, same order as the generated catalog; the discovery walk costs about 30 s, which is why it is lazy). |
| space ownership test (3) | The two plugin-owned laws (schema oracle, hostile batches) moved to `✏️s/🔌️plugins/🪐️space/🫀️core/📄️documents/🧪️tests/🔬️unit/🟦️.ts` (typed with a `Sources` cast; the four TS2322 are gone). The two hub source laws stay. Launch gate `⚖️gate🪐️space📄️documents🟦️`. |
| stdio shipped-fleet sqlite test (4) | Stdio-owned law, moved to `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts` with its fixtures `🧠️owners` and `🧩️dependencies`. The hub Rust shipped-fleet test shares the `🧠️owners` fixture, so its 5 `include_str!` paths were re-pointed to the new location (path exists; not compiled). Two tests previously failed on a wrong `🧫️fixtures` path (empty directory); fixed by the move (5 pass). The hub stdio script's two paths (`census check`, `census source`) were updated. |
| GIS io-boundary and three GIS receivers (5) | Declaration roundtrip is plugin-owned: moved to `gismap/💡️inference/🔌️client/🧪️tests/🚪️io/🟦️.ts` (gate `⚖️gate🌍️gis🚪️io🟦️`). `compileGisScopeExport` was a generic Ajv scope-export helper (and only `native-codecs` used it; `inference-control` and `component-cold-map-patch` imported it unused). It is now `compileSchemaScopeExport` in `🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts` on top of the canonical `semioSchemaAjvV1` (no duplicated keyword list), SDK entry updated, test appended to the validator tests (4 pass with `SEMIO_TEST_ARTIFACT_DIR`; the one env-dependent pre-existing test fails identically without it). The plugin copy and its directory were deleted; the two dead imports removed; the io-boundary source law now asserts that no GIS receiver imports a plugin and that `native-codecs` selects the framework owner. |

## 4. P10: vite artifacts tests to play (3 edges)

The four tests that import or read the Play pages publication moved byte-identical (only paths recomputed) from `🧰️framework/…/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` to `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/🧪️tests/🟦️.ts` (4 pass, 36 expect(), `tsc` clean). Launch gate `⚖️gate🏢️semio-tech🎡️playpage-publication-cohort` in both JSONs. The two remaining framework tests that read `🏢️semio-tech/🎡️play/📋️project.json` through `readFileSync` (stage barrier ordering, bundle job producers) are file reads, not imports, and were left in place (see residual).

## 5. Registrations (both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, unique order values, both parse as JSONC)
`⚖️gate🏢️semio-tech🎡️playpage-publication-cohort` 13.56, `⚖️gate🪐️space📄️documents🟦️` 13.57, `⚖️gate🌍️gis🚪️io🟦️` 13.58, `🌎️hub 🗄️stdio 🧩️composition 🧪️contribution-contract` 900.05686, `🧪️test🧰️framework🚪️io-ports` 900.0585155. Tools: `sce-tools/launch_insert.py`.

## 6. Commands run (real results)
- Cruiser before/after: see section 0.
- `bun test` of every file listed in section 1 (exit codes in the table). Framework validator tests with `SEMIO_TEST_ARTIFACT_DIR=…/🗑️generated/sce-artifacts`: 4 pass.
- `tsc --noEmit --strict …` before/after (section 1).
- `bun repo run @semio-tech/dsl-derive-rs:check-schema-index`: timed out in the launcher (section 7).
- No cargo command was run: no Rust source changed except 5 `include_str!` paths (static check: the target file exists).

## 7. Open points, foreign errors, hand-offs
- Schema index freshness: **not proven by the generator.** `bun repo run @semio-tech/dsl-derive-rs:check-schema-index` (`NX_FORCE_REUSE_CACHED_GRAPH=true`, cap 1500 s) ended with SIGALRM while still in the launcher's `build:*` stages, never reaching the check (`sce-p9-index-check.log`, status `exit=142`); the first attempt died on `ENFILE`. What I verified instead: the committed JSON still parses, and the index walker (`schemaIndexRoots` in `🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts`) only collects `🧬️schema` documents below directories named `🔌️plugins` or `🔨️modules`; the moved schema now sits under `🌎️hub/🧩️compositions/…`, like the hub owners schema that has never been indexed, so removing its one entry (and nothing else) is the projection. Re-run `generate-schema-index`/`check-schema-index` when the launcher is quieter.
- Peer SDK codemod (`@semio-tech/framework/<subpath>` imports; it also rewrote the framework validator import of the new space test to `@semio-tech/framework/schema` while I worked, which I left as is) rewrote my first `map-create-region-group` import to the old `…/document` subpath; I corrected it to `schema/oracles/validator` and added the SDK entries for the new symbols (`io/ports`, `compileSchemaScopeExport`). A later codemod run must keep them.
- Foreign failures seen while verifying (not mine): `🧪️tests/🧱️socket-grant-command-source` 1 fail (`hub schema module …🧩️component/🏗️compiler/♻️capture/🧬️schema is absent from …🔣️schema-catalog.json`, stale catalog); trinity composition tests 3 fail (known); `ENFILE: file table overflow` appeared three times under fleet load and was retried.
- Residual hub-to-s couplings that are file reads, not imports (invisible to the cruiser): `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧱️io-boundary` test 1 reads plugin and dev-variant sources; hub Rust `shipped-fleet/🦀️.rs` now `include_str!`s a stdio fixture; framework vite artifacts tests read `🏢️semio-tech/🎡️play/📋️project.json` and `♻️mit-bestand/🧺️demonstrator/📋️project.json`; the hub composition generator reads plugin artifact directories by data-declared paths.
- The `[DEBUG] ` console lines in the moved tests are the original permanent test output, copied verbatim; I added no temporary debug logs.
- Stale `🧫️fixtures` fixture `📱️app-verification-source-ownership` still names the removed `compileGisScopeExport` under a path that did not exist before this work.
