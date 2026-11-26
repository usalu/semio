# Framework test crates: tests-06 / tests-09-2

## Root cause of `artifact_app_laws` unresolved (tests-06)
Not a rename. `🧪️tests/🔬️tool-run/🦀️.rs` and `🧪️tests/🧪️time-travel/🦀️.rs` imported `semio_plugin_testing_laws::artifact_app_laws` behind `#[cfg(any(test, feature = "artifact-app-testing"))]`. tests-06 is a test-only crate; built as a plain lib (as a dependency, or by `cargo test -p` on a crate whose lib target is also built non-test) the gate was off and 14 uses failed.
Fix: removed the cfg gate on both imports (only tests-06 mounts them). Result: `cargo test -p semio-plugin-tests-06` passes (2 passed, 0 failed), and it now compiles as a dependency of tests-07/07-2/09/09-2.

## tests-09-2 (pre-existing, caused by peers, NOT fixed)
After the tests-06 fix `semio-plugin-tests-09-2` (lib test) still has ~175 compile errors from the part-5 contract laws mounted across the tests-05 crate boundary:
- `ContractApp` tuple fields private (fixed: made `pub`), `#[cfg(test)]`-only helpers of the instance crates (`test_history`, `test_dispatch_emit`, `test_snapshot`, `test_capture_presence_roots`, `test_interaction_selection_snapshot`) invisible from another crate, private fields (`peer_roster_publications`, `maintenance_stage`, `presence_store`), `OriginalTurnFailure.message`, changed signatures (several methods now take more args).
- Fixed on the way: duplicate `artifact_app_laws` import in `📨️owned-child-dispatch`, missing imports in part-5 root (`ReferenceSpaceMember`, `ARTIFACT_LIVE_OUTPUT_SLOTS` (still unresolved: not exported by `semio_plugin_ownership`; defined in `🏛️ownership/🦀️.rs`), `TestRuntimeApps`, `TestMemberStoreProducer`), `dispatch_addressed_fixture_on_original` now lives in `semio_plugin_testing_instance_laws` (part-4 and part-5 call sites + dependency in tests-09-2 Cargo.toml).

## Law `a_refused_interaction_verb_does_not_poison_the_next_verb`
NOT run: it lives in tests-09-2 (`part-5/🎚️selection-modes-verbs-and-hover`) whose crate does not compile (it also uses `app.0`/`app.1`, now fixed to pub). Needs the peer refactor of the cfg(test) helper exposure (e.g. a `testing` feature on instance crates) before the whole crate builds.

Logs: `🗑️generated/ft/` (law.log, t06.log, chk6.log).
