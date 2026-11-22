# Job-context completion (2026-11-22 night)

Lane 4, `semio-framework-job-context`. Not all 7 targets are green.

## Counts

`repo cargo --store=shared --configuration=prototype -- test --manifest-path` the job-context rust package `--features testkit -j 1 -- --test-threads=1`.

| Target | Result |
| --- | --- |
| `original-native-receiving-context` | 3 passed, 3 failed |
| `original-return-and-retirement-contracts` | 21 passed, 3 failed |
| other five targets from the earlier pass | not re-run |
| full crate | not re-run |

## Root causes and fixes

### Child construction receipt (fixed)

The decoder scope is admitted once, during warm or the birth service, and later calls are loaned. `quote` then tried to grow that claim whenever the new quantum exceeded stock on any axis. The child turn's unquoted work is half of a large fuel budget. After the first receive spent 4 work units, the next quote wanted 4 more units of work than the claim still held. `admit_scope` posts 1 item and `original_return_stock_top_up_copy_bytes` (81) for any non-zero top-up, including a work-only refill. The callback sampled `retained_progress` around that post, so the receipt was 2 items and 274 bytes instead of 1 item and 193 bytes (child progress plus the one parent byte).

`OriginalReturnScopePlan::quote` now returns the existing claim when it is loaned, not recovering, and the reserved body already covers the quoted items and copy. Work is clamped to that body. Capacity and release are clamped to stock on hand. No top-up is posted. The child-construction test passes.

### law_1 one call (fixed)

`receive_shared` performed one phase per call: prepare the sealed birth, or publish. The fixture body is already 32 items, which is enough for both phases, but one `with_native_receiving` only prepared the birth, so `output` stayed `None` and `input` stayed `Some`.

After a successful birth, if the effect frame is `Empty` again and the input is still present, the same call publishes and the two `RetainedCloneProgress` values are added. Both `law_1` receiving tests pass, and the child-construction test still passes (one call now finishes the short string, and the receipt is still that call's own progress plus one parent byte).

## Still failing

### law_4, fuel 0 vs work 1

`actual_builder_page_return_keeps_real_posting_refusal_at_original_receiver` reads `fuel_remaining` before `with_original_effect`, folds with cpu 0, and expects the fuel delta to equal `turn.work` (1). `with_original_effect_at` returns `(output, performed)` and never calls `consume_fuel`. `consume_fuel` also adds `work_units_spent` and `record_work`. `law_3` already passes while reading `fuel_remaining`, so a blanket debit inside every effect was not added.

### source_law_2

Both tests use `bare_turn`, so the birth service and the reserved receiving scope are not on this path.

- Allocation refusal: `shared allocation requires an empty actual marker` while `admit_step` runs under `refuse_one_allocation_on_this_thread`.
- Close: `step.progress().copied_bytes` is greater than `size_of::<bool>()` (1). Child metadata itself is 0 copy bytes; a later close phase posts the extra bytes.

### CPU loan, available work 0 vs 1

`native_receiving_uses_the_actual_current_cpu_loan_without_a_second_debit`. Case fixture: loan `quantum` 3, `sourcePlacementWork` 2, `childQuantum` 1, `sourceWork` 3. `available_work_units` is `NativeDecodeControl::pending_work`, which is `work.reserved()`, and nothing has issued a reservation before the callback's `prepare_work(1)`. Unquoted receiving work is `fuel.div_ceil(2)`, so a loan of 3 becomes a quantum of 2. A placement of 2 then leaves 0, which is the second debit the test names.

### Cancelled source, narrow retirement returns true

`cancelled_source_recovers_actual_pending_child_with_same_construction_issuer` cancels, then asks `with_native_retirement_receiving` on a grant of 1 item to return false. The scope reserved on the earlier stock-sized turns already covers that 1-item quote, so the call runs. The new loaned quote keeps that behavior for any quote the reserved body can fund.

### Retirement after cancel, projection frame

`original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap` fails inside `copy_native_text` with `InvalidValue`: `original projection must be driven or returned through its resident frame`. The refusal is raised from the projection phases of source admission when that phase is driven through `advance` instead of the resident projection frame. Effects on the failure are all zero.
