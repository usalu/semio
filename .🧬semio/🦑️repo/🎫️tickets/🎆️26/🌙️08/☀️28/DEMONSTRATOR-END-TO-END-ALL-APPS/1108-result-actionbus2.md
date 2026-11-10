# actionbus2 lane (1108): action-bus lib, took over from actionbus
Rerun: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-actionbus2-NN "🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-action-bus --lib`

## State: 31/32 (W-actionbus2-36). Open: `original_wire_tests::original_command_wire_law`
Last symptom: a test-local `Admission` in `wire/🌱️original/🧪️tests` drops unfinished ("original effect producer requires transfer or paid retirement"): some `built(..)` construction never settles in 64 events although its grant is derived from the refresh quote. W-actionbus2-37 would print per-event `LAST=(grant items, copy, fuel, d.copy, d.capacity, ceiling copy)` (thread-local, [DEBUG] in that test file: REMOVE `LAST`/`eprintln!("[DEBUG] event` when done). That run was blocked by a peer's in-progress dsl-record-model compile error (dslrecv, intrinsic/🦀️.rs:108 E0502); rerun as W-actionbus2-38.

## Production fixes done (all in the claims log)
1. Birth chain (`job/🛬️receiving/🎟️native-turn/🌱️birth`): `grant_admits` checks the summed whole-chain demand incl. items (`chain_demand`); exact whole-stage quotes (`birth_total`/`remaining_demand`, incl. per-element fill items and the action-lock bind); commit action charged by its real cost (`return_origin_commit_cost`, slack kept for admission); failed chains park completions in a fixed `Stash` (size `OriginalReturnBirthStage::COUNT+3`, no heap) retired by the next turn / `close_step`; `paid` abandons its Work actions through the scope (`abandon`: finish live proof, commit performed, cancel unperformed); a stage that cannot be funded returns `waits` -> `Narrow` (recovery only).
2. `grant_construction` compares to `original_source_total_grant` (ignores the turn ceiling) so a narrow turn yields instead of "can never fit".
3. Close contract: `close_original_effect_step` (and retained_turn, quoted_retained_step, decode `close_recipient_step`, headless pump) grant every admitted item (coordinator ruling); plain-array retirement retires one copy-quote-sized block per item (law updated: `🔢️array/🧪️tests/plain.{json,rs,schema,test.ts}`, 8192-element case 5 turns).
4. Diagnostic `FaultScope`/`FaultParams` boxed retirement: `boxed_original_work_demand` + `native_boxed_original_admission`.
5. Secondary-cancellation: admission helper sets the physical marker from a refused constructor's retained progress.
6. Nested-owner: `run_scoped_on_caller` clamps the stock settle to the held stock and records the over-quote as `source_refusal` (kept by `original_receiving_outcome`); frame `recover_receipt` finishes a Refused verdict via `finish_refused` into `receipt_refusal`; `prepare_transfer` accepts that.
7. Settled-construction helper: the 2x/64*size heuristic is replaced by the exact quote plus `original_construction_settle_overhead()` (5 items, bind/release/reset bytes, 5 fuel).
8. Tests: `close_failure` asserts `receipt + original_return_refresh_quote(cx)`; test `retire` helper uses `close_original_effect_step`; wire-law construct grant = 16 items + refresh quote.

## Not done
- Unquoted receives (`original_receiving_outcome` `quote: None` => half the turn, `None` arm in `job/⏱️context/🛬️owned`): still the heuristic; callers must ask the receiver for its quote. Callers: retirement/ready paths of OriginalDecodeState.
- Non-audit fixture check `roots_released >= born` (job/🧪️testing caller) not yet restored to equality.
- receiving-context tests (6 red), job-context contracts target, wasm32-wasip2 check of action-bus.

## Debts
- Plain array: `next_demand(body)` is an upper bound of the block size only through `body`; quote == actual when callers pass the grant copy limit.
STATUS: PARTIAL W-actionbus2-36 (31/32; wire law open)
