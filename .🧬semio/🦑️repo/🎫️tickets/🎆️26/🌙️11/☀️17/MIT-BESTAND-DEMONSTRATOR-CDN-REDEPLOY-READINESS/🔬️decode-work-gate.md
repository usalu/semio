# Decode work gate: why `receiving.with_effect(...)` at bundle `🦀️.rs:301` is always None

Status: static analysis only (no builds allowed). No production change, no law test: no deny condition is provable without the diagnostic run. Which line to read in that run is given at the end.

## 1. Which gate actually denies

The chain is `NativeDecodeControl::with_effect` (`🌱️value/🛬️decode/🏭️construction/📦️effect/🦀️.rs:29`) -> `invocation_extent` (work + `ORIGINAL_INVOCATION_WORK` = 1, the only `+1`) -> `OriginalEffectRun::begin` -> `OriginalWork::prepare_for_action(work + 1)` -> `OriginalWork::prepare` (`observed = actual - baseline`) -> `OriginalWorkLoan::admits` -> recipient `admits(observed, additional)`.

The publication unit's `receiving` is NOT the `ActualWorkTurn` in `🧵️job/⏱️context/🛬️decode/🦀️.rs` (that is the Source decode path). It is `run_scoped_on_caller` (`🧵️job/⏱️context/🛬️owned/🦀️.rs:164`) -> `OriginalJobWorkRecipient::admits` (`🧵️job/🛬️receiving/🎟️work/🦀️.rs`):

`ready && observed + additional <= F`

- `ready = !cancel.is_cancelled_now() && now_us().is_some_and(|now| now < deadline_us)` (`🛬️owned/🦀️.rs:160`), non-retirement branch.
- `F` = scope body fuel = `min(pooled.work - control_work, quantum_work)` = `quantum_work` = unit's `receiving_work = fuel.div_ceil(2).min(fuel - scope_work)` (typed-operation-unit `🦀️.rs:153`), `scope_work = 2 + 2*4 = 10`.
- `observed` = work already performed in this scope entry. It is reset by `begin_turn()` on every `with_work_recipient_matching` entry when no action is live (`🛬️decode/🎟️work/🦀️.rs:45`), so it does not accumulate across units. A leaked live action would make `prepare` return `Err(invalid)`, not `None`.

Two diagnostics already exist: the recipient prints `[DEBUG] work quote denied ready=.. observed=.. additional=.. fuel=..` on every denial (unthrottled), the `ActualWorkTurn` prints once with `paused/closing/cancelled/should_yield/fuel_remaining`. The bundle prints `[DEBUG] digest step not admitted work=.. grant=..`.

## 2. Hypotheses, ranked

1. **Wall-clock deadline elapsed before the quote (`ready == false`): most likely.** `now >= deadline_us` is persistent when the unit's pre-quote cost alone exceeds the turn's wall budget. Before `with_effect` the unit runs `owner.next_digest_work_demand()` (alias/projection work demand over the whole staged edit tree, `retained_alias_work_demand` -> `work_demand(child)`), `next_digest_demand()` and the surrounding scope admission; the same cost recurs on every retry, so it is permanent, not flaky. It matches the evidence: large edits (`setContributions`, Config lane: koordinator/aussuchen/bearbeiten) pass ~28 fold units then block on the first digest quote; small edits (energie, statik, aggregator) pass; native tests cannot reproduce it (no wall limit, fast clock); wasm plus unthrottled `eprintln` per unit is several times slower. The pump (`🏃️mounted-run-and-pump/🦀️.rs:134`) deliberately keeps pumping past `should_yield()` while publication custody exists, so nothing breaks the loop: the 4096-unit stall counter is the only exit. Verfolgen's `WindowConfig` lane (smaller tree, hash traversal) hits the same gate later.
2. **Cancel/clock source.** `is_cancelled()` is true forever once `StepCancelReference::Observed` is set (`observe_original_cancellation`), and `now_us() == None` makes both `deadline_exceeded()` and `ready` true/false permanently. Other panes boot through the same path, so a missing clock is unlikely; an observed cancellation of the typed operation lease would also have surfaced as a cancel fault. Low.
3. **Fuel / off-by-one (`maximum_work + 1 > F`): excluded for the observed budgets.** The `+1` is real (`invocation_extent`), and `F = min(ceil(fuel/2), fuel - 10)` is smaller than the turn fuel, so a unit needs `work + 1 <= F`. The largest quoted demand is `SEAL_TURN_BYTES = 256` (+1) for a tree step, so it fails only for `fuel < 514` (`ceil(fuel/2) >= 257`) and below 22 via `fuel - 10`; the unit already returns early with `note_publication_wait` when `fuel <= scope_work` (a different path than `Blocked`). The Source ledger shows `fuel: 49999986` and `pooled.work: 1048580`, i.e. millions per turn, so `observed + work + 1 <= F` holds with orders of magnitude to spare. It could only fire if a demand were itself of the order of F (a tree-size-proportional alias demand); that would show as `ready=true observed=.. additional=<huge> fuel=<F>`.
4. **`closing` pause.** Not applicable: `OriginalJobWorkRecipient` has no closing term; `ActualWorkTurn.closing` belongs to Source decode closing. Very low for this site.

## 3. What the diagnostic run must show

- `work quote denied ready=false ...` -> hypothesis 1 (or 2). Add `now`, `deadline_us` and the unit's start time in the recipient print to distinguish deadline from missing clock. The fix then belongs in the budget design, not in the quote: a publication unit that was entered while `!should_yield()` must be guaranteed its first effect quote (progress floor, as the pump already does for custody), or the demand computation must be made incremental/cached so it stays inside the wall budget (for example cache `next_digest_work_demand` per phase). It should not be fixed by relaxing `ready` for all effects, which would remove the wall bound.
- `ready=true observed+additional > fuel` -> hypothesis 3, with `additional` naming which demand is oversized.
- No `work quote denied` line at all but `digest step not admitted` -> the denial is a different `Ok(None)` source (`prepare_for_action != Some(true)`, i.e. `recover_quote` returned `Some(false/None)`); read `recover_quote` (`⚙️operation/🦀️.rs:143`) next.

## 4. Changes

None. No deterministic law reproduces a permanent `None` from the pure gate: `admit_work`/`admits` are correct functions of `(ready, observed, additional, F)`, and all three terms are fine for the known budgets except the wall-clock term, which is environmental (slow wasm), not a logic defect testable without a clock fixture.
