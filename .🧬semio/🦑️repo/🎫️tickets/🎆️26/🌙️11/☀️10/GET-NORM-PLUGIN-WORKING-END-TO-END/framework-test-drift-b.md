# Framework test drift, group B

## Status (check --tests, round 1 of 2)
Compile clean: manifest-mutation-inputs, manifest-view-model, math-rank, mesh-component-references, schema-validator, ui-scene-modelling. Tests NOT yet run (queue throttle).
Edited after round 1, not yet re-checked (round 2 queued: `drift-b-check2.log`): surface-node-graph (paren typo), tool-run-tick-wire, value-native-reservation, tool-run-tests, ui-host-tests.
NOT migrated: io-sqlite-snapshot-tests (60 errors: allocation-callback API changed, closures are no longer `NativeSendAllocationLoan`, `with_construction_recipient`, `run_as_original_caller`, `close_retirement_recipient`, effect-shaped steps). Needs a dedicated pass.

## Patterns used
- Controlled retirement / effect-shaped steps: path-mount `🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs`, run each turn as `effect::with_effect(grant, 1<<20, &mut None, |effect| observe(|| owner.step(grant.maximum_copy_bytes, effect)))`, settle with `effect::settled(poll).result`. Heap observed inside the effect so fixture allocations are excluded.
- Retirement queue: `reserve_step(effect).result`, `admit_owned(&mut Some(x), effect)`.
- Scene cursor (surface-node-graph): local `impl SceneDecodeCursor` helpers `advanced`, `closed`, `*_observed`, `begin` over `with_effect`.

## Visibility fixes (no casual publics)
- tool-run-tick-wire: `testkit` module (now also under `cfg(test)`) exports `effected`, `observe_heap` (tuple of requested/released), `effect_fixture`; `observe_effect!` is `#[macro_export]` (doc hidden, testkit/test only) and expands through `$crate::testkit`. Optional dep `semio-framework-trace` behind `testkit`. `ToolRunTickWireCursor::testkit_frontier()` (testkit only) replaces reads of private `total/depth/length/frames`.
- value-native-schema-engine: new `testkit` feature with `NativePagedReservation::testkit_frontier()` / `testkit_resume()`; the reservation crate enables it as dev-dep.
- testing-admission `Authorities::submitted()` (additive) so tests keep checking accepted construction progress.
- ui-host-tests: companion `browser` module re-exports `semio_ui_host::linear_memory_test_import` (testkit).

## Replacements found
- `NativeDecodeConstructionRecipient`/`NativeConstructionRecipientIssuer` -> `OriginalConstructionRecipient` + `OriginalConstructionAuthority::with_loan`, or `semio_framework_value_testing_admission::Authorities::nested`.
- `WorkerJobSession::try_admit_owned` / `WorkerJobSessionPreparation` -> `WorkerJobSessionAdmission::<J, BatchJobSession<J>>::advance(job, params, output, cx)`.
- `ClipboardCaller` -> `OriginalJobFixtureCaller` (`bare_turn`, `admit_alias`, `retire`, `refreshed_turn`); descriptors via `OriginalJobReturnOrigin` + `drive_step` + `receive_outcome`, `origin.outcome()`, `acknowledge_outcome`. The receipt-count assertions of the clipboard laws may need re-derivation after the first run.
- Removed unused `ClipboardInitializer` fixture (its trait path is no longer used by the admission flow).

## Dependencies/Cargo.lock
tick-wire (+testing-admission, optional trace), reservation (+testing-admission, +value, +schema-engine testkit), tool-run-tests (+effect-bank, +tool-run-entities), ui-host-tests (job `testkit`), schema-engine (`testkit` feature). Cargo.lock edited by hand for these.

## Open
Round-2 check result, running all tests of group B, io-sqlite-snapshot-tests port. Logs `drift-b-*` in `🗑️generated`.

## Round 2 (agent b2, interim)
- io-sqlite-snapshot-tests migrated in source (not yet compiled): companion root `🧪️tests/🌅️companion/🦀️.rs` gained `with_allocation_port` (closure -> actual NativeAllocationAuthority port -> Send loan), `with_original_authorities` (genuine Work + construction loans) and a single `effect` mount. Limits/unit/retirement/receiving tests re-expressed over loans, `receive(&mut published, closure(slot,output,marker,failure,native,body))`, effect-shaped recipient close (`decode_close` via effect.rs, `encode_close` via testing-admission `driven`), and `run_as_original_caller` replaced by an explicit recipient plus quote-derived `drain`. New dev-dep `semio-framework-value-testing-admission` (Cargo.toml and Cargo.lock edited by hand). `NativeSnapshotCause` is taken from `semio_framework_io_control`.
- One test run of all group B crates is queued (`drift-b2-test1.log`, started before the build pause). Pending: its result, ui-host-tests receipt-count re-derivation, and the `metadata --locked` Cargo.lock check (NOT queued because of the build pause).

## Round 2 result (agent b2)
- `drift-b-check2.log` finished rc=0: surface-node-graph, tool-run-tick-wire, value-native-reservation, tool-run-tests, ui-host-tests all compile with `--tests`. No further errors to fix there.
- My batched test run `drift-b2-test1.log` failed at argument parsing (`cargo test --keep-going` is invalid; use `--no-fail-fast`). Because of the coordinator's build pause it was NOT requeued. Script ready: `scratchpad/driftb2-test.sh` (replace `--keep-going` with `--no-fail-fast`). io-sqlite-snapshot-tests migration is therefore still uncompiled, and the test runs (including the ui-host-tests clipboard receipt-count re-derivation) and `metadata --locked` check are outstanding until builds resume.
