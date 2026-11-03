# crate-budget D (wave 2)
Scripts: `🧰️scripts/crate-budget/{d_lib,d_en1992_checks,d_en1993_special,d_en1998_mutations,d_test_support,d_test_support_lock}.py`, `🧰️scripts/leaf-kinds/keyed.py` (derived mounts). No cargo/rustc run; compile-untested.

## 1. Source-boundary gate
- en1992-checks 284 KB -> 214 KB: the generated input vocabulary (`registry/📚️inputs`, 79 KB) is its own crate `en1992-checks-case-inputs`; registry re-exports it as `case_inputs`.
- en1993-checks-special 277 KB / 4171 lines -> 194 KB / 3029: eight independent check families (frame, storey, restraint, thin-walled, built-up, weld group, parametric, hollow joint) mounted by new `en1993-checks-special-frames` (85 KB, `⚖️checks/🕸️special-frames`; files stay in `🔗️special-element-checks`, entry fns `pub(super)` -> `pub`); special `pub use`s it, so `restraint_checks`, `frame_eigen` paths are unchanged.
- en1998: the owner (EN 1998 v5) split scene/inferences/mutations in parallel. Before the coordinator's note I had moved `demo_mutation_cases` (40 KB) plus the text/binary codec unit tests that consume it into `en1998-mutations-demo` (`🧬️mutations/🎲️demo-cases`); the `testkit` feature of io/io-text (only gated that fn) is removed everywhere. Kept (small, consistent); no further en1998 work.
- Re-scan: no crate over except others' areas: en1995-scene 263404 B (just crossed) and en1994-model 4001 lines (one line over).

## 2. serde `test-support`
Every model crate (14 besides en1992) has optional `serde`, `test-support = ["dep:serde"]`, and `cfg_attr(any(test, feature = "test-support"), ...)`. Crates whose source closure (lib + tests, incl. mounted sqlite/compliance/editor tests) calls `serde_json::{to_value,to_string,from_value,from_str,...}` got `<model> = { workspace = true, features = ["test-support"] }` in dev-dependencies (43 manifests; existing plain dev entries merged). Cargo.lock edges added by `d_test_support_lock.py`. Verified by the same closure scan: no crate with serde_json calls lacks the feature.

## 3. Leaf mounts
`keyed.create_crates` now unions the group's new kinds, the mounts already in the root and every unmounted leaf dir (with `↩️inverse`) matching the group's verb-entity prefixes (`derived_mounts`). Test: with the root cut to one mount, en1990 `bridge-sls-rail` derives all 8. Dry re-run of `en1990_run.py`: 0 new kinds, 8 mounts remain.

## Checks
`verify build-owners sync` passed (no changes); `check-cargo-authority.py` 28 crates, 0 errors. `- ready for rebuild: crate budget D` appended to build-en1990..1999/latest.md.

## Round 2 (coordinator request)
- en1994-model 4001 lines: `model_twin_leaves.py 🧩️en1994` (four facet twin crates, `SNAPSHOT_LEAVES` -> `<crate>::LEAF`).
- en1995-scene 263404 B: pure `facts` module (11 KB) -> `en1995-scene-facts` (`d_en1995_scene.py`, `pub use ... as facts`, imports via extern aliases).
- Re-scan: no OVER crate in the norm tree; `check-cargo-authority.py` 0 errors; `verify build-owners sync` passed.
- Caveat: `verify build-owners sync` passed after round 1; after round 2 it aborts with "Bounded physical Source revision required" inside the framework graph-consumer capture (`build/📥️inputs/💾️storage/🔧️custody`, 64 KiB per framework source file), i.e. a framework source file outside this task (not a norm file) changed concurrently. Re-run once that is fixed. `check-cargo-authority.py` passes (28 crates, 0 errors).
