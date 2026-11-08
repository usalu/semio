# origin / capsule / frame close port (prepared, not applied)

Callees that stopped being grant-shaped (found by reading, confirm with compiler):
1. `job-failure` `⚠️failure/📦️frame/🦀️.rs:14` `OriginalReturnFrame::close_step(grant)` calls `self.owner.step(grant)` (ControlledRetirement::step now takes `&mut RetainedAllocationEffect` and returns `OriginalRetirementPoll`) and `self.owner.last_turn_work()`.
   Target: `pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll`; first branch `return self.owner.step(effect)` (work travels in the poll, drop last_turn_work); the rest compute `let grant=effect.grant();` and return `RetainedWorkPoll::Complete(RetainedWorkOutcome{result:..map_err(OriginalRetirementFailure::from),work})`. `descriptor.acknowledge(grant)` stays grant-shaped unless job-outcome-borrow changed (compiler decides).
2. Callers (use the new helper `close_original_effect_step(cx,demand,|effect|..)` from ♻️retirement/🎟️context, already exported from job-context):
   - `🎟️origin/🦀️.rs:104` and `📦️capsule/🦀️.rs:42`: `close_original_retained_step(cx,demand,|grant|frame.close_step(grant))` -> `close_original_effect_step(cx,demand,|effect|frame.close_step(effect))`.
   - `🎟️origin/🦀️.rs:80,102` and the descriptor path: `outcome.acknowledge(grant)` — keep `close_original_retained_step` if acknowledge stays grant-shaped.
   - Lines 53,55,56,57,101 and capsule 41,43 ignore the grant (`|_|`): no change.
3. `with_original_effect` quotes `maximum = fuel_remaining`; a close with no fuel returns Pending without running (matches `cx.fuel_remaining()==0` refusals elsewhere).
Apply order: job-failure frame first (it compiles before context), then origin/capsule call sites in one edit.
