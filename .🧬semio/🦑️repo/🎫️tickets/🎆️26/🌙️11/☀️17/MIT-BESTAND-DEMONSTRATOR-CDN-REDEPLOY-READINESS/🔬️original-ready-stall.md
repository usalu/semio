# Original readiness stall (10:15 probe, `ov-1015.log`)

## What the log says

- The four demonstrator panes print no `work quote denied` line. Their digest owner walks phases 0..8 (`Ready`); the stall message "waiting on the store owner refused the granted page" is a `Blocked` answer AFTER the digest, so fuel, the `+1` extent and `ready` are not the cause there.
- The 21 `work quote denied ready=false observed=0 additional=64 fuel=64` lines occur only in the generator pane: `OriginalJobWorkRecipient::admits` under the worker step loan (`fuel_per_step` 64). `observed=0` means the very first quote of the loan already finds `now >= deadline` or cancellation. That is the wall-clock hypothesis of `🔬️decode-work-gate.md`; NOT resolved here (no clock evidence).
- `digest zero work ... workq=0` with demand 1840 is the closing aftermath of the stall fault: a live owner quoted 0 work.

## Why next_digest_work_demand returned 0 with a non-zero demand

`NativeBatchDigest::next_digest_work_demand` forwarded the owner's quote unclamped; a live (not ready, not empty) owner can answer 0 (retirement computation, `take_output_work_demand`, `advance_work_demand`), yet every selected action charges at least one unit. Fix: the wrapper clamps live quotes to >= 1 (a ready owner keeps 0: the take path opens no effect). File: `🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`.

## Reproduced defect: exact quoted grant moved the seal with a zero receipt

Law (extended existing test `canonical_native_edit_seal_preserves_original_box_and_fields_after_framed_digest_and_cancellation`, `🔏️seal/🧪️tests/🦀️.rs`): every live seal quotes `next_work_demand() > 0` and the exact quoted grant yields a non-zero receipt.

Red before the fix, plain flow: the seal in `TakeOwner` with an already taken (terminal-empty) source quoted `RetirementDemand{0,0,0,depth 1}`; the step answered `Complete(default)`, switched to `SourceClose` and reported `RetainedCloneProgress::default()` with `last_turn_work() == 0`: a state transition with no receipt and no work. The one-item sealer reads a zero receipt as Blocked ("sealer blocked: zero progress", "canonical source zero progress ... seal (6,false,false)"), charges nothing and the stall counter keeps counting.

Fix (`📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`, `Phase::TakeOwner` Complete arm): a completed take with a zero receipt reports `copied_items: 1` and pays at least one work unit, like the other fixed transitions.

## Not resolved

- Generator loan `ready=false` at `observed=0` (deadline/cancel): needs `now`, `deadline_us`, `cancelled` printed in `OriginalJobWorkRecipient::admits`.
- Whether a further cause sits behind the first zero receipt for the demonstrator Blocked after phase 8: the sealer print is once-only. Re-probe after rebuilding `demonstrator`.

## Checks

- `-p semio-framework-replication-edit --lib canonical::seal`: 4 passed (red before the TakeOwner fix).
- `check -p semio-os-store-engine -p semio-plugin-instance-jobs`: Finished, no errors.

## Generator `ready=false observed=0` (static analysis)

Call site: the worker step (`🧵️retained-command/🔄️full-operation/🦀️.rs` ~411): `step_fuel = min(cx.fuel_remaining, contract.max_work_units_per_step)` (64), `step_deadline_us = now + max_step_micros`, then `StepContext::with_original_work_loan(step_fuel, step_deadline_us, ..)` sets `cx.deadline_us = min(step_deadline_us, cx.deadline_us)`. The quote that prints `additional=64 fuel=64` is `with_original_effect_at` (`⏱️context/🦀️.rs` ~350, `maximum = fuel_remaining`), whose `ready` is `retirement || now < deadline`.

Deadline scope: the loan deadline is per step, but it is capped by the TURN deadline, and the turn deadline is `OriginalOperationSource.deadlineUs = clock at admission + maximumElapsedUs` (`🎭️actor/🎟️retained-turn/🟦️.ts:46,58`): one absolute value per operation, identical for every issued turn. Jobs started via `originalSourceJob` get a fresh per-call wall; actor polls do not. So once an operation outlives its elapsed budget every later turn starts with `now >= deadline`: `should_yield()` is true from the first instruction and every effect quote (`observed=0`) is denied.

Path that counts attempts without yielding: the publication pump (`📬️publication/🏃️mounted-run-and-pump/🦀️.rs`) stops on `should_yield()` EXCEPT while it holds custody (`page_publication_pending`, `result_page_retirement_pending`, accepted receipt). Custody units therefore start past the deadline, their quotes are refused (zero receipt), the pump returned `attempted = true`, and `observe_publication_stall` grew the streak to 4096 -> `interactive-job.publication-stalled`.

## Fix (second form: refused by deadline = paused, not stalled)

`publication_pump_counts_as_attempt(attempted, moved, yielded) = attempted && (moved || !yielded)` in the pump file. The pump now tracks whether any unit of this turn advanced (`moved`) and, when the standing unit exits with `should_yield()` true, reports not-attempted, so the stall streak neither grows nor resets (`observe_publication_stall` returns 0 for not-attempted). A genuinely stuck unit on a live turn still counts. A fresh per-unit deadline is not possible in the design (the turn deadline is the operation's; a unit may not mint time), so the pause form is the one that fits.

Law: `a_standing_unit_past_its_turn_deadline_is_paused_not_counted` (pump file tests).

Residual: an operation whose absolute deadline has really expired now pauses forever instead of faulting by streak; faulting it by elapsed budget (typed deadline fault) is the correct next step and is not part of this change.

## Unfunded receiving step (`ov-1255.log`: "turn grant cannot fund one receiving step [0, ..]")

Cause: the unit (`📬️typed-operation-unit` ~110) quotes `receiving_quote.maximum_items = min(retained.items, phase.items - scope_items)` with `scope_items = 10` unless a scope is already held. The pump runs up to 256 units per host turn on ONE phase grant that each unit spends (`pubq`: 64 -> 40 within a turn); once `phase.items <= scope_items` the next unit has no fundable item, waits with a zero receipt and the pump still marked it `attempted` (turn not yielding), so the streak grew to 4096 although the next host turn's fresh grant (64) funds it. The pump's existing guard only looked at `remaining_plugin_turn_grant(turn, spent)`, which is the policy turn, not the phase grant.

Fix (`🏃️mounted-run-and-pump/🦀️.rs`): `publication_turn_funds_receiving_step(phase_items, scope_items) = phase_items > scope_items`. Before attempting a unit that goes through the receiving scope (pending artifact publication, no owned child group) the pump asks it against `cx.retained_grant().maximum_items` and the same scope headroom the unit uses (0 when a scope is held); if unfunded it hands the turn back and reports `publication_pump_counts_as_attempt(attempted, moved, true)`: counted only when an earlier unit of this turn moved the frontier (which resets the streak), otherwise not an attempt. The next host turn re-enters the pump on a fresh grant.

Law: `a_turn_without_items_beyond_the_scope_headroom_ends_unfunded_and_uncounted` (same file).

Residual: a host that never grants more than `scope_items` items per turn would now wait silently instead of faulting by streak; the unit's `note_publication_wait` still records that wait.

## `scalar close changed Source or Work` (14:27 build, `ov-1430.log`) -- NOT root-caused

The check (`record/🌳️source/🦀️.rs:150`) needs `invocation.source_turn()` to be Some and equal to `effect.work_source_turn()`. ov-1430 carries no `scalar close mode=` line (the build predates that print), so the three values are unknown.

Ruled out statically: my pump changes (hand-back and yield accounting) run before a unit opens any scope and never open or close a decode frame; `available_work` (11:54) was already in the green 12:51 build and returns a value only; the loan `admits`/`validate` source checks are untouched by the print removal (the removed items in `🔬️debug-print-inventory.md` are prints, once-guards and trace plumbing).

Found: `StepContext::with_native_retirement` (`⏱️context/🦀️.rs` ~229) lent `OriginalJobWorkRecipient::new(..)` WITHOUT `with_source`, so any scalar close retired through it had `source_turn == None` and failed this check deterministically (every other retiring path, `retire_receiving` and the receiving scope, carries the source). Fix: it now lends `.with_source(self.original_source_turn_identity())` (None stays None when the context has no Source turn, so no new acceptance without a Source). Check: `check -p semio-framework-job-context`: Finished. No law test: the path needs a full Job context with a resident decoder and a pending scalar frame, and the schema-engine test crate (`semio-framework-value-native-schema-engine`, `scalar` tests) does not compile right now (`unresolved imports crate::record::settlement_of, crate::record::Settle`, a peer's in-flight change).

Strongest remaining suspect (not mine, not changed): the `NativeReceivingEffect` producer/child relay refactor in `🏭️construction/📦️effect/🧳️frame/🦀️.rs` (13:44, `EffectRelay`, `unpack_producer`/`pack_producer` around `drive.invoke`/`finish`), which moves the invocation action out of the frame for the duration of the call, and the record `Settle` work in the same area. Next evidence: re-probe with the `scalar close mode=` print (invocation_turn vs work_turn) before deciding; if `invocation_turn=None` the retire path above was the cause, if both are Some and differ the relay refactor is.

## Generator `setActiveExample`: "waiting on the pending publication is closing" (15:05)

Answers:
- Nothing in the Publishing stage drives a closing pending publication. The unit (`📬️typed-operation-unit` ~124) answers `pending.is_closing() => wait; return Ok(())`; the close is only driven after the operation leaves Publishing (fault -> Retiring close ladder) or through the retained domain failure (`publication_failure` custody drained at the top of the unit). A publication that the owner abandoned (phase `Closing`) therefore sits there until the stall streak (4096) faults it, and the fault message carries only the wait, not the cause.
- My unfunded-turn guard is not what skips a driver: it runs before a unit only when `pending_artifact_publication` is Some and a closing pending is not driven by any branch at all (the unit's own closing branch is a wait). The guard hands the turn back; it cannot starve a closer that does not exist. The earlier stage that faults refusals (`fault_refused_typed_operation_publication`, my 0200 fix) only classified `UnsupportedOwner`/`InvariantViolated` refusals as final, so an abandoned preparation whose retained refusal has any other kind (WorkLimit, Canceled, OwnershipLimit, AllocationFailed, DepthLimit) or no retained refusal at all stayed silent: no console error, then the stall. That is also the shape of the demonstrator's masked first failure: once the owner begins its close after a refusal, the cause is swallowed and only a later, secondary close error (scalar close, `OwnershipLimit`/`WorkExceeded` ledger drain) is visible.

Fix: `PendingArtifactStorePublication::is_abandoned()` (phase `Closing`, not `Complete`) in `🧩️composition/📬️publication/🦀️.rs`; `MountedTypedCommandFullOperation::deterministic_preparation_refusal` (`🧵️retained-command/🔄️full-operation/🦀️.rs`) now faults the operation immediately for an abandoned publication: with its retained refusal of ANY kind (`refusal_has_no_retry_path(kind, abandoned)`), or with the owner's own `fault()` text, or a generic "abandoned ... without a retained cause"; it never preempts a retained domain failure that is still draining (`publication_failure` non-empty). Live preparations keep the old rule (structural kinds only).

Law: `an_abandoned_publication_refusal_is_final_for_every_kind_and_a_live_one_only_for_structural_kinds` (`publication_failure_tests`). 

Residual: whether the demonstrator's `scalar close changed Source or Work` is the original abandon cause or only the close of an abandoned owner is decided by the next probe: the fault text now names the retained refusal or owner fault instead of the stall wait.

Checks (abandoned-publication fix): `check -j 1 -p semio-plugin-retained-command -p semio-plugin-composition-publication`: Finished. The law test did NOT run: `cargo test` is refused before the compiler by the source boundary of `semio-plugin-retained-command` ("23 of 32 files, 266425 of 262144 bytes"). My additions are about 1.4 KB of that; the crate was already close to the 262144 limit and needs a split of `🔄️full-operation/🦀️.rs` (74 KB) before any test in it can run, so the law is written but unexecuted.

## "abandoned its pending publication without a retained cause" (16:00, all panes): my own regression

Root cause: `ArtifactStoreBatchPublication::acknowledge` (and the window config `acknowledge`: `self.closing=true`) moves a SUCCESSFULLY accepted publication into phase `Closing` (`self.phase = Closing; self.begin_close(); true`), so `Closing` also means "acknowledged, retiring", not only "abandoned". My `is_abandoned()` (phase `Closing`) plus the generic "without a retained cause" message therefore faulted every operation whose boot publication had completed and was merely retiring: all five panes, the same text, no retained cause because there was nothing to retain. Bearbeiten's secondary `native-return-refused: authority` is the drain after that fault, as before. No `begin_close` caller abandons the boot publication; I found none to fix.

Fixes:
- `PendingArtifactStorePublication::is_abandoned()` now needs phase `Closing` AND a retained cause (`preparation_refusal()` or `fault()`); the no-cause generic fault branch is gone (`deterministic_preparation_refusal` returns None for an acknowledged retirement).
- Every abandon carries its cause (your second ask): `PendingArtifactStorePublication::begin_close(cause: ValueError)` replaces the cause-less wrapper (no `Option`, no shim); each lane records the cause as its retained fault if none exists (`abandon(cause)` on `ArtifactStoreBatchPublication`, `ArtifactEphemeralOneItemPublication`, `ErasedWindowConfigPublication`, `ErasedWindowTransientPublication`). Callers: stale document root (`InvariantViolated`, `typed-operation-unit`), operation retirement (`Canceled`, `🪟️mounted/♻️frontier`). The lanes' own internal `begin_close()` stays: it is the retirement driver, and its abandon sites already store `fault`/`receiving_fault` first.

The earlier law (`an_abandoned_publication_refusal_is_final_...`) still holds for the kind rule; the retained-command crate still exceeds its source boundary so it cannot be run by `cargo test`. `check -j 1 -p semio-plugin-retained-command -p semio-plugin-composition-publication -p semio-plugin-instance-typed-ops -p semio-plugin-window-config -p semio-plugin-window-transient`: Finished.

## Closing branch of the publication unit drives the retirement (16:00 follow-up)

State found: `📬️typed-operation-unit` (~L109) already routes a closing pending publication through `mounted.advance_closing_publication(cx)` (`🔄️full-operation` ~L509), which calls `granted_retirement_step`, the same ladder the Retiring stage uses (`🪟️mounted/♻️frontier`): it closes the lane with its own `close_step` within the unit's grant, and when the owner is terminal-empty it takes `pending_artifact_publication` and answers `copied_items:1` plus the frame release, so the pump counts progress and the operation advances to completion. No second mechanism was added.

Bug found while verifying it: `granted_retirement_step` started the retirement with the cause-recording `begin_close(cause)` I had just introduced. `abandon` stores that cause as the lane's retained fault when none exists, so every ACKNOWLEDGED publication retiring through this ladder acquired a fault, and `is_abandoned()` (Closing + retained cause) then turned true: the stall fault would have fired on the success path again, one layer deeper. Fix: new `PendingArtifactStorePublication::continue_close()` (starts or continues the retirement and records no cause) is what the retirement ladder calls; `begin_close(cause)` stays for genuine abandons (stale root). Files: `🧩️composition/📬️publication/🦀️.rs`, `🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs`.

Check: `check -j 1 -p semio-plugin-retained-command -p semio-plugin-composition-publication -p semio-plugin-instance-typed-ops`: Finished. No test added: constructing a lane publication in the Closing phase needs a store fixture (the retained-command crate is over its test source boundary and composition-publication has no cheap Closing-phase constructor); the invariant "acknowledged retirement records no cause" is by construction (`continue_close` never writes `fault`).
