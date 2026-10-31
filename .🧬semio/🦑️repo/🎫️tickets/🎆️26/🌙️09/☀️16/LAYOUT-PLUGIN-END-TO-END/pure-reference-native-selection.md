# Pure Reference Native Selection

Read-only source review; no builds, tests, native execution or source edits. Root's current batch remains untouched. This companion is a separate future native gate: testing only semio-framework-tool-machine --lib does not execute it.

## Owning Selection And Mounts

Package semio-framework-tool-machine-tests has its actual manifest at framework/tool-machine/tests/companion/packages/rust/Cargo.toml, with workspace ../../../../../.. resolving to the owning 🧰️framework/Cargo.toml. That workspace explicitly lists the companion member. Its [lib] path ../../🦀️.rs resolves to the actual companion root, which has #![cfg(test)]. The root mounts the unit-law aggregator, node-graph-edit-rows and gesture-drive-law. Unit aggregation includes fixture readers plus reducer/id, gesture tools, runner, scrub, typing and node drag/once laws. The manifest enables core testkit hooks and supplies machine, replication, value, serde_json and third-party blake3 for the independent transaction-id oracle.

Exact intended command, documented only:

```sh
repo --service=off cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-tool-machine-tests --lib -- --nocapture
```

The native Cargo subcommand follows --, as required by the build skill. No test-name filter is applied. Source declares forty test functions across eight law files, mounted through eleven Rust files. These are authored source counts, not executed totals or a rerun of canonical source admission.

## Authored Laws

### 🔬️unit/🏃️runner-laws/🦀️.rs (4)

- a_yield_while_entering_the_initial_configuration_is_refused
- a_yield_after_the_close_within_one_event_publishes_nothing
- timers_route_to_the_host_and_settle_like_events
- a_host_abort_cancels_the_left_timers_and_invokes_and_rests_the_tool

### 🔬️unit/🌊️gesture-tool-laws/🦀️.rs (4)

- every_statechart_is_its_fixture_chart
- runner_replays_every_gesture_scenario
- resume_admits_exactly_the_fixture_rows
- press_drag_release_is_one_transaction_and_escape_leaves_zero_trace

### 🔬️unit/🫧️scrub-laws/🦀️.rs (7)

- scrub_tables_are_the_statechart_compilation_of_the_scrub_chart
- scrub_chart_is_the_fixture_chart
- scrub_phases_parse_like_the_fixture
- scrub_ledger_replays_every_fixture_scenario
- a_scrub_persisted_between_ticks_continues_the_same_press
- the_scrub_ledger_never_refuses_and_late_inputs_change_nothing
- a_persisted_scrub_of_another_chart_is_refused

### 🔬️unit/🧮️reducer-and-id-laws/🦀️.rs (5)

- states_yield_kinds_refusals_and_reasons_mirror_the_fixture
- reducer_matches_every_matrix_row
- resume_upserts_entries_in_order
- reducer_matches_every_case
- replication_mint_and_the_blake3_crate_reproduce_every_fixture_id

### 🔬️unit/⌨️typing-laws/🦀️.rs (7)

- typing_tables_are_the_statechart_compilation_of_the_typing_chart
- typing_chart_is_the_fixture_chart
- typing_phases_parse_like_the_fixture
- typing_algebra_folds_like_the_fixture
- typing_ledger_replays_every_fixture_scenario
- a_typing_run_persisted_between_edits_continues_the_same_run
- two_hundred_typed_characters_are_one_run

### 🔬️unit/🕸️node-drag-and-once-laws/🦀️.rs (4)

- node_drag_rows_decode_to_their_records_and_invalid_rows_are_refused
- a_released_node_drag_commits_one_transaction_of_its_leaves
- a_one_step_tool_commits_one_transaction_of_its_leaves
- independent_one_step_dispatches_never_share_a_transaction

### 🧪️node-graph-edit-rows/🦀️.rs (4)

- every_accepted_row_decodes_to_its_record
- every_refused_row_is_refused_alone_and_in_a_batch
- the_arguments_are_a_closed_bounded_batch
- a_node_drag_emits_one_transaction_plain_leaves_or_nothing

### 🧪️gesture-drive-law/🦀️.rs (5)

- the_phase_arguments_parse_like_the_fixture
- every_fixture_row_drives_to_its_expected_outcome
- every_host_fact_ends_a_gesture_with_the_fixtures_reason
- every_slot_scenario_steps_the_ledger_to_its_expected_outcome
- a_persisted_gesture_resumes_to_itself_and_refuses_what_its_chart_cannot_restore

## Canonical Representation Recheck

Typing laws line 140 emits canonical net directly from state.net. All twenty-nine open-state records in the shared typing-law fixture have net and none have entries. Schema TypingOpen requires net and rejects extra properties; schema TypingState likewise requires net. Persistence laws intentionally inspect resumed.transaction().entries(), because the internal generic ToolTransaction remains keyed and reconstructs positional typing keys at the reference boundary. That is not a stale TypingState consumer.

Scrub laws line 116 correctly retains state.entries; the generic transaction reducer and gesture laws retain keyed entries. No keyed-to-net change should be applied to those owners. Exact inspected companion consumers show no remaining stale TypingState.entries or erroneous scrub/gesture net field.

No concrete compile-field or fixture mismatch was identified in this focused source review. Native compilation, forty actual test outcomes and actual third-party Rust blake3 assertions remain unverified until the separate companion command completes. The current framework batch cannot establish those results by compiling the companion merely as an unselected package or dependency.
