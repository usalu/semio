# Source boundary carve 2026-11-25

Limits (policy `maxSourceFiles` 32, `maxSourceBytes` 262144, `maxActiveLines` 4000, no crate lowers them): not raised, no test dropped, ignored or thinned.

## Step 1: confirmation
Method: `real-closure-run.sh` (with `real-closure-jobs.py`, `real-closure-probe.mjs`) calls the repository's own `readRustSourceClosureV1` on every unit entry with the real rustc cfg (`rustc --print cfg [--test]`) plus the crate features, reading the files from disk. It reproduces the admission closure reader; it does not run `admitCargoSourceBoundariesV1` itself (needs a cargo graph). Real build attempts (`confirm-*.log`) all died earlier on a foreign blocker (below), so no refusal was confirmed by a build.

| Crate | Test unit before (files / bytes / lines) | Verdict |
| --- | --- | --- |
| en1992-checks | 25 / 336545 / 4097 | confirmed (bytes, lines) |
| en1993-checks | 21 / 312208 / 5329 | confirmed (bytes, lines) |
| en1994-editor | 38 / 185326 / 2376 | confirmed (files) |
| en1998-schema-formulas | 27 / 299076 / 4601 | confirmed (bytes, lines) |
| en1995-kernel | 18 / 298301 / 3749 | confirmed (bytes) |
| en1993-editor | 36 / 143972 / 2050 | confirmed (files) |
| vdi3805-inferences | 35 / 229466 / 4166 | confirmed (files, lines) |
| en1997-checks-catalogue | 21 / 260304 / 3539 | false flag (99.3% bytes) |
| en1999-scene | 15 / 256257 / 3540 | false flag |
| en1994-scene | 14 / 251471 / 3959 | false flag (lines 98.9%) |
| en1993-model | 5 / 254861 / 3402 | false flag |
| en1991-mut-rail-load-groups | 31 / 47454 / 769 | false flag (31 files) |
| semio-s-plugin-norm-app-surface-live-editor | 16 / 253052 / 3526 | false flag |

Build (non-test) units of all 13 are under the limits, also with all features. Active lines (4000) also count and were the binding limit for several crates; the audit report only ranked files and bytes.

## Step 2: carves (test unit after; every unit of every touched crate checked ok, `🗑️generated/closure-after.txt`)
Pattern: a tests-only sibling crate `#[path]`-mounts the existing test files unmoved and reaches the crate under test through `pub use <crate>::*`, so `crate::` paths in the tests keep resolving; the original crate drops the mounts. Oversized `include_str!` fixtures became run-time `fixture_text!` (semio-framework-test-fixtures, as in the en1995 precedent). Each new crate is registered in the family `Cargo.toml` (member, workspace dependency) and `📦️packages/🦀️rust/📜️script.ts`.

| Crate | After original | New crate (test unit) | What moved |
| --- | --- | --- | --- |
| en1992-checks | 22 / 242274 / 3955 | `-checks-reach-and-trace` 3 / 7934 / 151 | reach and spec-trace suites; demo-document JSON read at run time |
| en1993-checks | 18 / 233706 / 3795 | `-checks-laws` 4 / 78707 / 1541 | compliance and special-wiring suites |
| en1994-editor | 29 / 172542 / 2088 | `-editor-laws` 12 / 17354 / 357 | window, mode, panel, command unit suites; shared fixture app to `🧪️tests/🧰️context` mounted by both |
| en1993-editor | 27 / 129751 / 1747 | `-editor-laws` 12 / 19089 / 376 | same |
| en1998-schema-formulas | 15 / 260377 / 3864 | `-schema-formulas-worked-examples` 13 / 40642 / 778 | six worked-example suites |
| en1995-kernel | 17 / 245459 / 3750 | none | demo-document JSON read at run time |
| vdi3805-inferences | 27 / 205184 / 3898 | `-inferences-laws` 9 / 24720 / 277 | the inference unit suite |

Headroom is thin for en1998-schema-formulas (260377 of 262144 bytes) and en1992-checks (3955 of 4000 lines); they should not grow.

## Step 3: missing include_str target
`semio-s-artifact-norm-en1993-checks-core`: `📇️catalogue/🧪️tests/🔬️unit/🦀️.rs` had ten `include_str!("../../table-8-N.json")` that resolve to `📇️catalogue/` while the files live in `📇️catalogue/🧪️tests/` (the real reader refuses with `process.source.missing-input`). Fixed to `fixture_text!("../../../📇️catalogue/🧪️tests/table-8-N.json")` (dev-dependency added). Fixing the path alone would have pushed that unit to about 339 KB, so the run-time read is also what keeps it at 16 / 238873 / 3286.

## Test counts before and after
Static (test attributes incl. `async_test` plus test-generating macro invocations in the cfg-active closure; `real-closure-tests.sh`; before cannot be built because admission refuses; names in `🗑️generated/names-before.txt` and `names-after.txt`):

| Crate | Before | After (original + new) |
| --- | --- | --- |
| en1992-checks | 30 | 22 + 8 = 30 |
| en1993-checks | 147 | 62 + 85 = 147 |
| en1994-editor | 55 fns + 4 macros | 32 + 23 = 55 fns, 4 macros |
| en1993-editor | 54 fns + 4 macros | 31 + 23 = 54 fns, 4 macros |
| en1998-schema-formulas | 47 fns + 1 `entry_tests!` | 1 + 46 = 47 fns, 1 macro |
| en1995-kernel | 4 | 4 |
| vdi3805-inferences | 13 | 2 + 11 = 13 |
| en1993-checks-core | 23 fns + 1 macro (after the include fix) | unchanged |

## Observed test runs
(none yet: every `repo cargo` run is refused at admission by a foreign leftover, `mod dbg` in `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs` mounting a deleted `debug-ledger.rs`; `carve-run-all.sh` retries all eight runs in order, logs `🗑️generated/run-<name>.log`.)

## En1993 package router: two calls
`runArtifactRustPackageMain` (provider in `⚡️caching/📦️artifacts/🦀️rust/🟦️.ts`) builds a `ScriptRouter` and runs the requested segment (default `test`) on every call, and the `test` script runs `runArtifactRustPackagesTests` over `options.packages`. A second call therefore repeats the whole test run with its own list, so the two calls were not harmless. The committed version already had two calls with different lists (176 vs 182 packages; the first lacked the second's `mut-joint-plate`-style crates, the second lacked the first's `mut-fastened-joints*`, `mut-fatigue-stresses*`, `mut-buckling-lengths*`, `mut-forming-gains`, `mut-material-ductility`, `mut-member-warping-more`, `showcase`), and I had put `editor-laws` into the first only. `🔩️en1993/📦️packages/🦀️rust/📜️script.ts` now has one call with the union (195 distinct names, all of them workspace dependencies in `🔩️en1993/Cargo.toml`, the snapshot-sqlite tails were identical). The other five scripts (en1992, en1994, en1995, en1998, vdi3805) have exactly one call each and list their new crate.

## Static compile check of the new crates (`static-path-check.py`)
Every `crate::`/`super::` path and every unqualified name the moved test files use, checked against `pub` (not `pub(crate)`) declarations in the closure of the crate under test, plus the private root imports of the original crate:
- en1992-checks-reach-and-trace: `registry`, `En1992Cases`, `evaluate_member/anchor/material_cases`, `entity_cases::reach::*`, `fire_field`, `spec_trace` are `pub`; `ndp` and `f_bd` reach the root through `pub use specials::*` -> `resistances` (`pub use core::*`, `pub use detailing::*`). The tests used the root's private imports `AnnexChoice` and `ndp_is_verified`; the new root imports both. No other private root import is used.
- en1993-checks-laws: `clauses`, `frame_eigen`, `joint_*`, `combine_member_actions`, `psi_factors`, `uls_rows`, `AnnexParams`, `part_1_*` etc. are declared `pub` in the dependency crates re-exported by `pub use ..._special::*` / `member_checks::*`; both suites import everything else explicitly; none uses the root's private imports.
- vdi3805-inferences-laws: `part_10/19/66/100` are `pub mod` from a macro re-exported by `pub use parts::*`; the root repeats the compliance imports and `model::*` the test took from the old root; `attributes_from_records` and the sheet types come from the model glob. The old root's other private imports (`clause`, `subject_*`, `ANNEX`, ...) are not used by the test.
- en1998-schema-formulas-worked-examples, en1993-editor-laws, en1994-editor-laws: no used name is declared non-`pub` in the closure; editors use only `pub` items (`Host`, `En1993PlayApp`, `En1993Family`, `VARIANT`, `CONTROLLER_ID`, `modes`, `panels`, `commands`, `demo_session`, `create_en199N_app`); the root unit suite, which uses private `live_entities`/`live_handles`, stays in the editor.
Limit of this check: it is textual (name present as `pub` in the closure), not a compiler; glob-import ambiguities, trait-in-scope needs, feature-gated items and macro-expanded paths (`incremental_laws!`, `entry_tests!`) are only settled by the real build.

## Run-time fixture reads: dependence on cwd and CARGO_MANIFEST_DIR
`fixture_text!`/`fixture_bytes!`/`fixture_path!` expand `env!("CARGO_MANIFEST_DIR")` at compile time at the call site, so the path is an absolute constant of the crate that compiles the file; none depends on the current directory. All the new reads depend on the manifest directory of the crate that compiles the test file:
- en1992-checks (registry demo-document JSON, 5 levels up), en1995-kernel (demo-document JSON, 4 up), en1993-checks-core (ten `table-8-N.json`, `../../../📇️catalogue/🧪️tests/`): the file is compiled by the crate whose manifest dir is used; all resolve on disk now (checked).
- en1993-checks-laws: the pre-existing `fixture_path!("../..")` in the compliance suite is manifest-relative too; moving it to the laws crate changed the depth from five to seven levels (resolves to the family `✳️any` root, checked).
- The editors' asset test (`env!("CARGO_MANIFEST_DIR")/../../../🖼️assets`) was not moved. Mounted tests that kept `include_str!` (vdi3805 TSVs, en1998 TSVs, demo-session) resolve relative to their own file, independent of the crate.
Consequence of run-time reads: cargo no longer tracks those files as inputs, so editing them does not rebuild the test binary, and a cached binary embeds an absolute path of the workspace it was built in.

## en1992-checks: `is_case_entry` (test uncompilable at HEAD)
The first private-copy compile (`cp-carve-en1992.log`) failed with E0425 `cannot find function is_case_entry` in `⚖️checks/🗄️registry/🗂️cases/🧪️tests/🔬️unit/🦀️.rs` lines 7 and 64. The function has no definition anywhere in HEAD or the tree, so this unit test did not compile before the carve either (my edit there only swapped `include_str!` for `fixture_text!`). Derivation: line 7 asserts `demo cases == entries.filter(is_case_entry).count()`, line 64 asserts every entry has a demo case or is not a case entry, and `🧷️entity-cases/📍️reach` classifies every registry entry as `Route::Case`, `Entity`, `Check` or `View`. So a case entry is exactly an entry whose route is `Route::Case`. Implemented as `pub fn is_case_entry(entry_id: &str) -> bool` with a docstring in `🗄️registry/🗂️cases/🦀️.rs`, next to the catalogue it describes (re-exported by the existing `pub use cases::*`). Not run yet; the sibling test `an_entry_is_a_case_or_an_entity_check_never_both` treats a `View` entry as "neither" and may fail at run time on its own merits.

## Undefined-name check
`undefined-callee-check.py` lists every free-function call in the test closure of all 14 touched crates that no source in `✏️s` or `🧰️framework` defines or imports (the `is_case_entry` class). After the fix it reports nothing but `cfg_attr` noise. No test file left in an original crate refers to a moved test module (checked for en1992 `reach_tests`/`spec_trace_tests`, en1993 `compliance_tests`/`special_wiring_tests`, vdi3805 and en1998 `tests`).
