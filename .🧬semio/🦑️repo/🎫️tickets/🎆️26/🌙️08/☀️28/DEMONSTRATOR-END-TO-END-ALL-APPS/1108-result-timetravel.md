# timetravel lane result (11/10)

## Edits (time-travel, all grant -> effect)
- retirement: pending_command/session steps take the effect (child via effect.nested, settle_close_poll); store-state step uses close_owner/close_owner_controlled/close_factory_ticket(body,effect), ControlledRetirement/FactoryAuthority::step(body,effect), admit_owned_retirement(slot,effect), FactoryOwnedRetirement::admit_original(original,&mut factory clone,&mut output,effect); quotes use original_admission_demands.
- discard: discard_effects step takes the effect; ErasedSnapshotRetirement::close_step(body,effect) -> OriginalRetirementPoll (work = effect.performed_work()).
- finalize: close_original_effect_step for step_granted/begin_granted; replay retire via with_original_effect + with_native_retirement.
- store-state: StepPreview/StepReplay use with_native_receiving(&mut scope,..); not-ran -> Pending, scope failures -> typed refusal; OriginalRetirementFailure -> failure_value_error.
- ledger retire_step passes the effect.
## Edits (artifact-workflow-run)
- run sqlite snapshot: dropped the redundant subset_decoding/encoding overrides (trait defaults), decode/encode native take the output slot (space recipe).
- run schema: RunTriggerMap::map_into effect-shaped (perform/done), RunTrigger::receive_value with output slot.
## Verification
- W-timetravel-4 native (shell-session manifest, -p time-travel -p workflow-run): rc 0, both Checking+Finished, 0 errors.
- W-timetravel-5 wasm32-wasip2 hub, time-travel: rc 0.
- W-timetravel-6 wasm workflow-run (own manifest; hub resolver panics for this crate): rc 1, blocked upstream by semio-framework-value-native-owned (83 errors, not mine); no workflow-run diagnostics.
## Debts
- time-travel tests (retirement/tests) still call grant-shaped steps; replay not-ran Pending reports done/total 0.

STATUS: FIXED-verified W-timetravel-4 (native time-travel + workflow-run), W-timetravel-5 (wasm time-travel); workflow-run wasm BLOCKED by upstream value-native-owned (W-timetravel-6)
