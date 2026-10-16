# Result: SDK Boundary 2 (slice SDK-BOUNDARY-2, prefix `sdk2`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Run of 2026-10-16. Tools: `sdk2-tools/` in this folder (it reuses `sdkb-tools/lib.ts`, `plan.ts`, `apply-surface.ts`, `scan2.ts`, `runtests.ts`, `testdiff.ts`, `facade-load.ts`, `ruletable.ts`, `origin-delta.ts`). Logs and raw outputs: `🗑️generated/sdk2-*` (kept).

Path aliases: `F` = `🧰️framework`, `OS` = `F/🛍️products/💻️os`, `SDK` = `F/📦️packages/🟦️typescript`, `KIT` = `F/🔨️modules/⏪️time-travel/🧪️testing`.

## 0. Verdict

Done: L, B4, B7 (story and e2e spec), the source-text reads, and 14 of the 26 B6 edges. Handed off with a concrete design: the remaining 12 B6 edges (an os document-service ABI cluster), the B7 animate unit test, and the product edges inside L (B1/B2 content), plus B1/B5, B2, B3 as instructed.

| Target | Edges before | Edges after | Status |
|---|---:|---:|---|
| L layout artifacts | 23 | 7 | all 16 neutral imports on SDK entries (plus 4 relative-to-facade imports now package specifiers); the 7 left are product imports (4 repo script imports = B1, 3 store `child`/`link` imports = B2) |
| B4 os dev time-travel kit | 80 | **0** | kit moved to a neutral framework module, 15 SDK entries, puzzle scenario re-pointed, run command made data-driven (section 3) |
| B6 os internals | 26 | 12 | 14 fixed (section 4); 12 are one cluster, see hand-offs |
| B7 presentation product | 3 | 1 | story and its e2e spec relocated to the presentation product; the animate unit test import stays (section 5) |
| Source-text reads (`new URL`/`readFileSync`/`Bun.file`/`resolve` of `🧰️framework` paths in plugin tests) | 96 literal sites in 47 files | **0** | `@semio-tech/framework/test/source-text` (section 6) |

Scan numbers (AST scan, `sdkb-tools/scan2.ts`, 7,060 then 7,051 plugin files): violating import statements 799 to 685 (L 23 to 7, B4 80 to 0, B6 26 to 12, B7 3 to 1, B1/B5 519 to 517 because the gis bridge test left the plugin, B2 82 and B3 66 untouched).

Exact cruiser command (audit 2 section 1; `sdk2-tools/cruise.sh`, roots `🧰️framework ✏️s 🌎️hub ♻️mit-bestand`, same config as `RESULT-SDK-BOUNDARY.md` section 7):

| Rule | Before (06:57) | After (final, 11:17) | Attribution |
|---|---:|---:|---|
| `plugins-framework-sdk-only` | 725 (569 outside tests) | **619** (550 outside tests) | 106 edges removed, 0 added (ASCII-skeleton multiset diff, `sdk2-tools/attrib.py`) |
| `no-cross-technology-✏️s-to-🧰️framework` | 471 (180) | **372** (161) | 105 removed; 6 re-appear under a new origin: the relocated gis bridge test and its config now sit in `✏️s/🧑‍💻dev/…` and keep the same 6 relative framework imports they had as plugin files (net minus 99) |
| `framework-no-products` | 147 | **147** | unchanged, identical set |
| `not-to-unlisted` | 855 | 855 | 2 edges moved with the scalar-witness encoder (plugin-less, same two unlisted `@webassemblyjs` packages) |
| `no-circular` | 715 | 715 | 18 cycle lines moved with the kit modules (same cycles, new paths) |
| all rules | 13,477 | 13,206 | the rest is peer churn |

The failure sets of the three rules are identical or smaller. Origin split (`origin-delta`): `no-cross-package-relative-*` origin plugins 2,084 to 1,982 (minus 102), origin SDK package 133 to 148 (plus 15, the structural P11 facade family of `RESULT-SDK-BOUNDARY.md` section 5), origin framework plus 33 (the stay-behind os dev runner files import the kit by relative path, section 3.4).

## 1. Found state, baselines

No cut predecessor. Baseline recorded before the first edit: cruiser (`sdk2-before-err.txt`), `tsc` (`sdk2-tsc-before.txt`), tests (`sdk2-tests-before.json`, 47 files: the 17 files of the 68-file set of `RESULT-SDK-BOUNDARY.md` section 7 that belong to the touched plugins puzzle, gis, flow, draw, stdio, trinity, animate and layout, plus every test file that I edit).

- `tsc -p` is blind here: its 905 diagnostics before and after are all syntax errors (TS1005/1128/1434/1443/1351) in three foreign files (`puzzle/🧊️3d` and `puzzle/🖐️5d` `🧬️schema/🔺️diff/🟦️.ts`, `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🪪️document/🟦️.ts`). With a syntax error in the closure `tsc` never reaches the semantic phase. I therefore ran the compiler API (`sdk2-tools/tsc-sem.ts`, syntax errors do not suppress semantics) and report that (section 8.2).
- The baseline test set fails heavily before any edit (470 pass, 158 fail).

## 2. L: layout artifacts

`sdk2-tools/rewrite.ts` (the `sdkb` codemod without the layout skip). 9 files, 20 edits, 0 new entries: 16 neutral-module imports (`compiler/syntax/rust`, `schema`, `value/dictionary` (+`/native-json`), `io/sqlite-snapshot` (+`/artifact`, `/ieee754`), `value/schema/intrinsic`, `schema/record`) and 4 relative imports of existing SDK facade files, all now `@semio-tech/framework/<subpath>`. `sdk2-tools/verify-snap.ts` against the pre-edit snapshot: 20 changed lines, all import lines. The 7 remaining edges: `📦️packages/🦀️rust/📜️script.ts` (4: repo library runners, B1) and `🧬️schema/🟦️.ts` ×2, `🧬️schema/🔺️diff/🟦️.ts` ×1 (store `child`/`link` schemas, B2).

## 3. B4: the time-travel test kit

### 3.1 Why not an os package

The rule `plugins-framework-sdk-only` flags every plugin edge into `🧰️framework/` (also to workspace packages under it) that does not resolve to an authored `exports` target of `@semio-tech/framework`. An "os test-support package with explicit exports" would therefore still be flagged. The only route to zero is an SDK entry, and the SDK role `framework-neutral` forbids re-exporting `🛍️products/**`. So the neutral half of the kit had to leave the os product.

### 3.2 What moved

16 kit modules, pure Playwright helpers that import only each other, `mv` (no content change except two relative `@see` docstring links): `🧠️state 🧩️scenario 🧾️report 🌍️copy 🎧️console 🧱️chrome 🧊️wgpu-mirror 🎗️band 📄️page 🧭️arguments 📜️history 🖊️editor 🧵️history-session ♿️aria-oracle 🔁️replay-arm 🔄️reprojection`.
From `OS/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/<m>/🟦️.ts` to `F/🔨️modules/⏪️time-travel/🧪️testing/<m>/🟦️.ts` (`KIT`). `🧪️testing` is the repo's testing-support owner (`**/🧪️testing/**`) and, unlike `🧪️tests`, is not collected by test discovery (these files hold no tests). Path length: the longest path in the touched directories is 143 bytes (a pre-existing fixture), limit 240.

Stay in os dev (they import product code): the CLI root `🟦️.ts` (acceptance records, dev serve), `🎪️run` (parity, plugin build plan), `🌐️universal`, `📟️console-digest`, `🚀️boot`, `🪞️inventory`. Their 35 kit imports are relative paths to `KIT`.

### 3.3 SDK entries (15)

Created with `plan.ts`/`apply-surface.ts` from the scenario's real usage: `./time-travel/testing/{state,scenario,report,copy,console,chrome,wgpu-mirror,band,page,history,editor,history-session,aria-oracle,replay-arm,reprojection}`, 136 explicitly named re-exports, no `export *`. The 9 puzzle scenario files (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/**`, 79 imports) now import those specifiers (`verify-snap`: 79 changed lines, all imports). Live bindings (`export let` state) survive the facade; bun resolves the facade and the direct relative path to one module instance.

### 3.4 The run command (the 80th edge)

`puzzle/📦️packages/🟦️typescript/📜️script.ts` dynamically imported `runTimeTravelCli` from the os dev root. Inversion by data instead of an import:
- `OS/…/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts`: `runTimeTravelCli(invocation, repoRoot, defaultOutDir, segments)` loads `--scenario <repository-relative module>` itself (`loadScenario`: stays inside the repository, must export `timeTravelScenarioV1`); without `--scenario` and without `--universal` it throws a message naming `--scenario`.
- Puzzle: export renamed `puzzle2dTimeTravelScenario` to `timeTravelScenarioV1` (docstring command updated); `TimeTravelScript`, the `time-travel` target in `📋️project.json` and the `time-travel` script in `package.json` are removed.
- Launch entries `⚖️gate⏪️time-travel⚛️react` and `⚖️gate⏪️time-travel🧊️wgpu` (both JSONs) now run `bun repo run @semio-tech/framework-os-dev:time-travel -- --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts --serve … --renderer …`.
- The stay-behind os dev files import the kit by relative path, not through SDK specifiers: I tried the specifiers first and the compiler showed that the runner needs about 60 internal names (setters, lists, state) that no plugin uses. The SDK exposes only what plugins consume, so the relative path stays (`no-cross-package-relative` framework origin plus 33, hygiene P11).
- `OS/…/🧑‍💻dev/🔮️oracles/🔣️.json`: the two rationale strings now name `⏪️time-travel/🧪️testing/♿️aria-oracle`.

Not run: a real headless run against a puzzle dev serve (none listening). `RESULT-MISPLACED-E2E-TESTS.md` ran the kit headless through its smoke harness; I did not repeat it. What I proved: every moved module and facade loads, and the type-check is clean on the kit, the scenario and the runner (section 8.2).

## 4. B6: os internals, case by case

| # | File(s) | Decision | Edges |
|---|---|---|---:|
| 1 | stdio `📸️jpg/…/🫴️receiving/🧪️tests/🟦️.ts` pulled two os test files in by side-effect `import "…"` | removed both lines. Both os tests have their own routes (`os/📦️packages/🦀️rust` `test-config-paged-source`, `plugin/📦️packages/🦀️rust` `OriginalLifecycleSource`). Effect: 12 pass / 2 fail becomes 5 pass / 0 fail; the 2 failures and 7 passes were the os tests | 2 |
| 2 | flow `source-contract` calls `testFixtureProjectionRetirement()` | removed import and call; `os/📦️packages/🦀️rust/📜️script.ts` already runs it twice | 1 |
| 3 | flow `source-contract` uses `encodeScalarRecordFixture` (os pack tests) | the encoder (hand-written plus independent LEB128/IEEE754 library path) moved to `F/🔨️modules/🎒️pack/🌱️value/🔎️scalar-witness/🧪️testing/🟦️.ts`, SDK entry `./pack/value/scalar-witness/testing`; the os test imports it back, flow imports the entry. The libraries are loaded lazily with `createRequire`, so the entry loads even though `@webassemblyjs/ieee754` is not installed (section 10) | 1 |
| 4 | draw `📋️clipboard/🧪️tests/🔬️unit` tests the os completion fault projection | that test moved verbatim to `OS/…/🔌️plugin/📬️completion/⚠️fault/🧪️tests/🟦️.ts` (1 pass); routed through `OriginalFixtureCloseSource` in `plugin/📦️packages/🦀️rust/📜️script.ts` (the `test-original-fixture-close-source` command) | 3 |
| 5 | trinity `🔱️jack`/`♻️rewriting` sqlite test calls `testOwnedDocumentClosureOracle()` (the only caller anywhere) | call removed; the oracle runs in `MemberDialectCheckScript` of `os/📦️packages/🦀️rust/📜️script.ts` next to `testFixtureProjectionRetirement`. One-off proof: `bun -e` calling it passes | 1 |
| 6 | stdio `🧿️semio/…/📥️source/🧪️tests` self-test typed against os types and imported the os Generic domain schema | os types replaced by local structural types, the domain schema is now a parameter (`semioMemberStoreProducerSourceSelfTests(check, sources, domainSchema)`); no caller existed | 2 |
| 7 | gis `🌉️mcp` process test and its vitest config (spawn the real `semio-os-mcp`, import os mcp, repo runners) | relocated to the gis dev variant `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧵️gis-inference-bridge/🟦️.ts` and `…/🎚️config/🧵️gis-inference-bridge/🟦️.ts` (all relative imports recomputed; config include fixed); plugin script `inference-bridge-check --process` points there | 4 (+2 B1) |

Total fixed: 2 + 1 + 1 + 3 + 1 + 2 + 4 = 14. Cases 1 to 6 were verified by running the affected tests; case 7 only by loading the config under `bun --bun vitest list` (the process law itself needs the real binary and was never runnable here).

## 5. B7: presentation product

- Story `animate/📖️stories/🎭️presentation-deck/🧪️.story.tsx` (2 edges) moved to `F/🛍️products/🎤️presentation/🎯️targets/⚛️react/📖️stories/🎭️presentation-deck/🧪️.story.tsx`, title `🎬️presentation`. The product's `@semio-tech/presentation` `package.json` opts into the generated Storybook scope registry (`semio.storybook` table, the same form as `ui-styling-ts`); the `storybook-discovery` law now discovers the story under the new path (its remaining failures are foreign: three os story files were moved by a peer). The fixture `🧫️storybook-discovery/🔣️.json` and the browser-runner list `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts` are re-pointed; the e2e spec moved with the story to `…/🎤️presentation/🧪️tests/🎞️storybook-deck/🟦️.ts` (story id `presentation--three-slide-deck`).
- The animate unit test `🎛️apps/🎬️presentation/🧪️tests/🧩️index/🟦️.ts` (1 edge) is a byte-identical duplicate (apart from import path and describe title) of the product's `🧪️tests/🧭️slide-glob-assembly/🟦️.ts`. I deleted it, then had to restore it: the test is the only content of the TS package `@semio-tech/animate-js`, which is woven into four workspace `package.json` lists, `📋️build-owners.json`, `🔒️dependencies.json` and the launch pick lists. Restored byte-identical (git diff empty). Hand-off: delete that package as one change (section 11).

## 6. Source-text reads: the API

New framework module `F/🔨️modules/🧪️test/📖️source-text/🟦️.ts`, SDK entry `@semio-tech/framework/test/source-text`:
- `frameworkSourcePath(path)` resolves a framework-relative path (`🔨️modules/…`, `🛍️products/…`) to the absolute path; it refuses empty paths, absolute paths, `.`/`..`/empty segments, backslash and NUL and checks containment.
- `frameworkSourceUrl(path)` is the file URL; `readFrameworkSourceText`, `readFrameworkSourceJson` read.
- The law `…/📖️source-text/🧪️tests/🟦️.ts` plus the language-agnostic cases `🧫️fixtures/🔣️.json`: accepted paths must equal what `fast-glob` (third party, declared in the root `package.json`) finds below the framework root, refused paths throw, reads equal `node:fs`: 4 pass, 27 expects. Registered: `test-source-text` in `SDK/📜️script.ts`, `📋️project.json`, `package.json`, launch entry `🧪️test🧰️framework📖️source-text` (both JSONs, group `9_gates`, order 900.0585176).
- Codemod `sdk2-tools/reads.ts` (TypeScript AST; dry run first): every string or template literal containing `🧰️framework/` outside imports became `frameworkSourcePath("<tail>")`, `new URL(lit, base)` became `frameworkSourceUrl("<tail>")`, `resolve/join(<root-like>, lit)` collapsed. 101 edits in 47 files (69 path, 32 URL); `sdk2-snap-reads.json` snapshot, check: 87 changed lines, every one carried the literal, one added import line per file, line counts equal. Bootstrap entry `animate/🧪️tests/🎚️config/🟦️.ts` imports the facade by relative path (rule 7 of `RESULT-SDK-BOUNDARY.md`).
- Dynamic imports of framework modules by computed path (invisible to the cruiser) became SDK specifiers: `value/refusal` (2), `schema/oracles/validator`, `manifest`, `compiler/syntax/rust`; the closure oracle went to the os route (case 5).
- Defects of the first mechanical pass, found by diffing test results and fixed: a helper that joined the path onto a root (norm `oracle-source-ownership`, now `readFileSync(projectPath)`); a trailing-slash directory concatenated later (draw `export-document`, `frameworkSourcePath(…)+"/"`); schema dependency lists that `compileSchemaScopeExport` joined onto the repository root (the function now `resolve`s, so repository-relative and absolute paths both work; validator law 4 pass).
- Remaining `🧰️framework` string literals in plugin tests: 0 (grep over tracked and untracked files). 92 lines with such literals remain in plugin `📜️script.ts` build scripts (cargo manifest paths, runner arguments; B1 tooling), outside this slice.

## 7. SDK entries added (17 new, `exports` 125 to 142)

15 `./time-travel/testing/*` (section 3.3), `./test/source-text`, `./pack/value/scalar-witness/testing`. Each is explicit named re-exports only.

## 8. Verification

### 8.1 Entries and surface
All 17 entries imported by specifier in fresh bun processes (`sdkb-tools/facade-load.ts`, `sdk2-facade-load-final.log`): 15 + 1 + 1 ok, 0 bad. The SDK surface law (`SDK/🧪️tests/🔌️sdk-surface`, includes the TypeScript diagnostics over every facade and one fresh process per entry): 5 pass, 439 expects.

### 8.2 Type-check
- `tsc -p`: 905 diagnostics before and after, identical, all syntax errors in three foreign files (section 1). It tells nothing about my edits.
- Compiler API semantic check (`sdk2-tools/tsc-sem*.ts`, 256 root files = every touched file plus all facades): 59 diagnostics in the touched files, none in the kit, the scenario, the runner, any new file or any facade. For the 55 import-line-only files (layout and reads) I served the pre-edit texts from the snapshots: 53 diagnostics before, 52 after, nothing new (the one that disappeared was flow's `encodeScalarRecordFixture` not being exported by the os test). The semantic run found one real error of mine (the semio source type lacked `declaration`), fixed. The 58 others are in the baseline categories (bun test typings of `toEqual`, missing `graphlib` declarations, the `runExactCargoLaws` shape, foreign `readPackageName`).

### 8.3 Tests (`runtests.ts`, 47 baseline files, plus new)
Compared over the 46 baseline files that still exist (testdiff): before 470 pass / 158 fail, after 461 pass / 157 fail. 43 files identical, 3 changed, all explained: draw clipboard 17 to 16 pass (the moved test passes in the os, 1 pass); stdio jpg receiving 12/2 to 5/0 (case 1); norm `oracle-source-ownership` varies run to run because a peer's module `🧭️routing/📥️invocation/🏃️process` is missing in the framework process tree, so the file sometimes fails to load (4/2 before, 3/3 under load, 0/1 now; the only change of mine in it is the single `projectPath` read, validated while the file loaded). Moved or new: gis `🌉️mcp` process 0/1 before and after (vitest file under `bun test`); source-text 4 pass; os fault 1 pass; os scalar-witness law 0/1 before and after (missing dependency, section 10); closure unit file has no test.

## 9. Commands run (real results)

1. Cruiser, exact audit command, via `sh sdk2-tools/cruise.sh <label> full` (alarm 2400): `before` 13,477 errors, 17,150 modules, 46,538 dependencies; `after` (13,213), `after2` (13,183), `after3` = final (13,206 errors, 17,194 modules, 46,711 dependencies, 373 s, exit 150).
2. `bun sdkb-tools/scan2.ts` before (799 violating) and after (685); `bun sdk2-tools/cat.ts`, `bucket.ts` for the buckets.
3. `bun sdkb-tools/plan.ts` (layout: 9 targets, 0 new; kit: 15 targets, 15 new), `apply-surface.ts --write`, `sdk2-tools/rewrite.ts --write`, `sdk2-tools/reads.ts --write`, `verify-snap.ts` on three snapshots.
4. `node node_modules/typescript/bin/tsc -p 🗑️generated/sdk2-tsconfig*.json` before and after (905 each); `bun sdk2-tools/tsc-sem.ts` and `tsc-sem-before.ts` (sections 8.2).
5. `bun sdkb-tools/runtests.ts` before and after, `testdiff.ts`.
6. `bun test` of the source-text law (4 pass), the os fault test (1 pass), the draw clipboard file (16 pass), the validator law with `SEMIO_TEST_ARTIFACT_DIR` (4 pass), the SDK surface law (5 pass), `storybook-discovery` (the presentation story is discovered; the file fails on three foreign os stories).
7. `bun -e` one-offs (closure oracle ok; scalar-witness entry loads; wire `[1,3,0,1,0,4,7]` equals the os fixture capture).
8. `bun --bun node_modules/vitest/vitest.mjs list --config <gis bridge config> --filesOnly`: config loads.
9. `jsonc-parser` parse of both launch files: 0 errors, entries present.
10. `sdkb-tools/facade-load.ts` for the 17 entries, final: 0 bad.

## 10. Foreign issues observed (not mine, not fixed)

- `@webassemblyjs/ieee754` is imported by the os scalar-witness law and by the moved encoder, but is installed nowhere and the root `package.json` declares neither it nor `@webassemblyjs/leb128`. The os law and the flow `source-contract` test fail to load, before and after. The encoder entry itself loads now (lazy require).
- flow `source-contract` imports `…/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts`, which no longer exists (the module moved to `🏗️builder/♻️retirement/…`): an unresolved import, so no cruiser edge; I left it, because fixing the path would create a new B6-like edge into a framework test file.
- The three syntax-broken foreign files that blind `tsc -p` (section 1), and `🌎️hub` `animate` Cargo `[package.metadata.semio.storybook]` (scope `animate`, glob `✏️s/🔌️plugins/🎞️animate/📖️stories/**`), which now matches nothing.
- Taxonomy: the new folders are not registered in `🔣️taxonomy.json`. The simulator (`🗑️generated/taxsweep/sim.ts` on the new folders and their ancestors) lists them as unresolved: the 16 kit folders under `⏪️time-travel/🧪️testing`, the 16 SDK facade folders `SDK/⏪️time-travel/🧪️testing/*`, `📖️source-text` (module and facade), `🔎️scalar-witness/🧪️testing`, `🧵️gis-inference-bridge` (test and config), `🎞️storybook-deck` under the presentation tests. The kit names are already registered, but only under owner `tests`, not under `🧪️testing`; I did not edit the registry (it is volatile and another slice owns the sweep). Re-run `build2.py`/`validate.ts` of the sweep for these names.
- Cruiser churn: `after`, `after2`, `after3` differ by peer edits (about 30 lines); I attribute through the ASCII-skeleton diff (the cruiser output contains broken emoji bytes on some lines, which makes a naive line diff report moved edges as "added").

## 11. Hand-offs

1. B6, 12 edges, one cluster: the os document-service ABI used by gis. Files: `gis/💡️inference/👷️worker/🟦️.ts` (4: `DirectoryHttpError`, `DocumentScope`, `BackboneWorkerResponse`, `DocumentExecutionTargetLeaseFieldsV1` from `@semio-tech/framework-os`; `DocumentServiceWorkerHostV1` from the store worker; `InstalledServiceDriverV1`/`InstalledServiceOperationV1`), `🔌️client/🟦️.ts` and `🔌️client/🚪️io/🟦️.ts` and their test (4), `🪟️presentation/🟦️.tsx` (1), `🧪️tests/🔬️probe/🟦️.ts` (1), `🧬️schema/🟦️.ts` (1), `🧪️tests/🎚️config/🟦️.ts` (1, the vitest config derives from the os tests config), plus stdio `📜️docx/…/🧬️mutations/🟦️.ts` (1, pack codec). Real neutral contracts exist for three of them; none is SDK-exportable today because they sit in the os product:
   - installed-service declaration and wire contract: `OS/🔨️modules/💡️inference/🔌️service/{🟦️.ts,🚪️io/🟦️.ts,🧬️schema/🧺️value/🟦️.ts}` plus `OS/…/📇️directory/🔌️client/🌐️document-http/🧬️schema/🟦️.ts`: four pure TypeScript files without outside dependencies (a finite JSON-schema subset compiler and the declaration admission). Move them to `F/🔨️modules/💡️inference/🔌️service/`, re-point about 12 os importers (`🏪️store/👷️worker`, `🧵️backbone`, `os/🟦️.ts`, `🛫️spawned-session`, `🌐️browser-bundle/🎯️action-handoff/📤️publication`, the os package script `installed-service-source`), the Rust `include_str!` users in `🌉️mcp`, the fixtures `🧱️source-ownership.json` and `generator-ownership`, the schema-document index, the taxonomy; then add SDK entries `inference/service` (+`/io`). 5 gis edges.
   - pack value codec: `OS/🔨️modules/🎁️packs/🟦️.ts` (986 lines) imports only `@semio-tech/framework`, `@semio-tech/framework-replication` and `⏱️trace/🧮️memory`; it can move to a neutral module (`encodePackValue`, `decodePackValue`, `packUInt`, `isPackInteger` for the plugins) with 9 os importers and 4 `📋️project.json` input lists. 2 plugin edges.
   - `📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts` (141 lines, no imports) with its Rust twin and fixtures: 1 edge. The `DocumentServiceWorkerHostV1` host type and `DirectoryHttpError` need an explicit worker port (host interface in a neutral module, implemented by the os), which is a design step of its own.
2. The gis `🧪️tests/🎚️config/🟦️.ts` and `🎚️config/📺️renderer/🟦️.ts` (B6 and B3) derive from os vitest base configs; they need a neutral vitest base config in the SDK (or a move into the gis dev variant together with the 6 launcher, hub and ownership-test references to the config path).
3. B7: delete the TS package `@semio-tech/animate-js` as one change (four workspace `package.json` lists, `📋️build-owners.json`, `🔒️dependencies.json`, launch pick lists, its config and the duplicate `🧩️index` test), then the last presentation edge disappears.
4. L: the 4 repo-runner imports (B1) and the 3 store `child`/`link` imports (B2) follow their owners.
5. B1/B5 (repo library, repo test kit), B2 (store schemas), B3 (renderer hosts): untouched as instructed. The `no-cross-package-relative-*` structural family of the 17 new facades (plus 15 in the SDK package) belongs to P11.
6. Taxonomy registration of the new folders (section 10).
7. `@semio-tech/framework-os-dev:time-travel` now needs `--scenario <module>` or `--universal`; any other launcher entry that ran `@semio-tech/puzzle-js:time-travel` must switch (none other exists in the repo: grep).

## 12. Files changed

- SDK package `SDK/`: `package.json` (+17 `exports`, +`test-source-text`), 15 `⏪️time-travel/🧪️testing/<m>/🟦️.ts` facades, `🧪️test/📖️source-text/🟦️.ts`, `🎒️pack/🌱️value/🔎️scalar-witness/🧪️testing/🟦️.ts`, `📜️script.ts` (`SourceTextScript`), `📋️project.json` (`test-source-text`).
- Framework modules: `F/🔨️modules/⏪️time-travel/🧪️testing/**` (16 moved modules, 2 docstring links), `F/🔨️modules/🧪️test/📖️source-text/**` (module, law, fixture; new), `F/🔨️modules/🎒️pack/🌱️value/🔎️scalar-witness/🧪️testing/🟦️.ts` (new), `F/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts` (`resolve`).
- os: `OS/…/🧑‍💻dev/🧪️tests/🧪️time-travel/{🟦️.ts,🎪️run,🌐️universal,📟️console-digest}` (imports, `--scenario`), `OS/…/🧑‍💻dev/🔮️oracles/🔣️.json`, `OS/…/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts`, `OS/…/🔌️plugin/📬️completion/⚠️fault/🧪️tests/🟦️.ts` (new), `OS/…/🔌️plugin/📦️packages/🦀️rust/📜️script.ts`, `OS/📦️packages/🦀️rust/📜️script.ts`.
- Presentation: `F/🛍️products/🎤️presentation/📦️packages/🟦️typescript/package.json`, moved story and e2e spec, `F/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧫️storybook-discovery/🔣️.json`, `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts`.
- Plugins: 9 layout files; 9 puzzle scenario files (+`🟦️.ts` export and docstring), puzzle `📜️script.ts`, `📋️project.json`, `package.json`; 47 files with source-text reads; stdio jpg receiving, semio base source, `.test.ts`, binary raw, step history-inputs tests; flow `source-contract`; draw clipboard; trinity sqlite; gis `📜️script.ts` and the removed process test and config; animate vitest config (relative facade import) and the story/spec removals.
- gis dev variant: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧵️gis-inference-bridge/🟦️.ts` and `…/🎚️config/🧵️gis-inference-bridge/🟦️.ts` (moved).
- Registration: `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (2 gate commands re-pointed, 1 entry added).
- Tools and logs: `sdk2-tools/**`, `🗑️generated/sdk2-*` (kept).
