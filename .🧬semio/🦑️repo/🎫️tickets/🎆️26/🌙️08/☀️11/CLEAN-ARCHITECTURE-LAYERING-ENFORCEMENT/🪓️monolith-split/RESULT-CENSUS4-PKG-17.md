# Result CENSUS4-PKG-17 (prefix `c4p17`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools and pre-edit snapshots: `F/c4p17-tools/` (`orig/<tag>.rs.txt` + `orig/SHA256`). Logs: `F/🗑️generated/c4p17-*` (nothing deleted). Scratch (stages, sandboxes): scratchpad `c4p17/`.

## 0. Verdict (wave 7, 2026-10-17 08:00)

**DONE (static evidence; real compile not run).** All 8 files of Package 17 are split and installed: 37 children in 37 new folders (largest 655 lines), parents 25 to 414 lines. Wave 7 re-verified the live tree against the `orig/` snapshots: `c4p17-tools/live-lossless.py` shows 0 missing lines in 8 of 8 files (after normalising the declared `pub(super)`/`pub(crate)` widening and the `../` include re-bases; polynomial file: only the 6 `pub mod x {` heads, 6 closing braces and 2 declared docstring-emoji lines are replaced), mounts OK (37 of 37 children mounted exactly once), include/`#[path]` resolution OK, rustfmt 45 of 45 parse, no `[DEBUG]` added. The full typeck oracle `c1` finished clean: lib 14385 vs base 14386, **NEW 0**; `--test` 20551 vs 20547, NEW 14 (all flagged stub-layer noise, the same 14 as the pre-edit baseline `a0`), privacy/trait-scope/resolve/type/other 0 (non-stub, new) on both configurations, canary `E0308 E0616 E0624` reached on both sides. The scoped patched differential (section 4) and `--check-applied` (5 of 5 exit 0, `onlyCombined` 0) were run on the final tree; none of my files changed after 06:28 (the only later source changes in the five artifacts are a peer's `mesh-edit` files). The `COMPILE ADMITTED` line was not in `📓️FLEET.md` at 07:50 and the compile mutex was held by a peer, so no `bun repo cargo` call was made; the commands for the compile queue are in section 7.

## 0a. Found state (first session)

- Start of session: no `c4p17-tools/`, no `🗑️generated/c4p17-*`, no earlier RESULT file. The 8 target files had exactly the census line counts (1952, 1132, 1073, 1032, 2447, 1090, 1278, 1513) and no sibling child folders: nothing was split yet. All 8 were snapshotted with SHA256 before the first edit.

## 0b. Found state (wave 7, 2026-10-17 07:27)

- The earlier record was current: all 8 parents are live at the recorded line counts (48, 160, 206, 25, 414, 64, 140, 139). The tools folder holds the splitter, installers, oracles and `orig/` snapshots (SHA256 recorded); the scratchpad stages are gone (not needed, the trees are installed).
- The last typeck oracle run `b3` (07:07) died while running `base/test` on `OSError: Too many open files in system` (machine-wide file table pressure from peers); `b3` cur lib 14385 vs base lib 14386 (1 error fewer, the fixed inner-attribute error), cur test 20551, base test lost. `b1`/`b2` had been killed deliberately (exit 143). No half-applied batch exists: nothing to finish or roll back.
- Open items in wave 7 (all closed, see the verdict): a clean full typeck oracle run (label `c1`), a fresh independent live losslessness / mount / rustfmt / include re-check against the snapshots, the `COMPILE ADMITTED` line check, and the verdict rewrite.

## 1. Status and verdict (kept current)

- Earlier status (superseded by section 0): **all 8 files are split and installed** (37 new child files in 37 new folders, the largest child 655 lines; parents 25 to 414 lines). Lossless 8 of 8, rustfmt 45 of 45 files parse (0 failures), 37 of 37 children mounted exactly once (0 duplicate `.rs` mounts), include/`#[path]` literals resolve to the same absolute paths as before, `lit.py` 19 sites before = 19 after (0 new), staged rustc differential equal for lib, `--test` and `wasm32-wasip2` in all 7 groups. Typeck oracle, live differential and `--check-applied` after: see section 4 (all finished).
- Real `bun repo cargo` was not run (compile admission is closed for the fleet in wave 6); the commands for the compile queue are in section 7.

## 2. What changed

Method: item-level splitter `c4p17-tools/c4p17.py` (descendant of the c4p8/c4p6 tool; new here: a dangling `///` doc line before a region marker travels with the following item, orphan `//#endregion` markers and plain comments between items stay in the parent, `keep_empty` regions, `patch_parent`), a fragment splitter for the test root (`c4p5-tools/fragsplit.py`) and a module extractor for the six inline modules of the polynomial file (`c4p17-tools/polysplit.py`). Every top-level item keeps its text. Children are `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use`/`pub(crate) use`/`use <snake>::*;` (re-export rank = the highest visibility of a named item in the child, so every public path is unchanged). Private definitions used by another file of the split became `pub(super)` (widening planner `widen2.py`, 86 sites in total, listed per file in the table). Region markers are carried into the children (balanced); empty region shells were pruned from the parents except the declared `🧪️UnitTests` shell.

| file | lines before | parent after | widened | children (lines) |
| --- | ---: | ---: | ---: | --- |
| `➗️equation/…/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs` | 1952 | 48 | 0 | `📏️univariate` 518; `🕸️multivariate` 364; `🧮️finite` 179; `🔍️factor` 369; `🌱️roots` 134; `🔢️algebraic` 340 |
| `🀄️wfc/…/🖼️bitmap/…/✏️editor/🦀️.rs` | 1132 | 160 | 12 | `🎛️command-and-action-bridge` 267; `🧵️retained-commands` 194; `📬️store-preparation` 155; `📝️artifact-editor` 388 |
| `🀄️wfc/…/🧊️3d/…/✏️editor/🦀️.rs` | 1073 | 206 | 9 | `🎛️command-vocabulary` 124; `🧵️retained-commands` 217; `🔗️graph-view-and-editor` 411; `🧰️examples-and-graph-edit` 146 |
| `🀄️wfc/…/🧊️3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs` | 1032 | 25 | 0 | `📜️contract-and-request` 161; `🧮️job-core` 389; `🔁️job-lifecycle` 315; `🏭️factory-and-solve` 170 |
| `🌀️procedural/…/🌀️generation2d/…/✏️editor/🦀️.rs` | 2447 | 414 | 63 | `🛰️owner-and-io` 178; `🧵️retained-commands` 436; `🎁️contributions-route` 198; `📬️store-preparation` 335; `🎞️reserved-import` 316; `🕹️play-app-editor` 534; `🖱️graph-interaction-and-render` 94 |
| `🌀️procedural/…/🌀️generation2d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 1090 | 64 | 0 (include mode) | `🧰️production-fixtures-and-maintenance-law` 207; `🎛️command-surface-laws` 382; `📜️manifest-and-cross-cutting-laws` 140; `📏️surface-budget-and-context-menu-laws` 72; `🔌️port-laws` 137; `📇️window-action-archive-and-genesis-laws` 106 |
| `🧊️generation3d/…/🚪️io/📝️text/🔣️value/🦀️.rs` | 1278 | 140 | 1 | `🧊️snapshot-and-catalogue-values` 511; `🧬️mutation-delta-and-canonical-values` 655 |
| `🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | 1513 | 139 | 1 | `🪟️window-addressing` 261; `🧊️preview-geometry` 579; `⏱️tick-and-resolve` 260; `📈️status-json` 305 |

Special cases and decisions:
- **Polynomial file** is six top-level inline modules (`pub mod univariate { … }` and five more). Each became a file module with the body dedented by 4 spaces (multi-line string literals are not touched), `include!("🧪️tests/…")` re-based by one `../`, the `// #region` markers of each module kept around its mount in the parent. Module identity is unchanged (same module paths, privacy and `crate::polynomial::…` aliases), so nothing is widened. Two inner docstrings changed their leading emoji to keep the docstring emoji unique per file (`📈️`>`📏️` univariate, `🌱️`>`🔢️` algebraic; the parent and `roots` keep theirs): the only two content-changed lines, declared in the lossless check.
- **Test root `🔬️unit/🦀️.rs`** is the module `editor_unit_tests` of the integration test crate `editor-routes`; it was cut with `include!("<folder>/🦀️.rs");` fragments (as PKG-15), so the 32 test functions keep their paths (`editor_unit_tests::<name>`), nothing is widened and the three relative `include_str!` literals got one more `../`.
- **`gen2d`**: the pre-existing mis-ordered pair `use semio_framework_job_publication::…;` then `#![allow(clippy::result_large_err)]` (an inner attribute after an item is a rustc parse error, also at `HEAD`) was corrected by swapping the two lines in the parent (multiset unchanged; `rustfmt` now parses the file). The `app_commands!` command enum, `prepare_command`, `create_generation2d_app`, the `examples`/`wasm` mounts and the test-module mount stay in the parent (real API and wiring). A dangling `/// 🧪️ Shared test scaffolding …` doc line (inside the empty `🧪️UnitTests` region) travels with `GraphInteractionMarks` because rustc attached it to that item.
- **Parents** keep their `use` blocks, `mod tests;` mounts, `#![allow]`, and the real API: `create_wfc3d_editor`/`wfc3d_action`, the bitmap `create_bitmap_editor`, `create_generation2d_app`, `Generation2dCommand`, `split_endpoint`/`dag_host_snapshot_to_workflow`/`export_meshes_from_evaluation` of preview-eval.
- Files outside the list that I touched (one-line consumer re-points, section 5): `lib-wiring-ratchet` fixture, the flow wasm unit test.

## 3. Data and tables (run-time fixtures)

No file was changed for this item. Measured over the 8 files (multi-line and large string literals, `const`/`static` spans of 15 lines or more, lines over 400 characters): the only data are the 19-line JSON request schema `WFC3D_INFERENCE_REQUEST_SCHEMA` (the published contract of `s.wfc.wfc3d.solve`, used by a `const` contract struct, so it must stay a compile-time `&str`) and four `*_TOOL_IDS` / `*_PUBLICATION_CONTRACTS` tables of 22 to 24 lines. They are production constants of wasm plugins (no file system at run time), too small to be fixtures; bytes are identical by construction (lossless multiset). No fixture created, no byte-identity proof due.

## 4. Verification (every command was run; results are the real ones)

Paths: tools in `F/c4p17-tools/`, logs `F/🗑️generated/c4p17-*`, stages in the scratchpad `c4p17/stage/<tag>`.

| check | command | result |
| --- | --- | --- |
| lossless line multiset | `python3 c4p17.py lossless <tag>` (6 mod-mode tags), `python3 polysplit.py lossless`, `fragsplit.py` REMOVED/ADDED report for the test root; `inst17.py <tag> verify` = live files equal the stage | **8 of 8 LOSSLESS OK, 0 missing lines** (poly: only the 6 `pub mod x {` heads and 6 closing `}` are replaced by `#[path]`+`pub mod x;`, plus the 2 declared doc-emoji lines; gen2dunit: the 3 `include_str!` lines are re-based). Extras are wiring (`#[path]`, `mod`, `use x::*`), child docstrings, `use super::*;` and carried region markers. Region marker open/close counts equal the original (`gen2d`: 14 open / 15 close both sides, the extra close is the orphan `//#endregion 🔖️Constants` of the original). |
| test names | regex over `#[test]` and `…async_test` attributes, orig versus stage | gen2dunit 32 before, 32 after, identical set; the other 7 files hold 0 tests (their tests live in separate `🧪️tests` files) |
| typeck oracle (pos, prefix c4p17, 11 crates: my 6 plus generation2d, generation3d, 3d-io, 3d-editor, 3d-viewer; base `b84bf526c7b`) | `c4p17-tools/run-oracle.sh <label> lib,test` = `POS_PREFIX=c4p17 python3 pos-tools/pos.py run c4p17-tools/families/c4p17.json --cfgs lib,test --label <L>` | baseline `a0` (before the first edit): lib 14386 = 14386 base, **NEW 0**; test 20552 vs 20547 base, NEW 14 / GONE 9, all 14 flagged stub-layer noise (HEAD-versus-working-tree peers, inherited), canary `E0308 E0616 E0624` reached on both sides. After the final tree: `b1`/`b2` were killed (exit 143) and `b3` died on `Too many open files in system`; **`c1` (wave 7, `c4p17-tools/run-oracle.sh c1 lib,test`, log `c4p17-oracle-c1.log`): lib cur 14385 vs base 14386, NEW 0, GONE 1; test cur 20551 vs base 20547, NEW 14 (all stub-layer noise, same count as `a0`), GONE 10; privacy/trait-scope/resolve/type/other 0 (non-stub, new) on both; canary reached both sides** |
| rustc resolution differential (`+nightly-2026-07-20 --emit=metadata -o /dev/null`, sandbox with the original restored, neutralised externs and globs) | `c4p17-tools/run-diff.sh <label> lib,test,p2` = `oracle.py <group> lib,test,p2`; `stage1` = staged split, `live` = `C4P17_LIVE=1` installed tree | stage1: value 319/411/319, preview 396/445/396, gen2d 635/679/635, gen2dunit 681/1045/681, wfcbitmap 1566/2335/1566, wfc3d (with wfchost) 1551/2683/1551, equation 1602/2920/1602 errors **equal on both sides for lib / test / wasm32-wasip2, only-new 0**. Two benign differences: value has one extra `unused import: snapshot_and_catalogue_values::*` warning (the sandbox neutralises the globs the other modules of the real crate use through the re-export), and gen2dunit `--test` lists the same missing fixture `preview-eval-run.json` under a longer relative path (`include_str!` re-based; `incl_check.py` proves both resolve to the same file). `live` (installed final tree, `C4P17_LIVE=1`, logs `c4p17-diff-live.log` and `-live2.log`): the same numbers in all 7 groups and 3 configurations, **only-new 0**, except generation2d: 635>634 lib, 679>678 `--test`, 635>634 wasm32-wasip2 (the one error fewer is the fixed `an inner attribute is not permitted in this context`; `live2` re-ran gen2d and gen2dunit after that swap and the renames). |
| E0451 scan | `POS_PREFIX=c4p17 python3 rpv-tools/lit.py c4p17-tools/families/c4p17.json pre|post` | **19 sites before, the same 19 after, 0 new**; all 19 are `Symbol` literals in the equation crate (pre-existing, not in my files); the module tree grew from 514 to 545 files |
| private declarations used across files | `python3 defscan.py <tag>` x6 | 0 leftovers (15, 7, 4, 23, 1, 10 private declarations scanned) |
| include and `#[path]` resolution | `python3 incl_check.py stage|live` | all `include!`, `include_str!`, `include_bytes!` and `#[path]` literals resolve to the same absolute paths as in the originals (orig literals 28, new 28 after removing the own child mounts) |
| rustfmt | `python3 fmtcheck.py` (`rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --config-path <repo>`, parse and `--check` hunks) | **45 of 45 files parse, 0 failures**; rustfmt hunks 118 before + 40 for the gen2d snapshot with its inner attribute moved up (the unmodified snapshot did not parse) = 158, after 154 (not worse) |
| paths | `incl_check`/path script over the 37 children | longest repo-relative path 224 UTF-8 bytes (limit 240), longest absolute path 185 characters (limit 256) |
| duplicate mounts | `python3 mounts.py`, `python3 ttc-tools/dupmounts.py '✏️s/🔌️plugins/<artifact>/'` x5 | **37 of 37 children mounted exactly once, 0 missing mount targets, 0 duplicate `.rs` mounts**; the only duplicates reported (generation2d 8, generation3d 12) are `📦️packages/🦀️rust` and `#[path = "."]` directory targets that existed before and name none of the new folders |
| folder names | `bun c4p5-tools/rgi.mjs` and `bun c4p17-tools/tax.ts` (live `semanticDirectoryKindId`) | 37 of 37 RGI emoji + U+FE0F, 0 unresolved in the taxonomy; a script checked that every emoji is an existing repo folder prefix (`rf8-folder-emojis.tsv`), differs from every sibling folder, equals the docstring emoji of its file and is not the first docstring emoji of any other `.rs` file in the same subtree (after renaming `graph-view-and-editor`, `owner-and-io`, `contributions-route` and the play app, whose emoji clashed) |
| census inside the gate | `c4p17-tools/census.sh before|after` (canonical `readRustSourceClosureV1`, native, wasm32-unknown-unknown, wasm32-wasip2, lib and cfg(test)); `censuscmp.py` | see section 4a |
| `new crates <pkg> --check-applied --base=f856dff0813` | `c4p17-tools/checkapplied.sh before|after` | before: 5 of 5 exit 0, `onlyCombined` 0 (equation native 1340>1345 combined, test 2616>2622; wfc-bitmap 1383>1388 and 2135=2135; wfc-3d 1348>1353 and 2462=2462; generation2d 11=11 both; generation3d 52=52 both). **After: identical, 5 of 5 exit 0, `onlyCombined` 0 for native and `--test` in every artifact** (the first run of wfc3d, gen2d and gen3d died on a peer's in-flight deletion of `policy.json`, `Cannot find module`; `checkapplied-retry.sh` re-ran them: wfc3d on its second attempt, the other two on the first; logs `c4p17-checkapplied-after-*.log`, `c4p17-checkapplied-after-retry.exit`) |
| lib wiring budget | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` in `📚️library` | 1 pass, 2 fail: all failures are other roots (lowpoly engine +1 line, a mesh-edit file +1, the layout json export +19 lines, and 6 stale norm and puzzle pins); **neither generation2d's editor root (2448>415, pin lowered) nor preview-eval (1514>140, pin removed) appears** |
| source-text readers | `bun test ./🟦️.ts -t "original preview payload forwards"` in the flow wasm tests, `rd.ts` | reads the module tree now; it failed before and fails after on the same first `toContain` (generation2d, not my file); for generation3d the field string is found in the tree (true), the `to_value` string is absent before and after |
| other repo tests | `bun test ./🧪️tests/🔏️path-emoji-statutes/🟦️.ts` in `📚️library` | 10 of 37 fail on stdio-tsv, graph-manifest and normalization inputs; none of the 37 new folder names appears in the output (`c4p17-pathemoji.log`); foreign |
| `[DEBUG]` | count over the 45 live files versus the 8 snapshots | none added: 1 occurrence now, 1 in the original (`gen2dunit` line 912, an `eprintln!("[DEBUG] intrinsic Gen2D params …")` inside a test, now in the `🔌️port-laws` fragment); it belongs to the original and was left untouched |

### 4a. Census inside the gate (native; wasm32-unknown-unknown and wasm32-wasip2 are identical, checked)

| crate | closure | workspace | files | bytes | active lines | worst ratio | gate |
| --- | --- | --- | --- | --- | --- | --- | --- |
| mathematical-equation | lib | ➗️equatio | 163 > 169 | 848564 > 841766 | 14783 > 14789 | 5.094 > 5.281 | **over (was over before)** |
| mathematical-equation | test | ➗️equatio | 305 > 311 | 1328929 > 1322131 | 20545 > 20551 | 9.531 > 9.719 | **over (was over before)** |
| procedural-2d-editor | lib | 🌀️generat | 4 > 11 | 159565 > 162305 | 2478 > 2543 | 0.620 > 0.636 | inside |
| procedural-generation2d | lib | 🌀️generat | 1 > 1 | 5896 > 5896 | 84 > 84 | 0.031 > 0.031 | inside |
| procedural-2d-editor | test | 🌀️generat | 9 > 16 | 193783 > 196523 | 2997 > 3062 | 0.749 > 0.765 | inside |
| procedural-generation2d | test | 🌀️generat | 1 > 1 | 5896 > 5896 | 84 > 84 | 0.031 > 0.031 | inside |
| wfc-bitmap | lib | 🖼️bitmap | 115 > 119 | 527657 > 528893 | 8689 > 8725 | 3.594 > 3.719 | **over (was over before)** |
| wfc-bitmap | test | 🖼️bitmap | 220 > 224 | 844594 > 845830 | 13278 > 13314 | 6.875 > 7.000 | **over (was over before)** |
| wfc-3d | lib | 🧊️3d | 139 > 147 | 537563 > 539917 | 8800 > 8867 | 4.344 > 4.594 | **over (was over before)** |
| wfc-3d | test | 🧊️3d | 280 > 288 | 911202 > 913556 | 13854 > 13921 | 8.750 > 9.000 | **over (was over before)** |
| procedural-3d-io | lib | 🧊️generat | 1 > 1 | 45574 > 45574 | 640 > 640 | 0.174 > 0.174 | inside |
| procedural-3d-io-text | lib | 🧊️generat | 21 > 23 | 105937 > 106747 | 2001 > 2031 | 0.656 > 0.719 | inside |
| procedural-3d-preview-eval | lib | 🧊️generat | 4 > 8 | 162289 > 163349 | 2390 > 2425 | 0.619 > 0.623 | inside |
| procedural-generation3d | lib | 🧊️generat | 1 > 1 | 59416 > 59416 | 582 > 582 | 0.227 > 0.227 | inside |
| procedural-3d-io | test | 🧊️generat | 1 > 1 | 45574 > 45574 | 640 > 640 | 0.174 > 0.174 | inside |
| procedural-3d-io-text | test | 🧊️generat | 29 > 31 | 181986 > 182796 | 2338 > 2368 | 0.906 > 0.969 | inside |
| procedural-3d-preview-eval | test | 🧊️generat | 8 > 12 | 242192 > 243252 | 2738 > 2773 | 0.924 > 0.928 | inside |
| procedural-generation3d | test | 🧊️generat | 1 > 1 | 59416 > 59416 | 582 > 582 | 0.227 > 0.227 | inside |

- The wasm32-unknown-unknown and wasm32-wasip2 closures equal the native ones in every row (0 differences). Command: `c4p17-tools/census.sh before|after` then `python3 c4p17-tools/censuscmp.py`; raw rows `🗑️generated/c4p17-census-before.jsonl`, `-after.jsonl`, comparison `c4p17-census-compare.txt`.
- **Inside the gate (32 files / 262,144 bytes / 4,000 active lines) before and after:** `2d-editor` (files 4>11 lib, 9>16 test, ratio 0.62>0.64 and 0.75>0.77), `3d-io-text` (21>23 lib, 29>31 test; worst ratio 0.906>0.969, binding = files), `3d-preview-eval` (4>8, 8>12; ratio 0.62>0.62 and 0.92>0.93, binding = bytes), `generation2d`, `generation3d`, `3d-io` unchanged. `value` was cut into two children (not five) on purpose: five would have taken the `3d-io-text` test closure to 34 files, over the limit.
- **Not inside the gate, before and after (not caused by this split):** `semio-s-artifact-mathematical-equation` (163>169 lib files, 305>311 test; ratio 5.09>5.28 and 9.53>9.72), `semio-s-artifact-wfc-bitmap` (115>119, 220>224; 3.59>3.72, 6.88>7.00) and `semio-s-artifact-wfc-3d` (139>147, 280>288; 4.34>4.59, 8.75>9.00). These three artifact-root crates still mount the whole artifact (115 to 163 files) and were already refused; the planned remedy is the `new crates` carve, which `RESULT-PLUGIN-WAVE0-GENERATORS.md` 16.5 lists as blocked for wfc-3d (snapshot crate 106 files). The split adds 4 to 8 files and 1 to 3 percent to the file ratio of each. Bytes of equation went down (848,564 > 841,766) because the six module bodies are dedented. No crate went from inside to over the limit.

## 5. Consumers and fixtures re-pointed (one-line edits outside the package list)

| # | File | Change |
| --- | --- | --- |
| 1 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | pin of `generation3d/…/🧵️preview-eval/🦀️.rs` removed (root 1514 > 140 lines, now under the 150-line budget; the test "ratchet lists no root that complies" would fail otherwise); pin of `generation2d/…/✏️editor/🦀️.rs` lowered 2448 > 415 (pinned roots only shrink) |
| 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts` | the test "original preview payload forwards its actual retained caller authority" reads the preview-eval roots through `readModuleTreeSource` (import added) because `retained_grant: …RetainedCloneGrant` now lives in the `⏱️tick-and-resolve` child; the test fails before and after on its first `toContain` for generation2d (not my file), verified separately that the generation3d tree contains the field string |

Searched with `git grep -F` for every moved file path and for `include_str!`, `readFileSync`, `readModuleTreeSource`, `moduleRustSource` readers in the five artifacts: no other reader of the 8 files exists. `📋️build-owners.json`, `Cargo.toml` and `📋️project.json` name crates only.

## 6. Foreign errors and hand-offs

1. The three artifact-root crates (equation, wfc-bitmap, wfc-3d) are far over the source-boundary gate (section 4a). Only the `new crates` carve can fix that.
2. `generation2d/…/✏️editor/🦀️.rs` had a rustc parse error at `HEAD`: a `use` item before the inner attribute `#![allow(clippy::result_large_err)]`. Fixed here by swapping the two lines (the differential shows exactly one error fewer in lib, `--test` and `wasm32-wasip2`).
3. The flow wasm test "original preview payload forwards its actual retained caller authority" fails on a string (`("retained".to_string(), …to_value(&retained_grant))`) that neither `generation2d` nor `generation3d` preview-eval contains; foreign, pre-existing.
4. `lib-wiring-budget` has 2 failing tests from other roots (lowpoly engine, generation3d mesh-edit, layout json export grew; 6 stale pins in the norm and puzzle artifacts).
5. A peer deleted `⚡️caching/…/🛠️preparation/🔣️policy.json` in a working-tree edit while my `--check-applied` ran (`Cannot find module …policy.json` in `🗂️workspaces/🦀️cargo/🟦️.ts`); I retry in a loop (`checkapplied-retry.sh`), see section 4.
6. Machine load was 40 to 77 during the whole run; the oracles took 10 to 45 minutes each.

## 7. Real-check commands for the compile queue (not run)

Run from the repository root, one launcher call at a time under the fleet compile mutex (`🗑️generated/cargo-mutex.lock`):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path '<artifact>/Cargo.toml' <-p list> --lib --keep-going --message-format=short
```

| artifact | `<artifact>` | `<-p list>` |
| --- | --- | --- |
| equation | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation` | `-p semio-s-artifact-mathematical-equation` |
| wfc bitmap | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap` | `-p semio-s-artifact-wfc-bitmap` |
| wfc 3d | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d` | `-p semio-s-artifact-wfc-3d` |
| generation2d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d` | `-p semio-s-artifact-procedural-2d-editor -p semio-s-artifact-procedural-generation2d` |
| generation3d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d` | `-p semio-s-artifact-procedural-3d-io-text -p semio-s-artifact-procedural-3d-io -p semio-s-artifact-procedural-3d-preview-eval -p semio-s-artifact-procedural-generation3d` |

Then the same calls with `--tests` instead of `--lib` (for generation2d also `-p semio-s-artifact-procedural-2d-editor --test editor-routes`, the integration crate that mounts the six unit fragments as `editor_unit_tests`), and with `--target wasm32-wasip2 --lib`. After a green check run `test` on the same selection: the test paths must equal the pre-split ones (`editor_unit_tests::<name>` for the 32 editor-route laws; `<polynomial module>::<name>` for the six included polynomial test fragments, whose module paths did not change). Expected foreign errors: the equation, wfc-bitmap and wfc-3d crates exceed the source-boundary gate (section 4a), so the launcher refuses them before the compiler runs; nothing in this split changes that.

## 8. New folders (37, each with one `🦀️.rs`; prefixes relative to `F`'s repo root)

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (file modules):
  - `🎛️command-and-action-bridge`
  - `🧵️retained-commands`
  - `📬️store-preparation`
  - `📝️artifact-editor`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (file modules):
  - `🎛️command-vocabulary`
  - `🧵️retained-commands`
  - `🔗️graph-view-and-editor`
  - `🧰️examples-and-graph-edit`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/` (file modules):
  - `📜️contract-and-request`
  - `🧮️job-core`
  - `🔁️job-lifecycle`
  - `🏭️factory-and-solve`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/` (file modules):
  - `🛰️owner-and-io`
  - `🧵️retained-commands`
  - `🎁️contributions-route`
  - `📬️store-preparation`
  - `🎞️reserved-import`
  - `🕹️play-app-editor`
  - `🖱️graph-interaction-and-render`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/` (include fragments):
  - `🧰️production-fixtures-and-maintenance-law`
  - `🎛️command-surface-laws`
  - `📜️manifest-and-cross-cutting-laws`
  - `📏️surface-budget-and-context-menu-laws`
  - `🔌️port-laws`
  - `📇️window-action-archive-and-genesis-laws`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔣️value/` (file modules):
  - `🧊️snapshot-and-catalogue-values`
  - `🧬️mutation-delta-and-canonical-values`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/` (file modules):
  - `🪟️window-addressing`
  - `🧊️preview-geometry`
  - `⏱️tick-and-resolve`
  - `📈️status-json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/` (file modules):
  - `📏️univariate`
  - `🕸️multivariate`
  - `🧮️finite`
  - `🔍️factor`
  - `🌱️roots`
  - `🔢️algebraic`

## 9. Files

Edited (8 parents, rewritten in place): the eight files of the table in section 2; created: the 37 children of section 8. Edited consumers (2): section 5. Tools and snapshots: `F/c4p17-tools/` (`c4p17.py` splitter, `polysplit.py`, `mkspec.py` specs, `specs/*.py`, `specs/gen2dunit.fs.json` for `c4p5-tools/fragsplit.py`, `inst17.py`/`reinst17.py` installers, `widen2.py`, `rsitems.py`, `defscan.py`, `testnames.py`, `incl_check.py`, `mounts.py`, `fmtcheck.py`, `oracle.py` + `run-diff.sh`, `run-oracle.sh` + `c4p17-pos-run.py`, `census.ts`/`census.sh`/`censuscmp.py`, `checkapplied.sh`, `checkapplied-retry.sh`, `tax.ts`, `families/c4p17.json`, `orig/*.rs.txt` pre-edit snapshots with `orig/SHA256` and `orig/PATHS.tsv`). Logs: `F/🗑️generated/c4p17-*` (nothing deleted). Install journal: `F/🗑️generated/c4p17-installed.json` (first install only; `reinst17.py` replaced the folders of `wfc3d` and `gen2d` after four emoji renames and the `#![allow]` swap). Rollback: copy `c4p17-tools/orig/<tag>.rs.txt` back over the eight targets (`orig/PATHS.tsv` maps tag to path) and delete the 37 folders of section 8; revert the two consumer edits of section 5.
