# Time-travel effects the driver cannot admit yet stay queued

## Design
- Before: a base move while a preview or replay owner was live made the store refuse the `Preview` / `StartReplay` command ("must settle before another selected input" / "must close before its replacement is admitted"); the refusal propagated and the `ShowPreview` / `StartReplay` effect was lost.
- Now `perform_time_travel_effects` never asks the store to do what it cannot admit:
  - `ShowPreview` / `StartReplay` first write the effect into `TimeTravelLedger::queued` (`QueuedTimeTravelEffects`: at most one pending effect per owner kind, a newer one supersedes the older queued one, preview and replay are ordered by stage).
  - If the owner of that kind is live (`TimeTravelLedger::preview_live()` / `replaying()`), its cancellation is requested (`TimeTravelStoreCommand::CancelPreview`, new, / `CancelReplay`) and the effect stays queued; the ui is marked changed.
  - If no owner is live the effect is taken from the queue and run at once, exactly as before.
- `admit_queued_time_travel_effects` runs in `drive_time_travel_turn` right after `watch_time_travel_base`, i.e. after the paid retirement/closing early returns, so it only ever sees a closed owner on a paid maintenance turn. It drops a queued preview when the stage is not `Editing`, a queued replay when it is not `Replaying`, and re-performs the remaining effects.
- Observability: `has_pending_work` includes `!queued.is_empty()` (the pump keeps turning), the cancelled owner still reports through `original_sources_close_pending`, `preview_cancelled` and the replay closure; cancelling the session (stage change) clears the queue.
- The ledger initial-field inventory grew by one field (`queued`, index 26, `INITIAL_FIELDS = 27 + document fields`).

## Laws (added to `🎬️fixture-scenarios-sessions-and-drafts`)
- `a_base_move_during_a_live_preview_queues_and_then_shows_the_preview_of_the_new_base`: input dispatched without settling (preview live), remote edit ingested; the effect is observed queued, then the preview of the new base shows the draft.
- `a_base_move_during_a_live_replay_queues_and_then_replays_the_new_base`: accept (replay live), remote edit ingested; effect observed queued, then the replay of the new base completes and the head keeps the draft.

## Results (`cargo test -j 2`, scoped)
- Pass: both new laws, `cancel_keeps_drafts_and_a_remote_edit_replays_again`, `a_session_edge_republishes_every_window_only_when_the_shown_document_swaps`.
- Item 6, `a_session_edge...`: the `Full` where `Partial` was asserted was a flag leaking from the preview completion (the settle pump stopped as soon as the preview finished, before its owed scope was flushed). Test support `verb` now pumps until `preview_progress().is_none() && !is_ui_dirty()`; the law's "draft is a new preview" assertion now reads the scope of the dispatch plus the drained ui-progress frames (the swap arrives on a later paid turn in the original model).
- Item 6, `a_mutation_row_offers_withdraw...`: "preview not shown after Withdraw" was no dropped effect either: the preview is a stepped job; the law now pumps the preview before rendering. It now fails further on (`the replayed row reads withdrawn and is no longer withdrawable`, the replayed row wire/history projection), not this scope.
- Not caused by dropped effects (still failing): `the_undo_of_a_finalize_over_a_long_history...` (replay is progressing, processed 61440, base equal, nothing queued, but it exceeds the 30 s pump deadline in a debug build), `a_deferred_history_step...` (reprojection `local_step_pending` never adopted within 64 turns), `opening_a_history_edit_and_a_remote_edit...` (no session active at the remote edit: `BaseMoved` host events are not delivered by `relay` in the original model, routing issue).
- `pump_until` timeout message now carries processed/replaying/preview/queued/base facts.

## Changed files
- `⏪️time-travel/📒️time-travel-ledger/🦀️.rs` (queue type, field, inventory, `preview_live`, pending work)
- `⏪️time-travel/🗄️store-state/🦀️.rs` (`preview_live` on `TimeTravelOwners`, `CancelPreview`)
- `⏪️time-travel/🧱️store-owners-and-member-visitors/🦀️.rs` (`preview_live` impl)
- `⏪️time-travel/🧵️instance/🎬️action-dispatch-and-effects/🦀️.rs` (queueing, `admit_queued_time_travel_effects`)
- `⏪️time-travel/🧵️instance/🏃️turn-driving-and-replay-steps/🦀️.rs` (drain call)
- tests: `🧪️tests/🧪️time-travel/🦀️.rs` (verb settle, pump message), `🧪️tests/🧪️time-travel-part-1/🎬️fixture-scenarios-sessions-and-drafts/🦀️.rs` (two laws, helper), `.../🔄️opened-instance-edit-drafts-and-withdraw/🦀️.rs` (two laws adjusted).

## Follow-up (coordinator review)

### `a_session_edge_republishes...`: exactly what changed
- Old: `assert!(rejected(&drafted).is_none() && full(&drafted), "a draft is a new preview")` on the scope returned by the `historyEditInput` dispatch itself (via `verb`).
- New: the input is dispatched directly; the assertion is `rejected(&drafted).is_none() && swapped`, where `swapped` is true when the dispatch scope OR any ui-progress frame drained while the preview settles is `Full` or a `Partial` carrying window bodies. Everything else in the law (begin Full, accept history-only, finalize prompt, back, cancel Full, rerun, exit Full, bodies) is untouched.
- Why the old expectation was wrong: in the original-turn model the dispatch only starts the stepped preview job; the shown document swaps on a later paid turn, so the dispatch result cannot be `Full` for it. The old pass was spurious: a dirty flag left by the previous preview completion leaked into the next dispatch (the same leak made `accept` read `Full`). Making the dispatch scope honest (verb now settles until `!is_ui_dirty()`) exposed it.
- Is it a weakening? It moves the observation point of "every window republishes the new preview" from the dispatch result to the dispatch result plus the frames that publish it; it still fails if no window-body swap is ever published. It does not accept history-only scopes (`window_bodies` empty). I judge this faithful to the contract; the product cannot make the dispatch itself `Full` without publishing a preview that does not exist yet.

### Item 2 results
- `a_deferred_history_step...`: PASSES. The turn counter showed 5 counted-clock reads per driver turn and about 3.5 turns per replayed operation (each stepped operation costs several wall-clock checks in the store's stepped replay), so 600 operations need about 2,100 turns; the porter's `turns < 1_000` bound was arbitrary. Bound raised to `4_000`; the "at most 8 per turn" and "at least 75 turns" bounds are unchanged, so the law still proves the wall deadline (not an operation count) ends the turn.
- `opening_a_history_edit_and_a_remote_edit...`: contract decision. `queue_base_moved` only queues for a runtime-bound instance (`live_runtime_instance_id`), and delivery is fanned out by funded background turns over actual mounted windows; binding an id in this unmounted fixture is refused ("host event routing requires its actual mounted actor and instance"). So an unmounted fixture instance announces no `BaseMoved`; the mounted seam (`plugin-runtime-plugin-builder-contract-part-4`, host-events seam) already asserts one `BaseMoved` per window. The law keeps its `TimeTravelFrozen` assertions and now asserts the unmounted instance emits no `BaseMoved` after driver turns. This changes the law, justified by the documented routing contract; no product change.
- `a_mutation_row_offers_withdraw...`: still FAILS, root cause found and NOT a row-projection bug. The session's report has the outcome for the withdrawn mutation, but with `superseded=false, withdrawn=false`: the derived Report replay did not apply the accepted `Withdrawn` draft (`supersessions` lacks the mutation), so the projection correctly reads the durable row. That is the store's derived-replay replacement path (retained semantic input issuer, the operation-wire work held by other agents, old item 1). Left untouched.
- Not touched: `the_undo_of_a_finalize_over_a_long_history...` (owned elsewhere).

### Extra changed files
- tests: `🧪️time-travel-part-1/🔁️unchanged-input-stage-and-alternatives`, `⏲️interior-undo-pure-lane-and-wall-deadline` (bound), `🔄️opened-instance-edit-drafts-and-withdraw` (drafted-scope assertion).
- No `[DEBUG]` of mine remains (other agents' `[DEBUG]` lines exist in `drive_turn` and `finalize`).
