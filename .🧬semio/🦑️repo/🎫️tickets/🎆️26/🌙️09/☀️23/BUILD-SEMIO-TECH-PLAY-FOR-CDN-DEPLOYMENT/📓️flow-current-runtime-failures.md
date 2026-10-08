# Flow Current Runtime Failures

Actual whole177-suite result:174passed3failed0skipped.

```text
test editor::flow::commands::add_widget::tests::rename_rejects_blank_unchanged_and_taken_ids ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.23s

        PASS [   0.253s] ( 10/177) semio-s-artifact-flow-flow editor::flow::commands::add_widget::tests::rename_rejects_blank_unchanged_and_taken_ids
       START [         ] ( 11/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication

running 1 test

thread 'editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication' (1550921) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connect-media-ports/🧪️tests/🔬️unit/🦀️.rs:30:13:
assertion `left == right` failed
  left: 3
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication ... FAILED

failures:

failures:
    editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.00s

        FAIL [   0.029s] ( 11/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication
       START [         ] ( 12/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::removal_routes_publish_original_typed_owners

running 1 test
[DEBUG] Flow removeWidget published the original typed child owner; serde_json scene census matched widgets=2 synapses=0
[DEBUG] Flow disconnect published the original typed child owner; serde_json scene census matched widgets=3 synapses=1
[DEBUG] Flow deleteSelection published the original typed child owner; serde_json scene census matched widgets=2 synapses=1
test editor::flow::commands::connect_media_ports::tests::removal_routes_publish_original_typed_owners ... ok

```

```text
test editor::flow::commands::node_graph_edit::tests::a_dragged_inline_slider_is_one_child_transaction_and_a_cancel_leaves_zero_trace ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.14s

        PASS [   0.159s] ( 27/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_dragged_inline_slider_is_one_child_transaction_and_a_cancel_leaves_zero_trace
       START [         ] ( 28/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane

running 1 test

thread 'editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane' (1551140) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:409:5:
assertion `left == right` failed: b folds the edited offset
  left: (284.0, 48.0)
 right: (100.0, 48.0)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane ... FAILED

failures:

failures:
    editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.05s

        FAIL [   0.069s] ( 28/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane
       START [         ] ( 29/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_node_drag_is_one_child_transaction_row_naming_its_member_store

running 1 test
test editor::flow::commands::node_graph_edit::tests::a_node_drag_is_one_child_transaction_row_naming_its_member_store ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.10s

        PASS [   0.121s] ( 29/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_node_drag_is_one_child_transaction_row_naming_its_member_store
```

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.00s

        PASS [   0.021s] (113/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_mutations_match_neutral_fixture_and_codecs
       START [         ] (114/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows

running 1 test
[TRACE] Flow exact-window runtime failure before close: Fault { origin: Plugin, code: FaultCode("plugin.internal.document-archive-replacement.closure-rejected"), severity: Error, message: "document archive replacement failed its closure leg: recursive ownership closure validation rejected the candidate (Incomplete)", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }

thread 'flow-window-ownership-law' (1560087) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:158:25:
Flow exact-window ownership runtime law: "Fault { origin: Plugin, code: FaultCode(\"plugin.internal.document-archive-replacement.closure-rejected\"), severity: Error, message: \"document archive replacement failed its closure leg: recursive ownership closure validation rejected the candidate (Incomplete)\", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows' (1560086) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:163:10:
Flow window ownership law thread: Any { .. }
test editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows ... FAILED

failures:

failures:
    editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.17s

        FAIL [   0.191s] (114/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows
       START [         ] (115/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::options::grid::tests::the_factor_slider_range_matches_the_command_handler_clamp

running 1 test
test editor::flow::modes::edit::windows::main::options::grid::tests::the_factor_slider_range_matches_the_command_handler_clamp ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 176 filtered out; finished in 0.00s

        PASS [   0.020s] (115/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::options::grid::tests::the_factor_slider_range_matches_the_command_handler_clamp
```

```text
        PASS [   0.021s] (155/177) semio-s-artifact-flow-flow standards::v1::subsets::any::io::component::text::snapshot::tests::example_fixture_dsl_round_trips
        PASS [   0.019s] (156/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::component::tests::widget_id_and_kind_label_agree_across_variants
        PASS [   0.021s] (157/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::a_whole_artifact_diff_wins_over_every_content_diff
        PASS [   0.025s] (158/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::flow_diff_codecs_preserve_the_neutral_schema_and_child_handles
        PASS [   0.027s] (159/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law
        PASS [   0.021s] (160/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law
        PASS [   0.026s] (161/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::topology_counts_every_widget_exactly_once
        PASS [   0.023s] (162/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_widget
        PASS [   0.021s] (163/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::linear_chain_orders_roots_before_leaves_with_increasing_depth
        PASS [   0.021s] (164/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::mutations::component::tests::the_parent_vocabulary_is_empty_and_refuses_every_operation
        PASS [   0.019s] (165/177) semio-s-artifact-flow-flow tests::artifact_kind_names_the_store_schema
        PASS [   0.023s] (166/177) semio-s-artifact-flow-flow tests::authored_slider_labels_survive_child_content_round_trip
        PASS [   0.027s] (167/177) semio-s-artifact-flow-flow tests::default_snapshot_has_widgets
        PASS [   0.027s] (168/177) semio-s-artifact-flow-flow tests::editor_tool_proofs_declare_the_artifact_document_schema
        PASS [   0.041s] (169/177) semio-s-artifact-flow-flow tests::flow_scene_owner_fixture_is_language_neutral_and_bounded
        PASS [   0.029s] (170/177) semio-s-artifact-flow-flow tests::flow_scene_owner_holds_identity_isolation_aba_wire_omission_and_close
        PASS [   0.035s] (171/177) semio-s-artifact-flow-flow tests::widget_content_round_trips_through_the_composed_child_snapshot
        PASS [   0.031s] (172/177) semio-s-artifact-flow-flow viewer::flow::component::tests::create_flow_viewer_builds_a_definition_for_the_viewer_role
        PASS [   0.063s] (173/177) semio-s-artifact-flow-flow viewer::flow::component::tests::flow_viewer_member_factory_and_full_store_close_match_neutral_contract
        PASS [   0.058s] (174/177) semio-s-artifact-flow-flow viewer::flow::component::tests::viewer_dialect_matches_the_artifact_coordinate
        PASS [   0.025s] (175/177) semio-s-artifact-flow-flow viewer::flow::modes::view::component::tests::the_layout_lists_the_single_view_window
        PASS [   0.032s] (176/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::definition_declares_the_node_graph_surface_and_body_key
        PASS [   0.067s] (177/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::renders_node_graph_scene_for_the_default_document
        FAIL [   0.029s] ( 11/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication
        FAIL [   0.069s] ( 28/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane
        FAIL [   0.191s] (114/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows
error: test run failed
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/native/semio-nextest-TjWUF9
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
```

```text
        PASS [   0.019s] (156/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::component::tests::widget_id_and_kind_label_agree_across_variants
        PASS [   0.021s] (157/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::a_whole_artifact_diff_wins_over_every_content_diff
        PASS [   0.025s] (158/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::flow_diff_codecs_preserve_the_neutral_schema_and_child_handles
        PASS [   0.027s] (159/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law
        PASS [   0.021s] (160/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law
        PASS [   0.026s] (161/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::topology_counts_every_widget_exactly_once
        PASS [   0.023s] (162/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_widget
        PASS [   0.021s] (163/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::linear_chain_orders_roots_before_leaves_with_increasing_depth
        PASS [   0.021s] (164/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::mutations::component::tests::the_parent_vocabulary_is_empty_and_refuses_every_operation
        PASS [   0.019s] (165/177) semio-s-artifact-flow-flow tests::artifact_kind_names_the_store_schema
        PASS [   0.023s] (166/177) semio-s-artifact-flow-flow tests::authored_slider_labels_survive_child_content_round_trip
        PASS [   0.027s] (167/177) semio-s-artifact-flow-flow tests::default_snapshot_has_widgets
        PASS [   0.027s] (168/177) semio-s-artifact-flow-flow tests::editor_tool_proofs_declare_the_artifact_document_schema
        PASS [   0.041s] (169/177) semio-s-artifact-flow-flow tests::flow_scene_owner_fixture_is_language_neutral_and_bounded
        PASS [   0.029s] (170/177) semio-s-artifact-flow-flow tests::flow_scene_owner_holds_identity_isolation_aba_wire_omission_and_close
        PASS [   0.035s] (171/177) semio-s-artifact-flow-flow tests::widget_content_round_trips_through_the_composed_child_snapshot
        PASS [   0.031s] (172/177) semio-s-artifact-flow-flow viewer::flow::component::tests::create_flow_viewer_builds_a_definition_for_the_viewer_role
        PASS [   0.063s] (173/177) semio-s-artifact-flow-flow viewer::flow::component::tests::flow_viewer_member_factory_and_full_store_close_match_neutral_contract
        PASS [   0.058s] (174/177) semio-s-artifact-flow-flow viewer::flow::component::tests::viewer_dialect_matches_the_artifact_coordinate
        PASS [   0.025s] (175/177) semio-s-artifact-flow-flow viewer::flow::modes::view::component::tests::the_layout_lists_the_single_view_window
        PASS [   0.032s] (176/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::definition_declares_the_node_graph_surface_and_body_key
        PASS [   0.067s] (177/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::renders_node_graph_scene_for_the_default_document
        FAIL [   0.029s] ( 11/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication
        FAIL [   0.069s] ( 28/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane
        FAIL [   0.191s] (114/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows
error: test run failed
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/native/semio-nextest-TjWUF9
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
```

```text
        PASS [   0.021s] (157/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::a_whole_artifact_diff_wins_over_every_content_diff
        PASS [   0.025s] (158/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::diff::component::tests::flow_diff_codecs_preserve_the_neutral_schema_and_child_handles
        PASS [   0.027s] (159/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law
        PASS [   0.021s] (160/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law
        PASS [   0.026s] (161/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::component::tests::topology_counts_every_widget_exactly_once
        PASS [   0.023s] (162/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_widget
        PASS [   0.021s] (163/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::inferences::topology::component::tests::linear_chain_orders_roots_before_leaves_with_increasing_depth
        PASS [   0.021s] (164/177) semio-s-artifact-flow-flow standards::v1::subsets::any::schema::mutations::component::tests::the_parent_vocabulary_is_empty_and_refuses_every_operation
        PASS [   0.019s] (165/177) semio-s-artifact-flow-flow tests::artifact_kind_names_the_store_schema
        PASS [   0.023s] (166/177) semio-s-artifact-flow-flow tests::authored_slider_labels_survive_child_content_round_trip
        PASS [   0.027s] (167/177) semio-s-artifact-flow-flow tests::default_snapshot_has_widgets
        PASS [   0.027s] (168/177) semio-s-artifact-flow-flow tests::editor_tool_proofs_declare_the_artifact_document_schema
        PASS [   0.041s] (169/177) semio-s-artifact-flow-flow tests::flow_scene_owner_fixture_is_language_neutral_and_bounded
        PASS [   0.029s] (170/177) semio-s-artifact-flow-flow tests::flow_scene_owner_holds_identity_isolation_aba_wire_omission_and_close
        PASS [   0.035s] (171/177) semio-s-artifact-flow-flow tests::widget_content_round_trips_through_the_composed_child_snapshot
        PASS [   0.031s] (172/177) semio-s-artifact-flow-flow viewer::flow::component::tests::create_flow_viewer_builds_a_definition_for_the_viewer_role
        PASS [   0.063s] (173/177) semio-s-artifact-flow-flow viewer::flow::component::tests::flow_viewer_member_factory_and_full_store_close_match_neutral_contract
        PASS [   0.058s] (174/177) semio-s-artifact-flow-flow viewer::flow::component::tests::viewer_dialect_matches_the_artifact_coordinate
        PASS [   0.025s] (175/177) semio-s-artifact-flow-flow viewer::flow::modes::view::component::tests::the_layout_lists_the_single_view_window
        PASS [   0.032s] (176/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::definition_declares_the_node_graph_surface_and_body_key
        PASS [   0.067s] (177/177) semio-s-artifact-flow-flow viewer::flow::modes::view::windows::main::tests::renders_node_graph_scene_for_the_default_document
        FAIL [   0.029s] ( 11/177) semio-s-artifact-flow-flow editor::flow::commands::connect_media_ports::tests::connection_keeps_the_original_typed_owner_until_publication
        FAIL [   0.069s] ( 28/177) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane
        FAIL [   0.191s] (114/177) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows
error: test run failed
[TRACE] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/native/semio-nextest-TjWUF9
38 |       child.once("close", (code, signal) => accept({ code, signal }));
39 |     });
40 |     const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
41 |     if (!failure) return;
42 |     if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}`);
43 |     if (options.throwOnFailure) throw Error(failure);
```
## Complete Flow Native Retry Passed

Uncached full Flow run38223 actually completed:177 tests run,177 passed,zero failed,zero skipped,in117.761 seconds; Nx target test succeeded in18m15. The run preserved all ownership/retirement/lane/codec assertions. DEBUG runtime witnesses confirm three shared journal transitions with independent author-branch/peer-trunk heads, and a complete neutral owned-member archive restored through begin/poll/ack with document bytes unchanged. This closes all three failures from60060. Shared Value release-authority schema work is subsequent and still needs a final consistent build. Final four-page/browser acceptance remains pending.
