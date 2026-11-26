# Capacity gate starvation of large requests

## Mechanism (confirmed in `📏️decision` `decideHostCapacityV1`)

Ordering already was oldest-first within a class (aging turns batch into interactive after `agingMilliseconds`, 300 s). The starvation came from the admission pass after ordering:

1. The head only became a barrier (`blocked`) when `drains` held: its need had to fit `min(available + observed grant memory - floor, budget share)` at that very sample. Foreign (non-ledger) memory moves that figure. While it is below the head's need, nothing is held back and every later request is backfilled; long grants (hours, e.g. cargo-growth) then take the memory for good. A momentary dip makes the barrier flap for one poll only.
2. `session-cap` refusals `continue` before any barrier logic, so a capped head reserves nothing until it ages (the cap itself is exempt after `agingMilliseconds`, so it never blocks a head forever).
3. The 1800 s give-up: the `starved` refusal fired for every request that did not fit (also with `!memoryFits` and live grants), so a feasible request failed with "needs at least N MB ... after 1801 s" and retried into the same stream.

## Fix

- Head-of-line reservation (`reserved`): once a head that fits this host at all (`least need <= cap * budget`, cores under ceiling, no soft stop) has waited `bypassMilliseconds` or been bypassed `maximumBypasses` times, later non-quiet requests are refused (`queue-order`) whether or not the head currently drains. Drainable heads keep the old `blocked` barrier. Backfill: with nothing of ours running and nothing admitted in the pass, one later request may start, so an idle host is never held empty by foreign pressure.
- No silent give-up: `starved` is only issued for requests that can never fit (`infeasible`); feasible requests keep waiting with the existing progress lines (`pastDeadline` with holders), cancellation unchanged.
- Schema: description of `wait.starvedMilliseconds` updated (no new field).

## Laws

- `🧪️tests/🚦️starvation`: new foreign-memory scenario, admission second equal to an independent Python oracle (`🐍️.py`, `foreignAdmittedAtSecond`), bound `period + run`, and the same stream without the reservation never admits; small requests keep flowing without a large head; a feasible head is never `starved` past 3x the bound while an infeasible one is; a session's own head is not `session-cap` once aged and is not overtaken by its own later small request.
- Independent oracles updated for the new rule: SQLite oracle (`🔮️sqlite`), Python upgrade oracle (`🔼️upgrade/🐍️oracle`); stored corpora regenerated from them (8 decision corpus rows, upgrade and grow vectors).
- Host-capacity TS suites pass (decision 29, gate 34, starvation 7, upgrade, grow, unmanaged, warden, asks and the rest). One earlier run of a gate peak-time test failed once on timing under load and passed on rerun.

## Not verified

No cargo build, no live `repo capacity status` after the change (running processes use the old code until restarted).
