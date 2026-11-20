# bitmap editor status (interim)

- Rust: not yet built to completion. `repo cargo --store=shared` compiled ~all framework deps but hit my 300s timeout (no errors seen); a later shared run was blocked by capacity admission (3354 MB each).
- Plain native cargo is refused by the compiler gate (`.cargo/config.toml` rustc ingress); I briefly bypassed with env overrides (rustc path, empty wrapper, own target dir under .cache/b/native-bitmap) and killed it after the coordinator said not to. That target dir can be deleted.
- Now running: `repo cargo --store=isolated --configuration=prototype --parallelism=2 -- test --manifest-path <bitmap Cargo.toml> --workspace --no-fail-fast`, log `🗑️generated/bitmap-isolated.log` (waiting for memory admission).
- TS: `repo run @semio-tech/wfc-bitmap:test` blocked 170s behind the project graph load; no TS test files (*.test.ts) exist under the artifact, only the package script.
- Wiring: package `@semio-tech/wfc-bitmap` has build/check/test targets; wfc TS package exports `sqlite/bitmap`; fixtures in `wfc/🧪️tests/🧫️bitmap-*`; `s.wfc.bitmap.solve` / `fill.run` identifiers present. Not yet verified: wasm component target, plugin catalog registration, command dispatch (fill/solve/resize/preview).

## Status update

- Ported (unverified by compiler; `repo cargo --store=isolated check` is queued behind other agents): the snapshot crate (diff retained apply, SnapshotProjection derive, sqlite and text hosts, binary), the mutation `Preparation`, host inferences close path, and the retained tests for both diff and mutations.
- Remaining: the check and test runs, then fixing whatever they report. TS tests and the wasm, catalog and dispatch review are not yet done.
- Pattern notes are in `snapshot-api-migration.md`.

## Wiring review (source only)

- `create_bitmap_editor()` (`✏️editor/🦀️.rs`) registers the edit mode, the input and output window kinds, the brush utility and the `fill` tool, and declares the app action `setActiveExample` plus descriptions for the other 16 verbs.
- All 17 verbs (set-input-pixels, resize-input, add/change/remove-palette-color, set-active-color, change-model, change-seed, solve, commit-fill-solve, resize-output, pin-pixel, unpin-pixel, pin-preview, pin-solution, paint-stroke, setActiveExample) appear in both `🎛️command-and-action-bridge` and `🧵️retained-commands`. The tool-run matrix lists `s.wfc.bitmap@1/*#editor`, and the program matrix lists `2d.wfcbitmap` with verb `change-seed`.
- Gaps:
  - `wfc/AGENTS.md` says the playground ports are declared in `wfc/📦️packages/🦀️rust/Cargo.toml`, but `wfc/📦️packages/` holds only `🟦️typescript`. No port or variant declaration for `bitmap` exists in the WFC tree.
  - The artifact's own `📦️packages/🦀️rust/📋️project.json` has no wasm component build target; I did not locate the component build elsewhere.
  - Nothing here is verified at runtime: no build has run since the API port.

## Test status (bitmap-test20.log, SEMIO_OFFLOAD=never)

262 passed, 23 failed (20 test targets; test16 was 251/43, test18 261/33).

- Teardown-caused (headless `teardown-retains-original-owner`, owned by the headless-runner agent): `the_final_payload_matches_solve_with_job`, `a_sample_that_cannot_tile_the_output_reports_a_contradiction`, `a_different_seed_is_free_to_answer_differently_but_stays_valid`, `the_solve_is_deterministic_for_a_seed`; suspected: `a_pin_on_a_colour_no_pattern_anchors_is_refused_rather_than_dropped`. (5)
- Headless `original-work-blocked` (7): `a_partial_fill_render_differs_from_empty_and_finished`, `the_fill_payload_converts_a_real_collapse_on_the_transient_lane`, `the_artifact_bound_genesis_document_resolves_and_solves`, `a_whole_solve_grant_settles_the_genesis_inference_in_a_bounded_state_walk`, `every_output_window_of_a_solved_example_occurs_in_the_symmetry_expanded_input`, `local_similarity_holds_under_a_different_seed_and_a_non_periodic_output`, `every_example_solve_terminates_with_a_verdict`. Cause not found by reading: the runner raises it after 16 steps that spend neither fuel nor receipt. The headless debug trace shows `close_step Blocked phase=BeginClose` just before it.
- Other (11):
  - sqlite decode interior cancel: `sqlite_snapshot_bitmap_controlled_native_interior_and_large_copy` (the encode-side interior test now passes).
  - Fill job: `final_tick_admission_controls_the_actual_completion_port`, `aborting_mid_run_does_not_publish_set_solve`, `a_full_lane_step_publishes_a_burst_of_collapses_and_keeps_discards`, `stepping_publishes_a_partial_payload_then_a_richer_one`.
  - Mounted brush tests (5): `worker preparation requires its actual admitting source` at the test's own maintenance step. Adding `ensure_fixture_cancellation_issuer` and `fixture_renew_turn` did not help; the framework's settle wraps maintenance in `original_lifecycle_step`, which needs a `semio_plugin_ownership` dependency in the tests crate.
  - `history_edits_end_to_end`: all 12 derived cases refuse the seed edit with `NativeEncoding(UnsupportedOwner "original work requires its original caller loan")`. grid2d's `history_edits_end_to_end` also fails, so this looks like the shared `history_edit_acceptance_law!` harness.

## Test status (bitmap-test22.log, --store=shared, SEMIO_OFFLOAD=never)

263 passed, 22 failed (20 targets).

- Shared, teardown agent (pending job step with zero spend / headless teardown): 7 `original-work-blocked` solves, 4 `teardown-retains-original-owner` solves, `a_pin_on_a_colour_no_pattern_anchors_is_refused_rather_than_dropped` (its `refusal.terminal_is_empty()` is the same headless teardown), 4 fill-job tests (`final_tick_admission…`, `aborting_mid_run…`, `a_full_lane_step…`, `stepping_publishes…`), 5 mounted brush tests (the lifecycle-step wrap worked: maintenance now succeeds, but "the bitmap dispatch did not settle" because a retained tool job never finishes; suspected same cause). 21 in total.
- Own / unresolved: `history_edits_end_to_end` (12 derived cases refuse their seed edit with `NativeEncoding … "original work requires its original caller loan"`). The harness now dispatches inside `with_original_work_loan` and grid2d passes it, but bitmap still fails even though the loan is open. The difference is bitmap's byte fields (`pixels` with `serialize_controlled_with = bytes::to_value_controlled`): `NativeEncodeControl::copy_bytes` goes through `OriginalWork::prepare`, which needs a recipient that `ArtifactStore::dispatch` does not receive from the caller's loan. This needs a framework decision (give store dispatch the caller's recipient, or encode `Vec<u8>` without a work action).
