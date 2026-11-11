# composition-publication (+ document-media partial)

## semio-plugin-composition-publication: lib and tests compile (0 errors)
Checked with `check -p semio-plugin-composition-publication --lib` (en1990 workspace) and `--tests` (from /home/deck/semio root).
- Root `🦀️.rs`: `poll_step` helper (pending poll -> zero-progress retry); `PendingChildGroupPublication::close_step_controlled(effect, control) -> OriginalRetirementPoll`; controlled steps via `plugin_effect_close`; `PendingArtifactStorePublication::close_step(effect)` and `close_step_controlled(effect, control)` return `OriginalRetirementPoll`; `retirement_turn(effect)`.
- `🤝️group/🦀️.rs`: `advance_original_lane` uses `with_native_receiving(&mut scope, |native,_|..)` and checks the scope refusal.
- `🤝️group/📦️owner/🦀️.rs`: `advance_inputs(.., effect)` (metadata issuer is effect-based), origins and failure close via `plugin_effect_close`, `begin_granted(effect)` and `step_granted(body, effect)`.
- `🤝️group/⚠️failure`: new `VcsError::OriginalRetirement` projection (kind, message, view, key).
- Test helper `test_original_pending_child_group_closure` runs through `OriginalJobFixtureCaller`; failure test uses `EffectBank::run_outcome`; removed duplicate `#[global_allocator]` statics (testing allocator provides it).
- Cargo: dev-deps `semio-framework-value-testing-admission`, `semio-framework-job` (testkit); feature `artifact-app-testing = ["semio-framework-job/testkit"]`.
- Callers to adapt (other crates): `advance_inputs(effect)`, store publication `close_step(effect)`, `close_step_controlled(effect, control)`.

## semio-plugin-document-media (stopped per coordinator; not compiling)
Changed: stale crate names (`semio_framework_os_kernel`, `semio_framework_plugin_composition_emission`) in lib; `poll_step` in `🦀️.rs`; envelope-decode (Send+Sync bounds, effect close); store-replacement cancel close; `🗄️document-archive-load` (member and load `close_step(effect, work_turn) -> OriginalRetirementPoll`, `last_native_work` u128, admission source tuple usize); `../🎞️media/🦀️.rs` closers via `close_original_effect_step`; `../🎞️media/🛫️intrinsic` `IntrinsicMediaEncoding` as `RetireOwned` (seed + `close_step(body, effect)`).
Remaining 14 errors, all in `../🎞️media/🛫️intrinsic` and `../🎞️media/💥️fault`: `NativeSnapshotEncodeOwner::receive` now takes `(&mut output, &mut |slot,output,marker,unwind,native,wallet|..)` and `NativeSnapshotFailure::into_value_error` is gone; `cursor.close`/`advance_intrinsic` and `ControlledRetirement::step` need a `RetainedAllocationEffect`, which the encode owner cannot supply (no framework glue, only grant-shaped `effect()`). Needs new io-control/pack bridge or output-slot redesign.
