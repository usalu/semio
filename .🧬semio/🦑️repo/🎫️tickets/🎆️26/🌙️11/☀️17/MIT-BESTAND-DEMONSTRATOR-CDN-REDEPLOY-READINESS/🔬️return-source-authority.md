# Return-source authority refusal (`OwnershipLimit`, "receipt exceeds spent and held authority")

Status: first cause NOT proven statically; ledger defect fixed; one `[DEBUG]` probe added so the next demonstrator boot names the failing term.

## What the refusal is

`OriginalReturnLedgerIssue::OwnershipExceeded` (`framework/job/outcome/builder/return-escrow/🦀️.rs`), returned by `OriginalReturnBudgetSnapshot::advance_retained` whenever `!self.conserved()` after adding the receipt. `conserved()` is the conjunction of FIVE retained terms (spent fits grant; spent+held for items, copy, release, capacity) AND the work term (`work + held_fuel <= fuel`). `advance_retained` blamed `OwnershipExceeded` for any of them, so a ledger whose Work already exceeded its fuel was reported as an ownership refusal on the next retained receipt, even a zero one. The budget (`🪙️budget/🦀️.rs` `record_retained`) also overwrote an earlier sticky failure with the new one, hiding the first cause.

`spent=RetainedCloneProgress{0}` in the `poll_wit` line is NOT evidence: `spent` is a local initialised to default and assigned only after `dispatch_value_step` returns, so on any `?` failure it prints zero. The error's own `retained_progress` is zero because the sticky failure is a literal. Neither says anything about the ledger state.

## Timing evidence (04:50 run, `🗑️generated/ov-0450.log`)

In both panes the refusal appears only AFTER `[DEBUG] stalled 'setContributions' ... publication-stalled` (4096 publication units without moving the ladder; the stall fault cancels the operation lease) and a further `one-item publication blocked`. The refusal is not present in the 02:00 or generator runs. So it is a consequence of the stalled publication, not an independent early failure: a held claim keeps the Source ledger non-renewable (`budget.renewable()` is false with claims), so `commit_origin` keeps accumulating `spent`/`work` across epochs instead of resetting to the captured origin (`🪙️budget/🦀️.rs` line ~189), and every stalled epoch adds refresh/commit overhead (2 items, 2 work) plus the turn's own work. The Source fuel is the first-captured turn fuel. The likeliest failing term is therefore work (or held_fuel) crossing `fuel`, i.e. the stall drains the ledger; the "ownership" label was wrong.

## Changes

- `return-escrow/🦀️.rs`: `conserved()` split into `retained_conserved()` and `work_conserved()`; `advance_retained` returns `OwnershipExceeded` only for retained terms and `WorkExceeded` (kind `WorkLimit`) for the work term.
- `🪙️budget/🦀️.rs` `record_retained`: keeps the first sticky failure (like `record_work_exact`) and prints `[DEBUG] return ledger refusal <issue> progress <p> state <snapshot>` with granted, fuel, spent, work, held_* and claims. Remove the print after diagnosis.

Not changed: `Drop for StepContext` and the bundle `call_effects_before` (both reviewed; they do not touch the return-source ledger terms of this refusal).

## Test

`zero_retained_receipt_over_exhausted_work_blames_work_not_ownership` in `return-escrow/🦀️.rs` tests (crate `semio-framework-job-payload`): a snapshot with work 11 > fuel 10 and zero retained progress yields `WorkExceeded`/`WorkLimit` (previously `OwnershipExceeded`); a ninth item over a grant of eight still yields `OwnershipExceeded`.

- `repo cargo --store=shared --configuration=prototype -- test -j 4 --manifest-path <builder Cargo.toml> -p semio-framework-job-payload --lib zero_retained` -> 1 passed.
- `-p semio-framework-job-return-budget --lib` -> 10 passed.
- `-p semio-framework-job-context --test original-native-progress` -> 10 passed.

## Next step / rebuild

Rebuild `demonstrator` (koordinator, bearbeiten share it) and any guest linking `semio-framework-job-return-budget` / `semio-framework-job-payload`. The refusal should now read `WorkLimit` (if work) and the `[DEBUG] return ledger refusal` line names the exact term. The real fix is the publication stall (one-item sealer, other agent): once the publication advances, claims release, the ledger renews and the drain cannot happen. If the next boot still prints `OwnershipExceeded`, the state dump pins which retained term fails.
