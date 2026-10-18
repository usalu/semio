# Result CENSUS5-C7 (prefix `c5c7`, package 7 of `MONOLITH-CENSUS-5.md` section 4)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/nx/`bun repo`. `F` = this folder. `LY` = `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`. Tools: `F/c5c-tools/` (specs `sc ej ex ed`, fragment spec `FRAG["exu"]` in `specs5.py`). Logs: `F/🗑️generated/c5c7-*` and `c5c-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c5c/`.

## 0. Verdict: DONE (static evidence, re-verified on the live tree 2026-10-18)

The predecessor's `RESULT-CENSUS5-C.md` header ("edited nothing") is stale: it had already installed package 7 before its session ended (live files were 76/55/60/6/267 lines). I did not re-split. I verified snapshots and installed state, and re-ran every oracle on the live tree.

- Snapshots `F/c5c-tools/orig/*.rs`: `shasum -c orig/SHA256` OK for all 14 files (the five package 7 snapshots are the HEAD originals: 1,428 / 1,456 / 2,524 / 1,322 / 1,999 lines).
- Every file of the package is now under 1,000 lines (largest child 817).

## 1. Per file

| tag | file (tail under `LY/`) | before | after (root + children, max child) | mechanism | crate |
| --- | --- | ---: | --- | --- | --- |
| sc | `⚙️engine/🎬️scene/🦀️.rs` | 1,428 | 76 + 8 (282) | `#[path]` modules | `semio-s-artifact-layout-layout-engine` |
| ej | `⚙️engine/📤️export/🔣️json/🦀️.rs` | 1,456 | 55 + 4 (817 `expand-document`, holds the `impl TypedJsonCursor` cut at a method boundary into 2 impl blocks) | `#[path]` modules | `semio-s-artifact-layout-layout-engine-json` |
| ex | `⚙️engine/📤️export/🦀️.rs` | 2,524 | 60 + 5 (640) | `#[path]` modules | `semio-s-artifact-layout-layout-engine-export` |
| exu | `⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` | 1,322 | 6 + 3 fragments (471) | `include!` fragments (flat test module, 41 tests to 41) | `semio-s-artifact-layout-layout-engine-export` (test) |
| ed | `🦀️.rs` | 1,999 | 267 + 8 (342) | `#[path]` modules | `semio-s-artifact-layout-layout-editor` |

Children (all `<emoji>️<kebab>/🦀️.rs` next to their root, 28 new folders):
- sc: `🎨️display-list-and-hit-testing`, `🔤️text-fonts`, `⚙️layout-engine-and-views`, `✍️drawing-labels-and-rasters`, `🖌️drawing-strokes-and-paths`, `🧩️assemble-display-list`, `🔠️style-spans-and-preview-marks`, `🎬️scene-painting-and-queries`.
- ej: `📖️expand-document`, `🏗️cursor-construction-and-owner-pages`, `🌳️typed-json-nodes-and-scalars`, `🧾️limits-and-string-sources` (the existing `✅️json-validation` child is untouched).
- ex: `⏭️advance-commit-and-codec`, `📜️export-contract-and-state`, `🏗️job-admission-and-close`, `📕️pdf-png-and-package-encoding`, `✅️validate-plan-and-svg`.
- exu: `🎟️harness-pdf-and-dispatch-laws`, `🚧️caps-json-checkpoint-pdf-style`, `✒️pdf-drawing-laws`.
- ed: `🕹️layout-play-app`, `🎞️reserved-import-job`, `🧵️retained-command-factories`, `🎥️media-export-job-and-factories`, `📋️ui-values-and-context-menus`, `📤️export-tool-job`, `🧱️create-layout-app`, `🪧️window-engagement-and-labels`.

## 2. Evidence (all run on the live tree)

| check | result |
| --- | --- |
| losslessness | `modsplit.py lossless <tag> <live dir>` for ed, ej, sc, ex: `missing 0`; every extra line is wiring (`#[path]`, `mod`, `use child::*`), a child docstring, `use super::*;` or a declared impl-split pair; regions/endregions original/new ed 16/18, ej 1/1, sc 4/10, ex 7/11 (equal between region and endregion in each) |
| structure | `vmod.py live ed ej sc ex`: VMOD OK (single mount, docstring emoji first/equal to the folder/unique/different from the root, path lengths, brace depth, rustfmt parse, include targets, `#[test]` multiset, no `[DEBUG]`). `chk.py live exu`: CHK OK, 41 tests to 41, gone 0, extra 0 |
| definition scan | `defscan.py`: cross-file private leftovers 0 for ed, ej, sc, ex |
| glob ambiguity / module clash | `ambig.py live`: 0 candidates; `modclash.py`: OK |
| patched differential (`oracle5.py`, original restored in a sandbox, same extern/glob neutralisation both sides) | error and warning counts equal, `only-orig=0 only-new=0`: ex lib 84/84, test 382/382; ej 26/26 (both cfgs, 1 warning each); sc lib 38/38, test 137/137; ed lib 461/461, test 836/836. Log `c5c7-oracle-recheck.log` |
| typeck oracle (`posrun.py` family `c5c7`, labels `pre`, `post3`) | canary E0308/E0616/E0624 reached; privacy 0, trait-scope 0; lib rows pre 1926 / post 1926 (74 differing rows each are only stub trait renames, e.g. `InteractiveJob`/`ArtifactEditor` to `Tr`); test rows 3679 / 3681: the two extra rows are `E0277 ... : Tr` stub-bound rows inside the `include!` fragments `🎟️harness...:15` and `🚧️caps...:211`, which this oracle cannot see through `include!` (same effect as `rp` in `RESULT-CENSUS5-B.md`); the rustc-based `oracle5` above, which does follow `include!`, has equal counts (382/382). Logs `c5c-pos7-post3.log`, `c5c-posdiff7-post3.log` |
| formatting debt (`fmt.py`) | hunks original/new ed 42/42, ej 3/4, sc 16/13, ex 20/22; changed-line multisets identical in all four |
| census (`census-own.ts`, files / bytes / active lines, ratio vs 32 / 256 KiB / 4,000) | sc lib 11/103,254/1,558 (0.394), test 13/154,198/2,283 (0.588); ej 7/107,524/1,988 (0.497); ex lib 6/126,149/2,409 (0.602), test 10/209,410/3,927 (0.982, before 0.962); ed lib 10/135,502/2,072 (0.518), test 13/230,940/3,368 (0.881). Every closure inside the gate; ex test is the tightest (73 active lines of headroom). Log `c5c7-census-recheck.jsonl` |
| taxonomy | the 28 new dirs: `rgi.mjs` bad=0, `tax.ts` 28 dirs / 0 unresolved (live `semanticDirectoryKindId`); longest absolute path 178 chars |

## 3. Text readers

- `readers.py`: path hits for the layout roots are all module-tree aware or irrelevant: `source-access/🟦️.ts` lists `editor/🦀️.rs` and `export/🦀️.rs` in `POLICY_SPLIT_SOURCE_ROOTS` (read as a module tree by `policyReadModuleTreeSource`, so the retained-command scalar-config cohort law needs no change); the `ex`/`ed` hits in other plugins' fixtures name their own artifacts. No reader of `sc`, `ej`.
- Layout media-adoption corpus (`ex/🧪️tests/🟦️.ts`, fixture `🧫️fixtures/🔣️.json`, `🧪️testing/📥️source/🟦️.ts`): re-pointed by the predecessor to `readModuleTreeSource` and to the two editor children that now hold the adopter (`🕹️layout-play-app`) and the producer (`🎥️media-export-job-and-factories`). I re-checked the fixture by script (`c5c7-media-fixture-check.log`): all 13 negative bindings occur exactly once in their module tree, adopter implementation and method once.
- Edited by me: `LY/../../📦️packages/🦀️rust/📋️project.json`, target `test-media-adoption-source` inputs: the two stale root entries (`editor/🦀️.rs`, `export/🦀️.rs`) replaced by the two child sources the test now reads. JSON parses.
- Layout tool-job coverage probe (`tooljob-probe.ts`, logs `c5c-tooljob-layout-before/after.txt`): production sources 17,911 to 17,922, dispositions/proofs/app-owned rows equal, failures 1,321 before and after, `scan-then-monolith 0`.

## 4. Open risks / not verified

- `bun test` of the media-adoption corpus segfaults in this checkout (Bun 1.3.14, 2 GB RSS) and `bun ./📜️script.ts new crates … --check-applied` aborts with `Cannot find package 'ajv'`: workspace `node_modules` currently lacks `ajv` and the `@semio-tech/*` links (a concurrent framework layout move; `readModuleTreeSource` now lives under `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree`). Both are environmental, not caused by the split. The layout gate result I rely on is the predecessor's earlier run (`c5c-checkapplied-layout-after.log`: exit 0, `onlyCombined` 0 for native 2,491 and test 5,748, no sibling/admission/shared findings, base `dir:/Users/ueli/.cache/c5c/pre-layout`). The Python fixture check above substitutes for the media-adoption bun test; the coordinator should run `bun test` on `LY/⚙️engine/📤️export/🧪️tests/🟦️.ts` once the install is healed.
- `ts-media-probe.ts` in `c5c-tools` imports the module-tree reader by absolute path and needs the workspace links to run.
- Nothing compiled with cargo; `ed`/`ex` use cross-module `pub(super)` widenings chosen by `widen2.py` (not re-listed here).

## 5. Coordinator compile list

`cargo check -p semio-s-artifact-layout-layout-engine --lib --tests`
`cargo check -p semio-s-artifact-layout-layout-engine-json --lib --tests`
`cargo check -p semio-s-artifact-layout-layout-engine-export --lib --tests`
`cargo check -p semio-s-artifact-layout-layout-editor --lib --tests`
