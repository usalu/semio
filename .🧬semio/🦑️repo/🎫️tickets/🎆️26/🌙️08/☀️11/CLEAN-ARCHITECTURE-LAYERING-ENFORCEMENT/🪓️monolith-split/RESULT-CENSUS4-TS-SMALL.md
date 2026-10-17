# Result CENSUS4-TS-SMALL (prefix `c4ts`): small TS product packages 52, 55, 56, 58

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `PR` = `🧰️framework/🛍️products`. Tools in `F/c4ts-tools/` (copied from c4p51: `gsplit.cjs`, `lossless.cjs`, `cycles.cjs`, `csssplit.py`, `cssequal.cjs`, `vtsum.py`, plus new `vitest.sh`, `tsc.sh`), raw outputs in `F/🗑️generated/c4ts-*`, pre-edit snapshots in `F/🗑️generated/c4ts-snap/` (`SHA.txt`).

## 0. Verdict

**DONE for all four packages.** 15 files split into 74 new files (24 + 20 + 15 + 15) in 74 new folders or section folders; every root is at most 487 lines and every new child at most 569 lines (limit about 1,000); longest new path 210 UTF-8 bytes. Behaviour is unchanged where it can be run, with failure sets identical to the baselines: presentation vitest 140/140 and 56/56 (sets equal), print inference harness 631/631 and 659/659 against d3 plus gallery export digests equal, repo bun suites 366 passed / 25 failed / 6 errors before and after (20 named failures identical) and the extension-host registration tree and outcomes equal, pets vitest 1,133/1,133 and 175/175 (sets equal). `tsc` is identical or clean everywhere (0 / 0 / 8 pre-existing / 40 pre-existing / 0).

| Pkg | Verdict | File | Before | Root after | New files |
|---|---|---|---:|---:|---:|
| 52 | DONE | `🎤️presentation` interaction module | 1,170 | 236 | 3 |
| 52 | DONE | `🎤️presentation` react stylesheet | 1,406 | 9 | 7 |
| 52 | DONE | presentation-core test | 1,128 | 127 | 5 |
| 52 | DONE | interaction-dom test | 1,443 | 140 | 5 |
| 52 | DONE | deck-rendering test | 1,330 | 227 | 4 |
| 55 | DONE | print pipeline verification | 1,005 | 49 | 6 |
| 55 | DONE | visualization gallery | 1,322 | 57 | 6 |
| 55 | DONE | inference mark | 1,149 | 75 | 5 |
| 55 | DONE | inference hierarchy | 1,019 | 229 | 3 |
| 56 | DONE | vscode extension test | 1,969 | 130 | 6 |
| 56 | DONE | acceptance orchestration | 1,198 | 275 | 5 |
| 56 | DONE | schema contracts | 1,116 | 487 | 4 |
| 58 | DONE | pets stories | 1,414 | 109 | 6 |
| 58 | DONE | feeling unit suite | 1,094 | 173 | 5 |
| 58 | DONE | climbing unit suite | 1,016 | 289 | 4 |

## 1. Re-measure

Package 52: interaction.tsx 1,170; react `🎨️.css` (actually at `PR/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css`) 1,406; presentation-core test 1,128; interaction-dom test 1,443; deck-rendering test 1,330 (identical to the census).
Package 55: pipeline test 1,005 (census 1,033); gallery 1,322; mark 1,149; hierarchy 1,019.
Package 56: 1,969 / 1,198 / 1,116 (identical). Package 58: 1,414 / 1,094 / 1,016 (identical).

## 2. Package 52 baselines

| Check | Command | Result |
|---|---|---|
| react vitest | `c4ts-tools/vitest.sh pres-react-base <pkg>/📦️packages/🟦️typescript/🎯️targets/⚛️react ../../../../🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (long level) | 7 files, 140 tests, 140 passed (`c4ts-vtset-pres-react-base.txt`) |
| core vitest | same with `📦️packages/🟦️typescript` and `../../🧪️tests/🎚️config/🟦️.ts` | 2 files, 56 tests, 56 passed |
| tsc | `c4ts-tools/tsc.sh pres-base <os tsconfig> PR/🎤️presentation` (49 tracked ts/tsx roots, their closure) | 0 diagnostics |

## 3. Package 52 (presentation): DONE

Tools added: `dsplit.cjs` (new test-file splitter: splits the body of one giant `describe` or whole top-level describes into sibling test folders, moves shared fixtures into an exported kit, copies `let`/hooks into every child, prunes imports by identifier use), `csssplit2.py` (region-aware contiguous CSS sections), `body.cjs`, `newcheck.py` (folder shape, docstring emoji, taxonomy prefix, sibling uniqueness, path length), `vitest.sh`, `tsc.sh`; specs in `c4ts-tools/specs/p52-*.json`. Snapshots `c4ts-snap/p52-f{1..5}.*` (`SHA.txt`).

| # | File (under `PR/🎤️presentation/`) | Before | Root after | Children (largest) |
|---|---|---:|---:|---|
| 1 | `🎯️targets/⚛️react/🔨️modules/🫳️interaction/🟦️.tsx` | 1,170 | 236 | 3 (`🧲️interactive-disposition` 569, `🎛️interaction-hook` 258, `🗂️disposition-registry` 71) |
| 2 | `📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css` | 1,406 | 9 (seven `@import`s and `@source "."`) | 7 under `🎨️styles/` (`🖱️interaction-selection-and-enlarge` 315) |
| 3 | `🧪️tests/📽️presentation-core/🟦️.ts` | 1,128 | 127 | 5 sibling test folders (`🪡️thought-expansion` 336, kit `🧷️presentation-core-kit` 43) |
| 4 | `🧪️tests/🖱️interaction-dom/🟦️.tsx` | 1,443 | 140 | 5 (`📄️pdf-and-figure-interaction` 377, kit `🧱️interaction-dom-kit` 169) |
| 5 | `🧪️tests/🖼️deck-rendering/🟦️.tsx` | 1,330 | 227 | 4 (`🔍️figure-scroll-and-zoom-rendering` 355) |

Details:
- **Interaction module.** `usePresentationInteraction` (`🎛️interaction-hook`), the registry with its context, provider and `resolveEffectiveDispositionRect` (`🗂️disposition-registry`) and `InteractiveRowBand` plus the 530-line `InteractiveDisposition` component (`🧲️interactive-disposition`) moved out; the root keeps the pointer-target helpers, background interaction, the slide reset and the interaction layer. `//#region 🔖️Interaction` is repeated in every child that carried statements of it. Two importers re-pointed by `gsplit.cjs`: the react package barrel (`export { usePresentationInteraction }`) and `🧭️arrangement-section`. The root docstring no longer claims the barrel re-exports everything.
- **CSS.** Seven contiguous sections, each starting with a one-line emoji comment; the `Interaction` region (134 to 693) crosses two children and carries its markers in both; `RevealTypography` and `RevealChrome` stay whole in one child each. The root keeps the `@source "."` directive, moved behind the `@import`s because an `@import` after another rule is invalid CSS. Readers re-pointed: `🧰️presentation-deck-helpers` `globalsCssSource` now inlines the relative `@import`s (`readStylesheetTree`), and the five tests that read the stylesheet themselves (`✂️cropped-figure-and-sheet-rules` 2, `🔍️figure-scroll-and-zoom-rendering` 2, `📐️interaction-geometry` 1) use `globalsCssSource` instead of `readFileSync` of the root.
- **Test files.** The two giant `describe` blocks keep their title in every child, so every test keeps its full title. The shared `container`, `beforeEach`, `afterEach` are copied into each child describe; fixtures and pointer helpers are exported from the kit. Registered in the two vitest configs (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` 8 new includes, `🧪️tests/🎚️config/🟦️.ts` 4 new includes).

Verification actually run:

| Check | Result |
|---|---|
| react vitest (`c4ts-tools/vitest.sh pres-react-final`, long level) | 15 files, 140 tests, 140 passed; per-test (status, full title) set equal to the baseline (7 files, 140) |
| core vitest (`pres-core-final`) | 6 files, 56 tests, 56 passed; set equal to the baseline (2 files, 56) |
| tsc (`tsc.sh pres-final`, 66 roots incl. the 17 new files) | 0 diagnostics before and after |
| Lossless (`lossless.cjs`, `c4ts-lossless-p52.log`) | 0 lines lost except the rewritten docstrings (f1, f4, f5) and in f5 the deliberate reader re-point (readFileSync lines replaced by `globalsCssSource`); added only child docstrings, repeated region markers and the repeated `describe`/`container`/hook shell per child |
| CSS | `cssequal.cjs`: lightningcss bundle of the new root equals the original minified (32,693 bytes both, apart from the position of the single `@source "."`); `csssplit2.py` asserted the ordered non-blank, non-marker line sequence equal |
| Taxonomy | `bun F/tf8-tools/taxsim.ts <taxonomy> <24 new dirs>`: `validateTaxonomy` 0 problems, all resolve (tests: `members-of-tests`; module children: `crate-module-children`); no taxonomy edit needed |
| `newcheck.py` over the 24 new files | 0 problems: folder shape `<emoji>U+FE0F<kebab>`, docstring starts with the folder emoji, every emoji is a taxonomy prefix, unique among siblings, longest path 210 bytes, no `[DEBUG]` |
| Import cycles (`cycles.cjs` over the interaction module) | 0 SCCs above size 1 |

Not run: the Storybook render and a Tailwind pass over the split stylesheet (the lightningcss bundle equality stands in); the nx `presentation-react:test` route (the vitest configs it uses were run directly); no new launch route is needed because the tests run through the existing `bun ./📜️script.ts test` routes whose vitest configs now list the new files.

## 4. Package 55 (print): DONE

Baselines (before the first edit of this package): the repo launcher (`bun repo run @semio-tech/print:test*`) refuses in this environment with `Original bounded retirement catalog layout required` (a lease catalog defect owned by a peer), and the `📜️script.ts` routes need the process envelope, so the nx test routes could not be used. The runnable evidence is the following.

| Check | Command | Baseline |
|---|---|---|
| mark and hierarchy against d3 | `c4ts-tools/infrun.sh <label> quick|long` (imports `verifyVizChartInferences` of `@semio-tech/print-viz-inference` through `bun -e`; every check of the inference harness against its d3 oracle) | quick 631 checks passed (mark 45, hierarchy 17); long 659 checks passed (mark 58, hierarchy 17) |
| gallery | `c4ts-tools/exportdigest.ts` over the old and the new modules: every export is imported, zero-arity (and chosen optional-argument) functions are called and their JSON hashed (`NOARG`, `CATALOGARG`), values hashed | 29 exports, 14 called, 1 threw (the throwing call is the same before and after) |
| pipeline | `c4ts-tools/pipesmoke.ts`: the old snapshot and the new tree run the context-free verifications `verifyPrintToolchainManifest`, `verifyPrintApiFreshness`, `verifyPrintMacroStaging`, `verifyPrintDocumentCatalog` | all four PASS before and after (they read the fixtures through `import.meta.dir`, so they prove the rebased paths) |
| source-text readers | `bun test` of `📦️artifacts/📖️pdf/🧪️tests` and `📦️artifacts/📥️material/🧪️tests` | after the split with the old readers: pdf 9 pass 1 fail (`actual PDF worker is data-only…` expects `rasterPrintPdfV1` in the pipeline root); material 29 pass 14 fail |
| tsc | `c4ts-tools/tsc.sh print-base` (253 tracked ts roots of `PR/📓️print`, os tsconfig as base) | 8 pre-existing diagnostics in tests (`suncalc` and `d3-regression` declarations, `Series` types), none in the split files |

| # | File (under `PR/📓️print/`) | Before | Root after | Children (largest) |
|---|---|---:|---:|---|
| 1 | `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` | 1,005 | 49 | 6 (`🖼️gallery-carrier-and-kind-paint` 219, `🧾️api-freshness-and-titles` 203, `🛡️command-and-publication-verification` 196) |
| 2 | `🔨️modules/📊️visualization-gallery/🟦️.ts` | 1,322 | 57 | 6 (`🔑️implemented-keys` 319, `🧾️api-reference` 315, `📋️printed-key-tables` 152) |
| 3 | `🧬️schema/💡️inferences/✒️mark/🟦️.ts` | 1,149 | 75 | 5 (`🎢️curves-cardinal-and-catmull-rom` 357, `💠️curve-registry-and-symbols` 227) |
| 4 | `🧬️schema/💡️inferences/🌳hierarchy/🟦️.ts` | 1,019 | 229 | 3 (`🧱️treemap-tilings-and-partition` 302, `🫧️circle-packing` 260, `🌲️tidy-tree-and-cluster` 234) |

Design and repairs:
- **Dependency layering.** Children import their parent where the parent holds the shared core (hierarchy: node class, seeded stream; mark: path recorder, curve contract; gallery: product paths and loaders; pipeline: scope context, schema module reader, PDF open and glyph helpers), and the parents import no child, so there is no cycle (`cycles.cjs`: 0 SCCs above size 1 in all four trees). Splitting the first draft exposed three cycles that were broken by moving the shared helper to the lower layer (`lastApiTextArguments` into the printed key tables; `printSchemaModule`, `openPrintPdf`, `assertPrintPdfGlyphs` into the pipeline root; the gallery generation into its own child).
- **Mark.** The file-private `EPSILON`, `CurveState` and `noop` were not exported: `EPSILON` and `CurveState` moved into the only child that uses them inside a repeated `//#region 🔖️CurveContract`, and `noop` became the exported `vizCurveNoop` of the curve contract. The package index `📦️packages/🟦️typescript/🟦️.ts` gets one `export *` per new child next to the old line, because its `export * from mark` and `from hierarchy` lines named the whole module; the explicit entry `✒️mark` still re-exports `VizPathCommand` (it stayed in the mark root). Importers re-pointed by `gsplit.cjs`: `🌊flow`, `🥧shape`, `🖼️render`, `🧮transform-and-shape` probes, `🧮transform`, `🧭coordinate-and-hierarchy` probes.
- **Gallery.** The file carried an unmatched `//#endregion 🔖️Sources` in HEAD (no start marker); the start marker was added before the split so every region balances. Five importers re-pointed (`🌐viz-catalog-generation` command, the pipeline command and its tests, `📜️script.ts`, `📚️catalog-coverage`). The `README.md` path of `vizApiReference()` now names `🧾️api-reference`.
- **Pipeline.** The children sit one folder deeper than the old test file, so two kinds of runtime literals had to move with them (they are not import declarations and `gsplit.cjs` leaves them): the 18 `join(import.meta.dir, "../🧫️fixtures/...")` reads became `"../../🧫️fixtures/..."` and the single dynamic `import("../../../../📦️artifacts/📥️material/🟦️.ts")` gained one `../`. A script resolved all 18 fixture paths against the new locations: 0 missing. The command `🧪️print-pipeline-verification/🟦️.ts` imports each function from its owning child. The two source-text readers (`📖️pdf/🧪️tests` and `📥️material/🧪️tests`) now read the pipeline root plus every child `🟦️.ts` (a child-specific read; `readModuleTreeSource` cannot be used because the root imports no child): pdf test 10 pass, 0 fail (before the reader fix 9 pass 1 fail); material 30 pass 13 fail (before 29 pass 14 fail, the one that flipped is the same pdf-worker test, which lives in the material run too). The 13 remaining material failures are the launcher-envelope failures (`Original parent Script process invocation required`) that exist without my change.

Verification actually run (after the last edit):

| Check | Result |
|---|---|
| inference harness | quick 631/631 passed (mark 45, hierarchy 17), long 659/659 passed (mark 58, hierarchy 17); equal to the baseline |
| gallery digest | the new modules (root plus 6 children, 54 exports) reproduce all 29 original exports with identical hashes and the same one throwing call; the 25 additional exports are the declarations that crossed a file boundary |
| pipeline smoke | the four context-free verifications PASS before and after |
| tsc (`tsc.sh print-final`, 273 roots) | exit 2 with the same 8 diagnostics as the baseline (diff of normalised diagnostics empty); a first run showed 3 extra diagnostics from the unrebased dynamic import, fixed |
| Lossless (`lossless.cjs`, `c4ts-lossless-p55.log`) | f2 gallery lost 0 lines; f1 pipeline lost only the 18 rebased fixture reads and the one dynamic import (re-added with the new relative path); f3 mark lost only the old docstring, `noop` and its 10 uses (renamed `vizCurveNoop`); f4 hierarchy lost only the old docstring. Region markers: none lost, repeated where a region crosses children |
| Taxonomy | `taxsim.ts` over the 20 new dirs: `validateTaxonomy` 0 problems, all 20 resolve; no taxonomy edit |
| `newcheck.py` over the 20 new files | 0 problems, longest path 208 bytes |
| `catalog-coverage` scenarios (`bun -e` over the exported `scenario*` functions) | 7 pass, 2 fail (`scenarioGenerated`: stale generated `🖼️assets/🔣️viz-api.json`; `scenarioImplementedKeysDocumented`: geo-flow keys). Not caused by the split: the producing functions (`vizGeneratedFiles`, `vizImplementedFamilyKeys`, `vizApiReference`, `vizCoverageReport`) have identical digests before and after, so the generated file and the LaTeX keys disagree on the current tree |

Not run: the nx routes (`print:test`, `test-viz`, `test-macro-staging`) because the launcher refuses; the pipeline verifications that need an issued `withPrintPipelineVerificationV1` context (quick pipeline, API title rendering, gallery carrier, kind paint, PDFs, command boundaries, API publication) are covered by tsc, the lossless diff, the fixture path check and the two source-text readers. No launch route is needed (no new test entry point; the inference harness and the pipeline command already have routes).

## 5. Package 56 (repo): DONE

`gsplit.cjs` gained two options for this package: `sharedLines` (statements copied into every file, never an edge: the extension-host mocha contract) and `sideEffectImports` (the parent imports each child for its suites). New tools: `buntest.sh` (bun test with junit and a caller-owned artifact dir), `junitset.py`, `exthost.ts` (the extension-host oracle below), `starts.cjs`. Snapshots `c4ts-snap/p56-f{1,2,3}.ts` and `p56-f1.pre-split.ts`.

Baselines (before the first edit of this package):

| Check | Command | Result |
|---|---|---|
| goal gate laws (acceptance orchestration) | `buntest.sh acc-goalgate-base <T0>/🎯️acceptance ./🧪️tests/🎯️goal-gate/🟦️.ts` | 28 passed |
| source census laws | same for `🧮️source-census` | 35 passed |
| schema contracts | `buntest.sh sc-invariants-base <T0> ./🧪️tests/🧬️schema-invariants/🟦️.ts ./🧪️tests/🧭️fixture-resolution/🟦️.ts` | 137 tests: 136 passed, 1 failed (`🩺️ the committed tree is measured by the same checkers > the test platform's own subtree carries no schema-contract finding`, with 10 findings: dialect, owner-ineligible, catalog-stale), pre-existing |
| whole `🧪️test/🧪️tests` suite | `buntest.sh test-all-base <T0> <20 entry files>` | 391 tests, 366 passed, 25 failed, 6 errors (301 s) |
| extension-host suites (cannot run without a VS Code binary) | `exthost.ts`: vite-bundles the entry with the builder's own `extensionBuildConfig`, loads the bundle with collector mocha globals and a stub `vscode`, runs every test with a 2 s guard (`orig-prototype`, `orig-shake` = release tree-shaking without minification) | 198 tests in 23 suites, 1 `suiteSetup`; 163 PASS, 35 FAIL (the ones that need the real `vscode`), both modes |
| tsc, vscode package | `tsc -p` with the package tsconfig over the old test entry | 40 diagnostics, all in shared framework modules, none in the test |
| tsc, repo test module | `tsc.sh repo-final` (os-free base `PR/🦑️repo/tsconfig.json`, 113 roots) | 0 diagnostics |

| # | File (under `PR/🦑️repo/🔨️modules/`) | Before | Root after | Children (largest) |
|---|---|---:|---:|---|
| 1 | `💻️client/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts` | 1,969 | 130 | 6 sibling test folders (`🌊️view-and-provider-suites` 493, `📮️uri-and-event-suites` 451, `🏷️entity-and-codelens-suites` 341, kit `🧰️extension-test-kit` 260) |
| 2 | `🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts` | 1,198 | 275 | 5 (`🚦️goal-gate` 430, `📝️docstring-and-debug-census` 188, `🪢️artifact-neutrality` 142) |
| 3 | `🧪️test/📐️schema-contracts/🟦️.ts` | 1,116 | 487 | 4 (`📤️export-completeness` 222, `🩺️placement-diagnostics` 182, `🔗️reference-diagnostics` 151, `🏗️scope-eligibility-and-tree` 108) |

Design and repairs:
- **Extension test.** The entry stays the vite bundle input; the root keeps the activation `suiteSetup`, two suites and one side-effect `import` per suite child, so every suite still registers at bundle load. The mocha contract (`type`s and `declare function suite/test/suiteSetup`) is ambient, so it is copied into the root and every child instead of being exported. Two region starts in the original were corrupted (`// 🔒️#region 🎞️Constants`, `// 🌱️#region 🎼️Utilities`, the only two in the repo) and could not match their `#endregion`; they were restored to `// #region` before the split. The tooling that reads the entry (`🏗️builder/👷️worker`, the package tsconfig and script) names only the root, which is unchanged.
- **Orchestration.** The census children need nothing from the root, `🪢️artifact-neutrality` takes `TEST_SEGMENT` from `🧮️placeholder-and-job-census`, and the goal gate takes the vocabulary from the root; no cycle. Six dynamic `await import(...)` destructurings in `📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts` (not handled by the importer rewriter) were split by hand into one import per owning file; the named export list in `📦️packages/🟦️typescript/🎯️acceptance/📋️orchestration/🟦️.ts`, the two tests and `🧪️test/📜️script.ts` were re-pointed by the tool. Two source-text laws had named the old file: `DEBUG_TAG_CENSUS_EXEMPT` and `ARTIFACT_NAME_CENSUS_EXEMPT` exempt the file that carries the literal `[DEBUG]` and the framework artifact names, which is now `📝️docstring-and-debug-census` and `🪢️artifact-neutrality` (the censuses report 0 hits in the orchestration tree), and the build-surfaces fixture `environmentSources` (a negative law on `NX_` variables) lists the new `🚦️goal-gate` next to the root, since the process-spawning code moved there (`📚️library` build-surfaces: 17 passed).
- **Schema contracts.** The root keeps the contract vocabulary, the taxonomy and catalog readers, the export resolution, the owner levels and the submodule cache (487 lines); four children hold scope eligibility, the placement diagnostics, the export completeness and the reference diagnostics. Three cycles that showed up in the first draft were broken by keeping `schemaScopeOwnerLevels`, `matchesTaxonomyPathPattern`, `isFixtureOwnedPath`, `submodulePaths` and `schemaScopeFile`/`schemaExportFile` in the root. The package index re-exports from the owning children (four new `export { ... } from` lines replace one), and `✅️schema-validation` and `🌿️mutation-leaves` were re-pointed.

Verification actually run (after the last edit):

| Check | Result |
|---|---|
| extension-host oracle | `exthost.ts split-prototype` and `split-shake`: 198 tests, 1 hook, the flattened suite and test titles equal the original's, outcomes equal (163 PASS, 35 FAIL, same messages) in both modes |
| goal gate / source census laws | 28 passed / 35 passed (equal to the baseline; the census run failed once on a peer's missing framework module and passed on the retry) |
| schema contracts | 137 tests, 136 passed, 1 failed: the same pre-existing test with the identical 10 findings (diff of the finding lists empty) |
| whole `🧪️test/🧪️tests` suite | 391 tests, 366 passed, 25 failed, 6 errors; the 20 named failures are identical to the baseline's |
| `📚️library` build-surfaces law | 17 passed |
| tsc | vscode package: 40 diagnostics before and after, identical after dropping line numbers; repo test module: 0 diagnostics; acceptance tree plus the gates file: 0 diagnostics |
| Lossless (`c4ts-lossless-p56.log`) | f1 0 lost (added: the 5 contract lines times 6 files, region repeats, docstrings); f2 lost only the two exemption literals (re-pointed); f3 lost only the old docstring line |
| Taxonomy / `newcheck.py` | 15 new dirs resolve, `validateTaxonomy` 0 problems; the checker flags one `[DEBUG]` text in `📝️docstring-and-debug-census`, which is the census rule itself (the original file held it and was exempt), not a leftover log; longest path 183 bytes; `cycles.cjs`: 0 SCCs in all three trees |

Not run: the extension host itself (`vscode-test`) and the nx `repo-vscode` routes; the 35 stubbed-out tests that need the real `vscode` API are covered only by the registration-tree and outcome oracle. No launch route is added because the bundle entry and the bun test files keep their commands.

## 6. Package 58 (pets): DONE

Baselines (before the first edit of this package): pets core vitest `c4ts-tools/vitest.sh pets-base <PP>/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts` (long level) 26 files, 1,133 tests, 1,133 passed; pets-react vitest (`🎯️targets/⚛️react/📦️packages/🟦️typescript`, same config route) 9 files, 175 passed; `tsc.sh pets-base` (react tsconfig as base, 122 roots of the product) 0 diagnostics.

| # | File (under `PR/🐾️pets/`) | Before | Root after | Children (largest) |
|---|---|---:|---:|---|
| 1 | `🎯️targets/⚛️react/📖️stories/🟦️.tsx` | 1,414 | 109 | 6 (`🎬️studio` 309, `🔤️language-and-menagerie` 298, `🏟️sandbox` 262, `🐾️species-gallery` 231, `🎛️director-and-inspector` 196, `🎚️controls` 51) |
| 2 | `🔨️modules/💗️feeling/🧪️tests/🔬️unit/🟦️.ts` | 1,094 | 173 | 5 sibling topic folders (`⚗️encounters-and-chemistry` 233, `🎭️appraisal-wishes-and-contagion` 197, `🪜️species-states-and-tricks` 178, `⏳️mood-over-time-and-face` 158, kit `🧰️feeling-test-kit` 114) |
| 3 | `🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🟦️.ts` | 1,016 | 289 | 4 (`🪝️rope-and-shot-suites` 314, `🪜️ladder-suites` 249, `🗺️route-and-determinism-suites` 120, kit `🧰️climbing-test-kit` 71) |

Design:
- **Stories.** The page already was cut into regions; each region (except `Page`) became a child: language labels and menagerie source, the studio (specimen drawing), the small controls, the species gallery, the director and inspector, the sandbox. The root keeps `Stories` and `mountStories`, so `🌐️.html` (`import { mountStories } from "./🟦️.tsx"`) and the vite builder (it names only `📖️stories/` and the html) are untouched; the root keeps the stylesheet import. The first draft named the language folder `🌐️language-and-menagerie`, which clashes with the sibling document `🌐️.html` (same emoji among siblings); it was renamed `🔤️language-and-menagerie` before verification. The dependency graph is acyclic (`cycles.cjs`: 0 SCCs).
- **Unit suites.** Following the pets README (topic folders `🔨️modules/<module>/🧪️tests/<topic>/`, listed in `🧪️tests/🎚️config/🟦️.ts`), the helpers of each suite became an exported kit (`HERE` stays valid because the kit sits at the depth of the old file) and the `describe` blocks moved to topic folders under their module; the describe titles are unchanged, so every test keeps its full name. The 8 new entry files are listed in the include of `🧪️tests/🎚️config/🟦️.ts`. The Rust twins (`🔬️unit/🦀️.rs`) of both modules are not part of this slice and stay where they are (hand-off: the README asks for both languages next to each other in a topic folder).

Verification actually run (after the last edit):

| Check | Result |
|---|---|
| pets core vitest (`pets-final`) | 33 files, 1,133 tests, 1,133 passed; per-test (status, full title) set equal to the baseline (26 files) |
| pets-react vitest (`petsreact-final`) | 9 files, 175 tests passed; set equal |
| tsc (`pets-final`, 137 roots, react tsconfig) | 0 diagnostics before and after |
| Stories runtime (`c4ts-tools/storiessmoke.ts`: jsdom, stubbed css, chemistry-rules menagerie, 30 frames of 20 ms) | the old snapshot and the new tree mount to byte-identical normalised markup (21,991 characters, 337 elements, title `Pet stories`), also after the folder rename; two runs of the new tree are identical |
| Lossless (`c4ts-lossless-p58.log`) | f1 0 lost; f2 and f3 lost only the root docstrings (rewritten); regions: stories 16 to 16 |
| Taxonomy / `newcheck.py` | 15 new dirs resolve (`validateTaxonomy` 0 problems), 0 problems, longest path 164 bytes; `cycles.cjs` 0 SCCs |

Not run: a browser render of the gallery and the Storybook route (the jsdom mount covers the component tree, not the canvas painting, because jsdom has no 2D canvas); the Rust twins.

## 7. Files touched

Created: the 74 files listed in `F/🗑️generated/c4ts-new-files-p52.txt`, `-p55.txt`, `-p56.txt` (and the `-p56-ext.txt` subset), `-p58.txt`. Rewritten roots: the 15 files in section 0. Edited besides the roots: presentation (`🎯️targets/⚛️react/🔨️modules/🧭️arrangement-section`, the react barrel `🟦️.tsx`, the two vitest configs, `🧪️tests/🧰️presentation-deck-helpers`, `📐️interaction-geometry`); print (the inference package index, `🌊flow`, `🥧shape`, `🖼️render`, `🧮transform`, two probes, the pipeline command, `📜️script.ts`, `📚️catalog-coverage`, the `📖️pdf` and `📥️material` tests, `README.md`); repo (`📚️library/🎛️commands/✅️verify/🚦️gates`, the test package index and `🎯️acceptance` entry, `✅️schema-validation`, `🌿️mutation-leaves`, `🧪️test/📜️script.ts`, the `🔧️build-surfaces` fixture); pets (`🧪️tests/🎚️config/🟦️.ts`). Snapshots and logs stay in `F/🗑️generated/c4ts-*`, tools in `F/c4ts-tools/`.

## 8. Hand-offs and pre-existing defects seen

- `repo` launcher refuses (`Original bounded retirement catalog layout required`), so no nx or `bun repo run` route was usable in this session; every command above is a direct vitest, bun test, tsc or bun script run.
- Print: `catalog-coverage` scenarios `scenarioGenerated` (stale `🖼️assets/🔣️viz-api.json`) and `scenarioImplementedKeysDocumented` (geo-flow keys) fail on the current tree independent of the split; 13 `📥️material` bun tests fail on the missing script envelope; 8 tsc diagnostics in print tests (`suncalc`, `d3-regression`, `Series`).
- Repo: `schema-invariants` `the test platform's own subtree carries no schema-contract finding` fails with 10 findings (dialect, owner-ineligible, catalog-stale), the same list before and after.
- Pets: the Rust unit twins stay in `🔬️unit/🦀️.rs`.
- Fixed in passing (own files only): the unmatched `//#endregion 🔖️Sources` of the gallery, the two corrupted `#region` starts of the extension test.
