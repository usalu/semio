# Runtime fix tests (2026-11-27)

All runs: plain cargo, `-j 2`, target `/Users/ueli/t/src-tests` (deleted at the end). Logs: `🗑️generated/tests-*.log`.

## 1. Revert replay fix (retire_controlled / retire_ticket_controlled) - PASS
- No `history-revert-edit` report existed; the fix was taken from the working-tree diff of `🏪️store/🕰️history/🏁️replay/🦀️.rs` and `abort.rs` (selected original birth runs on its own turn before the close effect).
- New `retire-tests.rs` in the replay crate (mounted `#[cfg(test)]`, dev-deps added): `a_ticket_runs_its_selected_birth_on_its_own_turn_then_closes_without_refusal` and `an_effect_shaped_close_waits_for_its_birth_turn_and_then_runs_once_per_funded_turn`. Both pass (`cargo test -p semio-os-store-engine-replay retire_tests`).
- Shared fixtures added to `OriginalJobFixtureCaller` (`🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller`): `establish_decoder_receipt` (decoder owns the retirement receipt native-control closes require) and `close_decoder`. Existing replay `*-tests.rs` files are not mounted in any crate (orphans).

## 2. Shared publication window receipt - PASS (behavioural, indirect)
- `retirement_receipts_cover_the_whole_publication_window_under_every_host_grant` (editor `🧬️app/🦀️.rs`): two back-to-back module toggles under host per-turn grants 64/520K, 48/400K, 32/260K items/copy; each retires its operations in a window and must settle and apply. Passes. It cannot observe the window receipt itself (no hook), it fails only if the reconcile refuses the turn.
- Smaller grants (16/128K, 8, 4, 2) were NOT silently dropped. Case (a): they are below the instance birth quote (`ToolCancellationIssuer::admission_demands`, `♻️cancellation/🎟️issuer/🦀️.rs:68`; the issuer admits nothing while the grant is below it). They fail at the first dispatch in the fixture's `ensure_fixture_cancellation_issuer` ("cancellation issuer birth exceeded its authored turn bound"), before any operation, publication window or retirement exists. Asserted by `a_host_grant_below_the_instance_birth_quote_is_refused_before_any_operation_exists` (passes). Not shown: the host's real minimum is not documented in a single constant (headless host grant is 64 items / 1 MiB, `🧵️job/🏃️headless`, the browser 64/520K is only a test doc); I did not find a product defect, but the exact minimum was not computed.
- Earlier "hang" reports: the 900 s/420 s timeouts were cargo compile time under load 29 (the shared worktree rebuilt deps), not a test hang: run directly the test binary finishes in 1.9 s.

## 3. Discard staging receipt - PASS after fixing the host crate
- The test lives in `semio-plugin-tests-15` (via `testing_laws_unit_4` -> `time_travel_tests_2`), not `semio-plugin-tests-14-2` (14-2 hosts builder-contract-part-4 and reactor-driver; not compiled by me).
- Compile fixes in tests-15 (no shims): `cx` threaded into `verb`/`render_history`/`render_body`/`handle_action` calls (31 call sites in the part-2 files), `dispatch_emit` via `dispatch_ctx()`, `handle_action` replaced by `dispatch_fixture_action`, `added_transitions` via `transitions.get`, `TimeTravelBase/Target` imports, `refuse_validator` gated `cfg(any(test, feature="artifact-app-testing"))`, builder `schema-stamping` and `dependency-contribution` fixtures moved to current types (`TestSnapshot`/`TestMutation`, `ArtifactDsl` impl, orphan sqlite impls removed with the unused `.sql`), `semio-plugin` prelude import, dev-deps (`semio-plugin`, `semio-framework-schema`, `-trace`, `-value-testing-admission`, `semio-framework-job`).
- Harness fix: `collect_original_history_command_receipt` (`🧪️time-travel/🦀️.rs`) asserted `grant == mounted maintenance policy`, the old (buggy) invariant; now asserts the grant is within the policy.
- `a_discard_staging_receipt_never_exceeds_the_remaining_caller_authority`: PASS.
- Remaining part-2 failures (31 pass, 8 fail; runtime drift unrelated to the discard fix, NOT fixed): `a_mutation_row_names_why_edit_or_withdraw_is_refused`, `long_option_rows_state_their_choice_as_selected` (UI value arena has no accepting lifetime registration), `an_overwrite_head_equals_a_fresh_fold_of_the_edited_log`, `every_route_authors_as_its_acting_actor`, `several_drafts_from_a_review_finalize_as_one_overwrite_supersede`, `the_history_view_patches_only_the_rows_a_change_touched` (direct `store.dispatch` outside an original caller loan: "original work requires its original caller loan"), `time_travel_actor_session_retirement_original_inline_custody` (receipt mismatch 1 item/10 B vs 0), `replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status` (progress total 0 not bounded).

## 4. Back-to-back toggles while publishing - PASS
- `back_to_back_module_toggles_sent_while_the_first_still_publishes_both_apply`: pass (`-p semio-s-artifact-sourcing-curation-editor-tests --test editor_app`, run from `✏️s/.../🗂️curation`).

## 5. Discard preview cleanup (added by coordinator) - PASS
- `a_selected_preview_base_keeps_its_cleanup_issuers_across_retirement_drives_until_discard` (part-2 retirement file): begin, input, three ordinary retirement pumps while the preview base is live, discard, pump, close. Passes. Not verified against the pre-fix code (cannot revert peers' product edits).

## Notes
- Workspace root for plain cargo is `/Users/ueli/Documents/semio` (replay, tests-15) and `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation` (editor tests).
- A peer's mid-edit of `StepContext::cancelled_turn` broke one compile transiently; resolved by itself.
