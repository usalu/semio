# Hub surface test fix

## Failure 2 (guest codec table) - fixed, not yet verified by a run
- Root cause: `artifact_codec_table::<A>().print_mirror` (framework, `plugin/📜️declaration/🪟️surface/🧬️artifact-codec-table-and-replay/🦀️.rs`) built `native_artifact_codec::<A::Snapshot>` whose `reference` is `None` by design, so `.reference()` always returned "artifact codec requires an original retained application job". The test was correct; the framework table was the owner.
- Fix: print_mirror parses the pair with `parse_document_pack::<A::Snapshot,A::Mutation>` and prints through the snapshot's `reference_snapshot_codec().print_dsl` + `print_document_text_with` (coordinator's final version of the same fix).

## Failures 1 (viewer_never_mutates x3) - root cause not yet confirmed
- Message originates in `job/🛬️receiving/🎟️native-turn/🦀️.rs` `grant_construction`: `parent.recorded_clock_allows(cx)` returned false.
- `lend_refreshed_context` (job/⏱️context/🧪️testing/🎟️caller) calls `birth.advance` on a lent context; `allows` is false when the recorded decision is refused/transferred/not Some(false). Fixture clock is fixed 1000us with deadline 1500us, so the clock is not expired; suspect is the per-turn clock decision protocol in `birth::clock_begin` (`recorded_clock_allows(cx).is_ok()` treats Ok(false) as allowed and skips re-polling the decision; likely should be `matches!(..,Ok(true))`). Not changed: needs a run to confirm.
- All three kinds fail identically, so it is the generic testing-caller fixture, not hub setup.

## Run status
`hubfix-1.log` run blocked >50 min in repo capacity gate (memory-starved host); result not available at handoff.

## Update (hubfix-1 result)
- hubfix-1 (with coordinator's clock_begin Ok(true) fix compiled): 6 passed, 3 failed. guest_codec test passes. viewer_never_mutates x3 still fail with the same "declined new construction".
- So clock_begin was not the (sole) cause. Temporary `[DEBUG]` eprintln lines were added in `job/🕰️clock-identity-and-budget/⏱️recorded/📦️source/🦀️.rs` `allows` (two lines) and run hubfix-2.log (`surface_tests::puzzle2d_viewer`, --nocapture) is building (>90 min compile). Read `[DEBUG] clock allows=false ...` lines there for refusal/transferred/failed/decision. REMOVE the [DEBUG] edits afterwards.
- Untested hypothesis: `lend_refreshed_context` skips what `turn_original` does for a Source turn (`clock_stride.begin((clock)())`, `restore_clock_stride`, serve), so the fresh lent context's clock stride/decision state differs from a host turn.
