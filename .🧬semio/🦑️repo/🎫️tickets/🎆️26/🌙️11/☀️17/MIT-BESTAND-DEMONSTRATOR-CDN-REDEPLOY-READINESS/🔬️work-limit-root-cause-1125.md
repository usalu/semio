# Work-limit root cause (static, 2026-11-25): effect quote ignores held Source fuel

## Path
`OriginalReturnBudgetSnapshot::work_conserved` needs `work + held_fuel <= fuel`. A receiving scope claim (`receiving_work = ceil(fuel/2)`, `held_fuel` in the budget, `🛬️owned/🦀️.rs` `original_receiving_outcome`) stays held across turns: the decoder owner's `scope` and the budget claim persist, the budget is not `renewable()`, and `succeed()` (`🪙️budget/🦀️.rs` ~266) resets `work` to 0 and installs the new turn's `fuel` while keeping `held_fuel`. Source-side headroom of the new turn is therefore `fuel - held_fuel` (about half), which `StepContext::fuel_remaining()` (the METHOD, `⏱️context/🦀️.rs` ~566) already clamps to: `min(field, source.remaining().fuel)`.

`StepContext::with_original_effect_at` (~348) reads the FIELD instead: `let fuel=self.fuel_remaining;let maximum=fuel;`. The field is the whole fresh turn fuel (the `[DEBUG] big work quote 49999995 available=Some(49999995)` line). `OriginalJobWorkRecipient::admits` (`🛬️receiving/🎟️work/🦀️.rs`) admits any `observed+additional <= *fuel`, `available_work` reports the same, so every plain original effect (window config store owners, publication units, the pump's up to 256 units per turn) is allowed to spend up to the whole turn fuel while half of it is held. Its performed work is recorded through the sink into the ledger (`record_work`/`advance_work_exact`), so once the effects of a long boot turn (setContributions, typed operations with many items) sum past `fuel - held_fuel`, `advance_work_exact` yields `WorkExceeded` and the host sees `authority (work-limit)`. Within one turn the claim itself is consistent (`consume_fuel(work)` after `admit_scope` lowers the field), so the defect needs a claim carried from an earlier turn, which matches: refusal only appears after the first multi-unit publication turns, persists for every later poll (overrun is not held as native refusal) and hits exactly the two panes with large typed operations (Koordinator, Bearbeiten).

The same field is read by `unquoted_receiving_work(self.fuel_remaining)` (3 sites in `🛬️owned/🦀️.rs`) and `plan.quantum_work.min(self.fuel_remaining)`; those are later clamped by `OriginalReturnScopePlan::quote(.., self.fuel_remaining(), ..)`, so they only size the wish, not the admission.

## Why the reverted cap was wrong and this one differs
`fuel_remaining - ORIGINAL_INVOCATION_WORK` removed a unit from the worker step's own ceiling (no handoff unit exists on this path). The proposal removes nothing when no Source is bound (`fuel_remaining()` returns the field), and with a bound Source removes exactly what the ledger already owes to held claims and prior spent work, the same clamp `with_original_effect`'s sibling `retained effect` path (~327: `fuel_remaining.min(source fuel)`) and `attach_original_return_budget` already apply.

## Fix (NOT applied: demonstrator build running, queue-d-8.log active)
`🧩️work-limit-fix.patch`: in `with_original_effect_at`, `let fuel=self.fuel_remaining();`. Side effect: when `source.remaining()` errs (ledger already over) the quote is 0 and the effect is denied instead of overrunning, the intended refusal point.

Not proven: no test run (build running, shared crates). A native reproduction needs a bound Source with a held claim and a fresh `StepContext`: budget `succeed(grant, F)` with `held_fuel = F/2`, then `with_original_effect` whose closure performs more than `F/2` work; expect today `WorkLimit` at the ledger, after the fix `Ok(None)` (quote denied) or work capped at `F/2`. Place it in `🧵️job/⏱️context/🧪️tests` using the existing `original-native-progress` fixtures.

## Probe that decides (the `first_work_refusal` line in the running build)
`[DEBUG] return ledger first work refusal site=.. delta=.. work=.. held_fuel=.. fuel=..`
- `held_fuel` about `fuel/2` (e.g. 25M of 50M) and `work + held_fuel - fuel <= delta`: this root cause confirmed; apply the patch.
- `held_fuel` small (2-6) and `work` about `fuel`: the turn really spent everything; then the stall (publication loop) is the cause and the 4096-unit pump needs to yield, not the quote.
- `site=work` with `delta == fuel`: single oversized effect; same patch covers it.
