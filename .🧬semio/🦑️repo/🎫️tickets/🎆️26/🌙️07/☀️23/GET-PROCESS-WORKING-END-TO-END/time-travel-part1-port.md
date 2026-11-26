# Time-travel part-1 laws ported to the original-turn API

## Result
- `semio-plugin-tests-07-2`: 177 type errors to 0. `cargo check --tests` passes; no law deleted or weakened, every original assertion kept.
- `cargo test`: 28 laws, 11 pass, 17 fail. The 17 failures are runtime behaviour of the product under the new model, not type errors (see Open).
- Building: the managed `repo cargo` shim refuses to compile `syn` (host proc-macro, cold estimate 2686 MB above the 1536 MB broker grant, on both the shared and isolated store). The build ran through `repo capacity run --memory=3200..4000 -- tt1-cargo.sh` (private target dir, same pattern as `hub-build.sh`). The private dir (3 GB) and all outputs are deleted; `tt1-cargo.sh` stays.

## Approach
- Every turn-taking call now takes the test's `StepContext` (`cx`): `render_body`, `render_history`, `verb`, `relay`, `tick_backbone`, `load_document(_text)`, `hydrate_pure_head`, `render_history_window`, `seeded_mutation_node`.
- `handle_action` sites go through a new local `dispatch` (part-1 replicas file) over `artifact_app_laws::dispatch_fixture_action`; a refusal comes back as the `Fault` of `OriginalTurnFailure::primary_fault`, so `.err().map(|fault| fault.code)` assertions are unchanged. `history_verb` / `history_verb_refusal` settle with `settle_framework_reserved_admission(app, admitted, cx)`.
- `dispatch_emit` still exists but only on `DispatchCtx`: sites use `app.dispatch_ctx().dispatch_emit(..)`.
- Store seeds dispatch with `artifact_app_laws::fixture_identity()` (the identity with the original work lender); the plain test identity answers "original work requires its original caller loan".
- Owners-only laws (inert label op, unfunded replay): `TimeTravelStoreState::run` is synchronous and takes `Option<&mut StepContext>`; new helpers `replay_turn` (renewed turn under the given grant, so an underfunded turn is really underfunded), `replayed_report`, `committed_overwrite` (`commit_finished` request, then `StepCommit` turns), `close_inert`; the catalog is built by `cold_replay_document_store_owners` with `admit_constructor` paid through `with_original_effect`; those three laws run under an `OriginalFixtureCaller` context.
- `poll_load_turn` and `hydrate_pure` (renewed turns until the lane reports done) in the interior-undo file.

## Product and support fixes
- `#[cfg(test)]` to `#[cfg(any(test, feature = "artifact-app-testing"))]` on `TimeTravelLedger::set_turn_clock`, `TimeTravelLedger::editor_mut` and `RenderCtx::test_history` (companion crates could never reach them).
- tests-06 support (`🧪️tests/🧪️time-travel`): `original_time_travel_fixture_caller` now grants the interactive component policy (the authored 1-item / 16 KiB grant could not fund the 33,976 B history-replay preparation step, which made every seed `publish_as` fail with "fixture original receiving was not funded"); `apply_long_edit` uses `fixture_identity`; `commit_scenario` takes the caller's `cx` (its own inner caller could never retire next to the app's cancellation issuer; callers in part-1 and part-2 updated); `verb` acts as the app's own actor (host-event routing refuses a foreign actor) and pumps the draft preview to settled after an accepted verb.

## Open: failing laws with evidence
1. Operation wire issuers (same blocker as `cancel-receipt.md`): `a_blocking_mutation_without_editable_inputs...`, `an_inverse_refusal_is_one_mutations_fatal...` fail in the Report replay with "original operation has no authored conflict path issuer"; `every_fixture_scenario...`, `a_stage_change_ships...`, `editing_targets_through_the_selection...` fail in the draft preview with "original history replacement requires its retained semantic input issuer". The instance ingress installs only `semantic().with_batch_digest()`; this is the operation-wire work other agents hold uncommitted. Not touched.
2. Driver refusals that lose their effect: a base move while a preview or replay owner is live makes `perform_time_travel_effects` return "original preview cancellation must settle before another selected input" / "original history replay must close before its replacement is admitted" and the `ShowPreview` / `StartReplay` effect is dropped (`opening_a_history_edit_and_a_remote_edit...`, `cancel_keeps_drafts_and_a_remote_edit_replays_again`, `a_deferred_history_step...`, `the_undo_of_a_finalize_over_a_long_history...` never leaves Replaying). A correct fix keeps the effect queued on the ledger until the owner closed; I did not start that design change.
3. `an_opened_instance_acts_as_its_admitted_actor...`: "native history original publication frontier changed" while settling `revertToCommand`.
4. `an_alternative_history_edit_is_undone...`: "original finalization result lost its matching Session stage"; `undo_and_redo_of_a_finalize...` and `a_remote_edit_while_editing...` end with the wrong head (undo does not restore, overwrite misses the remote edit).
5. `a_pure_lane_hydrates...`: the archive load fails with "construction requires its original empty candidate" (document archive replacement initializer).
6. `a_session_edge_republishes...` (UI scope `Full` where `Partial` is asserted) and `a_mutation_row_offers_withdraw...` (preview not yet shown after Withdraw).
7. `an_unfunded_replay_step_defers...`: the replay part passes, `close_inert` leaves the owners non-terminal under the paid retirement turns (retire through `with_original_effect` inside `with_original_retirement`).

## Changed files
- `🧪️tests/🧪️time-travel-part-1/*` (root and all eight law files).
- `🧪️tests/🧪️time-travel/🦀️.rs` (tests-06 support), `🧪️tests/🧪️time-travel-part-2/*` only the two `commit_scenario(.., cx)` call sites.
- `⏪️time-travel/📒️time-travel-ledger/🦀️.rs`, `🧵️instance/🖼️render/📥️staging/📜️history-view/🦀️.rs` (cfg).
- Ticket: `tt1-cargo.sh`, `tt1-summ.py`, this file. No `[DEBUG]` left.
