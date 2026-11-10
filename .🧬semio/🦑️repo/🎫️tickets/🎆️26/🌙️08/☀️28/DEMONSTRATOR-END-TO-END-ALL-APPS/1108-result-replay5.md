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
