# Result: TS-TYPE-ERRORS-5 (prefix `tt5`)

Executor: Sonnet, 2026-10-17 15:35 to 19:55 (resumed 17:12 after the usage-limit reset). Foreground, no sub-agents, no modifying git command, no ticket tool. Logs in `🗑️generated/tt5-*`, tools in `tt5-tools/`. Continues `RESULT-TS-TYPE-ERRORS-4.md`.

## 0. Verdict

PARTIAL, by design of the remaining owner-deep families.

| Ruling | State |
| --- | --- |
| 1. 159 `expect<unknown>(...)` sites typed properly | DONE. 0 `expect<unknown>(` / `expect<any>(` remain outside ticket folders (`git grep`). The 159 sites (89 files) plus a second population of 95 sites (53 files) found while reverting are typed; no `unknown`/`any` widening, no cast introduced. |
| 2. `verify typescript check` reports parse errors per file and still type-checks the rest | DONE in the permanent route, with 3 new tests (23 pass, 0 fail). |
| 3. TS/TSX parse sweep | DONE. 5 real syntax errors (all ticket scratch files) fixed; sweep over 18,468 files ends with 0 files with errors. |
| 4. Continue the 315 real diagnostics | 295 real (b0) -> 97 real (w6). Of the 97: 14 belong to a peer's in-flight test split (3.6), 4 were fixed after the run (verified by scoped checks), 79 are owner-deep and catalogued in section 4 as work orders (48 custody/storage-admission tests, 18 generator/schema stubs, 13 missing packages and wasm). |

## 1. Counts

Whole workspace through the permanent route (`bun ./📜️script.ts verify typescript check --out ... --max-diagnostics 100000`), same 13,8xx-root program each time.

| Run | Time | Roots | Diagnostics | Skipped by order (TS2307, TS7016, mit-bestand) | Real |
| --- | --- | --- | --- | --- | --- |
| `tt5-b0` (start, TT4 runner) | 15:39 | 13,800 | 557 | 262 | 295 |
| `tt5-w1` | 17:37 | 13,852 | 553 | 262 | 291 |
| `tt5-w2` | 18:03 | about 13,860 | 489 | 262 | about 227 |
| `tt5-w4` (all matcher sites typed) | 18:57 | 13,870 | 428 | 252 | 176 |
| `tt5-w5` | 19:25 | 13,905 | 347 | 252 | 95 |
| `tt5-w6` (final whole run) | 19:57 | 14,000 | 363 | 266 | **97** |

`tt5-w6` is higher than `tt5-w5` only because a peer slice added a new test directory during the run (`library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests`, 14 real diagnostics plus 14 TS2307 for `../🟦️.ts` imports that its owner move has not landed yet). Without them: 83 real, 349 total. Four more diagnostics (gltf base document, two in the CAD renderer story, the media-transport lifecycle oracle) were fixed after `tt5-w6` started and are verified clean by scoped runs `tt5-x2` and `tt5-x3` (their only remaining diagnostics are TS2307 for missing workspace packages): effective real count 93, in-scope 79.

Top codes of the skipped class in `tt5-w6`: TS2307 102, TS7016 15, mit-bestand 161 (139 of them TS7006). The 15 TS7016 are `jsdom` 4, `yargs-parser` 4, `d3-regression` 2, `bidi-js`, `check-error`, `color-convert`, `suncalc`, one `jcoprobe` browser-host `.js`; `graphlib` had 25 before its declaration file and has none now.

Matcher TS2769 inside the 89 + 53 matcher files: 158 + 84 -> 0.

## 2. Verification done (real results)

- Permanent route test `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔷️typescript-project/🟦️.ts`: **23 pass, 0 fail** (3 new tests: a syntax-broken file is reported per file with its first error while the rest is still type-checked (tsc itself reports only TS1109); log byte-equal with `tsc` on a parsing project; syntax summary).
- Bun comparison before/after per file (`tt5-tools/tt5-bun.sh`, `tt5_bun_cmp.py`; snapshot copies, files importing `vitest` are listed as `VITEST` and skipped by the script):
  - 89 matcher files: baseline `base` vs `after2`: 86 identical failing sets, 2 improved (playground diff fixture handcrafted to the current flat `{schema}` schema 11/6 -> 12/5; taxonomy-leading-grapheme 9/1 -> 10/0), 1 worse: `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts` 63/0 -> 62/1. The HEAD version of that test run now against the current tree fails 61/2 (it additionally fails the dev-reference law), so the failure is source drift: a peer added the dev-dependency `semio-framework-test-fixtures` to a Cargo.toml the test pins. Not caused by tt5.
  - 53 second-population files (`ubase` -> `uafter`): 51 identical, 2 improved (CAD brep diff test, which still passed an obsolete `{breps:{values}}` diff and now uses the current `removed/inserted/moved` delta: 64/3 -> 65/2; paint 25/4 -> 29/0).
  - 25 files edited in the last round (`r4base` = HEAD version of each test run against the current sources, `r4after`): 24 identical, 1 changed-not-worse.
  - Single files: readme-current-source-revision 33/0, owner-projection-policy 26/9 (identical to baseline 26/9), admission 1/0, filesystem discovery 1/0, GIS mutation inputs 4/0, CAD snapshot sqlite 65/2 (the two failures are pre-existing GraphQL/protobuf sibling checks).
- Scoped type checks: `tt5-u5` (55 files) leaves only TS2307 `@xmldom/xmldom`; `tt5-x1` (6 files) 0; `tt5-x2`, `tt5-x3` only TS2307.
- Tests that could not be run in bun here (their own gates): vitest suites (row-action-admission, media-transport lifecycle, puzzle compose5d, catalog host-activation), `production-cache-input-boundary` (its fixture sample paths no longer exist: first assertion fails before and after), the note document oracle (see 3.4).

## 3. What was fixed

### 3.1 Ruling 3, parse sweep
`tt5-tools/tt5_parse.mjs` (TypeScript compiler `createSourceFile`, diagnostics only) over 18,468 tracked and untracked-not-ignored `.ts/.tsx/.mts/.cts`. Five files had real syntax errors, all ticket scratch files excluded from the workspace program: a nested `*/` in a docstring (`ARTIFACT-SCHEMA-FACETS/...region.ts`), a U+FFFD-corrupted duplicated tail (`STDIO-ARTIFACTS-AND-IO/generators/w1-discovery-contracts.ts`), an unescaped `/` in a regex literal plus an extra `)` (`product-descriptor-icon-inputs{,-2}/📜️script.ts`), a literal newline in `split("\n")` plus a duplicate import (`DASHBOARD-LAUNCH-COCKPIT/r2-l2-ids.ts`). After the fixes `tt5-parse1.txt` is empty.

### 3.2 Ruling 2, permanent route
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts` no longer runs the `tsc` CLI. It runs itself under Node (`--experimental-strip-types`, `--typescript-check` child, erasable syntax only) and type-checks through the compiler API: all diagnostics are written in `tsc` spelling, syntax diagnostics go to a separate `.syntax.txt`, semantic checks are skipped only for files that do not parse, and `check` prints every non-parsing file with its first error. Tests in `📚️library/🧪️tests/🔷️typescript-project/🟦️.ts`.

### 3.3 Ruling 1, matchers (techniques in order of preference)
Real value types (typed row or fixture type, Ajv `compile<T>` type guards, `db.query<Row, Params>`, `applyPatch<PostState>`, `Promise.all([a, b])` instead of a generic `Awaited<S>`), `[...tuple]` for tuple vs `number[]`, swapping actual and expected where the fixture is the widened side (mismatch messages print "expected" and "received" swapped for these sites), `Object.is` for identity checks, `toMatchObject` in both directions where a branded or JSON type cannot be assigned (arrays and objects only; a `null` actual would throw instead of compare), `Bun.deepEquals` for one nullable delta, small decoders (`triple`, `ownerPairs`, `nxInputRow`, `Wire`) where a fixture row is a union of shapes. New helper `canvasNode` maps SVG scene nodes to canvas nodes in the SVG export test instead of casting.

### 3.4 Ruling 4, tests behind redesigned APIs, owned build vs Vite, third-party typings
- Owned build vs Vite boundary: `hostBuildPlugins(plugins): Plugin[]` in the React build tooling admits owned plugins at run time (`hasHostPluginShape`) without a cast; `.storybook/main.ts` (6 call sites), `.storybook/preview.tsx`, the pets Vite config and three vitest configs (`defineOwnedTestConfig`) use it.
- Scripts moved to the current custody API: graph `📜️script.ts` (issued control, `runRepositoryCargoTestsV1`), space-core (`receiveScriptProcessInvocation` + `runRepositoryCargoTests`), gltf (6-argument `prepareCargoWorkspaceInvocation`; the package routing facade now re-exports `createRequestedIssuedResourceControlV1`), semio (4-argument `prepareJavascriptDependencies`, duplicate `packages` removed), stdio rust (`Bun.TOML.parse` workspace read).
- Third-party typings as proper declaration files in `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/`: `🔠️opentype-hinting` (module augmentation of `RenderOptions.hinting`), `🕸️graphlib` (full `Graph` and `alg` surface, `export =` namespace so default and named imports both work), `📊️d3-shape-stack-order` (array overloads of the six `stackOrder*` functions). Excluded the broken `🧫️fixtures/📁️some` tree from `tsconfig.json`.
- Generated parser vs schema: `💾️binary/…/🔖️raw/…/🧬️schema/🔣️.json` bytes became an integer array 0..255 and its parser was regenerated; `parseArtifactChild(value, at)`; forms `formsArtifactJson`, `formsBlockFieldJson`, `formsQuestionsDeltaJson`, `formsDiffJson` (native JSON binders for the current diff shape) and the forms document test uses them; note document test rewritten to the current `NoteDiff` rows (`assignedJson`, `patchJson`, `rowJson`, `assetRowJson`) and given `artifactReferenceSchema` in its dependencies.
- New owner function `parseGeneratedCatalogProjectionV1` in `catalog-view` (also used by `readGeneratedCatalogProjection`) so `hostProjection` in the dev catalog tests is decoded instead of being `any`; `StoryCatalogV1` now carries `rows: PluginCatalogRowsV1` and the WGPU story boots with the rows (the dev `useInventory` provides them).
- Owner types: `DependencyPolicyRule.to.dependencyTypesNot`, `rows` of the story catalog.
- Stale tests updated to the current API (examples): CAD brep sparse diff (also made the "reject foreign dialect" half meaningful, it used to throw on an unknown field), dashboard coverage oracle (`this.nx`, renamed from `build`), supersede replay (`Replayable`), dependency-policy, readme current-source revision (narrowed contract, mutable declaration copy), owner-projection-policy, build-graph admission, bootstrap parsers (`Object.is` on the issued invocation), nx-plugin arguments, mesh jobs (`ownerPairs`), brep reachability (`Set<Object3D>`), receiving sqlite (`authoredSql`), rust-source-direction fixtures (assertions on schema paths that the fixtures no longer declare removed), row-action-admission (`InputOutcomeV1`, valid refusal reason `mutation-rejected` instead of the nonexistent `row-conflict`), control-plane (`type: "lib" as const`, typed project data), media-transport lifecycle oracle (`mediaKind`, `selectionPair`), puzzle compose5d (`Dependencies` picked from the target module), GLTF base document (`documentOf`), CAD renderer story (`Parameters<typeof useInteractionRuntime>[1]`, `InstanceType<typeof Model>`), fs discovery (`DiscoveryNode`), node-graph scene (`Wire`), fem surface (`triple`), rewriting contract (`kindRows`), curation inverse (import of `CuratedItem` from its owner module), GIS mutation inputs (`validator.schemas`).

### 3.5 Work not done on purpose
No cast was added to hide a mismatch, no test was weakened except where an assertion named a fixture field that no longer exists (rust-source-direction schema paths). The note document oracle still fails at runtime for a different, older reason: its `mutationRoots` discovery expects `<subset>/🧫️fixtures/🧬️mutations` and the committed counts 66/33, but the fixtures now live in nested `🧪️tests/🧫️fixtures` folders (counts are 0/0). Its types are fixed.

### 3.6 Peer interaction
- `library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/{,📤️derivative,📤️derivative/⚠️failure}` appeared during `tt5-w6` (new split by another slice): 14 real + 14 TS2307. Not touched.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` was split while I worked (the diagnostics of `tt5-w4` for it pointed to lines of the old long file); its new parts have 2 diagnostics listed below.
- `tt5-w6` first attempt failed to load (`📇️directory/🧬️schema/📜️event/🟦️.ts` momentarily missing during a peer move) and succeeded on retry.

## 4. Residual work orders (79 real diagnostics in scope, by owner)

### A. Custody and storage-admission tests (48), owner: repository storage-admission redesign
Tests still call the pre-redesign shapes: `CargoArtifactBuildOptionsV1` now requires `consume`, `maximumObservationBytes`, `control: CompilerAdmissionControl`, `storage: NativeStorageReservationV1`, `sourceBoundary`; `CargoTestAdmissionV1` has no `command`; `stageRepositoryArtifacts(destination, owner, files)` takes a destination issued by `withRepositoryArtifactDestinationV1(options, run)`; `withNxSdkClonePublicationV1` takes a `control`; `acquire`/`lease` helpers take an extra argument; `requireNativeStoragePathV1` is no longer exported. Update each test to obtain the issued invocation, storage reservation and destination exactly like the production callers (`dev/🧩️catalog/🌐️browser-host/🏗️staging`, `hub/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication`) do.
- `process/📦️artifacts/🏗️native-build/🧪️tests` 7, `…/🎯️recipe/🧪️tests` 6, `…/🎟️admission` 1, `…/💾️storage/🍎️image/🧪️tests/📚️checkpoint` 1, `…/💾️storage/🧹️cleanup/🎛️command` 1, `…/🔒️lease` 1, `…/🧪️tests/📦️publication` 1
- `process/🧪️testing/🦀️cargo/🧪️tests` 6, `…/🔒️lease/🧪️tests` 3
- `process/🏗️build/🧪️tests/🏗️native` 4, `library/🏗️build/🧪️tests/🏗️native` 1
- `library/⚡️caching/🚀️bootstrap/🛠️tools/🧪️tests` 4, `library/⚡️caching/📦️artifacts/🧪️tests` 2, `library/🔍️discovery/🕸️runtime/🧪️tests` 2 (evidence shapes `RuntimeBuildResourceObservationV1`, `cargoStreamingStatus` options)
- `server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🧪️tests` 3 (project graph typing), `…/📥️inputs/🧪️tests` 1, `…/💾️storage/🧪️tests/🔌️nx` 1
- `dev/🧪️tests/🧪️ticket-owned-browser-host-staging/{🏗️profile-and-catalog-build,🚚️vendor-transport}` 2, `dev/🧩️catalog/🧪️tests/🎮️session` 1 (`stagePlaygroundSession` takes a fourth argument and the surrounding flow was redesigned)

### B. Generated parsers against stub schemas (18), owner: stdio artifact schemas and the parser generator
`schema parsers report` still refuses 1,295 positions, these are among them. Cause: the JSON Schema of several diffs is a placeholder stub while the hand-owned Rust diff (and the TS interface) is rich, so the generated parser and the TS type disagree.
- `gif/87a` and `gif/89a` diff (`bytes?: number[]` interface vs `bytes: string` schema; Rust `GifDiff` is a sparse width/height/gct/images diff) 2, `bcf/2.1` markup diff 1
- `semio/v1` mesh diff 3 (`NamedTripleDiff` fields parsed as plain objects), graph diff 2 (`SemioGraphPropertiesDiff` not generated), model text snapshot 1
- `pdf/1.7` base diff 2 (generic `T` left in generated interfaces), `ifc/2x3` binary snapshot 1, binary diff 1 and sqlite test 1, `xml/1.0` schema 1, `html/5` schema 1, `equation` diff 1, `wav/riff-pcm` mutations test 1 (`setPadBytes` missing from the test table)

### C. Missing packages, wasm outputs and follow-ons (13)
- `@xmldom/xmldom` not installed: SVG export test 7 (all downstream of the unresolved import), plus 7 TS2307 in the skipped class
- `rxjs` (async poll test 1), `tree-kill` (termination test 1), `d3-regression` (scatter-trend test 2 plus 1 TS7016), `yargs-parser`, `jsdom` (skipped class TS7016): add declaration files in `🌿️ambient/` or install the types
- CAD workspace packages `cad-js-module-*`, `brepjs`, `brepjs-opencascade` unresolved (skipped class)
- `trinity/jack/lsp` needs `JackLspSession` from the wasm `pkg` (not built), `flow/core` needs `semioSetRuntimeAssets` from `🕸️bindings/flow_core.js` (not built) 2

### Skipped by order
TS2307/TS7016 for workspace packages absent from `node_modules` and the whole `♻️mit-bestand` family (161 diagnostics, 139 TS7006).

## 5. Commands run

- `bun ./📜️script.ts verify typescript check [--files-from LIST] --out <file> [--heap-mb 8192] --max-diagnostics 100000` (b0, m1..m4, w1..w6, u1..u5, x1..x3)
- `bun ./📜️script.ts schema parsers report|apply` (restate of the binary raw schema; the report also produced the refusal census in B)
- `bun test ./<file>` per family and `tt5-tools/tt5-bun.sh`, `tt5-bun-head.sh`, `tt5_bun_cmp.py` (phases `base`, `after1`, `after2`, `ubase`, `uafter`, `r4base`, `r4after`)
- `node tt5-tools/tt5_parse.mjs` for the parse sweep
- Tools in `tt5-tools/`: `tt5_edit.py` (asserted unique anchors), `tt5_find.py`, `tt5_show.py`, `tt5_sites.py`, `tt5_matchers.py`, `tt5_blocks.py`, `tt5_list.py`, `tt5_run.py`, `tt5-buckets.py`, `tt5_type.mjs`, `apply_route.py`, `orig-m/` (originals of the matcher files)

## 6. Files changed (main areas)

Permanent route and its test (3.2); `.storybook/main.ts`, `.storybook/preview.tsx`; React build tooling `hostBuildPlugins`; pets and three vitest configs; scripts of graph, space-core, gltf, semio, stdio rust; ambient declarations `🔠️opentype-hinting`, `🕸️graphlib`, `📊️d3-shape-stack-order`; `tsconfig.json`; routing package facade; `catalog-view` (`parseGeneratedCatalogProjectionV1`); `StoryCatalogV1` and the dev `useInventory`; `DependencyPolicyRule`; `parseArtifactChild`; binary raw schema and parser; forms json module and tests; about 150 test files named in 3.3 and 3.4 (89 + 53 matcher files plus the round-four list in `🗑️generated/tt5-r4-files.txt`, matcher lists `tt5-m-files.txt`, `tt5-u-files.txt`); the 5 ticket scratch files of 3.1.

Remaining ticket artifacts: `🗑️generated/tt5-*` (logs, tsc outputs, bun result folders) are generated output and may be swept by the coordinator; `tt5-tools/` and this report stay.
