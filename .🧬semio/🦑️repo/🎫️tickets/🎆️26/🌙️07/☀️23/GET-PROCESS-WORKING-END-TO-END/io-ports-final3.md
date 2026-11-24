# IO Ports Final 3: the five remaining control tests, the three refund tests and txt

Continues `io-ports-final2.md`. Builds via `stale-cargo.sh` (labels `g1` to `g18t`), logs under `🗑️generated/`.

## Results (last verified runs)

| Target | final2 | now |
| --- | --- | --- |
| `semio-framework-io-control` lib | 35 of 35 | 35 of 35 (`stale-g17.txt`) |
| `semio-framework-io-control-tests` | 51 of 56 | 56 of 56 (`stale-g17.txt`, `stale-g16.txt`) |
| txt lib | 15 of 16 | 16 of 16 (`stale-g20t.txt`) |
| txt `preparation_law` | fails in the close accounting | 1 of 1 (`stale-g20t.txt`) |

## 1. Unwound effect settlement (product fix, `cursor_advance_unwind`)

Cause: `NativeSnapshotBodyWallet::encode_effect_in` (`⏱️control/📦️octets/🦀️.rs`) caught an effect panic, moved only the failure carrier into the caller's `unwind` slot and resumed the unwind. The retained `NativeEncodeEffectSettlement` (performed receipt, physical post, Work commit) stayed unresolved in the frame's `action_marker`, nothing ever posted the recorded prefix to the original recipient, and the settlement's `Drop` later asserted "original effect settlement remains unresolved". The decode twin `run_snapshot_effect` already documented and did the right thing ("posting its recorded prefix before returning or resuming unwind").

Why the settlement is recovered at the unwind and not in `NativeSnapshotReceiving::close_step`: settling needs the encode control's own Work and construction loans (`NativeEncodeEffectRecovery::new(&mut action, &mut construction, ..)`), and a frame's `close_step` only sees a close effect and the failure-storage control, never the encode control, so it cannot post to the effect's Work issuer. At the panic site the control and both loans are live. `encode_effect_in` now replays the same quoted effect once, under `catch_unwind` (the existing `performed` replay path settles the claimed receipt and posts without invoking the closure again), folds an effect refusal into the `unwind` failure (so the physical receipt refusal and any work refusal stay retained until the original close) and then resumes the first panic; a second panic inside the replay is dropped in favour of the original payload. A replay that stays pending leaves the marker retained exactly as before. Law: `cursor_advance_unwind_keeps_effect_and_refusal_until_original_close` (shared fixture `🧾️receipt/cursor-effect.json` gains `unwindPostsRecordedPrefix: true`); it asserts after the close that the recipient refused the unwind receipt, which only happens if the receipt was posted at the unwind. No TS twin exists for this area.

`combined_allocation_...` was not a product gap: after a completed turn the claimed Work of the allocation carrier is accounting only (`NativeEncodeEffectState::settle_claimed_work`: "so a finished marker may retire"), and its `retirement_ready` correctly refuses until the caller moved it. The test closed the carrier without `allocation.reset_completed()`, the documented lifecycle call that the production loops make (`copy_bytes_encoded`). The test now calls it; no assertion was weakened.

## 2. The three refund tests

- `snapshot_primitive_original_loan_...`: with the refund, an idle receiving scope settles its own actual work into the context (`observe_scope_loan` writes the scope's fuel and `work_units_spent`). The test additionally called `cx.consume_fuel(outcome.work)` inside the Work loan, charging the same unit twice; `with_original_work_loan` then correctly refused ("replaced its initiating context or refilled its work": physical 2 > issued 1). Measured before the extra charge: `row fuel=1`, `cx.fuel=0`, `cx.work_units_spent=1`, `outcome.work=1`, i.e. the scope had already paid exactly the outcome's work. The redundant `consume_fuel` is removed from the primitive test and from the `turn()` helper of `🧭️drive/🦀️.rs` (in which it was a silent double charge that only a narrowed loan can expose).
- `snapshot_encoder_primitive_...` and `original_job_encoder_keeps_the_actual_append_...`: the close loop spun 65,535 turns with zero receipts. Measured demand 13,888 B copy against a turn grant of 6,797 B copy: the caller was admitted with the per-turn grant `fixture_grant()` (64 KiB copy) as its finite Source stock; after the birth, one encoder turn and two close steps only 6,797 B of the 65,536 B stock remained (58,739 B spent), so the next close step of the failure (13,888 B) could never be admitted. The sibling tests admit with `stock()` (16 MiB copy); both tests now do the same. Arithmetic: 65,536 - 58,739 = 6,797 < 13,888 demanded, and the failure's remaining ladder needs several more steps of that size, so no 64 KiB stock can close it.
- Not the refund: every decode `close` failure that showed "1,692,8xx B born / 856,xxx B freed against a receipt of 0 to 6,912" (and the identical 1,693,030 / 856,729 of the txt law) came from a `[DEBUG]` instrument in `🧵️job/⏱️context/🦀️.rs` `post_retained` added at 17:12 by another agent: for every new `released_bytes` size it runs `Backtrace::force_capture().to_string()`, whose symbolization allocates about 1.7 MB and frees about 0.85 MB inside the observed window of the very first turn that posts that size (a raw `backtrace()` tap of the allocator located it: `post_retained -> Backtrace::to_string -> gimli resolve -> RawVec::finish_grow`). That is a measuring artefact of the temporary instrument, not an unreceipted product allocation. I gated that block behind `std::env::var_os("SEMIO_DEBUG_RELEASE_TRACE")` instead of deleting it, so its owner keeps the instrument and sets the variable; with it off all receipts match exactly. The owner should delete it when finished.

## 3. txt

- `preparation_law` close accounting: `drive_one_item_preparation_law` observed the whole `caller.turn_with_decoder` heap but compared it with the close step's progress alone, ignoring every other receipted allocation of the same turn (return table and scope service, the turn's own charges). It now compares the heap with `caller.receipt()` (the receipt of the whole turn) and separately asserts the turn receipt covers the step's progress. Together with the gated instrument this removes the 1.69 MB / 0 disagreement; the law's own step quote check is unchanged.
- `original_owner_cancellation_and_funded_close...`: the zero grant close in `lent::close_encode` built the bank with the grant as its whole stock, but a `driven` call runs three funded runs (the call, then the reply transfer and the producer transfer). New `semio_framework_value_testing_admission::driven_stock::<F>(grant)` (in `🧪️testing/📥️admission/driven.rs`) sums exactly those three grants (grant + the two quotes `driven` itself uses), so the stock is derived from the quotes, not invented. Verified: txt lib 16 of 16.

## Instrumentation

Removed again: the allocation size tap and raw backtrace in `⏱️trace/🧮️memory/🧪️testing/📥️requests` (the committed `HEAD` still contains the first tap; the tree now has it removed), the `[DEBUG] close` prints in `🧭️drive`, the primitive prints, the law print. Left for its owner: the gated `post_retained` block (above) and the `[DEBUG] work quote denied` print in `🛬️receiving/🎟️work`.

## Run status

All green on the last runs: control lib 35/35, control tests 56/56 (`stale-g17.txt`), txt lib 16/16 and `preparation_law` 1/1 (`stale-g20t.txt`).

One more law fix surfaced once the close accounting passed: `drive_one_item_preparation_law` dropped its `OriginalJobDecodeOwner` and caller while the decoder still held its Source alias and escrow row ("decoder needs Source retirement"). The law now closes the decoder on its Source (`close_on_source` turns under the receiving floor grant until terminal), then drops it and calls `caller.retire`, exactly like the control drive tests' `close_decoder`; the same law serves the bmp, epw and semio preparation tests.

Also changed for the txt law: `🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/⚖️law/🦀️.rs` (turn receipt comparison, decoder retirement).
