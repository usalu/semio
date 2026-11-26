# Withdrawn drafts honored in the replay outcome

## Cause
The shared `derived-replay/replacement` module already skips a Withdrawn draft (phase 4, then 5, `ReplacementTurn::Withdrawn`, prepared operation with no next state), in the Report replay and the draft preview alike. What was missing was the outcome: `ReplayOwnedState` (`store/replay/replay-owned-state-operations`, operation preparation settlement) built `MutationReplayOutcome.superseded/withdrawn` from `self.supersessions` only, and drafts live in `self.drafts`, so an accepted Withdrawn draft reported `superseded=false, withdrawn=false` and the row projection read the durable row.

## Fix
The settlement now selects the effective input as the draft of the mutation when present, otherwise the authored supersession (the same precedence as `select_replacement`): `superseded` is true for either, `withdrawn` is true when that input is Withdrawn and valid. One change, one place (`replay-owned-state-operations`, the `supersession` binding and the two outcome fields). The cold edit path already overlays drafts via `draft_supersessions` and was right.

## Results (`semio-plugin-tests-07-2`, `time_travel_tests_1`)
- Before: `a_mutation_row_offers_withdraw...` failed (`the replayed row reads withdrawn...`); `time_travel_tests_1` 14 pass / 16 fail at the last recorded run, the inert-label laws `a_blocking_mutation...` and `an_inverse_refusal...` failed on the Withdrawn flag.
- After: `a_mutation_row_offers_withdraw...` passes; both inert-label laws pass; whole target 26 pass / 4 fail.
- Remaining failures, not Withdrawn related: `an_unfunded_replay_step_defers_without_consuming_the_replay`, `every_fixture_scenario_reaches_its_status_body_and_rows`, `a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse`, `the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver`.
- No new law was added: the existing row law and the two inert-label laws exercise the outcome end to end (Report replay, preview and finalize).

## Changed file
- `store/replay/replay-owned-state-operations/🦀️.rs`

## Follow-up: `every_fixture_scenario_reaches_its_status_body_and_rows`
- First failure: scenario `overwrite-supersedes-in-place`, assertion "the session closed" (the first scenario whose steps end in `commit`; every earlier scenario passes its assertions).
- Cause: not a product defect. A commit completes through paid retirement turns (`drive_time_travel_finalize_outcome` swaps the finished Session for an Inactive one), but the law read `status()` right after the `commit` verb, before those turns ran. `commit_scenario` (used by every other commit law) already pumps "the finalize retires" until no work is pending; this law did not.
- Change (law only, no assertion touched): when a scenario commits, the law pumps `!has_pending_work()` before reading the status (`tests/time-travel-part-1/fixture-scenarios-sessions-and-drafts`).
- Not verified: the rerun was queued behind the capacity gate and I cancelled it at the coordinator's request (no new cargo until further notice). Later scenarios (`alternative`, `superseded` scope assertions) may still reveal further failures; rerun `a_` target `every_fixture_scenario_reaches_its_status_body_and_rows` when builds resume.

## Update: scenario law verified
- `every_fixture_scenario_reaches_its_status_body_and_rows` passes with the pump added (all scenarios, no further product cause found).
- Whole `time_travel_tests_1`: 28 pass / 2 fail (was 26 / 4). Remaining, owned by another agent: `a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse`, `the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver` (`an_unfunded_replay_step_defers...` now passes).
