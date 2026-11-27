# Authoring replay retention (undo of a finalize over 3061 operations)

## What was measured (temporary `[DEBUG]` counters, all removed)

Per-phase receipts of `step_derived_report_replay_batched`, keyed by which owners were live (prepared output, active preparation, replay retirement, history source and input):

- Gross capacity births are about 50 KB per replayed operation, but every owner is released again within the same operation: per phase the releases exceed the births (about 300 MB released against about 300 MB born over two replays). Nothing grows with history length in what is actually held.
- The job charges capacity as a gross, cumulative currency (`retained_remaining` subtracts only `retained_capacity_bytes`; `released_bytes` is a separate stock). So a streaming replay that births and frees an owner per operation still drains the stock by its sum. The stall at cursor 20 of 28 was exactly the cumulative capacity stock reaching 0 (`remaining_cap` 16.7 MiB per turn until the stock fell below it, then 3 KB).
- Births per operation by phase: prepared-output apply and retire ~17 KB, preparation step ~12 KB, replay retirement ~5 KB, preparation birth (`size_of` of the producer) ~15 KB.

## Fix 1: the replay posts the peak it holds, not the sum it ever held

- `OriginalNettedCapacity` (`semio-framework-value` birth ledger): nets one streaming owner's births against its own releases. It posts capacity only when the outstanding capacity exceeds the peak already posted, and offsets releases first against the births it absorbed, so a full close still releases at least what was posted. Pure and total; laws `the_posted_capacity_is_the_peak_held_and_the_posted_release_covers_it` (64 pseudo-random runs against a brute-force oracle) and `a_steady_stream_posts_one_owner_once`.
- `ArtifactDerivedReplay.capacity_ledger` holds it; `step_derived_report_replay_batched` (Report replay and authoring replay) nets the turn receipt through it on success (a refusal keeps its raw retained progress).
- Measured effect (authoring replay of the 3061-operation undo): capacity charged to the stock over the whole replay went from about 142 MB to about 0.4 MB.

## Fix 2: an exhausted stock refuses typed

`observe_turn_funding` / `time_travel_underfunded` (time-travel ledger): 256 consecutive owned turns that spent nothing (authoring replay step and publication step) refuse with `ValueRefusalKind::WorkLimit` ("the job's retained stock cannot fund the next history unit"). The authoring replay retains it as its failure and closes; the publication returns it as a turn failure. Before, the publication spun forever at stage 1 phase 4 with 45 copy bytes left. Law: `an_unfunded_run_refuses_at_the_limit_and_spend_restarts_it`.

## Law

`the_authoring_replay_retains_capacity_independently_of_the_history_length` (acceptance-blocked-review-and-long-history): the capacity the whole undo (authoring replay and its publication) charges grows by at most 4 KiB per extra operation between 100 and 1000 operations. Measured with the fix: 1.17 MB at 100 operations, 3.54 MB at 1000 (about 2.6 KB per operation: the result's per-operation outcome rows, inverses and metadata). Before the fix the same growth was about 50 KB per operation.

## Remaining blocker for the long undo law (not solved)

With fixes 1 and 2 the undo no longer stalls on capacity or spins: its replay reaches `ready` and the publication then refuses typed (`WorkLimit`, "the job's retained stock cannot fund the next history unit") after 256 unfunded turns, in about 2 minutes instead of never. The binding stock is now copy bytes, which is work and cannot be netted: the authoring replay copies about 110 KB per operation (337 MB for 3061), the result publication about 51 KB (157 MB) and the first publication's close another 14 MB, together more than the 528 MiB source total (`original_total_retained`) of the fixture caller (`copied_bytes` 528482304). Measured totals (`original_source_total_grant`): 357.8 MB left when the authoring starts, 20.7 MB when it is ready, 45 B when the publication refuses.

Tried and reverted: draining the finalize before the undo, and re-issuing the initiating-event stock on every `pump_until` turn (`issue_initiating_event`). Neither changes `original_total_retained`, so the total still runs out. Options for the owner of the budget: (a) raise the fixture stock `grant.maximumCopyBytes` in `testing/caller/tests/fixtures/caller.json` in proportion to the 3000-operation history, (b) cut the per-operation copy quotes (preparation birth `size_of` of the producer is ~30 KB copy per operation, prepared-output apply/retire ~600 B per unit over ~190 units), (c) let the authoring turns settle on the command receipt lane like the Report replay's `ownership` receipt.

## Result

`time_travel_tests_1`: 32 pass, 1 fail (`the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver`, now a typed refusal instead of a stall). The new laws `the_authoring_replay_retains_capacity_independently_of_the_history_length` passes (run `f3`); the unit laws of `OriginalNettedCapacity` and `observe_turn_funding` are written but were not run (their crates' test targets were not built here). The last test-side reverts (removing the ineffective drain and pump renewal) were not re-run.
