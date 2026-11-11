# replay5 result (takes over from replay4)

Rerun: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-replay5-N <replay operation Cargo.toml> native -p semio-os-store-replay-operation --lib` (queue wait + compile 20-40 min under fleet load; run it in the background and poll `$G/W-replay5-N.rc`).

## Edits (unverified until W-replay5-1/2 report)
1. value/control contract `🎟️authoring/🦀️.rs` `invoke_original`: `CallProducer` now carries `effect.work_source_turn()` (invocation.source_turn() == effect.work_source_turn(), the law the Source owner `prepare_dispatch` checks; red A "source owner handback changed original action"). An unfunded reply handoff no longer drops the live producer action (red D "original effect producer action remains live"): the over-quote handoff is recorded on the receipt (`effect.performed`), the reply is taken unpaid and the action finished, so the Work commit reports the overshoot.
2. replay operation `native/🔗️input` `HoldCustody::drive` chains Source actions inside one effect (nested effect over the remaining grant/Work, next exact quote required to be admitted); `drive_once` is the single action used by close (wide-grant law F).
3. replay operation tests: `OriginalLawCaller::scope_floor()` (smallest fuel that opens a quoted receiving scope); terminal-ticket unadmitted turn = ceiling `quote`, loan `floor + quote` (red C).

## W-replay5-1 (first run, 4/9 again, 55 min incl. queue)
- A (zero-work-loan) moved from "handback changed original action" to `typed retirement receipt exceeded or changed its declared full grant` at the close loop (controlled `process_child_step`): the HoldRetirement child's Progress does not fit `cursor_grant` or its capacity differs from the quoted birth. Debug message added ([DEBUG], controlled/🦀️.rs `process_child_step`, REMOVE after).
- B (semantic factory): same `original work requires its original issuer and resolved quote`; stage code now exposed through `DEBUG_STAGE` thread-local (native/🦀️.rs, REMOVE after) and printed by the two `cursor.advance(...)` unwraps in the factory test.
- C: unadmitted turn with loan floor+quote was still admitted; replaced by `admission_floor(g,quote)` probe (smallest loan whose scope admits the effect) and loan = floor-1.
- D: `with_effect` returns Err("authoring work commit was refused") when the child pay(2) overshoots; fixed in the test child (it keeps its primary and builds `from_primary_and_work(primary, pay refusal)`), fixture workRefusal = "retirement close exceeds its work ceiling"; the unfunded reply handoff is left unpaid (accounting debt: handoff of a refused action is not charged).
- E: stall persists; `close` now drives `step_original_birth` when `requires_original_birth()`, and the stall panic prints [DEBUG] birth/terminal/demand.
- F: new test `original_mutation_source_chains_actions_under_a_wide_grant` (<= WIDE_GRANT_TURNS=2); HoldCustody::drive chains.

## W-replay5-2/3 findings (lib 4/10 with the new wide test, then 4/9 after removing it)
- A: the handback and receipt-vs-grant faults are gone from the close loop. Root cause of the receipt fault: `SourceHold::unwind_demand` quoted `capacity_bytes` for the `ControlledRetirement::new` wrap step, which allocates nothing (fixed in value/🔗️source-hold). The starved close turns (STARVED=8) open no effect (`retire` returns None); the test now asserts that instead of unwrapping a Some.
- C: the unadmitted turn is ceiling=loan=scope floor (`scope_floor()`, smallest fuel that opens the scope); the shell quote is the terminal header work (`native_header_release().work()`), not `next_computational_work` (0 for a terminal ticket).
- D: body passes now (primary pointer kept + work refusal); the remaining failure was `finish` (grant copy override 4096 below the demand) - fixed like `close`.
- E: stall root cause was the test's grant: `maximum_copy_bytes: 4096` below the quote copy 5496. `close` and `finish` now use `d.copy_bytes.max(4096)`. History then fails at `finish` with "decoder retains receiving stock" (scope retirement_required) - open.
- B (semantic factory cancel-close): `original work requires its original issuer and resolved quote` at the cancel close loop; [DEBUG] backtrace in work-action `invalid()` added to locate it (REMOVE).
- F (wide grant): ONE effect owns one allocation marker, so chaining actions inside `ArtifactMutationSource::advance(effect)` fails ("admission birth slots changed"); chaining per action through `probe_effect` on the control (one effect per action) fails with "source owner changed original Source or Work" on the second action of a turn (the owner authenticates the Work binding captured at capture). Both attempts removed; the law is NOT implemented (needs a design decision: owner binding per turn vs per action).

## W-replay5-4/5 findings (lib 4/9)
- B root cause (backtrace): `NativeSemanticPreparation::close_step` refills failure storage (`refill_original_failure_storage` -> `prepare_work`) inside the retire effect when the pool is empty; a second Work action cannot open under the live effect action (`OriginalWork::prepare_for_action` -> invalid). The retirement pump stocks the pool before the effect; the fixture caller now does the same: `OriginalLawCaller::new()` calls `prepare_failure_storage()` (explicit calls removed) and `finish()` first runs `release_failure_storage()` (pool `release_step` inside a retire effect).
- `finish` needs `maximum_items: 2` (`close_on_source` source-loan return refuses with items < 2), copy `max(demand, 4096)`.
- Preparation shell turn (zero-work test): loan WORK with ceiling = law `preparationShellWork` via `retire_with` (a loan of 1 cannot open scope plus effect).
- Next suspects after these fixes: returned-snapshot drain loop stalls (`registry.has_returned()` stays true; [DEBUG] message prints demand), history "decoder retains receiving stock" (scope retirement_required).
- [DEBUG] instrumentation still in tree (REMOVE before finishing): controlled `DEBUG_RECEIPT`/`debug_receipt`, native `DEBUG_STAGE`, test messages (grep `\[DEBUG\]` and `DEBUG_STAGE` under replay/operation and controlled).

## W-replay5-6/7 (lib 6/9; terminal-ticket and child/work-commit failure tests green)
- Fixes verified: starved close turns, `scope_floor` unadmitted turn, header-work quote, failure pool stocked in `new()` and released in `finish()`, `finish` grant items 2 (`close_on_source` source-loan return), returned-snapshot admission quoted with `ArcRootRetirement::original_admission_demands` (the hand-built capacity-only quote funded no copy).
- Zero-work and factory tests now fail only in the returned-snapshot drain: `advance_returned_snapshot_read` closes its `active` ArcRootRetirement with the legacy grant-shaped `close_step` ("store root retirement requires its borrowed producer"). Test helper `drain_returned` now admits through `advance_returned_snapshot_read` and closes the box through `close_ticket(original mode)` (W-replay5-8).
- History test still red at `finish`: "decoder retains receiving stock" (job return budget scope `retirement_required` is sticky once a scope turn charged capacity; no reset path found besides `OriginalCreditedOutput::adopt`/`transfer_scope_credit`).

## W-replay5-11 (lib 7/9)
- Green: derived, history input x2, paged projection, terminal ticket, child/work-commit failures, semantic factory (returned-snapshot drain via `close_via_reply` + header close in `drain_returned`).
- Red 1: `original_zero_work_loan...` assertion `edit_denied == 1` (the loop never sees Stage::Edit at an iteration start in the non-cancel run); W-replay5-12 adds a [DEBUG] stage trace (Stage as u8: 4 SourceCapture,5 Source,6 ProjectionBirth,7 Projection,8 TakeProjection,10 EditBirth,11 PostCapture,12 Edit) - REMOVE `trace` and the message after reading it.
- Red 2: `original_history_source_birth...` fails in `finish` with "decoder retains receiving stock": sticky `retirement_required` of the persistent scope (finish_scope: `scope.retirement_required || retains_output || actual.capacity_bytes != 0`, never reset); the quoted `native_scope(denied, .., true)` turn is the only structural difference to the passing tests - needs an owner decision (adopt the scope credit into the output, `OriginalCreditedOutput::adopt`, or a reset path).
- F (wide grant) still not implemented, see above.

## W-replay5-13 (lib 8/9)
- `original_zero_work_loan...` green: the cancel check moved after the Projection starved turn (the law expects `projectionAdvanceDenials == 1` in both runs). Only `original_history_source_birth...` red (sticky scope credit); W-replay5-14 adds `settle_scope_credit` to `finish` (precedent: action-bus admission tests adopt_credit/settle_credits).

## W-replay5-14/15: lib 9/9, then 10/10 with the wide-grant law
- `settle_scope_credit` in `finish` (adopt the sticky scope credit, settle it, then close the decoder) fixed the history test.
- The earlier wide-grant failure was my test mixing a bank effect (close) with a native-control source (advance); the owner authenticates the Work binding. With advance and close both on the caller the chained source passes: `ArtifactMutationSource::advance_chained(control, grant)` opens one original effect per action through `probe_effect` over the remaining grant; test `original_mutation_source_chains_actions_under_a_wide_grant` asserts <= `WIDE_GRANT_TURNS` (2) turns, heap-exact receipts, funded close and decoder finish. A quote-exact grant still takes the 29 single-action turns (factory test asserts it).
- Debt: production callers (store, tool-run, time-travel) still call `advance(effect)`; they need a control to use `advance_chained`.
- Debt: `invoke_original` leaves an unfunded reply handoff unpaid after a child overshot its quote (the action is finished, the reply kept) instead of dropping the live producer action.

## Final state
- Rerun: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-replay5-N <replay operation Cargo.toml> native -p semio-os-store-replay-operation` (all targets).
- FIXED-verified native W-replay5-19: replay operation lib 10/10 (9 old + wide-grant law) and `operation-unit` 5/5; W-replay5-17 control-contract lib 41/41; W-replay5-18 source-hold builds.
- Production changes: control-contract `invoke_original` (CallProducer carries the effect Source turn; an unfunded reply handoff finishes the live producer action instead of dropping it), source-hold `unwind_demand` (the ControlledRetirement wrap step quotes no capacity), replay operation `ArtifactMutationSource::advance_chained`.
- Test/fixture changes: fixture caller stocks the failure pool in `new()` and releases it plus settles the scope credit in `finish()` (items 2, copy max(demand, 4096)); starved close turns open no effect; `scope_floor`/`probe_scope`; header-work quote; `drain_returned` (admission quote from `ArcRootRetirement::original_admission_demands`, reply-protocol close); the cancel check sits after the Projection starved turn; ticket child keeps its primary on a refused pay; fixture workRefusal = "retirement close exceeds its work ceiling".
- Debts: (1) production callers of `ArtifactMutationSource::advance(effect)` (store, tool-run, time-travel) do not use `advance_chained` yet; (2) the unfunded reply handoff after an over-quote child is left unpaid; (3) store/read/retirement quotes `returned_admission_demands` with capacity+depth only (its test twin needed `original_admission_demands` copy bytes) - check that production site; (4) no wasm check of the replay crate this round.
- No [DEBUG], no mem::forget (only ManuallyDrop pairing in the fixture credit settle, mirroring action-bus), no AuthoringWallet, no limit raised.

STATUS: FIXED-verified W-replay5-19
