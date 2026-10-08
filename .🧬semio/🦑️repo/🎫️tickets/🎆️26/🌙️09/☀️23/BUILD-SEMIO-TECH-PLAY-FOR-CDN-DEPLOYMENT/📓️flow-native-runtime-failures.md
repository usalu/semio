# Flow Native Runtime Failures

Actual full native Nx exit 1: 175 tests ran,158 passed,17 failed,1 existing ignored manual asset writer. The four-case neutral Flow diff text/binary codec passed and printed its runtime witness. Original assertions are retained; final full suite remains required.

## editor::flow::commands::disconnect::tests::disconnecting_an_unknown_synapse_is_refused_by_name

```text
       START [         ] ( 14/175) semio-s-artifact-flow-flow editor::flow::commands::disconnect::tests::disconnecting_an_unknown_synapse_is_refused_by_name
running 1 test
thread 'editor::flow::commands::disconnect::tests::disconnecting_an_unknown_synapse_is_refused_by_name' (1288125) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️disconnect/🧪️tests/🔬️unit/🦀️.rs:11:5:
disconnect found no synapse "nope"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::disconnect::tests::disconnecting_an_unknown_synapse_is_refused_by_name ... FAILED
```

## editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane

```text
       START [         ] ( 26/175) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane
running 1 test
thread 'editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane' (1288344) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:249:81:
historyEditCommit: Fault { origin: Framework, code: FaultCode("time-travel.actor"), severity: Error, message: "history edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::a_member_finalize_reaches_the_other_replica_on_the_member_lane ... FAILED
```

## editor::flow::commands::node_graph_edit::tests::a_release_that_wires_and_drags_is_labelled_by_the_drag

```text
       START [         ] ( 28/175) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::a_release_that_wires_and_drags_is_labelled_by_the_drag
running 1 test
thread 'editor::flow::commands::node_graph_edit::tests::a_release_that_wires_and_drags_is_labelled_by_the_drag' (1288378) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:328:5:
assertion `left == right` failed
  left: "Insert edge (+1)"
 right: "Drag 1 node by (10, 0)"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::a_release_that_wires_and_drags_is_labelled_by_the_drag ... FAILED
```

## editor::flow::commands::node_graph_edit::tests::editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows

```text
       START [         ] ( 29/175) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows
running 1 test
thread '
editor::flow::commands::node_graph_edit::tests::editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows
' (
1288391) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:
249:
81:
historyEditCommit: Fault { origin: Framework, code: FaultCode("time-travel.actor"), severity: Error, message: "history edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows ... FAILED
```

## editor::flow::commands::node_graph_edit::tests::finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child

```text
       START [         ] ( 30/175) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child
running 1 test
thread 'editor::flow::commands::node_graph_edit::tests::finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child' (1288413) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:249:81:
historyEditCommit: Fault { origin: Framework, code: FaultCode("time-travel.actor"), severity: Error, message: "history edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child ... FAILED
```

## editor::flow::commands::node_graph_edit::tests::withdrawing_a_node_drag_in_history_puts_the_node_back

```text
       START [         ] ( 40/175) semio-s-artifact-flow-flow editor::flow::commands::node_graph_edit::tests::withdrawing_a_node_drag_in_history_puts_the_node_back
running 1 test
thread 'editor::flow::commands::node_graph_edit::tests::withdrawing_a_node_drag_in_history_puts_the_node_back' (1288610) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:249:81:
historyEditCommit: Fault { origin: Framework, code: FaultCode("time-travel.actor"), severity: Error, message: "history edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::node_graph_edit::tests::withdrawing_a_node_drag_in_history_puts_the_node_back ... FAILED
```

## editor::flow::commands::set_active_example::tests::set_active_example_demo_loads_the_published_demo_graph

```text
       START [         ] ( 45/175) semio-s-artifact-flow-flow editor::flow::commands::set_active_example::tests::set_active_example_demo_loads_the_published_demo_graph
running 1 test
thread 'editor::flow::commands::set_active_example::tests::set_active_example_demo_loads_the_published_demo_graph' (1288953) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs:12:88:
demo parses: TextError { kind: InvalidValue, message: "expected Record, found Absent", span: TextSpan { line: 1, column: 1, length: 0 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::set_active_example::tests::set_active_example_demo_loads_the_published_demo_graph ... FAILED
```

## editor::flow::commands::toggle_extension::unit::an_unknown_extension_action_id_is_refused_by_name

```text
       START [         ] ( 53/175) semio-s-artifact-flow-flow editor::flow::commands::toggle_extension::unit::an_unknown_extension_action_id_is_refused_by_name
running 1 test
thread 'editor::flow::commands::toggle_extension::unit::an_unknown_extension_action_id_is_refused_by_name' (1289068) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🧪️tests/🔬️unit/🦀️.rs:30:5:
runExtensionAction has no extension action "third.party.nope"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::toggle_extension::unit::an_unknown_extension_action_id_is_refused_by_name ... FAILED
```

## editor::flow::commands::toggle_extension::unit::toggle_extension_and_run_action_reorganizes_fixture

```text
       START [         ] ( 54/175) semio-s-artifact-flow-flow editor::flow::commands::toggle_extension::unit::toggle_extension_and_run_action_reorganizes_fixture
running 1 test
thread 'editor::flow::commands::toggle_extension::unit::toggle_extension_and_run_action_reorganizes_fixture' (1289082) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🧪️tests/🔬️unit/🦀️.rs:14:5:
a disabled automation action is refused by name: runExtensionAction "flow.extension.reorganize" belongs to the disabled extension "auto-layout"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::commands::toggle_extension::unit::toggle_extension_and_run_action_reorganizes_fixture ... FAILED
```

## editor::flow::component::declared_verb_laws::every_declared_flow_verb_honours_its_declaration

```text
       START [         ] ( 56/175) semio-s-artifact-flow-flow editor::flow::component::declared_verb_laws::every_declared_flow_verb_honours_its_declaration
running 1 test
[cargo:assert] running elapsedMs=20004
[native:owner-command] running elapsedMs=1110269
[artifact-rust:semio-s-artifact-flow-flow:test] running elapsedMs=1110379
thread 'editor::flow::component::declared_verb_laws::every_declared_flow_verb_honours_its_declaration' (1289104) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📨️emission/📦️preparation/🦀️.rs:69:24:
child emission preparation dropped before its exact typed owners and accepted prefix were returned or retired
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'editor::flow::component::declared_verb_laws::every_declared_flow_verb_honours_its_declaration' (1289104) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:8084:85:
registered fixture close: Fault { origin: Framework, code: FaultCode("interactive-job.instance-owner-poisoned"), severity: Error, message: "app-instance operation owner is poisoned", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
test editor::flow::component::declared_verb_laws::every_declared_flow_verb_honours_its_declaration ... FAILED
```

## editor::flow::component::declared_verb_laws::the_content_child_is_the_one_scene_every_verb_and_window_reads

```text
       START [         ] ( 57/175) semio-s-artifact-flow-flow editor::flow::component::declared_verb_laws::the_content_child_is_the_one_scene_every_verb_and_window_reads
running 1 test
thread 'editor::flow::component::declared_verb_laws::the_content_child_is_the_one_scene_every_verb_and_window_reads' (1289929) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️declared-verbs/🦀️.rs:40:5:
the main window renders the widget the child edit added
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::component::declared_verb_laws::the_content_child_is_the_one_scene_every_verb_and_window_reads ... FAILED
```

## editor::flow::component::interactive_job_tests::every_graph_operation_route_is_admitted_by_its_own_retained_factory

```text
       START [         ] ( 62/175) semio-s-artifact-flow-flow editor::flow::component::interactive_job_tests::every_graph_operation_route_is_admitted_by_its_own_retained_factory
running 1 test
thread 'editor::flow::component::interactive_job_tests::every_graph_operation_route_is_admitted_by_its_own_retained_factory' (1289971) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/📨️emission/📦️preparation/🦀️.rs:69:24:
child emission preparation dropped before its exact typed owners and accepted prefix were returned or retired
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'editor::flow::component::interactive_job_tests::every_graph_operation_route_is_admitted_by_its_own_retained_factory' (1289971) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:139:124:
retained Flow command publication: Fault { origin: Framework, code: FaultCode("interactive-job.app-owned-output"), severity: Error, message: "job-session.terminal-fault", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
test editor::flow::component::interactive_job_tests::every_graph_operation_route_is_admitted_by_its_own_retained_factory ... FAILED
```

## editor::flow::component::unit_tests::action_cohort_fixtures_match_the_exact_route_census

```text
       START [         ] ( 73/175) semio-s-artifact-flow-flow editor::flow::component::unit_tests::action_cohort_fixtures_match_the_exact_route_census
running 1 test
thread 'editor::flow::component::unit_tests::action_cohort_fixtures_match_the_exact_route_census' (1290023) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:337:9:
assertion `left == right` failed: NotePlayApp: the retained route index must exactly name the migrated groups
  left: ["setGridVisible", "setGridSpacing", "setGridSubdivisions", "setGridOpacity", "setSnapEnabled", "setSnapGridSpacing", "setPencilWidth", "setEraserRadius", "addBlock", "moveBlock", "deleteBlock", "deleteSelection", "duplicateBlock", "duplicateSelection", "patchBlocks", "nudgeSelection", "nudgeSelectionUp", "nudgeSelectionDown", "nudgeSelectionLeft", "nudgeSelectionRight", "nudgeSelectionUpFast", "nudgeSelectionDownFast", "nudgeSelectionLeftFast", "nudgeSelectionRightFast", "setActiveExample", "loadDocumentJson", "navigatorEngagementInput", "saveDownload", "inkApplyEvents", "engagementSubmit", "setCamera", "setCameraZoom", "engagementInput"]
 right: ["setGridVisible", "setGridSpacing", "setGridSubdivisions", "setGridOpacity", "setSnapEnabled", "setSnapGridSpacing", "setPencilWidth", "setEraserRadius", "addBlock", "moveBlock", "deleteBlock", "deleteSelection", "duplicateBlock", "duplicateSelection", "patchBlocks", "setActiveExample", "loadDocumentJson", "navigatorEngagementInput", "saveDownload", "inkApplyEvents", "engagementSubmit", "nudgeSelection", "nudgeSelectionUp", "nudgeSelectionDown", "nudgeSelectionLeft", "nudgeSelectionRight", "nudgeSelectionUpFast", "nudgeSelectionDownFast", "nudgeSelectionLeftFast", "nudgeSelectionRightFast", "setCamera", "setCameraZoom", "engagementInput"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::flow::component::unit_tests::action_cohort_fixtures_match_the_exact_route_census ... FAILED
```

## editor::flow::modes::edit::windows::main::config::tests::flow_two_window_config_commands_in_one_turn_both_land

```text
       START [         ] (110/175) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_two_window_config_commands_in_one_turn_both_land
running 1 test
Flow one-turn window-config failure before close: Fault { origin: Framework, code: FaultCode("window-config.actor"), severity: Error, message: "window config edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
thread 'flow-window-config-turn-law' (1290294) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:251:25:
Flow two window-config commands in one turn: "Fault { origin: Framework, code: FaultCode(\"window-config.actor\"), severity: Error, message: \"window config edit actor differs from its opened session\", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'editor::flow::modes::edit::windows::main::config::tests::flow_two_window_config_commands_in_one_turn_both_land' (1290293) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:256:10:
Flow one-turn window config law thread: Any { .. }
test editor::flow::modes::edit::windows::main::config::tests::flow_two_window_config_commands_in_one_turn_both_land ... FAILED
```

## editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows

```text
       START [         ] (112/175) semio-s-artifact-flow-flow editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows
running 1 test
[TRACE] Flow exact-window runtime failure before close: Fault { origin: Framework, code: FaultCode("window-config.actor"), severity: Error, message: "window config edit actor differs from its opened session", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
thread 'flow-window-ownership-law' (1290310) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:158:25:
Flow exact-window ownership runtime law: "Fault { origin: Framework, code: FaultCode(\"window-config.actor\"), severity: Error, message: \"window config edit actor differs from its opened session\", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows' (1290309) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs:163:10:
Flow window ownership law thread: Any { .. }
test editor::flow::modes::edit::windows::main::config::tests::flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows ... FAILED
```

## standards::v1::subsets::any::io::component::binary::snapshot::tests::pack_round_trips_and_agrees_with_dsl

```text
       START [         ] (142/175) semio-s-artifact-flow-flow standards::v1::subsets::any::io::component::binary::snapshot::tests::pack_round_trips_and_agrees_with_dsl
running 1 test
thread 'standards::v1::subsets::any::io::component::binary::snapshot::tests::pack_round_trips_and_agrees_with_dsl' (1290525) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:6:105:
parse default snapshot: TextError { kind: InvalidValue, message: "expected child_id, found Some(Absent)", span: TextSpan { line: 1, column: 1, length: 0 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::binary::snapshot::tests::pack_round_trips_and_agrees_with_dsl ... FAILED
```

## standards::v1::subsets::any::io::component::text::snapshot::tests::example_fixture_dsl_round_trips

```text
       START [         ] (153/175) semio-s-artifact-flow-flow standards::v1::subsets::any::io::component::text::snapshot::tests::example_fixture_dsl_round_trips
running 1 test
thread 'standards::v1::subsets::any::io::component::text::snapshot::tests::example_fixture_dsl_round_trips' (1290903) panicked at 🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:5:76:
parse default snapshot: TextError { kind: InvalidValue, message: "expected child_id, found Some(Absent)", span: TextSpan { line: 1, column: 1, length: 0 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::text::snapshot::tests::example_fixture_dsl_round_trips ... FAILED
```

