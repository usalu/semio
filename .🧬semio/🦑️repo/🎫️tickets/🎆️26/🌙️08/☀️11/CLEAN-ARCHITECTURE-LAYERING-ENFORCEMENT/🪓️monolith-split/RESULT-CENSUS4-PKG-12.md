# Result CENSUS4-PKG-12 (prefix `c4p12`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. Tools: `F/c4p12-tools/`. Logs: `F/🗑️generated/c4p12-*`. Baseline = `HEAD b84bf526c7b`.

## 0. Verdict (kept current)

**DONE** (static; real compile queued). All 8 files are split into 58 cohesive children in 58 new folders (largest child 465 lines, parents 55 to 292 lines). On the live tree, re-run in wave 7 (07:3x to 08:2x): losslessness 8 of 8, per-file region balance, rustfmt parse 66 of 66, every child mounted exactly once, test-fn and fn-name multisets equal, private-item scan 0 real leftovers, taxonomy admission 58 of 58, path length max 232 absolute bytes, census 0 over the gate and identical to the 07:01 run, typeck differential (label `w7`) with 0 non-stub new rows and, after normalising the moved module path, only peer-edit residuals. `--check-applied` for cad, layout and en1990 exits 0 with `onlyCombined` 0. The norm plugin has no artifact package, so the gate does not apply there (exit 1, usage message). The launcher compile is queued: there is no `COMPILE ADMITTED` line in `📓️FLEET.md` yet (section 5).

## 1. Stale first record (kept for the history; found state after the second restart, 2026-10-17 00:2x)

- Targets (`F/c4p12-tools/files.txt`, 8 files): cad `🔺️diff` is layout (1), cad picking (2), cad editor lib root (3), cad editor unit test (4), cad inferences (5), norm compliance (6), norm app-surface (7), en1990 inferences (8).
- **No source file had been split**: no child folder exists next to any of the 8 files; `F/🗑️generated/c4p12-*` does not exist; `F/c4p12-tools/specs/` held only an empty `__init__.py` (the first attempt died while the toolchain was being prepared).
- Toolchain left by the first attempt (kept and reused): `c4p12.py` (item-level splitter, descendant of c4p8), `widen2.py`, `rsitems.py`, `defscan.py`, `oracle.py`, `lst.py`, `files.txt`, and `orig/{1..8}.rs` with `orig/SHA256`.
- Live tree against the snapshots: files 1, 2, 3, 5, 6, 7, 8 are byte-identical to `orig/<n>.rs`. **File 4 (cad editor unit test) drifted**: a peer edited it at 00:16 after the 00:15 snapshot (7 lines: new `UNBOUNDED_STEP_GRANT` const and `*_retained` locals at 4 `StepContext::new` call sites for the retained-clone grant migration; 2526 -> 2533 lines). Not my edit. The snapshot is re-taken from the live file before staging, and the install refuses to overwrite a file that differs from its snapshot.
- Files 1, 3, 4, 7 also differ from the `HEAD` blob (peer edits staged or unstaged in the working tree); the working tree is the baseline, not `HEAD`, for those.
- Roll-back needed: none. Roll-forward: specs are written from scratch against the snapshots and installed.


## 2. Found state (wave 7, after the third restart at about 07:05)

- The section 1 text above is stale. Before the cut the previous executor had finished the whole apply batch: all 8 files are split and installed (parents of 55 to 292 lines, children of 56 to 465 lines, 58 child folders), `regs.py apply` was applied (two split roots registered in `POLICY_SPLIT_SOURCE_ROOTS`, the callers corpus source re-pointed to the editor child, the cad dev script reads the editor through `readModuleTreeSource`, the lib-wiring ratchet pins lowered or removed), and the census-after (07:01) and per-file lossless logs (07:00) exist.
- The oracle run labelled `pre` (06:55, log `c4p12-oracle-pre.log`, cur = split tree, base = HEAD sandbox) finished with 0 non-stub new rows. The run labelled `post` (07:07) died with `OSError: Too many open files in system` (machine file table exhaustion) and wrote no report. It was repeated as label `w7` (section 4).
- Live tree against the snapshots `c4p12-tools/orig/{1..8}.rs`: the line multiset check (`c4p12.py lossless <tag> <live dir>`) reports `LOSSLESS OK` for all 8 files, so no peer edit landed in these trees after the install.
- Roll-back needed: none. Roll-forward: re-verify (rustfmt, mounts, paths, taxonomy, definition scan), repeat the oracle, run the `--check-applied` gates, then rewrite this verdict.

## 3. Package files and result

`F` = this folder. Roots live under `✏️s/🔌️plugins/`. Line counts are file lines (before = the working-tree snapshot in `c4p12-tools/orig/`).

| # | File | Lines before | Parent after | Children (lines) |
| --- | --- | ---: | ---: | --- |
| 1 | layout `🔺️diff` | 1,497 | 104 | `🧱️grid-text-and-guide-deltas` 188, `🗂️data-entries-delta` 110, `🖼️page-frames-delta` 131, `🩹️row-patches` 393, `📍️positional-algebra` 115, `🧮️diff-apply-and-algebra` 197, `🧩️patchable-frames` 315 |
| 2 | cad `🧲️picking` | 1,389 | 55 | `🏷️pick-vocabulary` 296, `🪣️geometry-buckets-and-curves` 390, `🎯️pick-targets-and-indices` 217, `🎚️visibility-and-toggles` 228, `👁️reveal-and-selection-merge` 135, `🎨️target-style` 111 |
| 3 | cad editor lib root | 1,710 | 292 | `⌨️command-from-action` 117, `🧵️retained-command-tools` 292, `📤️media-export-job` 166, `🔧️config-store-preparation` 131, `🗄️artifact-store-preparation` 154, `🪟️editor-and-interaction-scope` 390, `🧭️app-manifest` 231 |
| 4 | cad editor unit test | 2,533 | 131 | `🧰️command-fixtures-and-retained-lanes` 374, `🌲️forest-mesh-and-schema` 280, `🖼️rendered-scene-and-manifest` 267, `👁️view-model` 202, `⚙️operations-and-file-effects` 141, `🎮️engagement-and-import` 193, `⏪️history-and-convergence` 262, `🧩️contributions` 182, `🎚️engagement-commit-and-render-probes` 168, `🧬️geometry-media-and-owners` 401 |
| 5 | cad `💡️inferences` | 1,084 | 147 | `🔄️derive-transformation` 339, `🔍️construct-query` 140, `🖥️scene-compute` 464 |
| 6 | norm `⚖️compliance` | 1,129 | 114 | `📐️quantities-clauses-and-text` 121, `✅️check-results` 390, `📜️annex-and-tables` 100, `🏗️design-situation-and-shared` 175, `🏛️norm-error-and-family` 171, `🔤️op-text-and-pack-codec` 106 |
| 7 | norm `🖥️app-surface` | 2,360 | 144 | `🛡️store-ownership-macros` 72, `🏷️ids-chrome-and-labels` 155, `🧭️value-path` 255, `🔖️field-meta` 123, `✏️edit-rules` 276, `🌳️report-tree` 233, `📝️document-editors` 219, `📚️catalogue-and-inspection` 220, `🪟️manifest-and-media` 139, `⌨️command-args-and-dispatch` 87, `🏭️command-and-tool-job-macros` 177, `🧵️retained-commands` 357 |
| 8 | en1990 `💡️inferences` | 1,128 | 144 | `🧰️report-helpers` 94, `🏗️ultimate-limit-state` 245, `📏️serviceability-and-fatigue` 161, `🌍️working-life-and-bridge-annexes` 247, `🧮️partial-factor-sets` 75, `📋️compliance-evaluation` 56, `⛓️referential-integrity` 170 |

Children are mounted from the parent with `#[path = "<child>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (the crate's public API is unchanged; a few children whose items were never public use a plain `use`). Test companions stay in the parent (`#[cfg(test)] #[path = "🧪️tests/…"] mod tests;`). The cad inference file already held three inline modules (`derive_transformation`, `construct_query`, `scene_compute`); they became file modules in place (same module identity), and their two `include!` fragment paths gained one `../` level. Private definitions that a sibling child names were widened to `pub(super)` / `pub(in …)`; function bodies are unchanged. File 4 (cad editor unit test) was re-snapshotted after a peer edit at 00:16 (retained-clone grant migration), so its baseline is the working tree.

## 4. Verification (all on the live tree)

| Gate | Command | Result |
| --- | --- | --- |
| Lossless | `python3 c4p12-tools/c4p12.py lossless <tag> <copy of the live tree>` for the 8 specs (copies in the scratchpad) | `LOSSLESS OK` 8 of 8: missing 0; extra lines are only child docstrings, `use super::*;`, `#[path]`/`mod`/`pub use` wiring, region markers and the two declared `include!` path edits |
| Live structure | `python3 c4p12-tools/verify.py` | `VERIFY OK`: 58 of 58 children mounted exactly once (0 duplicate mounts, 0 missing targets), `fn` name multiset and `#[test]` count equal to the snapshot per file (e.g. 148 fns in app-surface, 100 fns and 13 tests in the cad unit test), every child folder carries U+FE0F, rustfmt parses 66 of 66 files (`rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout`; a deliberately broken file returns rc 1, so the check detects failures) |
| Region markers | inline stack check of every file (parents, children, snapshots) | 0 unbalanced files: each `//#region` has its `//#endregion` with the same label in the same file; totals equal (e.g. app-surface 14 to 20 pairs, every extra pair is a repeated balanced pair in a child) |
| Private items | `python3 c4p12-tools/defscan.py <tag> <copy>` | 0 cross-file leftovers for 7 specs; 3 for the cad inferences (`handle`, `normal`, `centroid` fields of a struct inside `derive_transformation`) which are false positives: the struct stays in the same module as before (inline module lifted in place), and the names occur elsewhere only as unrelated identifiers |
| Taxonomy | `bun c4p12-tools/tax.ts c4p12-tools/newdirs.txt` | `dirs 58, unresolved []` through the live `semanticDirectoryKindId` |
| Path length | `verify.py` | longest file 232 absolute bytes, 204 repo-relative bytes (limits 256 chars / 240 bytes) |
| Census | `zsh c4p12-tools/census.sh w7` (canonical `readRustSourceClosureV1`; budget 32 files, 262,144 bytes, 4,000 lines) | 0 of 48 rows over the gate; byte-identical to the 07:01 run `c4p12-census-after.txt`; see the table below |
| Typeck differential | `PYTHONHASHSEED=0 ulimit -n 65536; zsh c4p12-tools/run-oracle.sh w7` (pos oracle, family `c4p12` = 7 split crates plus 14 dependents, cfg lib and test; log `c4p12-oracle-w7.log`, exit 0) | lib: cur 5180 vs base 5257 errors, test: cur 7133 vs base 7271; `privacy`, `trait-scope`, `resolve`, `type`, `other` all 0 (non-stub, new); the typeck canary E0308, E0616, E0624 is reached on both sides. NEW 70 / GONE 147 (lib), NEW 70 / GONE 208 (test) are all stub-involved rows. I re-compared the final diagnostic dumps after normalising the moved module path (`…::row_patches::X` to `…::X`) and restricting to the 7 split crates: lib 2105 vs 2104 rows, test 2874 vs 2873; the whole residual is 4 rows against 3 in each cfg, all from peer edits that predate my snapshot (below) |
| `--check-applied` cad | `LC_ALL=en_US.UTF-8 bun ./📜️script.ts new crates "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust" --check-applied --base=HEAD --sandbox=<scratchpad>` via `c4p12-tools/checkapplied.sh w7` | exit 0; 36 crates both sides; native 2459 = 2459, `--test` 3224 = 3224, `onlyCombined` 0 / 0, admission, shared and sharedForeign empty |
| `--check-applied` layout | same, `📏️layout/🗿️artifacts/📏️layout` | exit 0; 17 crates both sides; native base 2584, combined 2501 (fewer), `--test` base 6055, combined 5759, `onlyCombined` 0 / 0; admission and shared empty. The combined side is smaller because of the peer layout-drawing decoupling in the working tree; `healedBase` lists peer-owned layout export files |
| `--check-applied` en1990 | same, `📕️norm/🗿️artifacts/⚖️en1990` | exit 0; 15 crates both sides; native 508 = 508, `--test` 762 = 762, `onlyCombined` 0 / 0 |
| `--check-applied` norm | same, `📕️norm` | exit 1, `verifyAppliedLayout: … has no artifact package`: the norm plugin crates (`compliance`, `app-surface`) are not an artifact package, so the gate does not apply; their closure is exercised by the en1990 gate (15 crates) and the differential |

Residual rows of the differential (2 NEW and 1 GONE per cfg in `layout-diff`, 2 and 2 in `cad-editor`):
- cad editor: `begin_demand` E0599 on `CadConfigStorePreparationFactory` and `CadArtifactStorePreparationFactory`, identical in both sides except the printed type path (`__m_…::Type` against bare `Type`).
- layout diff: the baseline reports `E0600 unary ! on ()` for `if !set_drawing_text(...)`; the current side reports `E0599 map_err on ()` and `MutationApplyError::new` at the same place. The cause is a peer edit, not the split: `git show HEAD:…/🔺️diff/🦀️.rs` line 1055 has `if !…set_drawing_text(…) {`, while the working-tree snapshot `c4p12-tools/orig/1.rs` already has `let edited = …set_drawing_text(…).map_err(…)?; if !edited {` and drops the `SemioDrawingSnapshot` import (layout drawing decouple). Both sides are stub-layer noise (`set_drawing_text` is a stubbed external).

Census (files / bytes / active lines), before (06:17) to after (live):

| Crate | lib | test |
| --- | --- | --- |
| layout-diff | 5 / 72,211 / 1,484 to 12 / 74,492 / 1,547 | 6 / 83,061 / 1,669 to 13 / 85,342 / 1,732 |
| cad-picking | 1 / 67,004 / 1,388 to 7 / 68,828 / 1,437 | 2 / 112,342 / 2,099 to 8 / 114,166 / 2,148 |
| cad-editor | 4 / 131,291 / 1,727 to 11 / 133,746 / 1,797 | 5 / 131,651 / 1,735 to 12 / 134,106 / 1,805 |
| cad-inferences | 10 / 91,179 / 959 to 13 / 88,344 / 968 | 14 / 99,648 / 1,303 to 17 / 96,813 / 1,312 |
| norm-compliance | 1 / 42,677 / 1,090 to 7 / 44,546 / 1,144 | 4 / 57,155 / 1,358 to 10 / 59,024 / 1,412 |
| norm-app-surface | 1 / 120,158 / 2,356 to 13 / 123,751 / 2,465 | 4 / 173,249 / 3,181 to 16 / 176,842 / 3,290 |
| en1990-inferences | 6 / 59,578 / 1,196 to 13 / 62,360 / 1,267 | 10 / 93,053 / 1,940 to 17 / 95,835 / 2,011 |

Native, wasm32-unknown-unknown and wasm32-wasip2 rows are identical. No closure is over the gate before or after (largest: app-surface test 3,290 of 4,000 lines; cad-picking and cad-inferences test are the binding file-count rows at 8 and 17 of 32). The cad inference lib closure has fewer bytes after (91,179 to 88,344) because lifting the three inline modules into files removed their indentation.

## 5. Consumers and readers

- Source-text readers (applied by `c4p12-tools/regs.py apply`, verified present in the tree): the CAD editor and norm app-surface roots are registered in `POLICY_SPLIT_SOURCE_ROOTS` (`📖️source-access/🟦️.ts`, lines 173 and 174), so the tool-job source readers follow the mounted children; the callers corpus entry (`📋️callers.json`) now points at the editor child `🪟️editor-and-interaction-scope/🦀️.rs` that holds the artifact editor impl; the CAD dev script (`🧑‍💻dev/📐️cad/…/📜️script.ts`) reads the owner through `readModuleTreeSource`; the lib-wiring ratchet pins of the 7 roots were removed or lowered (`📏️lib-wiring-ratchet/🔣️.json`).
- Doc pointers re-pointed this session (comment-only, the line numbers had already drifted): `♾️infinite/🌍️world/🌉️bridge/🖌️overlays-and-gumball/🦀️.rs` (`🧲️picking/🎨️target-style/🦀️.rs`), `♾️infinite/🌍️world/🗃️model/🧬️scene-records/🦀️.rs` (`🧲️picking/🏷️pick-vocabulary/🦀️.rs`) and `🌍️world3d/🦀️.rs` (`🧲️picking/` folder).
- Facet leaves: `cad_artifact_inference_descriptor` and the en1990 equivalent keep `include_str!("🦀️.rs")` of their own root, as every other split inference root in the repo does; the leaf now carries the parent only. No test or reader in the repo compares that text to the file.
- Other literal readers (`include_str!`, `readFileSync`, `?raw`, `#[path]`) of the 8 files: none besides the ones above (searched with `git grep -F` per path in `✏️s/🔌️plugins/📐️cad`, `📏️layout` and `📕️norm`; the other hits are different files such as `🚪️io/…/🔺️diff` or the `wfc`/`writer` copies).
- The baseline sandbox of the oracle is `HEAD b84bf526c7b`; differences between `HEAD` and the working tree in files 1, 3, 4 and 7 are peer edits (section 3), not mine.

## 6. Limits, queue and foreign errors

- No `bun repo cargo` call: `📓️FLEET.md` has no `COMPILE ADMITTED` line (checked at 08:1x) and the compile mutex `🗑️generated/cargo-mutex.lock` was held by `coord-probe2` (07:31). The static proof is the typeck differential above, which covers name resolution and (through the canary) type and privacy errors on the non-stub side; it does not cover coherence or trait selection against the real externals. Command for the compile queue (one batched call under the mutex of `📓️WAVE5-BRIEF.md`):
  `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-layout-layout-diff -p semio-s-artifact-cad-cad-picking -p semio-s-artifact-cad-cad-editor -p semio-s-artifact-cad-cad-inferences -p semio-s-plugin-norm-compliance -p semio-s-plugin-norm-app-surface -p semio-s-artifact-norm-en1990-inferences --lib --tests --keep-going --message-format=short --manifest-path <workspace Cargo.toml>`
  with one call per workspace manifest (the crates are in four cargo workspaces): `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml` (layout-diff), `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml` (cad-picking, cad-editor, cad-inferences), `✏️s/🔌️plugins/📕️norm/Cargo.toml` (norm-compliance, norm-app-surface) and `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/Cargo.toml` (en1990-inferences).
- The first `post` oracle run died with `OSError: Too many open files in system` (machine file table exhaustion), which is why `ulimit -n 65536` is set for all long runs; `git` printed `daemon terminated` during several read-only calls (fsmonitor noise, unrelated).
- Foreign errors: none attributable to the split. Foreign observations only: the sibling `📦bounds` / `📦️packages` emoji pair in the cad inference folder predates the package; HEAD and the working tree differ in the layout diff root (peer edits, see section 4).

## 7. Files

- Edited (8 roots): the files in the table of section 3.
- Created (58 children, one `🦀️.rs` per new folder): the folders in the table of section 3, next to their root.
- Edited outside the package (all by `regs.py apply` before the cut): `📖️source-access/🟦️.ts`, `📋️callers.json`, `🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/📜️script.ts`, `📏️lib-wiring-ratchet/🔣️.json`; this session: the three doc comments of section 5.
- Tools, kept: `F/c4p12-tools/` (`c4p12.py`, `specs/`, `mkspec.py`, `regs.py`, `verify.py`, `defscan.py`, `oracle.py`, `run-oracle.sh`, `checkapplied.sh`, `census.sh`, `census.ts`, `tax.ts`, `orig/` snapshots, `families/c4p12.json`).
- Logs, kept in `F/🗑️generated/`: `c4p12-oracle-{pre,w7}.log` and the `c4p12-c4p12-*` dumps, `c4p12-lossless-*.log`, `c4p12-census-{before,after,w7}.txt`, `c4p12-checkapplied-w7-*.log`.
