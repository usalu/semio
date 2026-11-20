# fem3d native test failures (model lib, 3d test targets)

## 1. `semio-s-artifact-fem-3d-model` lib: 3 failures -> 5/5 pass
Root cause: test code, not framework retirement and not the return budget. `SealedShared::drop` (value/arc-custody/sealed) debug-asserts that the last owner was retired explicitly ("sealed source requires original granted handback"). `Fem3dSnapshot` holds `nodes` and `materials` as `SealedShared<PersistentList<..>>`, and three tests dropped a `Fem3dSnapshot::default()` without retiring it:
- `schema::inferences::component::tests::inference_default_law` (`💡️inferences/🧪️tests/🔬️unit`)
- `schema::inferences::bounds::component::tests::inference_default_law` (`📦bounds/🧪️tests/🔬️unit`)
- `io::binary::snapshot::semio_protocol_conformance::verify_protocol_bytes_against_encoded_pack`
Fix: each test now binds the snapshot and `std::mem::forget`s it after the assertions (the model crate has no retirement fixture dependency; the empty default owns only one Arc).

The staged `origin_prior.is_none()||self.renewable()` in `🧵️job/📬️outcome/🪙️budget` is NOT involved: the model crate does not link the job/return-budget code, and the three panics are the Drop law above. It remains a separate suspect for the job test `original_return_place_keeps_foreign_busy_...` (see `📓️job-return-place-tests.md`); not touched, not re-measured here.

## 2. 3d workspace test targets (`cargo check --tests --keep-going`)
Compiling now (verified): `fem-3d-io` lib tests, `fem-3d-viewer`, `fem-3d-editor-interaction` (gumball), and all of `fem-3d-snapshot-tests` except the last two items below were not re-verified (disk floor).
Changes:
- io tests (`🚪️io/🧪️tests/🔬️unit`, concrete-forest and house example tests): `SealedShared<String>` compared via `.as_str()`; node table zip via `.nodes.iter()`.
- gumball test: `close_step` -> `close_step_cold` (the disposer now takes an effect; the cold variant funds its own wallet).
- viewer unit test: `fem3d_demo_snapshot()` returns `Result`, `.expect(..)`.
- snapshot-tests crate (`🧪️tests`): decode/encode work recipients are now `OriginalWorkLoan`s. Added `Authorities::work_loan` (test-support, `value/🧪️testing/📥️admission/driven.rs`); the four test files wrap their bodies in `Authorities::new().work_loan(|loan| ..)` and pass `&mut *loan`. Fuel is `u128`; list access is `.nodes.get().get(i)`; retirement loops use `semio_s_fem_receiving::testkit::{retire,granted}` and `close_retirement_recipient(4096, effect)`; derive path is `semio_framework_value::RetireOwned`; `crate::standards` -> `semio_s_artifact_fem_3d_model::standards` in roundtrip tests.
- Deleted the stale duplicate `🧪️tests/cold-tests.rs` (+ its `mod`): it mutates a sealed list via `cold_dismantle`, which a `SealedShared` handle no longer allows; the current cold coverage lives in `🧬️schema/📸️snapshot/cold-tests.rs` (compiles).

## Not done / open
- Last verified state of the snapshot-tests crate had 3 remaining errors (identity-index u128 multiply, cold-tests); both are edited (u128 cast, cold-tests removed) but NOT recompiled: free disk fell to 3.8 GB (floor 6 GB), build dirs deleted. Re-run `cargo check --tests` in the 3d workspace.
- `fem-3d-editor` test targets are extensively stale beyond the four named APIs and were left untouched: `🔬️app`/`🔬️app-2` (`edit::` alias, `fem3d_entity_target`, `FEM3D_RETAINED_TOOL_IDS`, private command struct imports, `SealedShared` vs `String` in command unit tests, `Arc<ViewModel>`), `🪟️window-config-ownership` (`WindowConfigPack.files` -> `.source`, `load_window_config_pack` removed, same drift in the 2d twin), `🪧️pane-smoke` (`owner.step` -> `step_into`), `🔬️unit` (`assert_registered_presence_round_trip` replaced by `assert_presence_round_trip(source, peer, ..)`). These need the owners of the command/panel restructure.
- Nothing run after the final edits: no before/after for 3d test targets beyond compile state; model lib verified 5/5 after the fix.
