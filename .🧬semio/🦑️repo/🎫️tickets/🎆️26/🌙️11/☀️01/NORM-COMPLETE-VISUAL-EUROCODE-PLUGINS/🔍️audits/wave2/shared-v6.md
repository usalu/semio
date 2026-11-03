# Shared v6 (wave 2)

Nothing compiled (no cargo). Python run: none needed. "ready for rebuild: shared v6" appended to integrate/latest.md and build-en1993/en1995/en1999.

## 1. Rust oracle adapters
- New shared file `norm/🧮️mechanics/🎯️oracle-wiring/🦀️.rs`: `mechanics.analyse-section` (outer, holes, nu) to `analyse_section`, `mechanics.analyse-thin-walled` (walls) to `analyse_thin_walled`, all `SectionConstants` quantities under the python field names (moduli only for nu = 0 and FE). Mounted by `#[path]` in the en1993 `oracle-wiring` (ADAPTERS 36) so there is no new crate dependency or Cargo.lock edit.
- `section-mechanics.json` moved to the en1993 leaf fixtures (ids are en1993-prefixed); en1993 `oracle-comparison` feature has the Examples row; the pytest law reads it from the artifact case list. The mechanics `drive` now reproduces it in `evaluators_reproduce_the_blessed_oracle_cases` and the comparison projection.
- Modal, buckling and frame second order: Python cases (`en1998.4.3.shear-building-modal`, `en1999.6.3.1.flexural-buckling`, `en1999.6.3.2.ltb`, `en1994.6.4.ltb-restrained`, `en1993.5.2.alpha-cr-eigen`, `mechanics.lateral-frame`, portal, beam, LM1) all already have Rust adapters; no gap found.

## 2. Combination track
- Helper `projection_kit::tracks::{COMBINATION_TRACK, combination_track, push_combination_track}` (dedupes captions, none below two), law `test_support::assert_combination_track_law(scene, n)`, unit test in projection-kit.
- Wired: en1995 (`facts::combination_captions` over `enumerate_combos`) and en1999 (`tracks::combination_captions` over `uls_combinations`); en1990 and en1991 already emit. Other codes have no document-level combination list. Law test in en1995 and en1999 unit tests; two existing track-count asserts adjusted. Goldens need re-bless.

## 3. Viewers
`step_view` in app-surface view-steps (pure, read-only) with test `a_viewer_steps_checks_and_combinations_without_any_edit`. Viewer apps cannot keep the lane: no host transient slot (request to framework recorded).

## 4. Interpreter resolution
The three spawns use `python_oracle::command()` (single resolution, already in compliance-core; both crates dev-depend on compliance with `compliance-testing`).

## 5. Requests and warnings
445 done, 449 obsolete; the rest listed with reasons in requests.md (section Shared v6). integrate/4 warnings (17, unused qualifications in the framework value-control contract) fixed.

## Risks (unverified by rustc)
Mounted adapter file uses `serde_json::Value` indexing; `ViewLane: PartialEq` in the new test; `combo.kind` match exhaustiveness in en1995 facts; `tracks::combination_captions` visibility from the en1999 unit test child module.
