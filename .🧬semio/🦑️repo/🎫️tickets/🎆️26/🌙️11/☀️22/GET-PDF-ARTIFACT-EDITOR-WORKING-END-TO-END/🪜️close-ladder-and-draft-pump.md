# Close ladder accounting and draft pump

## 1. `closes_mid_reserved_commit` (pdf14, pdf14a, pdf14x)
Cause (found with a `[DEBUG]` probe on the rung, removed again): the `ReservedCommit` rung of `close_ladder_step` (`🧵️instance/📥️staging/🔒️close-ladder`) computed its receipt with `original_lifecycle_delta` and only afterwards ran `self.pending.reserved_commits.pop_front()`. Dropping the finished commit (its stage with the revert/repin owners and the undo backing) released 4,544 bytes onto the context (released 4 -> 4548) after the receipt was taken, so the step reported 0 released while 4,544 were posted. 1.7 documents leave no such stage backing at that point; 1.4's pending reserved undo does.
Fix: the rung takes the delta after the pop, so the receipt covers the release. `original_lifecycle_unposted` is unchanged.

## 2. Time-travel draft pump
Probe findings (removed again): the refusal `original preview cancellation must settle before another selected input` fired on the first draft of a re-begun session, not after the discard. The harness (`acceptance_pump` users in `🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive`) pumped the begin preview to settled only for the first `historyEditBegin`; after discarding a blocked trial it re-began inside the candidate loop and drafted immediately, with the begin preview job (`preview_job`) still in flight (`has_pending_work` was already true there, nobody pumped). The guard then cancelled the job and refused, and every later candidate hit the same cancelled-but-unsettled owners.
Fixes:
- Harness: the re-begin inside the candidate loop now pumps until `!has_pending_work`, like the first begin.
- Store state: one predicate `preview_owners_live` (`⏪️time-travel/🗄️store-state`) is read by both the next-selected-input guard and `has_pending_work` (`preview_cancelled && (preview_owners_live() || preview_input)`), so a cancelled preview whose `preview_job`/`preview_derived` is unsettled keeps the pump running (before it only covered original/input/source/native_input).
Regression: framework test `🧪️history-edit-acceptance/🧪️tests/🔎️ready-candidate/🟦️.ts` (bun, 2 pass) pins the shared predicate; the PDF 1.4a/1.4x history fixtures were restored to their original page-index-1 edit, whose first candidate (index 2) is blocked and discarded before a valid one is accepted, so `history_edits_end_to_end` now exercises discard-then-accept end to end.

## Verification
`-p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- editor::pdf14`: 34 passed, 0 failed (before: 29/5, then 31/3). Includes closes_mid_reserved_commit x3 (pdf14, 14a, 14x) and history_edits_end_to_end x3. The six 1.7 closes_mid_reserved_commit tests still pass. Not re-run: the other framework Rust suites and the 3D puzzle / norm editors (same close-ladder code path).
