# FEM 2D core crates: test state (agent a2core)

Env: native cargo, `CARGO_TARGET_DIR=C:\t\2dc`, `CARGO_BUILD_BUILD_DIR=C:\t\2dcb` (switched from `2d-a` per coordinator; the old dirs were deleted by someone mid-session, which cost two runs with spurious `E0463 can't find crate`), `-j 6`, debug 0. Scripts: `a2core-env.sh`, `a2core-test.sh <name> <cargo args>`; logs `🗑️generated/a2core-*.log`.

## Result (last full runs: `a2core-all8.log` for six crates, `a2core-all13.log` for the main crate lib)

| Crate | Result |
| --- | --- |
| semio-s-artifact-fem-2d-io | 10 / 10 pass (incl. `surface_corpus_exports_obj_and_stl...` run with `SEMIO_FEM_SURFACE_RESULTS` set) |
| semio-s-artifact-fem-2d-model | all green (cold_reference_graph 5, dialect_conformance 3, inference_default_laws 2, original_fem_receiving 4, structure_laws 3, lib) |
| semio-s-artifact-fem-2d-mutations | green (58 + kind suites) |
| semio-s-artifact-fem-2d-results-state | 10 / 10 |
| semio-s-artifact-fem-2d-viewer | 5 / 5 |
| semio-s-artifact-fem-2d (lib, sqlite) | 7 / 8 pass (`a2core-all13.log`); 1 open, see below |

`original_fem_receiving` byte mismatch is gone with the removed framework debug print (no change needed).

## Fixes (all under `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d`, standards path `🏅️standards/🔖️1/🪆️subsets/🌐️any`)
Retirement of sealed snapshots dropped in tests:
- `🚪️io/🧪️tests/🔬️unit/🦀️.rs` (wrapper `from_dsl_text` returning `Fem2dSnapshotRetiring`, surface test no longer retires by hand), io `Cargo.toml` dev-dep model with `testkit`.
- `📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs` (inference laws), `🧬️schema/💡️inferences/🧪️tests/🔬️default-laws/🦀️.rs`, `🚪️io/🧪️tests/🔬️dialect-conformance/🦀️.rs` (explicit `retire_decoded`, these model integration tests have no testkit).
- viewer: both test files + dev-dep model `testkit`.
- mutations `🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `assert_owned_document_*_round_trip` and `detach_backbone` (mirror of the 3D store test).
- results-state `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🔬️unit/🦀️.rs`: publication close now driven through `semio_framework_value_clone_source_testing::drive` (the `testkit::granted` effect has no original producer invocation, hence "ticket close requires its original producer invocation"); dev-dep in `🎚️config/📦️packages/🦀️rust/Cargo.toml`.

Product fixes:
- `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: `retire_sqlite_snapshot` now retires through `semio_s_fem_receiving::cold::retire` (the default `drop(self)` panics on sealed lists, "sealed source requires original granted handback"); dep `semio-s-fem-receiving` added to `📦️packages/🦀️rust/Cargo.toml` (main crate). Mirrors 3D.
- `🧬️schema/📸️snapshot/receive-record.rs`: support DOF lists received with `enum_words` (authored order, repeats admitted, like 3D) instead of `enum_list`; the sqlite fixture deliberately stores `Tx, Tx, Ty, ...` and the model keeps the list as authored.

Test corrections:
- Viewer mesh-edge test was not a product bug: scenes are encoded as spine plus paged lanes, so the test must assemble through `artifact_app_laws::built_surface_scene` (the 3D twin does the same); layers were never empty in the product.
- sqlite tests: `native_decode_actual_interior_owned_work` extends the fixture nodes (`bulk{n}`) instead of replacing them (elements referenced the replaced nodes); bun test expects `authored_id` equal to the fixture's first node id (was `''`); `every_word` only writes positive finite section values (the model guard `section_plausibility` requires them), the declaration test skips non-finite words (the receiving record requires finite numbers; non-finite words stay covered by the direct round trip test).

## Open / notes
- `sqlite_snapshot_fem2d_native_decode_actual_interior_owned_work` (main crate lib, identical test in 3D): the decode is cancelled in the interior of the DecodeNative phase and the harness closing step (`original-snapshot-owners` `decode_native`, line 82) then fails with "original decode recipient closes: ... native receiving original type, field or phase is invalid"; the run also takes about 200 s. This is the same message as the open 3D `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication` and the snapshot-tests cancel-prefix items: a framework receiving/cancel-close defect, not a 2D fixture issue, root cause not found here.
- `surface_corpus_exports_obj_and_stl...` panics by design when `SEMIO_FEM_SURFACE_RESULTS` is unset (it is a producer for the TS contract command); under plain `cargo test` it fails. The TS agent owns the orchestration; recommend the test return early when the variable is absent.
- The `semio-s-artifact-fem-2d-editor` lib was briefly uncompilable (`with_original_receiving` bounds, another agent's in-progress edit of `🎛️play-app-editor`); resolved by the time of the last run.
