# L30: 3D import budget, clipboard, source-string laws, settlement ledger

All fixes are source-verified only. No cargo build or test was run. Every test below is UNVERIFIED.
E = /home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor

## Cluster 1: import wire budget (5 tests plus the action-producer law)
- Tests: target_volume_action_and_undo_reach_the_command_through_the_framework_declared_producer, a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle, a_chunked_pick_reaches_import_snapshot_as_one_whole_file_through_the_framework_staging, an_import_above_the_export_budget_refuses_with_a_notice, import_snapshot_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations (the last also fails on cluster 5).
- Cause: the current contract is the 5D one. ArtifactEditor::original_action_command_producer() is the declared producer; the framework default (declared_action_command_bridge) encodes every command at RUNTIME_COMMAND_WIRE_MAXIMUM (64 KiB). The L17b override in E/🧩️play-app/🦀️.rs was no longer in the tree, and the test (correctly) asserts it is Some. The retained producers in build_tool_job already pass PUZZLE3D_IMPORT_RAW_BYTES, so only the wire-encoding side was missing. The test expectation is current.
- Fix: added `original_action_command_producer` to `impl ArtifactEditor for Puzzle3dPlayApp` returning `OriginalActionCommandBridge(Self::command_from_action, |command| importSnapshot => PUZZLE3D_IMPORT_RAW_BYTES, else RUNTIME_COMMAND_WIRE_MAXIMUM)`, the same shape as the 5D editor.
- File: E/🧩️play-app/🦀️.rs

## Cluster 5: source-string laws (exportSnapshot, importSnapshot, openImportSnapshot)
- Cause: the law is outdated, the code is right. The tests `include_str!("../../../🦀️.rs")` (E/🦀️.rs), but the `"importSnapshot" => bound!(... Puzzle3dWindowCommandWork ...)` arms live in E/🧩️play-app/🦀️.rs since the play-app split. The arms are present and none routes to BoundedFirstStep.
- Fix: the three include_str paths now point at `../../../🧩️play-app/🦀️.rs`.
- Tests: leftover_export_snapshot_downloads_the_boot_example_json, import_snapshot_of_a_distinct_two_object_json_against_a_one_object_live_fixture_emits_operations (law half), leftover_open_import_snapshot_requests_file_open (same stale path, previously masked).
- Files: E/🧪️tests/🔬️unit/🧈️relocate-clipboard-and-snapshot-defect-laws/🦀️.rs (2 sites), E/🧪️tests/🔬️unit/🥓️segmented-export-and-leftover-defect-laws/🦀️.rs (1 site).

## Cluster 4a: copy_then_paste_clones_selection_as_one_mutation (two copies of the test)
- Cause: outdated assertion. paste is a framework reserved job (Puzzle3dClipboardJob) whose edit commits through the typed operation lane, so the admitted InvocationResult.mutations is empty by design (the sibling clipboard_actions_apply_anchor... test pastes the same way and counts objects, and passes).
- Fix: removed the `!pasted.mutations.is_empty()` assertion; the object count delta of exactly one and the fresh id assertions remain.
- Files: E/🧪️tests/🔬️budgets/📥️snapshot-exchange-laws/🦀️.rs, E/🧪️tests/🔬️unit/🧈️relocate-clipboard-and-snapshot-defect-laws/🦀️.rs

## Cluster 4b: gesture_kinds::cancelling_clipboard_admission_{releases_the_unpublished_mount,restores_the_unpublished_operation_slot}
- Failure (from the older log l12-3d-results.log, gesture-kinds line 293): "the cancelled caller must release its empty mount shell".
- Cause (framework): `OriginalEmissionAdmission::matches` is `!source_released && matches_retirement`. Once the cancelled source's sub-owners are closed, source_released flips true, so `owns_original_reserved_mount` turns false and `maintenance_source_terminal_is_empty` reports true while the empty shell is still in `media.reserved_mount`. The shell-taking branch of `close_original_reserved_mount` (the `owner.terminal_is_empty()` arm, which already has its own demand quote) is therefore never reached.
- Fix: `owns_original_reserved_mount` uses `owner.matches_retirement(cx)` (shell present for the original caller and decoder), so the shell stays owned until the closing step takes it.
- File: /home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🔖️reserved-mount/🦀️.rs
- Risk: this is framework code near L31's close-custody area, and the test's phase 4 path (slot restore) is only inferred to share the cause. Medium confidence.

## Cluster 7a: settlement_work_ledger_is_finite_and_progresses
- Cause: test bug. The loop dropped the presented TypedOperationResultPage after reading its token; TypedOperationResultPage::Drop asserts the body is Cold or Empty, so a live page panics "typed result page reached Drop before funded original or receiver closure". The contract is `close_fixture_result_page(&mut page, grant)` first, then ack the returned token (as the 2D pump_until does).
- Fix: `let mut page`, `let token = close_fixture_result_page(&mut page, policy.close)`, ack with that token.
- File: E/🧪️tests/🔬️action-producer/🦀️.rs
- Caveat: the [DEBUG] "one-item publication blocked" probe is another agent's; if the settlement still fails after this, the next failure is a different one.

## Cluster 7b: leftover_export_snapshot_downloads_the_boot_example_json
- Covered by cluster 5 (stale include_str path).
