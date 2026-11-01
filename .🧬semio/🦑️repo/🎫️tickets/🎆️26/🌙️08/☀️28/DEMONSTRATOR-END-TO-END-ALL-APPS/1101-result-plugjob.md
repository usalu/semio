STATUS: GATE GREEN: `C/1101-tools/gate.sh plugjob-29` exits rc 0 (`C/🗑️generated/1101/plugjob-29.rc`=0, zero `error` lines in `plugjob-29.txt`), wasm32-wasip2 --lib, hub manifest. `framework-green` NOT written by me (coordinator loop writes it). Tests of touched crates are not ported.

## Done in plugin-job
- `step(cx, claim) -> InteractiveJobStep` for the `framework_reserved_job!` macro, `FrameworkConfigurationBinaryJob`, `TypedCommandFullOperationJob`, `ChildEmissionPreviewJob` via `step_original` + `InteractiveJobStep::from_callback` (stepsplit.py pattern).
- Source-boundary: the crate was at 264837 bytes against `maxSourceBytes=262144` (no limit raise). The replay autowrap had duplicated `built/built_with/retired` in 9 files; now defined once as `pub(crate)` in the crate root `🦀️.rs` and imported (crate total now below the cap, ~257 KB).
- Staged admissions: `RetainedCloneSourceAdmission` in `child-vcs/checkpoint/cascade` (new `admission` field, phases 0/close/demands/terminal), `RetainedCloneOwner::take(grant)` in child checkpoint phase 3.
- `.result` on `begin_granted`, `select().step`, `close_step`, `artifact_retirement_*` outcomes; `admit_loan(&mut slot, cx)`.
- Configuration job failure: `OriginalDomainReservation` place + `admit_failure` (mirrors `🛬️receiving/📥️pack`), `OriginalConfigurationFailure::Native.producer` is now an `OriginalTurnFailure` built from `OriginalTurnPrimary::Receiving`; new `place` field also added at the single struct literal in `🔌️plugin/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs`.
- Configuration worker uses `pump_one_for_interactive_turn(cx)` and `acknowledge_checked_out_outcome(cx)`.

## Notes
- Splitting the 45 KB child-vcs/checkpoint subtree into its own crate was tried and reverted: a new package needs a Cargo.lock entry in ~65 lockfiles (`--locked`), which only the coordinator's `dependencies lock` may produce. If the crate grows again, that split is the clean fix.
- `reserved_pump_fault_is_transient` is now unused (pub, no warning).
- Tests of plugin-job not ported (only --lib checked).


## Dependents ported after plugin-job
- composition-emission/publication, tool-run, tool-machine (incl. scrub host, follow, settlement, tag, gesture issuer staged `SealedSharedBirth::publication`), document envelope/archive, media, command (typed/admission/proof/wire/latest), plugin-program (reactor surfaces native/read with staged `RetainedCloneSourceAdmission`, `cursor.take(grant)`, builder program), reactor-lifecycle cleanup pumps (`step` split), runtime-instances cleanup turns (`acknowledge_outcome(cx)`), opening `admit_loan(&mut slot, cx)` (edits in place, compiled only through plugjob-21 stage; runtime-instances itself not yet re-checked).
- `NativeDecodeControl` for crates without a native-owned dependency is reached as `semio_framework_value::NativeDecodeControl` (no Cargo changes).

## Open (first errors in `plugjob-21.txt`)
- plugin-runtime-ops (reactor/runtime/ops typed-operations, ingress, custody manifest): needs the `NativeReceivingOriginal` API (`shell_demand`, `begin_shell`, `original_capture_demand`, `admit_original`, `receiver_birth_demand`, `begin_receiver` are gone; use `NativeReceive::receiver()`), plus the usual `.result`/helper wrap.
- plugin-extension (8) and inference gateway/receiving (`close_factory_ticket` outcome, `owner.step(grant)`).
- runtime-instances (`opening`, `cleanup-turns`) edited after plugjob-20 but not re-verified.
- Tools: `C/1101-tools/plugjob/autofix.py` (absolute-path variant of fwfix autofix) and `C/1101-tools/errs.py <log>` (deduplicated error list). Warning: running autofix then `loop.py` can emit `.result.result`; grep for it after every cycle.

## Runtime wave (this round)
- runtime-ops: manifest `metadata.rs` ported to `NativeReceivingOriginal::new(source)` + staged `RetainedCloneSourceAdmission` (create/advance/take_output phases, `return_inputs` on close, `next_return_demand`), `take(grant)` through `built`, `OriginalDomainReservation` place + `admit_failure` for parse failures; manifest `🦀️.rs` `raw_place`; typed-operations intent/response (`built_with`, `.result`, `SharedUtf8::admit(..).result`); ingress `FailedDecoded` media/compound failures use reservations held in `OriginalDecodedPluginCommand` (`media_place`, `compound_place`, `admit_place`), no variant/constructor change outside the crate.
- runtime-instances opening/cleanup-turns re-checked green. reactor `📸️checkpoint` `SharedUtf8::admit(..).result`.
- `OriginalReceivingPrimary` is `semio_framework_job::OriginalReceivingPrimary`; folded to `ValueError` via local `fold` where a `Result<_,ValueError>` closure is required.

## Source limits
- plugin-job closure is 258796 bytes against `maxSourceBytes` 262144; `🏛️registration/🦀️.rs` is 14678 bytes (the earlier refusal was the crate-wide remaining budget, not a per-file cap). No source-boundary refusal appears in `plugjob-8`, `plugjob-24`, `plugjob-26` (grep `Rust crate Source` = 0). No limit was raised.
- Caveat: runtime-ops sizes were verified only by the admission pass (compiled), not by an explicit byte count.

## Reactor-turn wave
- `🔄️turn/📥️receiving`: `.result` on admit/close/step_granted, `fold(OriginalReceivingPrimary)->ValueError` in staged.rs, `OriginalNativeDecodeIssued::<_,ValueError>`, completed-event decode failure now via `issue_place` reservation (`admit_issue`, also on close). No source-limit refusal in `plugjob-29.txt`.