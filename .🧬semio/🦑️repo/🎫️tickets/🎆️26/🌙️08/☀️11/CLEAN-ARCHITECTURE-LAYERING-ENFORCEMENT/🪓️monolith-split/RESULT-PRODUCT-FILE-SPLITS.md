# Result: product file splits (slice PRODUCT-FILE-SPLITS, prefix `pfs`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool, 2026-10-16 13:40 to 15:00. `F` = this folder. Tools: `F/pfs-tools/` (`msplit.mjs`, `reimport.mjs` and `lossless*.mjs` are copies of the ts3 tools with the output names changed to `pfs-`; `rs/` holds a copy of the frs Rust oracle plus my splitter `pfs_rs.py` and the three plans). Logs and baselines: `F/🗑️generated/pfs-*` (nothing deleted; originals of the nine files in `pfs-orig/`). No `[DEBUG]` log was added. Ordered by the slice list.

## Verdict

All nine files are split. Every child is at most 961 lines (largest: Rust reference world 961, probe table child 364, grammar family child 608). The test-title multisets, the pass and fail sets and the `tsc` error sets are identical or smaller everywhere; the Rust differential is IDENTICAL in every configuration. No `no-circular` hit touches a file of mine. Nothing was compiled or run with cargo (the launcher is still gated); the Rust proof is static plus a differential that type-checks deeper than the resolution-only probe. Four honest caveats are in "Caveats".

| # | File (before) | After | Verification |
|---|---|---|---|
| 1 | pets clearance Rust unit tests (2,258) | hub `🔬️unit/🦀️.rs` (39) + 6 children, 151 to 961 lines | differential IDENTICAL (lib, test), 64 test fns identical |
| 2 | pets contract `🧬️schema/🦀️.rs` (1,851) | hub (47) + 5 children, 196 to 709 | differential IDENTICAL in 6 configs, serde-attr probe |
| 3 | pets clearance unit tests TS (1,768) | 6 children, 84 to 705 | 66 titles identical, line multiset identical |
| 4 | pets `🚶️locomotion/🟦️.ts` (1,732) | 10 children, 40 to 524; parent removed | 1,133 pets tests identical, importers re-pointed |
| 5 | presentation `📦️packages/🟦️typescript/🟦️.ts` (2,011) | entry (15) + 11 children, 35 to 373 | 56 + 140 tests identical |
| 6 | print `🧬️native-chart-grammar/🟦️.ts` (2,209) | parent (323) + 6 children, 9 to 608 + 1 helper | 4 bun readers identical, 64 exports identical |
| 7 | print `🔬️probes/🟦️.ts` (1,766) | parent (167) + 7 children, 41 to 364 | 631 of 631 checks identical |
| 8 | quiz `🚶️learner-journey/🟦️.tsx` (2,138) | 5 children, 269 to 618; parent removed | 47 titles identical |
| 9 | teaching proctor `🌐️end-to-end/🦀️.rs` (1,771) | hub (49) + 5 children, 221 to 398 | differential IDENTICAL, 21 test fns identical |

Not touched: `ts3`'s `🐾️pets/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts` and `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx` (they were already gone when I started; I only edited the two lines named under "Edits in ts3 files").

## Found state and baseline (before the first edit, 13:40 to 13:57)

- All nine files were clean in `git status`. ts3's children of the stage test and the presentation deck test already existed (the pets run therefore had 23 files, 1,133 tests).
- `bun repo run` was not used; vitest was run directly: `node node_modules/vitest/vitest.mjs run --config <cfg> --reporter=json` (`pfs-tools/vt.sh`), default test level. Summaries: `node pfs-tools/vsum.cjs <json>`, multiset comparison `pfs-tools/cmp-titles.sh <before> <after> <filter>` (the sorted `status<TAB>fullName` rows).
- Baseline sets: pets config 23 files, 1,133 passed, 0 failed; quiz-react filter `learner-journey` 1 file, 47 passed; presentation core 2 files, 56 passed; presentation react 7 files, 140 passed.
- Defect found in the baseline: the presentation core config (`🎤️presentation/🧪️tests/🎚️config/🟦️.ts`) listed `../🧭️slide-glob-assembly/🟦️.ts` and `../📽️presentation-core/🟦️.ts`, which resolve below `📦️packages` and match nothing (`No test files found`, exit 1). I ran the baseline through a corrected copy (`pfs-tools/presentation-core.vitest.mjs`) and then fixed the repo config (`../../🧪️tests/...`). The fixed repo config finds the same 2 files and 56 passing tests.
- `tsc --noEmit -p`: pets `tsconfig.json` 0 errors; quiz-react `tsconfig.json` 1 error (`jsdom` types in `🚏️navigation`, not mine); presentation and print closures through generated tsconfigs (`pfs-tools/mktsc.mjs`, files lists `files-presentation.txt`, `files-print.txt`): 0 and 2 (two `T` errors in the peer's `native-build/💾️storage`, which a peer fixed meanwhile).
- bun tests of the four grammar source-text readers (`pfs-tools/bun-tests.sh`): paint 1 pass 3 fail (the three failures are existing `registerBuildTestLawV1` invocation failures), refusal 8 pass, pdf 10 pass, operation grammar 2 pass; expect counts 84, 72, 103, 13.
- Probe table: all 631 checks of `vizInferenceChecks("quick")` run through `pfs-tools/probe-run.ts` against a temporary same-directory copy of the original (deleted at once); one row per check with the digest of subject and oracle output.
- Grammar module: `pfs-tools/exports-digest.ts` over the original, 64 exports with a digest of each function source.
- Rust: `snapshot.py` copies of the three crate trees (`🗑️generated/pfs-work/{clearance,schema,e2e}/snap`) with the file sha. Baseline of the differential: clearance 281 errors (lib) and 1,171 (test), contract 262 / 1,022 (285 with `testkit`), end_to_end 569 (test).

## What changed

Method: `msplit.mjs` moves top-level TypeScript statements into children by start line (checker-based; imports rebuilt, `export` added only for cross-child names, relative specifiers and dynamic imports rebased, regions closed and reopened, cluster cycles refused). For every split it reported no cluster cycle. `pfs_rs.py` does the same for Rust line ranges (see "Rust method").

### 1 and 3. Clearance unit suite (Rust and TypeScript twin)
Topic folders under `🚧️clearance/🧪️tests/` hold both languages next to each other: `🌍️reference-world` (Rust 961, TS 705), `🧰️clearance-tools` (521, 84; the generated cases are in the Rust tools), `📦️boxes-and-places` (174, 187), `📜️claims-and-order` (210, 289), `🪑️seating-and-heads` (258, 306), `🏛️world-laws` (151, 191). The TS parent is gone. The Rust hub `🔬️unit/🦀️.rs` stays where the crate mounts it (`#[path = "🧪️tests/🔬️unit/🦀️.rs"] pub(crate) mod tests;` is unchanged), keeps the docs and the four `use` lines and mounts the children with `#[path = "../<folder>/🦀️.rs"]`. Only `reference_world` (`pub(crate) use`, so `tests::Rules` and friends keep their paths) and `clearance_tools` are glob-re-exported; the four test children are plain `mod`. The 21 helper fns of the tools child are `pub(super)` (usage based). `include_str!("../../🦀️.rs")` in the determinism test keeps its path because the child sits at the same depth. Registered in the pets vitest config: four `include` rows after the schema row (the config is shared with ts3, edited with an exact-string replace).

### 4. Locomotion
Children in `🚶️locomotion/`: `📐️constants`, `⚓️footing`, `🛬️courses`, `🛫️departure`, `🏃️motion`, `✋️hand`, `🪜️outings`, `🧳️trips`, `🪄️mischief`, `🚪️leaving`. The first draft had a cycle (footing used `bounced` from motion); moving `bounced` into footing fixed it. `unpacked`, `freed` and `ungear` (release helpers) sit with footing. The module docstring moved into `⚓️footing` with the `@see` paths rebased. 10 importers re-pointed by `reimport.mjs` (it splits an import by owning child), and the module sits in no barrel. The Rust twin's doc line and the stage façade's `@see` line were updated.

### 5. Presentation package entry
The entry `📦️packages/🟦️typescript/🟦️.ts` keeps its taxonomy path and is still `exports["."]` of `@semio-tech/presentation`. Its body sits in 11 children: `🧱️vocabulary`, `🎭️embodiments`, `🔭️scope`, `🎞️structure`, `🧩️tiles`, `📐️resolution`, `🔖️bookmarks`, `📂️slide-files`, `🎬️intro`, `🧭️slide-glob`, `🔄️analogy-and-play`. The entry is the package surface and not a compatibility barrel: it lists by name exactly the 153 names the file exported before (`pfs-tools/mkentry.mjs`; `grep -c '^export'` of the original is 153), so helpers that became cross-child exports do not leak. Two cycles had to be broken by statement choice: `resolutionScopeForArrangement` lives with the structure, and `IntroSpec` with the slide files. No importer changed. Header region `🧲️Header` stays in the entry.

### 6. Native chart grammar (print)
Parent `🧬️native-chart-grammar/🟦️.ts` keeps `compileNativeGrammar` (323 lines with imports) and its docstring; children: `🧰️grammar-kit` (`equal`), `🧱️basics`, `🎛️family-and-scientific`, `🗺️geo-planar`, `🏭️domain-effects`, `🧠️neural-and-domain`, next to the existing `🎨️paint`, `📥️emission`, `💹️financial`. New helper `📖️source-files/🟦️.ts` lists the parent and the six children (`nativeGrammarSources()`), because four source-text laws scan the grammar text. They now read it through the helper:
- `🎨️paint/🧪️tests` counts `nativeControlPdfPaint(` calls (still exactly 40, each awaited with `probe.control`); its two unused node imports were removed.
- `📥️refusal/🧪️tests` scans for `inspectVizProbeDocumentRefusalV1`, `probe-out` catches and `case:` literals over all files, and finds `compileNativeSpecializedPlacementAdmission` in whichever file holds it.
- `📖️pdf/🧪️tests` checks the joined text for `probe.inspect()` and for no `pdfjs-dist` import.
- `🎛️operation/🧪️tests/📥️grammar` got the six new paths in its list.
I did not use `readModuleTreeSource` because it would also follow the existing paint, emission and financial children and change the counts. The print package script and the inference script only import the parent's `compileNativeGrammar` or pass its path to `tsc`.

### 7. Probe table
Parent `🔬️probes/🟦️.ts` keeps `vizInferenceChecks`, `inferredLayerChecks` and `renderGrammarChecks`; children `🧰️probe-kit` (levels, check shape, samples, `recordD3Path`), `📐️scale-and-format`, `🧮️transform-and-shape`, `🧭️coordinate-and-hierarchy`, `🕸️network-and-flow`, `🌍️geo-and-spatial`, `🖼️theme-and-render`. The print inference script now imports `VIZ_INFERENCE_LEVELS` and `Level` from the kit. d3 resolves from the same package `node_modules` because the children sit below it. The one dynamic import with a relative path (`print-design-token-paints`) got its extra `../`.

### 8. Learner journey (quiz)
Children under `❓️quiz/🧪️tests/`: `🧰️learner-journey-kit` (fixtures, `FakeProctor`, builders, the original header docstring), `🚶️journey-and-tabs`, `🧩️deputy-and-contract`, `🏔️challenge-session`, `🧷️run-truth`. My first names `🫡️deputy-and-contract` and `⛰️challenge-session` repeated the emoji of the siblings `🫡️deputy-decisions` and `⛰️challenge-rules`; they were renamed before any verification. Registered in the quiz-react vitest config and in the package `tsconfig.json` (one row replaced by four). `?raw` stylesheet import unchanged (same depth). The README of the quiz architecture named the old folder; it names the new ones.

### 9. Proctor end to end
The crate root stays `🌐️end-to-end/🦀️.rs` (`[[test]] path` unchanged); children `🧰️harness`, `🎮️learner-journey`, `🏆️challenge-sheets`, `🛠️operator`, `🛡️edge` under it. Harness items used by the tests are `pub(super)`, including `Scratch`, `Running` and their fields and `shut_down` (the first draft left the fields private and the oracle reported 68 + 7 + 24 E0616 and 22 E0624; the splitter now widens a struct that flows out of a widened fn). Only `harness`, `operator` and `edge` are glob-imported.

### 2. Contract schema
Hub `🧬️schema/🦀️.rs` keeps docs, `use serde::{…}` and the test mount; sections became `🏷️scalars`, `🦴️rig-and-animation`, `🐾️menagerie-and-terrain`, `🎬️stage`, `🎞️frame`, each `pub use <child>::*;` so every path `schema::Name` stays (the facade's 480 names are untouched). Three private serde helpers are used across sections through `deserialize_with = "…"` strings and are `pub(super)`: `nullable`, `meeting`, `elapsed`.

### Edits in ts3 files (minimal, recorded)
- `🎪️stage/🧪️tests/🪄️mischief-and-arithmetic/🟦️.ts`: its arithmetic law read `<part>/🟦️.ts` for 13 parts including locomotion. A `sourcesOf(part)` helper reads the part's own file or the file of each child folder; the loop checks each source.
- `🎪️stage/🧪️tests/🦾️body-and-gear/🟦️.ts`: import re-pointed to the children by `reimport.mjs`.
- `🧪️tests/🎚️config/🟦️.ts` of pets: the four clearance `include` rows.

## Verification

Commands that produced the numbers (all run by me, results as stated):

| Check | Before | After |
|---|---|---|
| pets vitest, whole config (`vt.sh`, JSON) | 23 files, 1,133 passed | 26 files, 1,133 passed, `cmp-titles.sh`: missing 0, extra 0 |
| clearance titles (filter `clearance`) | 66 | 66, IDENTICAL (4 files) |
| quiz-react, the learner-journey tests | 1 file, 47 passed | 4 children, 47 passed, IDENTICAL; whole quiz config 25 files, 923 passed |
| presentation core, repo config | 56 (through the corrected copy) | 56, IDENTICAL |
| presentation react config | 7 files, 140 passed | 7 files, 140 passed, IDENTICAL |
| bun: paint / refusal / pdf / operation grammar | 1p 3f / 8p / 10p / 2p; expects 84 / 72 / 103 / 13 | 1p 3f / 8p / 10p / 2p; expects 84 / 72 / 103 / 13 |
| probe table `quick` | 631 rows | 631 rows, `cmp` equal |
| grammar export digests | 64 | 65: the same 64 plus `equal`; 62 digests equal, 2 differ only by the rebased dynamic import path (below) |
| `tsc` pets / presentation / quiz / print (final run) | 0 / 0 / 1 / 2 | 0 / 0 / 1 (same message) / 0 |
| dependency-cruiser `no-circular` (`cycscan.sh`, roots = my trees) | n/a | pets 0 violations; presentation 0; grammar and probes closure 15, none touching my trees; quiz closure 90, none touching my trees |

- Line multisets (`lossless2.mjs`, imports and the leading docstring and a leading `export ` ignored): clearance TS 1,571 = 1,571, missing 0 extra 0; quiz 1,915 = 1,915, 0 / 0; locomotion missing 0, extra 2 (the `//#region 🔖️Gear` pair reopened across children); presentation missing 1 (the Header `#endregion`, which sits in the entry) and 8 synthetic region markers; probes missing 1 and extra 4, where the one real change is the dynamic import `"../../../../../🔨️modules/🎨print-design-token-paints/🟦️.ts"` rebased with one more `../`, the rest are region markers; grammar 2 + 2 rebased dynamic imports (`🧬️schema/💡️inferences/🎨theme` and `🦑️repo/…/🗂️workspaces`), the same two functions as in the digest table. No region marker was lost.
- Rust (`lossless-rs.py`, `rs-tests.py`): original line multisets equal the union of hub and children except the intended `pub(super)` widenings, the child docs and mounts, and one changed doc line (`@see` to the moved TS world); `#[test]` fn names identical: clearance 64 = 64, end_to_end 21 = 21.
- Static Rust oracle (`pfs-tools/rs/oracle.py`; `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, sandbox copies only, globs and externs patched, external crates stubbed by the frs `stubgen` so privacy errors surface; baseline sandbox = `git`-clean snapshot, current = the live tree): `oracle.py clearance live` lib 281 = 281 and test 1,171 = 1,171; `schema live` lib 262, test 1,022, lib+testkit 285, test+testkit 1,022, lib+all 285, test+all 1,022, all IDENTICAL; `e2e live` test 569 = 569. `--warn`: no new warning in any of the three (the first drafts had unused glob imports for the test children; they were removed). `rustfmt --check` on the live trees: clearance 0 hunks, schema 2 and end_to_end 22 hunks, both equal to the baseline (existing wrapping debt). `rustc -Zunpretty=normal` parses all three live trees (exit 0).
- Oracle sensitivity (negative controls in the sandbox, not in the repo): re-hiding `kept_apart` gives E0425; hiding `Running.base` gives 29 E0616; hiding `registrations` gives E0425. The oracle cannot expand serde derives, so the three `deserialize_with` helpers were proved by import probes: adding `use super::{nullable as _, elapsed as _, meeting as _};` to the stage, frame and menagerie children gives no new error, while `encounter as _` (private, in another section) gives E0432.
- Source-boundary census (read-only `readRustSourceClosureV1`, host cfg, limits 4000 lines / 32 files / 262,144 bytes): contract lib 8 files / 114,570 B / 2,743 lines (was 3 / 112,087 / 2,685), contract test 11 / 172,352 / 3,628 (was 6 / 169,869 / 3,570); clearance lib 1 / 34,835 / 734, clearance test 8 / 143,254 / 3,056 (was 2 / 140,111 / 2,994); teaching end_to_end test target 6 files / 130,957 B / 1,828 lines. Everything is under the gate. Longest new path: 168 UTF-8 bytes (limit 240).

## Caveats

1. **Rust test names gain a module segment.** A `#[test]` in a child module is `tests::<child>::<fn>` (clearance, for example `tests::world_laws::quick::generated_cases_agree_with_brute_force`) and `<child>::<fn>` in end_to_end. The fn names, the count and the inline `quick` and `exhaustive` modules are identical, and the level filters are substring filters (`quick::`, `exhaustive::`), which still match. I found no launch entry, script or document that names one of these tests (checked with `git grep` on the fn names).
2. **Not compiled.** No `bun repo cargo` call was made. The differential is a stubbed type-check, so it is stronger than resolution only (it found the field and method privacy cases), but it cannot see serde derive output, trait coherence or code generation. Queue the commands below.
3. **Bun test 4 flaked once under load.** In the final batch (machine load about 80) the operation-grammar test hit its default 5 s limit (`timed out after 5000ms`, 5.5 s); three reruns passed (2 pass, 0 fail, 13 expects, 4.0 to 5.2 s). The test parses seven grammar files with the TypeScript compiler instead of six. The baseline run took 5.4 s in total, so the margin was thin before the split too.
4. **No `no-circular` baseline run.** The originals are gone from the tree, so I could not take a "before" scan. The argument: every split passed the cluster cycle check, a new cycle would need an old cycle through the old file, and the scan of the current trees reports no cycle through any file of mine.

## Real checks for the compile queue

Acquire the fleet mutex first (`mkdir F/🗑️generated/cargo-mutex.lock`), one launcher call, release after. Pets are members of the root workspace; the proctor has its own workspace.

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-pets-contract -p semio-framework-pets-clearance --lib --tests --keep-going --message-format=short
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --tests --keep-going --message-format=short
```
Then run the suites: `-- test -p semio-framework-pets-contract -p semio-framework-pets-clearance --lib` (plus `quick` and `exhaustive` through `bun repo run @semio-tech/pets-rs:test quick|exhaustive`), and `-- test --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --test end_to_end`. Expect a lock refresh (`update --workspace --offline`) only if a peer added crates; no manifest or dependency changed here.

## New folders for the taxonomy sweep (56)

Parents are given relative to `🧰️framework/🛍️products/` (teaching: `🎓️teaching/`). The list with line counts is `🗑️generated/pfs-inventory.tsv`; the plain list is `🗑️generated/pfs-newfolders.txt`.
- `🐾️pets/🔨️modules/🚧️clearance/🧪️tests/`: `🌍️reference-world`, `🧰️clearance-tools`, `📦️boxes-and-places`, `📜️claims-and-order`, `🪑️seating-and-heads`, `🏛️world-laws` (each holds `🦀️.rs` and `🟦️.ts`).
- `🐾️pets/🔨️modules/🚶️locomotion/`: `📐️constants`, `⚓️footing`, `🛬️courses`, `🛫️departure`, `🏃️motion`, `✋️hand`, `🪜️outings`, `🧳️trips`, `🪄️mischief`, `🚪️leaving`.
- `🐾️pets/🧬️schema/`: `🏷️scalars`, `🦴️rig-and-animation`, `🐾️menagerie-and-terrain`, `🎬️stage`, `🎞️frame`.
- `🎤️presentation/📦️packages/🟦️typescript/`: `🧱️vocabulary`, `🎭️embodiments`, `🔭️scope`, `🎞️structure`, `🧩️tiles`, `📐️resolution`, `🔖️bookmarks`, `📂️slide-files`, `🎬️intro`, `🧭️slide-glob`, `🔄️analogy-and-play`.
- `📓️print/🧪️tests/🧬️native-chart-grammar/`: `🧰️grammar-kit`, `🧱️basics`, `🎛️family-and-scientific`, `🗺️geo-planar`, `🏭️domain-effects`, `🧠️neural-and-domain`, `📖️source-files`.
- `📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/`: `🧰️probe-kit`, `📐️scale-and-format`, `🧮️transform-and-shape`, `🧭️coordinate-and-hierarchy`, `🕸️network-and-flow`, `🌍️geo-and-spatial`, `🖼️theme-and-render`.
- `❓️quiz/🧪️tests/`: `🧰️learner-journey-kit`, `🚶️journey-and-tabs`, `🧩️deputy-and-contract`, `🏔️challenge-session`, `🧷️run-truth`. Removed: `🚶️learner-journey` (the file was deleted and the empty folder removed; `🦑️repo/…/📚️library/🔣️taxonomy.json` line 14797 still lists it, the sweep owns that file).
- `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/`: `🧰️harness`, `🎮️learner-journey`, `🏆️challenge-sheets`, `🛠️operator`, `🛡️edge`.
Removed files: `🚧️clearance/🧪️tests/🔬️unit/🟦️.ts`, `🚶️locomotion/🟦️.ts`, `🚶️learner-journey/🟦️.tsx`.

## Registration

- Vitest includes: pets config (four clearance rows), quiz-react config and its `tsconfig.json` (four rows replace one), presentation core config (paths corrected). The grammar and probes have no per-file include (reached by import).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: no edit. The nine files had no entry of their own (`git grep` on every moved path and on `learner-journey`, `native-chart-grammar`, `🔬️probes`, `locomotion`, `clearance`, `end-to-end` finds none); the existing routes already run the children: `🧪️test🐾️pets🟦️`, `🧪️test🐾️pets🦀️`, `🧪️test❓️quiz⚛️react`, `bun repo run @semio-tech/presentation:test`, `⚖️test📓️print🧬️native-grammar` (and its phase variants), `bun repo run @teaching/proctor:test`, plus the per-crate `🦀️ semio-framework-pets-{contract,clearance} …` rows. Both files are untouched and unchanged in size.

## Hand-offs

1. Taxonomy registration of the 56 folders (held file `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`).
2. The Rust differential and the real `cargo` runs above, including the unit suites at `quick` and `exhaustive`.
3. ts3: the three lines under "Edits in ts3 files" are in their files; their `🧰️stage-helpers` kit is unchanged.
4. The existing repo `📜️script.ts` of `🎤️presentation/📦️packages/🟦️typescript` and the Storybook source roots still point at the package folder, which now has the 11 child folders.
5. The 15 and 90 `no-circular` violations in the print and quiz closures are other owners' (print `🧬️schema/💡️inferences` shape, transform and spatial, refusal, process leases, 2d).

## Files

Created: the 56 child folders above (`🟦️.ts`, `🟦️.tsx` or `🦀️.rs` each; 76 files with the hubs). Rewritten parents: `🚧️clearance/🧪️tests/🔬️unit/🦀️.rs`, `🧬️schema/🦀️.rs`, presentation entry `🟦️.ts`, `🧬️native-chart-grammar/🟦️.ts`, `🔬️probes/🟦️.ts`, `🌐️end-to-end/🦀️.rs`. Edited: the 10 locomotion importers, `🎪️stage/🟦️.ts` (import and `@see`), `🚶️locomotion/🦀️.rs` (doc line), the pets, quiz-react and presentation configs, the quiz `tsconfig.json`, `🏛️architecture/❓️quiz/README.md`, `🐾️pets/README.md` (three rows), `📓️print/🧪️tests/🧬️native-chart-grammar/🎨️paint/🧪️tests`, `📥️refusal/🧪️tests`, `📖️pdf/🧪️tests`, `🎛️operation/🧪️tests/📥️grammar`, the print inference `📜️script.ts`. Temporary: `F/pfs-tools/`, `F/🗑️generated/pfs-*`.
