# Pointer-move work loan refusal (2026-11-23)

## Symptom
`canvasPointerMove` (input #20, gesture window drawing-composite) faults the guest with `original work loan callback replaced its initiating context or refilled its work`. It is the first gesture command of the journey; every later gesture input then fails too.

## Root cause
- `with_original_work_loan` (`job/⏱️context/🦀️.rs`) issues `min(requested, fuel)` work units and, on return, `matches_original` requires `physical_work_spent - reserved <= issued`. The single message covers both a replaced context and an overspend, so the text is misleading here: nothing was replaced or refilled.
- The typed-operation pump (`retained-command/🔄️full-operation/🦀️.rs`) lends `step_fuel = min(fuel, contract.max_work_units_per_step)` to `session.pump_one_for_interactive_turn`. The Drawing gesture contract is `ToolExecutionContract::resumable(DRAWING_GESTURE_RAW_BYTES, 32, 1, ...)`, so each step is lent exactly one work unit.
- `DrawingGestureOperationJob::turn` (`✏️editor/⚙️gesture-jobs/🦀️.rs`) charged `context.consume_fuel(page.len().max(1))` for every wire page it validated. A pointer-move wire page has dozens of bytes, so the step spent far more than its single issued unit. `consume_fuel` raises `work_units_spent` by the full amount (fuel only saturates at zero), so the loan check correctly saw an overspend.
- The generic retained command job (`retained-command/🧵️retained-command-job`) charges `consume_fuel(1)` per page, which is the contract the pump assumes. The gesture job was the outlier.

The invariant check is right (a callback must not spend more than its loan); the caller was wrong.

## Fix
`✏️editor/⚙️gesture-jobs/🦀️.rs`: a wire page costs one work unit (`context.consume_fuel(1)`), identical to every other step of that job and to the generic retained command job.

## Test
`✏️editor/⚙️gesture-jobs/🧪️tests/🦀️.rs`: `drawing_gesture_wire_page_costs_one_work_unit_inside_its_step_loan` builds a 64-byte wire page, runs `job.turn` inside `with_original_work_loan(contract.max_work_units_per_step, ...)` and expects it to yield with the page cursor advanced. It fails with the loan refusal on the old code.

## Notes
- A rebuild of the Draw component is needed (guest Rust change).
- Not changed: the loan refusal text. A distinct message for overspend versus replaced context would shorten the next diagnosis.
- Possible further overspend sources in the same step (not observed): `DrawingInstanceOperationOwner::dispatch` runs `operations.close_step` in a retained loan when the fixed gesture slots are saturated, and that path can consume several fuel units. If the journey still faults after the rebuild, check this first.

## Verification
- Isolated test run: the editor crate with the fix compiled and `drawing_gesture_wire_page_costs_one_work_unit_inside_its_step_loan` ran; its first fixture had too small a retained grant for `RetainedToolWireInput::admit_original` (`wire admitted` expect). The grant was raised to `grant(4, 1 << 20)`.
- The re-run after that fixture fix did not finish: the cache volume ran out of space (`No space left on device`) during the isolated rebuild. The fixed test has therefore not been observed passing, and the pre-fix failure was not observed either; the root cause comes from reading the code.
- `repo cargo --store=shared ... check -p semio-s-artifact-draw-drawing-editor` failed in `semio-s-artifact-draw-drawing-model`: another agent's concurrent edit left `🧬️schema/🦀️.rs:804` (`pub mod snapshot`) pointing at a missing `🚪️io/💾️binary/📸️snapshot/🦀️.rs`. Unrelated to this change; re-run once that settles.
