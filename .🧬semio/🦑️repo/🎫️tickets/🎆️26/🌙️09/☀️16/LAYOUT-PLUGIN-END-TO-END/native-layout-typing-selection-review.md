# Native Layout Typing Selection Review

Source-only recheck; no builds, tests or native execution. Actual file is layout-native-review.md (the requested native-layout-review.md does not exist). The original framework batch 34369 remains root's sole native lane; this report starts no competing job.

## Current Selection

All fifteen previously selected package names still resolve uniquely to actual manifests in the owning Layout workspace at ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml. Each manifest is explicitly listed as a member and has an existing library source. The unchanged command below selects all fifteen library test roots explicitly; none is merely a dependency.

The transform companion library source begins #![cfg(test)] and unconditionally mounts ⌨️typing-edit/🦀️.rs at lines 668–669. The child contains two actual test attributes, without extra feature gates. Its include_str fixture resolves to the real framework SDK typing-edit fixture. The companion has direct first-party dependencies on plugin-typing-edit and typed-ops with artifact-app-testing; its editor dev-dependency enables testkit and job dev-dependency enables testkit. Editor's unit_tests module is mounted by cfg(any(test, feature = "testkit")) at editor source lines 249–251; its feature forwards plugin/artifact-app-testing. Thus the new child tests and their real registered Layout fixture are reachable under this actual selected library root when compilation succeeds.

## Actual Authored Tests

The current source has 19 test functions: 17 in the transform root and two in its mounted typing child. The earlier counts of sixteen/eighteen are stale. In particular direct_intent_qualified_action_rewrite_is_refused_without_control_effects is already present at root line 653. This is declaration/reachability evidence, not an executed test count.

- 🦀️.rs:146: one_gumball_drag_is_one_edit_one_row_and_one_transaction
- 🦀️.rs:173: a_cancelled_gesture_leaves_zero_trace
- 🦀️.rs:194: two_gestures_are_two_transactions
- 🦀️.rs:215: one_shot_turns_and_scalings_are_one_transaction_each
- 🦀️.rs:242: a_streamed_drag_of_two_frames_is_one_transaction
- 🦀️.rs:260: a_document_moved_under_an_open_gesture_aborts_it
- 🦀️.rs:274: mounted_original_follower_settles_and_closes_on_the_same_cancelled_source
- 🦀️.rs:389: a_gumball_drag_and_its_downstream_turn_edited_in_time_travel_replay_exactly
- 🦀️.rs:440: a_drag_edited_onto_a_missing_frame_blocks_finalize_until_its_targets_are_fixed
- 🦀️.rs:473: reference_chips_name_frames_by_kind_content_and_page
- 🦀️.rs:501: original_press_settlement_preserves_lane_backings_under_denial_commit_and_cancellation
- 🦀️.rs:540: original_tool_tag_capture_preserves_exact_metadata_and_closes_partial_text
- 🦀️.rs:563: original_metadata_copy_restores_after_expiry_without_renewing_clock_or_stock
- 🦀️.rs:589: original_typing_dispatch_retains_metadata_until_atomic_finish_or_cancel
- 🦀️.rs:616: direct_intent_typing_commit_requires_original_catalog_qualification
- 🦀️.rs:637: direct_intent_cancelled_qualification_closes_before_typing_control
- 🦀️.rs:653: direct_intent_qualified_action_rewrite_is_refused_without_control_effects
- ⌨️typing-edit/🦀️.rs:5: mounted_typing_edits_publish_previous_transaction_and_keep_latest_preview
- ⌨️typing-edit/🦀️.rs:17: original_typing_settlement_moves_actual_layout_leaves_and_keeps_cancelled_resident

The two new SDK/Store laws exercise the mounted registered app typing dispatch, previous transaction publication/latest preview/final commit, and original settlement ownership with actual LayoutMutation leaves, denial/one-item progress and cancellation. Their record prefixes are mounted-typing-edit= and original-typing-edit-custody=. They are in the same selected transform binary, rather than an unselected SDK dependency's private tests.

The directly inspected transform Rust files total 74,363 bytes (64,556 + 9,807), distinct from root's canonical five-file source-closure accounting. This review did not rerun the canonical source evaluator and does not independently claim its five-file/77,731-byte count.

## Exact Intended Command

Documented only, not executed:

```sh
repo --service=off cargo --store=shared --configuration=prototype test --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout-editor-transform-tests -p semio-s-artifact-layout-layout-engine -p semio-s-artifact-layout-layout -p semio-s-artifact-layout-layout-editor-tests -p semio-s-artifact-layout-layout-engine-scene-tests -p semio-s-artifact-layout-layout-engine-flow-tests -p semio-s-artifact-layout-layout-editor-commands-tests -p semio-s-artifact-layout-layout-mutations-content-tests-1 -p semio-s-artifact-layout-layout-mutations-content-tests-2 -p semio-s-artifact-layout-layout-mutations-frames-tests-1 -p semio-s-artifact-layout-layout-mutations-frames-tests-2 -p semio-s-artifact-layout-layout-diff-plan -p semio-s-artifact-layout-layout-sqlite-member-tests -p semio-s-artifact-layout-layout-engine-export-unit-tests -p semio-s-artifact-layout-layout-editor --lib --features=testkit -- --nocapture
```

This command has no test-name filter and uses --lib --features=testkit -- --nocapture. Upon successful compilation, selected private library roots and the two mounted child tests are eligible to execute. Actual per-test outcomes and the nineteen-test transform result require native output. The previous integrated Layout attempt failed during precompiler source admission and supplies no execution proof.
