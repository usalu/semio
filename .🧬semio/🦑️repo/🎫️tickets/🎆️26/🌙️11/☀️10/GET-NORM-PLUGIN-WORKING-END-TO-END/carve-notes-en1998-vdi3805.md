# Carve notes: en1998-schema-formulas and vdi3805-inferences (2026-11-25)

Both flags are confirmed by the real closure reader (`real-closure-run.sh`, `readRustSourceClosureV1` with rustc `--test` cfg), test unit `lib[cfg(test)]`:

| Crate | Before (files / bytes / active lines) | After original | After new crate |
| --- | --- | --- | --- |
| semio-s-artifact-norm-en1998-schema-formulas | 27 / 299076 / 4601 (over: bytes, lines) | 15 / 260377 / 3864 | 13 / 40642 / 778 |
| semio-s-artifact-norm-vdi3805-inferences | 35 / 229466 / 4166 (over: files, lines) | 27 / 205184 / 3898 | 9 / 24720 / 277 |

Build lib units stay as before (en1998 14 / 259649 / 3841, vdi 26 / 204681 / 3883).

## en1998-schema-formulas
Cause: six hand-recomputation test files (assessment-state, soil-structure, critical-region, isolation, liquefaction-hazard, design-case catalogue incl. 7 embedded TSV fixtures) were mounted in the lib's cfg(test) closure on top of a build closure that already used 99.2% of the bytes.
Split: new tests-only crate `semio-s-artifact-norm-en1998-schema-formulas-worked-examples` at `📐️formulas/🧪️tests/🔬️worked-examples/`. Its lib root `#[path]`-mounts the six existing test files (not moved) inside wrapper modules that re-export the module under test (`pub use ...formulas::<module>::*`) and replicate the private imports the tests got through `use super::*`. The original removes the six `#[cfg(test)] #[path] mod tests;` mounts. The two inline test modules (spatial-variability) stay.
Files: changed `📐️formulas/{🏚️assessment-state,🪨️soil-structure,🔩️critical-region,🛞️isolation,🫧️liquefaction-hazard,📇️catalogue}/🦀️.rs`; created `📐️formulas/🧪️tests/🔬️worked-examples/🦀️.rs` and `.../📦️packages/🦀️rust/Cargo.toml`; registered in `🫨️en1998/Cargo.toml` (member + workspace dependency) and `🫨️en1998/📦️packages/🦀️rust/📜️script.ts`.
Counts (static, real closure, multiset of test fn names): before 47 fns + 1 `entry_tests!` invocation; after 1 + 46 fns + 1 `entry_tests!` invocation (original + new). Sorted name lists identical (`diff` empty).

## vdi3805-inferences
Cause: the 263-line unit test mounted by the lib root plus its 7 embedded TSV fixtures pushed the test closure to 35 files and 4166 lines.
Split: new tests-only crate `semio-s-artifact-norm-vdi3805-inferences-laws` at `💡️inferences/🧪️tests/⚖️laws/`; root re-exports the inferences crate (`pub use`), repeats the imports the test took from the lib root and `#[path]`-mounts `../🔬️unit/🦀️.rs` unmoved (so its `include_str!` fixtures resolve unchanged). The original drops its `mod tests;` mount; the small outline test stays.
Files: changed `💡️inferences/🦀️.rs`; created `💡️inferences/🧪️tests/⚖️laws/🦀️.rs` and `.../📦️packages/🦀️rust/Cargo.toml`; registered in `🏭️vdi3805/Cargo.toml` and `🏭️vdi3805/📦️packages/🦀️rust/📜️script.ts`.
Counts: before 13 fns; after 2 + 11 = 13; sorted name lists identical.
The real build attempt `confirm-vdi3805-inferences.log` ended `EXIT 1` before reaching this crate's admission (see blocker); the refusal was confirmed by the direct reader only.

## Blocker observed
Every `repo cargo` run (my confirm runs and the en1998 test run, `EXIT 1`) fails at admission with
`Rust source 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs: module dbg has no single matching file`:
line 6 of that file (changed 09:18 by another agent) mounts `.../🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs`, whose ticket folder no longer exists. Not touched by this slice.
