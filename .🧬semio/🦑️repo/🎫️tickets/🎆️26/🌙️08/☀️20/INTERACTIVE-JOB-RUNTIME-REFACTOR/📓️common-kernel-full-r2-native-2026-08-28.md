# Common Kernel Full R2 Native Gate

Actual full common framework (`semio-framework`, not OS Kernel): 266 passed, 0 failed, 0 skipped, 4.898s. Nx exit 0; existing exhaustive profile, no-fail-fast, same retained target and jobs=2. No exclusions. This supersedes the R1 263/2 result after the test-only action-bus conservation repair/new law and Dag's canonical document_dsl source join. Neither grants nor runtime timing were relaxed. It is not full Plugin, guest, WGPU or composition proof.

The terminal tool result was truncated; the complete still-present raw Markdown was immediately read in checked 350-line chunks (each exit 0, none truncated), stored, and copied below. No missing bytes were reconstructed. The Nx historical flaky-task notice is preserved; this invocation reports all 266 passing and no retry.

Selected 131-file pre-dispatch SHA capture: `📓️common-kernel-r2-selected-inputs-2026-08-28.md`; not a complete atomic closure capture. Dag held Kernel/Actor/Manifest/WGPU sources throughout.

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-rs:test-wire-retirement-native --skip-nx-cache --args='exhaustive --lib --no-fail-fast -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-common-kernel-full-r2-2026-08-28.md'
```

## Complete Raw Output

```text

> nx run @semio-tech/framework-rs:test-wire-retirement-native --args=exhaustive --lib --no-fail-fast -- --nocapture

> bun ./📜️script.ts test-wire-retirement-native exhaustive --lib --no-fail-fast -- --nocapture

────────────
[32;1m Nextest run[0m ID [1mb4644498-293d-41c4-a540-95b0f0a10f67[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m266[0m tests across [1m1[0m binary
[32;1m       START[0m [         ] (  1/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mcancel_before_and_after_seal_are_terminal_and_non_advancing[0m

running 1 test
test abi::tests::cancel_before_and_after_seal_are_terminal_and_non_advancing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.063s] (  1/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mcancel_before_and_after_seal_are_terminal_and_non_advancing[0m
[32;1m       START[0m [         ] (  2/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mempty_single_max_and_max_plus_one_preserve_bounds_and_caller_bytes[0m

running 1 test
test abi::tests::empty_single_max_and_max_plus_one_preserve_bounds_and_caller_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.025s] (  2/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mempty_single_max_and_max_plus_one_preserve_bounds_and_caller_bytes[0m
[32;1m       START[0m [         ] (  3/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mexact_ack_duplicate_ack_and_generation_errors_are_distinct[0m

running 1 test
test abi::tests::exact_ack_duplicate_ack_and_generation_errors_are_distinct ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (  3/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mexact_ack_duplicate_ack_and_generation_errors_are_distinct[0m
[32;1m       START[0m [         ] (  4/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mhandle_generation_exhaustion_quarantines_slots_without_aliasing[0m

running 1 test
test abi::tests::handle_generation_exhaustion_quarantines_slots_without_aliasing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (  4/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mhandle_generation_exhaustion_quarantines_slots_without_aliasing[0m
[32;1m       START[0m [         ] (  5/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mhandle_table_rejects_unknown_stale_and_aba_reuse[0m

running 1 test
test abi::tests::handle_table_rejects_unknown_stale_and_aba_reuse ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (  5/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mhandle_table_rejects_unknown_stale_and_aba_reuse[0m
[32;1m       START[0m [         ] (  6/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1minterrupted_close_retains_state_and_terminal_empty_is_idempotent[0m

running 1 test
test abi::tests::interrupted_close_retains_state_and_terminal_empty_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (  6/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1minterrupted_close_retains_state_and_terminal_empty_is_idempotent[0m
[32;1m       START[0m [         ] (  7/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1minterrupted_port_callback_returns_the_exact_owned_message[0m

running 1 test
test abi::tests::interrupted_port_callback_returns_the_exact_owned_message ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (  7/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1minterrupted_port_callback_returns_the_exact_owned_message[0m
[32;1m       START[0m [         ] (  8/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mlost_handle_late_reply_and_duplicate_reply_cannot_cross_generations[0m

running 1 test
test abi::tests::lost_handle_late_reply_and_duplicate_reply_cannot_cross_generations ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (  8/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mlost_handle_late_reply_and_duplicate_reply_cannot_cross_generations[0m
[32;1m       START[0m [         ] (  9/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mmalformed_tag_length_utf8_and_missing_optional_fail_closed[0m

running 1 test
test abi::tests::malformed_tag_length_utf8_and_missing_optional_fail_closed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (  9/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mmalformed_tag_length_utf8_and_missing_optional_fail_closed[0m
[32;1m       START[0m [         ] ( 10/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mnative_and_wasm_ledger_is_fixed_little_endian_and_deterministic[0m

running 1 test
test abi::tests::native_and_wasm_ledger_is_fixed_little_endian_and_deterministic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 10/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mnative_and_wasm_ledger_is_fixed_little_endian_and_deterministic[0m
[32;1m       START[0m [         ] ( 11/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mpage_and_transfer_max_plus_one_return_exact_allocations[0m

running 1 test
test abi::tests::page_and_transfer_max_plus_one_return_exact_allocations ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.017s] ( 11/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mpage_and_transfer_max_plus_one_return_exact_allocations[0m
[32;1m       START[0m [         ] ( 12/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mreader_preflights_every_rejection_before_allocation_or_copy[0m

running 1 test
test abi::tests::reader_preflights_every_rejection_before_allocation_or_copy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 12/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mreader_preflights_every_rejection_before_allocation_or_copy[0m
[32;1m       START[0m [         ] ( 13/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mretained_writer_is_credit_deadline_and_interruption_aware[0m

running 1 test
test abi::tests::retained_writer_is_credit_deadline_and_interruption_aware ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 13/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mretained_writer_is_credit_deadline_and_interruption_aware[0m
[32;1m       START[0m [         ] ( 14/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mschema_and_language_agnostic_fixture_publish_all_owned_contracts[0m

running 1 test
test abi::tests::schema_and_language_agnostic_fixture_publish_all_owned_contracts ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] ( 14/266) [35;1msemio-framework[0m [36mabi::tests[0m[36m::[0m[34;1mschema_and_language_agnostic_fixture_publish_all_owned_contracts[0m
[32;1m       START[0m [         ] ( 15/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1maliases_require_an_existing_exact_factory_and_never_fallback[0m

running 1 test
test action_bus::tests::aliases_require_an_existing_exact_factory_and_never_fallback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 15/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1maliases_require_an_existing_exact_factory_and_never_fallback[0m
[32;1m       START[0m [         ] ( 16/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mdispatch_returns_a_resumable_job_and_preserves_operation_identity[0m

running 1 test
test action_bus::tests::dispatch_returns_a_resumable_job_and_preserves_operation_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 16/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mdispatch_returns_a_resumable_job_and_preserves_operation_identity[0m
[32;1m       START[0m [         ] ( 17/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mduplicate_factory_key_is_rejected_without_partial_registration[0m

running 1 test
test action_bus::tests::duplicate_factory_key_is_rejected_without_partial_registration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 17/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mduplicate_factory_key_is_rejected_without_partial_registration[0m
[32;1m       START[0m [         ] ( 18/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mduplicate_key_inside_one_factory_is_rejected_atomically[0m

running 1 test
test action_bus::tests::duplicate_key_inside_one_factory_is_rejected_atomically ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 18/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mduplicate_key_inside_one_factory_is_rejected_atomically[0m
[32;1m       START[0m [         ] ( 19/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mexact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode[0m

running 1 test
test action_bus::tests::exact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] ( 19/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mexact_wire_admission_rejects_alias_schema_and_raw_limit_before_decode[0m
[32;1m       START[0m [         ] ( 20/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mheterogeneous_factories_share_one_bus_with_exact_key_ownership[0m

running 1 test
test action_bus::tests::heterogeneous_factories_share_one_bus_with_exact_key_ownership ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 20/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mheterogeneous_factories_share_one_bus_with_exact_key_ownership[0m
[32;1m       START[0m [         ] ( 21/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mmaximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix[0m

running 1 test
test action_bus::tests::maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 21/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mmaximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix[0m
[32;1m       START[0m [         ] ( 22/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mproduction_typed_payload_and_retained_pages_enter_the_same_registered_factory_job[0m

running 1 test
test action_bus::tests::production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 22/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mproduction_typed_payload_and_retained_pages_enter_the_same_registered_factory_job[0m
[32;1m       START[0m [         ] ( 23/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mregistration_rejects_every_non_migrated_factory[0m

running 1 test
test action_bus::tests::registration_rejects_every_non_migrated_factory ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.032s] ( 23/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mregistration_rejects_every_non_migrated_factory[0m
[32;1m       START[0m [         ] ( 24/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation[0m

running 1 test
test action_bus::tests::retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.046s] ( 24/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation[0m
[32;1m       START[0m [         ] ( 25/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes[0m

running 1 test
[DEBUG] retained-number-close zero-items=blocked zero-bytes=blocked logical=7+1 backing-logical=0 terminal=true
test action_bus::tests::retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 25/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mretained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes[0m
[32;1m       START[0m [         ] ( 26/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1munknown_controller_is_an_explicit_dispatch_error[0m

running 1 test
test action_bus::tests::unknown_controller_is_an_explicit_dispatch_error ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] ( 26/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1munknown_controller_is_an_explicit_dispatch_error[0m
[32;1m       START[0m [         ] ( 27/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mwire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint[0m

running 1 test
test action_bus::tests::wire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] ( 27/266) [35;1msemio-framework[0m [36maction_bus::tests[0m[36m::[0m[34;1mwire_dispatch_uses_the_factory_decoder_and_preserves_the_restart_checkpoint[0m
[32;1m       START[0m [         ] ( 28/266) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation[0m

running 1 test
test action_bus::wire_retirement_tests::retained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.030s] ( 28/266) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation[0m
[32;1m       START[0m [         ] ( 29/266) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_short_close_conserves_logical_bytes_and_physical_backing[0m

running 1 test
[DEBUG] wire-short-close released=8 backing-capacity=1->0 zero-grants-preserve=true
test action_bus::wire_retirement_tests::retained_wire_short_close_conserves_logical_bytes_and_physical_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 29/266) [35;1msemio-framework[0m [36maction_bus::wire_retirement_tests[0m[36m::[0m[34;1mretained_wire_short_close_conserves_logical_bytes_and_physical_backing[0m
[32;1m       START[0m [         ] ( 30/266) [35;1msemio-framework[0m [36minteraction::component::tests[0m[36m::[0m[34;1minteraction_definition_round_trips_through_json[0m

running 1 test
test interaction::component::tests::interaction_definition_round_trips_through_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.030s] ( 30/266) [35;1msemio-framework[0m [36minteraction::component::tests[0m[36m::[0m[34;1minteraction_definition_round_trips_through_json[0m
[32;1m       START[0m [         ] ( 31/266) [35;1msemio-framework[0m [36minteraction::component::tests[0m[36m::[0m[34;1moutline_projects_id_granularity_ids_and_selection_only[0m

running 1 test
test interaction::component::tests::outline_projects_id_granularity_ids_and_selection_only ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 31/266) [35;1msemio-framework[0m [36minteraction::component::tests[0m[36m::[0m[34;1moutline_projects_id_granularity_ids_and_selection_only[0m
[32;1m       START[0m [         ] ( 32/266) [35;1msemio-framework[0m [36mio::io_fidelity_tests[0m[36m::[0m[34;1mio_fidelity_class_parse_and_rank[0m

running 1 test
test io::io_fidelity_tests::io_fidelity_class_parse_and_rank ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 32/266) [35;1msemio-framework[0m [36mio::io_fidelity_tests[0m[36m::[0m[34;1mio_fidelity_class_parse_and_rank[0m
[32;1m       START[0m [         ] ( 33/266) [35;1msemio-framework[0m [36mio::io_fidelity_tests[0m[36m::[0m[34;1mio_fidelity_declaration_validate[0m

running 1 test
test io::io_fidelity_tests::io_fidelity_declaration_validate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] ( 33/266) [35;1msemio-framework[0m [36mio::io_fidelity_tests[0m[36m::[0m[34;1mio_fidelity_declaration_validate[0m
[32;1m       START[0m [         ] ( 34/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mconformance_runs_after_deserialize[0m

running 1 test
test io::io_mechanism::laws::conformance_runs_after_deserialize ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.036s] ( 34/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mconformance_runs_after_deserialize[0m
[32;1m       START[0m [         ] ( 35/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mduplicate_entry_is_a_typed_error[0m

running 1 test
test io::io_mechanism::laws::duplicate_entry_is_a_typed_error ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] ( 35/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mduplicate_entry_is_a_typed_error[0m
[32;1m       START[0m [         ] ( 36/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1midentify_only_sniffs_carriers[0m

running 1 test
test io::io_mechanism::laws::identify_only_sniffs_carriers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 36/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1midentify_only_sniffs_carriers[0m
[32;1m       START[0m [         ] ( 37/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mregistration_is_all_or_nothing[0m

running 1 test
test io::io_mechanism::laws::registration_is_all_or_nothing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 37/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mregistration_is_all_or_nothing[0m
[32;1m       START[0m [         ] ( 38/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_is_deterministic[0m

running 1 test
test io::io_mechanism::laws::route_is_deterministic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 38/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_is_deterministic[0m
[32;1m       START[0m [         ] ( 39/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_never_cycles[0m

running 1 test
test io::io_mechanism::laws::route_never_cycles ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 39/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_never_cycles[0m
[32;1m       START[0m [         ] ( 40/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_prefers_higher_minimum_fidelity[0m

running 1 test
test io::io_mechanism::laws::route_prefers_higher_minimum_fidelity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] ( 40/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_prefers_higher_minimum_fidelity[0m
[32;1m       START[0m [         ] ( 41/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_respects_max_hops[0m

running 1 test
test io::io_mechanism::laws::route_respects_max_hops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.022s] ( 41/266) [35;1msemio-framework[0m [36mio::io_mechanism::laws[0m[36m::[0m[34;1mroute_respects_max_hops[0m
[32;1m       START[0m [         ] ( 42/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_kind_id_accepts_canonical_grammar[0m

running 1 test
test io::tests::artifact_kind_id_accepts_canonical_grammar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.023s] ( 42/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_kind_id_accepts_canonical_grammar[0m
[32;1m       START[0m [         ] ( 43/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_kind_id_rejects_non_canonical_grammar[0m

running 1 test
test io::tests::artifact_kind_id_rejects_non_canonical_grammar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 43/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_kind_id_rejects_non_canonical_grammar[0m
[32;1m       START[0m [         ] ( 44/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_parse_uri_rejects_malformed_input[0m

running 1 test
test io::tests::artifact_ref_parse_uri_rejects_malformed_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 44/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_parse_uri_rejects_malformed_input[0m
[32;1m       START[0m [         ] ( 45/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_to_uri_matches_expected_shape[0m

running 1 test
test io::tests::artifact_ref_to_uri_matches_expected_shape ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.033s] ( 45/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_to_uri_matches_expected_shape[0m
[32;1m       START[0m [         ] ( 46/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_uri_round_trips[0m

running 1 test
test io::tests::artifact_ref_uri_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.073s] ( 46/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1martifact_ref_uri_round_trips[0m
[32;1m       START[0m [         ] ( 47/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_budget_enforces_limits_and_shared_cancellation[0m

running 1 test
test io::tests::codec_budget_enforces_limits_and_shared_cancellation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.046s] ( 47/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_budget_enforces_limits_and_shared_cancellation[0m
[32;1m       START[0m [         ] ( 48/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_context_bounds_streaming_random_access_recursion_and_resolved_resources[0m

running 1 test
test io::tests::codec_context_bounds_streaming_random_access_recursion_and_resolved_resources ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.051s] ( 48/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_context_bounds_streaming_random_access_recursion_and_resolved_resources[0m
[32;1m       START[0m [         ] ( 49/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_result_requires_valid_owned_spans_and_deterministic_opaque_order[0m

running 1 test
test io::tests::codec_result_requires_valid_owned_spans_and_deterministic_opaque_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.048s] ( 49/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mcodec_result_requires_valid_owned_spans_and_deterministic_opaque_order[0m
[32;1m       START[0m [         ] ( 50/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mformat_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims[0m

running 1 test
test io::tests::format_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.038s] ( 50/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mformat_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims[0m
[32;1m       START[0m [         ] ( 51/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_compose_via_chains_two_registered_hops[0m

running 1 test
test io::tests::io_compose_via_chains_two_registered_hops ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.027s] ( 51/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_compose_via_chains_two_registered_hops[0m
[32;1m       START[0m [         ] ( 52/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_compose_via_surfaces_hub_resolve_failure[0m

running 1 test
test io::tests::io_compose_via_surfaces_hub_resolve_failure ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.028s] ( 52/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_compose_via_surfaces_hub_resolve_failure[0m
[32;1m       START[0m [         ] ( 53/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_registry_rejects_a_conflicting_key_without_replacing_the_first_entry[0m

running 1 test
test io::tests::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] ( 53/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mio_registry_rejects_a_conflicting_key_without_replacing_the_first_entry[0m
[32;1m       START[0m [         ] ( 54/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mresolved_resources_cannot_outlive_their_cancellation_budget[0m

running 1 test
test io::tests::resolved_resources_cannot_outlive_their_cancellation_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.023s] ( 54/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mresolved_resources_cannot_outlive_their_cancellation_budget[0m
[32;1m       START[0m [         ] ( 55/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mwire_rejects_oversized_and_unbounded_dialect_inputs_before_interning[0m

running 1 test
test io::tests::wire_rejects_oversized_and_unbounded_dialect_inputs_before_interning ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 55/266) [35;1msemio-framework[0m [36mio::tests[0m[36m::[0m[34;1mwire_rejects_oversized_and_unbounded_dialect_inputs_before_interning[0m
[32;1m       START[0m [         ] ( 56/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mdefault_is_empty_and_promoted_subset_holds_trivially[0m

running 1 test
test manifest::agent_contributions_tests::default_is_empty_and_promoted_subset_holds_trivially ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.024s] ( 56/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mdefault_is_empty_and_promoted_subset_holds_trivially[0m
[32;1m       START[0m [         ] ( 57/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mnever_conflated_with_capability_requests[0m

running 1 test
test manifest::agent_contributions_tests::never_conflated_with_capability_requests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 57/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mnever_conflated_with_capability_requests[0m
[32;1m       START[0m [         ] ( 58/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mpromoted_subset_of_capabilities_holds_and_is_violated_correctly[0m

running 1 test
test manifest::agent_contributions_tests::promoted_subset_of_capabilities_holds_and_is_violated_correctly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 58/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mpromoted_subset_of_capabilities_holds_and_is_violated_correctly[0m
[32;1m       START[0m [         ] ( 59/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mserde_round_trip_uses_camel_case_and_skips_empty_promoted[0m

running 1 test
test manifest::agent_contributions_tests::serde_round_trip_uses_camel_case_and_skips_empty_promoted ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 59/266) [35;1msemio-framework[0m [36mmanifest::agent_contributions_tests[0m[36m::[0m[34;1mserde_round_trip_uses_camel_case_and_skips_empty_promoted[0m
[32;1m       START[0m [         ] ( 60/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_control_serializes_tagged[0m

running 1 test
test manifest::app_label_tests::action_arg_control_serializes_tagged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 60/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_control_serializes_tagged[0m
[32;1m       START[0m [         ] ( 61/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_def_builder_chain[0m

running 1 test
test manifest::app_label_tests::action_arg_def_builder_chain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 61/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_def_builder_chain[0m
[32;1m       START[0m [         ] ( 62/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_def_json_schema_covers_the_core_shapes[0m

running 1 test
test manifest::app_label_tests::action_arg_def_json_schema_covers_the_core_shapes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 62/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_arg_def_json_schema_covers_the_core_shapes[0m
[32;1m       START[0m [         ] ( 63/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_definition_requires_and_serializes_args_field[0m

running 1 test
test manifest::app_label_tests::action_definition_requires_and_serializes_args_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 63/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_definition_requires_and_serializes_args_field[0m
[32;1m       START[0m [         ] ( 64/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_definition_semantics_default_from_kind_and_builders_compose[0m

running 1 test
test manifest::app_label_tests::action_definition_semantics_default_from_kind_and_builders_compose ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 64/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_definition_semantics_default_from_kind_and_builders_compose[0m
[32;1m       START[0m [         ] ( 65/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_kind_interaction_round_trips_through_json[0m

running 1 test
test manifest::app_label_tests::action_kind_interaction_round_trips_through_json ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.028s] ( 65/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_kind_interaction_round_trips_through_json[0m
[32;1m       START[0m [         ] ( 66/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_semantics_defaults_match_language_neutral_fixture[0m

running 1 test
test manifest::app_label_tests::action_semantics_defaults_match_language_neutral_fixture ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.064s] ( 66/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_semantics_defaults_match_language_neutral_fixture[0m
[32;1m       START[0m [         ] ( 67/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_semantics_for_kind_matches_the_defaults_table[0m

running 1 test
test manifest::app_label_tests::action_semantics_for_kind_matches_the_defaults_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.034s] ( 67/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1maction_semantics_for_kind_matches_the_defaults_table[0m
[32;1m       START[0m [         ] ( 68/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_definition_and_window_kind_definition_serde_round_trip_interactions[0m

running 1 test
test manifest::app_label_tests::app_definition_and_window_kind_definition_serde_round_trip_interactions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.031s] ( 68/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_definition_and_window_kind_definition_serde_round_trip_interactions[0m
[32;1m       START[0m [         ] ( 69/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_ref_serde_round_trips_as_camel_case[0m

running 1 test
test manifest::app_label_tests::app_ref_serde_round_trips_as_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.022s] ( 69/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_ref_serde_round_trips_as_camel_case[0m
[32;1m       START[0m [         ] ( 70/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_role_as_str_and_from_str_round_trip[0m

running 1 test
test manifest::app_label_tests::app_role_as_str_and_from_str_round_trip ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] ( 70/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_role_as_str_and_from_str_round_trip[0m
[32;1m       START[0m [         ] ( 71/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_role_serde_wire_strings_are_exactly_viewer_and_editor[0m

running 1 test
test manifest::app_label_tests::app_role_serde_wire_strings_are_exactly_viewer_and_editor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 71/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_role_serde_wire_strings_are_exactly_viewer_and_editor[0m
[32;1m       START[0m [         ] ( 72/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_window_label_skips_empty_app_named_and_duplicate_trailing_window_labels[0m

running 1 test
test manifest::app_label_tests::app_window_label_skips_empty_app_named_and_duplicate_trailing_window_labels ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.022s] ( 72/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mapp_window_label_skips_empty_app_named_and_duplicate_trailing_window_labels[0m
[32;1m       START[0m [         ] ( 73/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mchild_element_id_suffixes_and_normalizes_segments[0m

running 1 test
test manifest::app_label_tests::child_element_id_suffixes_and_normalizes_segments ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.030s] ( 73/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mchild_element_id_suffixes_and_normalizes_segments[0m
[32;1m       START[0m [         ] ( 74/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcommand_and_action_invocations_round_trip_owner_qualified_addresses[0m

running 1 test
test manifest::app_label_tests::command_and_action_invocations_round_trip_owner_qualified_addresses ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] ( 74/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcommand_and_action_invocations_round_trip_owner_qualified_addresses[0m
[32;1m       START[0m [         ] ( 75/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcommand_definition_round_trips_camel_case_with_defaults[0m

running 1 test
test manifest::app_label_tests::command_definition_round_trips_camel_case_with_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 75/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcommand_definition_round_trips_camel_case_with_defaults[0m
[32;1m       START[0m [         ] ( 76/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcompose_tutorial_ui_applies_snapshot_then_deltas[0m

running 1 test
test manifest::app_label_tests::compose_tutorial_ui_applies_snapshot_then_deltas ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 76/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mcompose_tutorial_ui_applies_snapshot_then_deltas[0m
[32;1m       START[0m [         ] ( 77/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdialog_definition_builder_chain[0m

running 1 test
test manifest::app_label_tests::dialog_definition_builder_chain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 77/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdialog_definition_builder_chain[0m
[32;1m       START[0m [         ] ( 78/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdialog_definition_round_trips_camel_case_with_defaults[0m

running 1 test
test manifest::app_label_tests::dialog_definition_round_trips_camel_case_with_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] ( 78/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdialog_definition_round_trips_camel_case_with_defaults[0m
[32;1m       START[0m [         ] ( 79/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdispatch_action_effect_round_trips_camel_case[0m

running 1 test
test manifest::app_label_tests::dispatch_action_effect_round_trips_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 79/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mdispatch_action_effect_round_trips_camel_case[0m
[32;1m       START[0m [         ] ( 80/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_pass_seed_through_wholesale_when_no_fields_are_declared[0m

running 1 test
test manifest::app_label_tests::effective_args_pass_seed_through_wholesale_when_no_fields_are_declared ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 80/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_pass_seed_through_wholesale_when_no_fields_are_declared[0m
[32;1m       START[0m [         ] ( 81/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_prefer_staged_then_default[0m

running 1 test
test manifest::app_label_tests::effective_args_prefer_staged_then_default ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] ( 81/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_prefer_staged_then_default[0m
[32;1m       START[0m [         ] ( 82/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_preserve_a_seeded_arg_not_declared_as_a_form_field[0m

running 1 test
test manifest::app_label_tests::effective_args_preserve_a_seeded_arg_not_declared_as_a_form_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 82/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_preserve_a_seeded_arg_not_declared_as_a_form_field[0m
[32;1m       START[0m [         ] ( 83/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_seed_prefills_a_declared_field_until_staged_overrides_it[0m

running 1 test
test manifest::app_label_tests::effective_args_seed_prefills_a_declared_field_until_staged_overrides_it ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] ( 83/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1meffective_args_seed_prefills_a_declared_field_until_staged_overrides_it[0m
[32;1m       START[0m [         ] ( 84/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1melement_id_authoring_helpers[0m

running 1 test
test manifest::app_label_tests::element_id_authoring_helpers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 84/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1melement_id_authoring_helpers[0m
[32;1m       START[0m [         ] ( 85/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1melement_id_segment_normalizes_and_is_idempotent[0m

running 1 test
test manifest::app_label_tests::element_id_segment_normalizes_and_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] ( 85/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1melement_id_segment_normalizes_and_is_idempotent[0m
[32;1m       START[0m [         ] ( 86/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mempty_collections_serialize_as_arrays_rather_than_vanishing_from_the_manifest[0m

running 1 test
test manifest::app_label_tests::empty_collections_serialize_as_arrays_rather_than_vanishing_from_the_manifest ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 86/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mempty_collections_serialize_as_arrays_rather_than_vanishing_from_the_manifest[0m
[32;1m       START[0m [         ] ( 87/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mformats_app_label_for_chrome[0m

running 1 test
test manifest::app_label_tests::formats_app_label_for_chrome ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 87/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mformats_app_label_for_chrome[0m
[32;1m       START[0m [         ] ( 88/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mhost_resolved_arg_builders_derive_their_pre_d6_controls[0m

running 1 test
test manifest::app_label_tests::host_resolved_arg_builders_derive_their_pre_d6_controls ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 88/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mhost_resolved_arg_builders_derive_their_pre_d6_controls[0m
[32;1m       START[0m [         ] ( 89/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteraction_action_definitions_empty_when_app_has_no_interactions[0m

running 1 test
test manifest::app_label_tests::interaction_action_definitions_empty_when_app_has_no_interactions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 89/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteraction_action_definitions_empty_when_app_has_no_interactions[0m
[32;1m       START[0m [         ] ( 90/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteraction_action_definitions_full_set_when_app_has_interactions[0m

running 1 test
test manifest::app_label_tests::interaction_action_definitions_full_set_when_app_has_interactions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 90/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteraction_action_definitions_full_set_when_app_has_interactions[0m
[32;1m       START[0m [         ] ( 91/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteractive_job_classification_is_explicit_and_release_validated[0m

running 1 test
test manifest::app_label_tests::interactive_job_classification_is_explicit_and_release_validated ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 91/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1minteractive_job_classification_is_explicit_and_release_validated[0m
[32;1m       START[0m [         ] ( 92/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_demonstration_round_trips_and_defaults[0m

running 1 test
test manifest::app_label_tests::introduction_demonstration_round_trips_and_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.024s] ( 92/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_demonstration_round_trips_and_defaults[0m
[32;1m       START[0m [         ] ( 93/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_gesture_drag_orbit_default_button_and_modifiers[0m

running 1 test
test manifest::app_label_tests::introduction_gesture_drag_orbit_default_button_and_modifiers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 93/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_gesture_drag_orbit_default_button_and_modifiers[0m
[32;1m       START[0m [         ] ( 94/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_gesture_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::introduction_gesture_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] ( 94/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_gesture_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] ( 95/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_interaction_kind_round_trips_tagged[0m

running 1 test
test manifest::app_label_tests::introduction_interaction_kind_round_trips_tagged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] ( 95/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_interaction_kind_round_trips_tagged[0m
[32;1m       START[0m [         ] ( 96/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_interaction_round_trips_and_defaults[0m

running 1 test
test manifest::app_label_tests::introduction_interaction_round_trips_and_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.035s] ( 96/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_interaction_round_trips_and_defaults[0m
[32;1m       START[0m [         ] ( 97/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_point_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::introduction_point_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] ( 97/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_point_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] ( 98/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_step_serde_defaults[0m

running 1 test
test manifest::app_label_tests::introduction_step_serde_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] ( 98/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mintroduction_step_serde_defaults[0m
[32;1m       START[0m [         ] ( 99/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mis_element_id_accepts_dotted_camel_case_and_rejects_the_rest[0m

running 1 test
test manifest::app_label_tests::is_element_id_accepts_dotted_camel_case_and_rejects_the_rest ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] ( 99/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mis_element_id_accepts_dotted_camel_case_and_rejects_the_rest[0m
[32;1m       START[0m [         ] (100/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mmissing_required_args_treats_unset_select_as_missing[0m

running 1 test
test manifest::app_label_tests::missing_required_args_treats_unset_select_as_missing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (100/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mmissing_required_args_treats_unset_select_as_missing[0m
[32;1m       START[0m [         ] (101/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mnon_empty_vec_index_iter_first_mut_and_try_from[0m

running 1 test
test manifest::app_label_tests::non_empty_vec_index_iter_first_mut_and_try_from ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (101/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mnon_empty_vec_index_iter_first_mut_and_try_from[0m
[32;1m       START[0m [         ] (102/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mopen_dialog_effect_round_trips_camel_case[0m

running 1 test
test manifest::app_label_tests::open_dialog_effect_round_trips_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (102/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mopen_dialog_effect_round_trips_camel_case[0m
[32;1m       START[0m [         ] (103/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_group_anchor_and_as_str_cover_all_variants[0m

running 1 test
test manifest::app_label_tests::panel_group_anchor_and_as_str_cover_all_variants ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (103/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_group_anchor_and_as_str_cover_all_variants[0m
[32;1m       START[0m [         ] (104/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_tab_kind_id_str_covers_framework_and_app_variants[0m

running 1 test
test manifest::app_label_tests::panel_tab_kind_id_str_covers_framework_and_app_variants ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (104/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_tab_kind_id_str_covers_framework_and_app_variants[0m
[32;1m       START[0m [         ] (105/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_tab_kind_settings_default_apps_id_str[0m

running 1 test
test manifest::app_label_tests::panel_tab_kind_settings_default_apps_id_str ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (105/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mpanel_tab_kind_settings_default_apps_id_str[0m
[32;1m       START[0m [         ] (106/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mparse_surface_app_id_rejects_missing_hash_and_unknown_role[0m

running 1 test
test manifest::app_label_tests::parse_surface_app_id_rejects_missing_hash_and_unknown_role ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (106/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mparse_surface_app_id_rejects_missing_hash_and_unknown_role[0m
[32;1m       START[0m [         ] (107/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrecord_tutorial_action_definition_is_shell_intercepted_and_out_of_palette[0m

running 1 test
test manifest::app_label_tests::record_tutorial_action_definition_is_shell_intercepted_and_out_of_palette ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (107/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrecord_tutorial_action_definition_is_shell_intercepted_and_out_of_palette[0m
[32;1m       START[0m [         ] (108/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrequest_file_open_effect_round_trips_multiple[0m

running 1 test
test manifest::app_label_tests::request_file_open_effect_round_trips_multiple ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (108/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrequest_file_open_effect_round_trips_multiple[0m
[32;1m       START[0m [         ] (109/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrequest_media_frames_effect_round_trips_camel_case[0m

running 1 test
test manifest::app_label_tests::request_media_frames_effect_round_trips_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (109/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mrequest_media_frames_effect_round_trips_camel_case[0m
[32;1m       START[0m [         ] (110/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_app_label_uses_terminology_override_else_falls_back_to_native_label[0m

running 1 test
test manifest::app_label_tests::resolve_app_label_uses_terminology_override_else_falls_back_to_native_label ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (110/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_app_label_uses_terminology_override_else_falls_back_to_native_label[0m
[32;1m       START[0m [         ] (111/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_layout_for_mode_prefers_named_then_default_then_none[0m

running 1 test
test manifest::app_label_tests::resolve_layout_for_mode_prefers_named_then_default_then_none ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.024s] (111/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_layout_for_mode_prefers_named_then_default_then_none[0m
[32;1m       START[0m [         ] (112/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_declared_order[0m

running 1 test
test manifest::app_label_tests::resolve_mode_tools_declared_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (112/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_declared_order[0m
[32;1m       START[0m [         ] (113/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_isolates_other_modes[0m

running 1 test
test manifest::app_label_tests::resolve_mode_tools_isolates_other_modes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (113/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_isolates_other_modes[0m
[32;1m       START[0m [         ] (114/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_skips_unresolvable_refs[0m

running 1 test
test manifest::app_label_tests::resolve_mode_tools_skips_unresolvable_refs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (114/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_mode_tools_skips_unresolvable_refs[0m
[32;1m       START[0m [         ] (115/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_excludes_history_and_set_active_utility_orphans[0m

running 1 test
test manifest::app_label_tests::resolve_window_actions_excludes_history_and_set_active_utility_orphans ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (115/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_excludes_history_and_set_active_utility_orphans[0m
[32;1m       START[0m [         ] (116/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_explicit_scoping[0m

running 1 test
test manifest::app_label_tests::resolve_window_actions_explicit_scoping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (116/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_explicit_scoping[0m
[32;1m       START[0m [         ] (117/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_includes_injected_interaction_actions[0m

running 1 test
test manifest::app_label_tests::resolve_window_actions_includes_injected_interaction_actions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (117/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mresolve_window_actions_includes_injected_interaction_actions[0m
[32;1m       START[0m [         ] (118/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1msix_arg_builder_helpers_derive_the_pre_d6_control[0m

running 1 test
test manifest::app_label_tests::six_arg_builder_helpers_derive_the_pre_d6_control ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (118/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1msix_arg_builder_helpers_derive_the_pre_d6_control[0m
[32;1m       START[0m [         ] (119/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mstart_tutorial_action_definition_offers_declared_tutorials_as_select_options[0m

running 1 test
test manifest::app_label_tests::start_tutorial_action_definition_offers_declared_tutorials_as_select_options ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (119/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mstart_tutorial_action_definition_offers_declared_tutorials_as_select_options[0m
[32;1m       START[0m [         ] (120/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1msurface_app_id_round_trips_through_parse_surface_app_id[0m

running 1 test
test manifest::app_label_tests::surface_app_id_round_trips_through_parse_surface_app_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (120/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1msurface_app_id_round_trips_through_parse_surface_app_id[0m
[32;1m       START[0m [         ] (121/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_artifact_event_kind_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::tutorial_artifact_event_kind_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (121/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_artifact_event_kind_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] (122/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_asset_src_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::tutorial_asset_src_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (122/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_asset_src_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] (123/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after[0m

running 1 test
test manifest::app_label_tests::tutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (123/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_at_holds_first_pose_before_first_keyframe_and_last_pose_after[0m
[32;1m       START[0m [         ] (124/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_hold_snaps_at_keyframe[0m

running 1 test
test manifest::app_label_tests::tutorial_camera_interpolation_hold_snaps_at_keyframe ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (124/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_hold_snaps_at_keyframe[0m
[32;1m       START[0m [         ] (125/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_lerps_position_and_target[0m

running 1 test
test manifest::app_label_tests::tutorial_camera_interpolation_lerps_position_and_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (125/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_lerps_position_and_target[0m
[32;1m       START[0m [         ] (126/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_zooms_in_log_space[0m

running 1 test
test manifest::app_label_tests::tutorial_camera_interpolation_zooms_in_log_space ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (126/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_interpolation_zooms_in_log_space[0m
[32;1m       START[0m [         ] (127/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_state_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::tutorial_camera_state_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (127/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_camera_state_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] (128/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_definition_serde_defaults[0m

running 1 test
test manifest::app_label_tests::tutorial_definition_serde_defaults ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (128/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_definition_serde_defaults[0m
[32;1m       START[0m [         ] (129/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_document_track_language_neutral_serde_parity[0m

running 1 test
test manifest::app_label_tests::tutorial_document_track_language_neutral_serde_parity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (129/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_document_track_language_neutral_serde_parity[0m
[32;1m       START[0m [         ] (130/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_event_kind_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::tutorial_event_kind_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (130/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_event_kind_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] (131/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_slice_forward_and_reverse_cross_artifact_events[0m

running 1 test
test manifest::app_label_tests::tutorial_slice_forward_and_reverse_cross_artifact_events ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (131/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_slice_forward_and_reverse_cross_artifact_events[0m
[32;1m       START[0m [         ] (132/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_slice_partitions_events_artifact_and_ui_by_track[0m

running 1 test
test manifest::app_label_tests::tutorial_slice_partitions_events_artifact_and_ui_by_track ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (132/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_slice_partitions_events_artifact_and_ui_by_track[0m
[32;1m       START[0m [         ] (133/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_ui_change_round_trips_tagged_camel_case[0m

running 1 test
test manifest::app_label_tests::tutorial_ui_change_round_trips_tagged_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (133/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mtutorial_ui_change_round_trips_tagged_camel_case[0m
[32;1m       START[0m [         ] (134/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mui_dirty_scope_defaults_to_full[0m

running 1 test
test manifest::app_label_tests::ui_dirty_scope_defaults_to_full ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (134/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mui_dirty_scope_defaults_to_full[0m
[32;1m       START[0m [         ] (135/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mui_dirty_scope_partial_serializes_fields_as_camel_case[0m

running 1 test
test manifest::app_label_tests::ui_dirty_scope_partial_serializes_fields_as_camel_case ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (135/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mui_dirty_scope_partial_serializes_fields_as_camel_case[0m
[32;1m       START[0m [         ] (136/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mutility_definition_and_utility_ref_construction[0m

running 1 test
test manifest::app_label_tests::utility_definition_and_utility_ref_construction ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (136/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mutility_definition_and_utility_ref_construction[0m
[32;1m       START[0m [         ] (137/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mvalidate_tutorial_rejects_unsorted_and_out_of_range_tracks[0m

running 1 test
test manifest::app_label_tests::validate_tutorial_rejects_unsorted_and_out_of_range_tracks ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (137/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mvalidate_tutorial_rejects_unsorted_and_out_of_range_tracks[0m
[32;1m       START[0m [         ] (138/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mwindow_kind_deserializes_without_utilities_field[0m

running 1 test
test manifest::app_label_tests::window_kind_deserializes_without_utilities_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.025s] (138/266) [35;1msemio-framework[0m [36mmanifest::app_label_tests[0m[36m::[0m[34;1mwindow_kind_deserializes_without_utilities_field[0m
[32;1m       START[0m [         ] (139/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mactivation_event_can_be_retained_by_manifest_owners[0m

running 1 test
test manifest::kernel::extension_activation_tests::activation_event_can_be_retained_by_manifest_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (139/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mactivation_event_can_be_retained_by_manifest_owners[0m
[32;1m       START[0m [         ] (140/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mextensions_extending_filters_by_extends_at_scale_and_returns_none_for_an_unknown_plugin[0m

running 1 test
test manifest::kernel::extension_activation_tests::extensions_extending_filters_by_extends_at_scale_and_returns_none_for_an_unknown_plugin ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (140/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mextensions_extending_filters_by_extends_at_scale_and_returns_none_for_an_unknown_plugin[0m
[32;1m       START[0m [         ] (141/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfault_after_last_page_ack_closes_the_empty_descriptor_shell_without_page_release_theater[0m

running 1 test
test manifest::kernel::extension_activation_tests::fault_after_last_page_ack_closes_the_empty_descriptor_shell_without_page_release_theater ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (141/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfault_after_last_page_ack_closes_the_empty_descriptor_shell_without_page_release_theater[0m
[32;1m       START[0m [         ] (142/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfixed_command_driver_registry_returns_exact_colliding_owner_without_replacement[0m

running 1 test
test manifest::kernel::extension_activation_tests::fixed_command_driver_registry_returns_exact_colliding_owner_without_replacement ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.024s] (142/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfixed_command_driver_registry_returns_exact_colliding_owner_without_replacement[0m
[32;1m       START[0m [         ] (143/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfixed_page_rejects_nonzero_padding_outside_declared_length[0m

running 1 test
test manifest::kernel::extension_activation_tests::fixed_page_rejects_nonzero_padding_outside_declared_length ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.024s] (143/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mfixed_page_rejects_nonzero_padding_outside_declared_length[0m
[32;1m       START[0m [         ] (144/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_backpressure_retains_the_exact_page_and_retry_cursor[0m

running 1 test
test manifest::kernel::extension_activation_tests::generic_backpressure_retains_the_exact_page_and_retry_cursor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (144/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_backpressure_retains_the_exact_page_and_retry_cursor[0m
[32;1m       START[0m [         ] (145/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_cancel_after_first_page_releases_untouched_tail_in_one_bounded_close_step[0m

running 1 test
test manifest::kernel::extension_activation_tests::generic_cancel_after_first_page_releases_untouched_tail_in_one_bounded_close_step ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (145/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_cancel_after_first_page_releases_untouched_tail_in_one_bounded_close_step[0m
[32;1m       START[0m [         ] (146/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_multi_page_owner_advances_only_after_each_exact_ack[0m

running 1 test
test manifest::kernel::extension_activation_tests::generic_multi_page_owner_advances_only_after_each_exact_ack ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (146/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_multi_page_owner_advances_only_after_each_exact_ack[0m
[32;1m       START[0m [         ] (147/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_stale_generation_status_is_rejected_without_releasing_the_owner[0m

running 1 test
test manifest::kernel::extension_activation_tests::generic_stale_generation_status_is_rejected_without_releasing_the_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (147/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mgeneric_stale_generation_status_is_rejected_without_releasing_the_owner[0m
[32;1m       START[0m [         ] (148/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mmalformed_first_presence_fault_retains_then_closes_each_untouched_page[0m

running 1 test
test manifest::kernel::extension_activation_tests::malformed_first_presence_fault_retains_then_closes_each_untouched_page ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.040s] (148/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mmalformed_first_presence_fault_retains_then_closes_each_untouched_page[0m
[32;1m       START[0m [         ] (149/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mmalformed_middle_presence_fault_preserves_fifo_tail_for_bounded_close[0m

running 1 test
test manifest::kernel::extension_activation_tests::malformed_middle_presence_fault_preserves_fifo_tail_for_bounded_close ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.039s] (149/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mmalformed_middle_presence_fault_preserves_fifo_tail_for_bounded_close[0m
[32;1m       START[0m [         ] (150/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mrejected_command_build_registry_retains_collision_and_releases_one_exact_page[0m

running 1 test
test manifest::kernel::extension_activation_tests::rejected_command_build_registry_retains_collision_and_releases_one_exact_page ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.090s] (150/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mrejected_command_build_registry_retains_collision_and_releases_one_exact_page[0m
[32;1m       START[0m [         ] (151/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mretained_batch_arena_has_no_nested_page_or_descriptor_destructor[0m

running 1 test
test manifest::kernel::extension_activation_tests::retained_batch_arena_has_no_nested_page_or_descriptor_destructor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (151/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mretained_batch_arena_has_no_nested_page_or_descriptor_destructor[0m
[32;1m       START[0m [         ] (152/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mscope_capabilities_to_parent_intersects_and_drops_what_the_parent_lacks[0m

running 1 test
test manifest::kernel::extension_activation_tests::scope_capabilities_to_parent_intersects_and_drops_what_the_parent_lacks ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (152/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mscope_capabilities_to_parent_intersects_and_drops_what_the_parent_lacks[0m
[32;1m       START[0m [         ] (153/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mscope_capabilities_to_parent_is_empty_when_the_parent_grants_nothing[0m

running 1 test
test manifest::kernel::extension_activation_tests::scope_capabilities_to_parent_is_empty_when_the_parent_grants_nothing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (153/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mscope_capabilities_to_parent_is_empty_when_the_parent_grants_nothing[0m
[32;1m       START[0m [         ] (154/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mstale_command_driver_resume_cannot_reanimate_a_reused_direct_slot[0m

running 1 test
test manifest::kernel::extension_activation_tests::stale_command_driver_resume_cannot_reanimate_a_reused_direct_slot ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (154/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mstale_command_driver_resume_cannot_reanimate_a_reused_direct_slot[0m
[32;1m       START[0m [         ] (155/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1msuspended_command_driver_becomes_bounded_close_authority_if_caller_does_not_resume[0m

running 1 test
test manifest::kernel::extension_activation_tests::suspended_command_driver_becomes_bounded_close_authority_if_caller_does_not_resume ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (155/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1msuspended_command_driver_becomes_bounded_close_authority_if_caller_does_not_resume[0m
[32;1m       START[0m [         ] (156/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mzero_presence_page_ack_releases_the_present_empty_owner_then_completes[0m

running 1 test
test manifest::kernel::extension_activation_tests::zero_presence_page_ack_releases_the_present_empty_owner_then_completes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (156/266) [35;1msemio-framework[0m [36mmanifest::kernel::extension_activation_tests[0m[36m::[0m[34;1mzero_presence_page_ack_releases_the_present_empty_owner_then_completes[0m
[32;1m       START[0m [         ] (157/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_dialect_tests[0m[36m::[0m[34;1mreturn_content_existing_dialect_invocation_remains_exact_app_frame[0m

running 1 test
test manifest::kernel::return_content_dialect_tests::return_content_existing_dialect_invocation_remains_exact_app_frame ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.019s] (157/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_dialect_tests[0m[36m::[0m[34;1mreturn_content_existing_dialect_invocation_remains_exact_app_frame[0m
[32;1m       START[0m [         ] (158/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_dialect_tests[0m[36m::[0m[34;1mreturn_content_existing_dialect_presence_preserves_all_render_plane_fields[0m

running 1 test
test manifest::kernel::return_content_dialect_tests::return_content_existing_dialect_presence_preserves_all_render_plane_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (158/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_dialect_tests[0m[36m::[0m[34;1mreturn_content_existing_dialect_presence_preserves_all_render_plane_fields[0m
[32;1m       START[0m [         ] (159/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_framing_tests[0m[36m::[0m[34;1mreturn_content_framing_header_matches_neutral_records_at_each_byte_grant[0m

running 1 test
test manifest::kernel::return_content_framing_tests::return_content_framing_header_matches_neutral_records_at_each_byte_grant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (159/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_framing_tests[0m[36m::[0m[34;1mreturn_content_framing_header_matches_neutral_records_at_each_byte_grant[0m
[32;1m       START[0m [         ] (160/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_framing_tests[0m[36m::[0m[34;1mreturn_content_framing_reader_owns_split_prefix_without_consuming_body[0m

running 1 test
test manifest::kernel::return_content_framing_tests::return_content_framing_reader_owns_split_prefix_without_consuming_body ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (160/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_framing_tests[0m[36m::[0m[34;1mreturn_content_framing_reader_owns_split_prefix_without_consuming_body[0m
[32;1m       START[0m [         ] (161/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_all_endpoints_match_independent_bytes_without_payload_parsing[0m

running 1 test
test manifest::kernel::return_content_message_tests::return_content_message_all_endpoints_match_independent_bytes_without_payload_parsing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (161/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_all_endpoints_match_independent_bytes_without_payload_parsing[0m
[32;1m       START[0m [         ] (162/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_invalid_instance_and_wrong_effect_refuse_without_consuming_source[0m

running 1 test
test manifest::kernel::return_content_message_tests::return_content_message_invalid_instance_and_wrong_effect_refuse_without_consuming_source ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (162/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_invalid_instance_and_wrong_effect_refuse_without_consuming_source[0m
[32;1m       START[0m [         ] (163/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_large_payload_and_cancel_keep_original_source_allocation[0m

running 1 test
test manifest::kernel::return_content_message_tests::return_content_message_large_payload_and_cancel_keep_original_source_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.09s

[32;1m        PASS[0m [   0.104s] (163/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_content_message_tests[0m[36m::[0m[34;1mreturn_content_message_large_payload_and_cancel_keep_original_source_allocation[0m
[32;1m       START[0m [         ] (164/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots[0m

running 1 test

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (8953805) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:94:13:
fixture producer failed after owned placement
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
   3: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   8: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
   9: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
  10: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots' (8953805) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🧪️component.rs:94:13:
fixture producer failed after owned placement
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
   3: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
   4: <core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
   5: std::panicking::catch_unwind::do_call::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}>, ()>
   8: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots
   9: semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0}
  10: <semio_framework::manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test manifest::kernel::return_source_entries_tests::return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.02s

[32;1m        PASS[0m [   0.029s] (164/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots[0m
[32;1m       START[0m [         ] (165/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff[0m

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (165/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff[0m
[32;1m       START[0m [         ] (166/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_over_admission_reports_and_retains_exact_empty_backing[0m

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_over_admission_reports_and_retains_exact_empty_backing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (166/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_over_admission_reports_and_retains_exact_empty_backing[0m
[32;1m       START[0m [         ] (167/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_reserve_before_placement_and_preserve_original_allocation[0m

running 1 test
test manifest::kernel::return_source_entries_tests::return_source_entries_reserve_before_placement_and_preserve_original_allocation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (167/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_entries_tests[0m[36m::[0m[34;1mreturn_source_entries_reserve_before_placement_and_preserve_original_allocation[0m
[32;1m       START[0m [         ] (168/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_inventory_tests[0m[36m::[0m[34;1mreturn_source_native_layout_census_before_backing_admission[0m

running 1 test
[DEBUG] return-source native layout census {"borrowedMessageCursorBytes":96,"effectAlignment":16,"effectBytes":192,"effectPageDescriptorBytes":24,"fixedReturnPageBytes":4098,"fixedReturnResultBytes":4144,"nativeOwnerMounted":false,"pointerBytes":8,"presenceAlignment":8,"presenceBytes":576,"presencePageDescriptorBytes":24,"sourceBackingAdmitted":false,"turnResultAlignment":8,"turnResultBytes":2040,"uiTurnPatchBytes":1768}
test manifest::kernel::return_source_inventory_tests::return_source_native_layout_census_before_backing_admission ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (168/266) [35;1msemio-framework[0m [36mmanifest::kernel::return_source_inventory_tests[0m[36m::[0m[34;1mreturn_source_native_layout_census_before_backing_admission[0m
[32;1m       START[0m [         ] (169/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mrefused_turn_patch_transfer_restores_the_exact_retirement_owner[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::refused_turn_patch_transfer_restores_the_exact_retirement_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (169/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mrefused_turn_patch_transfer_restores_the_exact_retirement_owner[0m
[32;1m       START[0m [         ] (170/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_drop_hands_back_without_waiting_for_arena[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_drop_hands_back_without_waiting_for_arena ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (170/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_drop_hands_back_without_waiting_for_arena[0m
[32;1m       START[0m [         ] (171/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_normal_close_does_not_wait_for_arena[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_normal_close_does_not_wait_for_arena ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (171/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_normal_close_does_not_wait_for_arena[0m
[32;1m       START[0m [         ] (172/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (172/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants[0m
[32;1m       START[0m [         ] (173/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_retirement_max_plus_one_refuses_before_owner_transfer[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_retirement_max_plus_one_refuses_before_owner_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (173/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_retirement_max_plus_one_refuses_before_owner_transfer[0m
[32;1m       START[0m [         ] (174/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_retirement_rejects_stale_epoch_release_and_closes_one_owner_per_step[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_retirement_rejects_stale_epoch_release_and_closes_one_owner_per_step ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (174/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_retirement_rejects_stale_epoch_release_and_closes_one_owner_per_step[0m
[32;1m       START[0m [         ] (175/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_handback_reports_exact_typed_descendant_bytes[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_handback_reports_exact_typed_descendant_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (175/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_handback_reports_exact_typed_descendant_bytes[0m
[32;1m       START[0m [         ] (176/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_lease_drop_hands_back_without_waiting_for_arena[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_lease_drop_hands_back_without_waiting_for_arena ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (176/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_lease_drop_hands_back_without_waiting_for_arena[0m
[32;1m       START[0m [         ] (177/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_max_plus_one_returns_exact_owner_and_session_close_is_incremental[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_max_plus_one_returns_exact_owner_and_session_close_is_incremental ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (177/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_max_plus_one_returns_exact_owner_and_session_close_is_incremental[0m
[32;1m       START[0m [         ] (178/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_normal_close_does_not_wait_for_arena[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_normal_close_does_not_wait_for_arena ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (178/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_normal_close_does_not_wait_for_arena[0m
[32;1m       START[0m [         ] (179/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery[0m

running 1 test

thread 'manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery' (8953909) panicked at 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1997:88:
controlled transport mutex poison
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}::{closure#0}
   3: semio_framework::manifest::kernel::with_ui_turn_patch_transport_arena::<(), semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}::{closure#0}>
   4: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}
   5: std::panicking::catch_unwind::do_call::<semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}, ()>
   6: ___rust_try
   7: std::panic::catch_unwind::<semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}, ()>
   8: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery
   9: semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0}
  10: <semio_framework::manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.021s] (179/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_poison_retains_exact_owner_until_explicit_test_recovery[0m
[32;1m       START[0m [         ] (180/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_producer_drop_hands_back_without_waiting_for_arena[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_producer_drop_hands_back_without_waiting_for_arena ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (180/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_producer_drop_hands_back_without_waiting_for_arena[0m
[32;1m       START[0m [         ] (181/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_rejects_truncated_stale_and_cancelled_tokens[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_rejects_truncated_stale_and_cancelled_tokens ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.031s] (181/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_rejects_truncated_stale_and_cancelled_tokens[0m
[32;1m       START[0m [         ] (182/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_round_trip_is_single_claim_and_preserves_populated_owner[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_round_trip_is_single_claim_and_preserves_populated_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.020s] (182/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_round_trip_is_single_claim_and_preserves_populated_owner[0m
[32;1m       START[0m [         ] (183/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_session_close_waits_for_exact_external_handback[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patch_transport_session_close_waits_for_exact_external_handback ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (183/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patch_transport_session_close_waits_for_exact_external_handback[0m
[32;1m       START[0m [         ] (184/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_close_retires_one_op_or_patch_owner_per_step[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patches_close_retires_one_op_or_patch_owner_per_step ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (184/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_close_retires_one_op_or_patch_owner_per_step[0m
[32;1m       START[0m [         ] (185/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_fixed_serde_visitor_rejects_plus_one[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patches_fixed_serde_visitor_rejects_plus_one ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (185/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_fixed_serde_visitor_rejects_plus_one[0m
[32;1m       START[0m [         ] (186/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_max_plus_one_returns_the_exact_patch_owner[0m

running 1 test
test manifest::kernel::ui_turn_patch_tests::ui_turn_patches_max_plus_one_returns_the_exact_patch_owner ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (186/266) [35;1msemio-framework[0m [36mmanifest::kernel::ui_turn_patch_tests[0m[36m::[0m[34;1mui_turn_patches_max_plus_one_returns_the_exact_patch_owner[0m
[32;1m       START[0m [         ] (187/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_error_messages_are_human_readable[0m

running 1 test
test manifest::media_vocabulary_tests::media_error_messages_are_human_readable ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (187/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_error_messages_are_human_readable[0m
[32;1m       START[0m [         ] (188/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_fingerprint_structured_hashes_json_binary_reuses_blob_hash[0m

running 1 test
test manifest::media_vocabulary_tests::media_fingerprint_structured_hashes_json_binary_reuses_blob_hash ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (188/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_fingerprint_structured_hashes_json_binary_reuses_blob_hash[0m
[32;1m       START[0m [         ] (189/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_types_compatible_covers_direct_any_convert_and_reject[0m

running 1 test
test manifest::media_vocabulary_tests::media_types_compatible_covers_direct_any_convert_and_reject ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.055s] (189/266) [35;1msemio-framework[0m [36mmanifest::media_vocabulary_tests[0m[36m::[0m[34;1mmedia_types_compatible_covers_direct_any_convert_and_reject[0m
[32;1m       START[0m [         ] (190/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1martifact_contribution_descriptor_round_trips[0m

running 1 test
test manifest::plugin_dependency_tests::artifact_contribution_descriptor_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.057s] (190/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1martifact_contribution_descriptor_round_trips[0m
[32;1m       START[0m [         ] (191/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mdependents_returns_direct_dependents_sorted[0m

running 1 test
test manifest::plugin_dependency_tests::dependents_returns_direct_dependents_sorted ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (191/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mdependents_returns_direct_dependents_sorted[0m
[32;1m       START[0m [         ] (192/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_dependency_serde_round_trips_as_a_plain_string[0m

running 1 test
test manifest::plugin_dependency_tests::plugin_dependency_serde_round_trips_as_a_plain_string ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.032s] (192/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_dependency_serde_round_trips_as_a_plain_string[0m
[32;1m       START[0m [         ] (193/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_manifest_dependencies_and_contributions_default_absent_on_the_wire[0m

running 1 test
test manifest::plugin_dependency_tests::plugin_manifest_dependencies_and_contributions_default_absent_on_the_wire ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (193/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_manifest_dependencies_and_contributions_default_absent_on_the_wire[0m
[32;1m       START[0m [         ] (194/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_manifest_with_dependencies_and_contributions_round_trips[0m

running 1 test
test manifest::plugin_dependency_tests::plugin_manifest_with_dependencies_and_contributions_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (194/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mplugin_manifest_with_dependencies_and_contributions_round_trips[0m
[32;1m       START[0m [         ] (195/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_accepts_a_self_satisfying_empty_graph[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_accepts_a_self_satisfying_empty_graph ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (195/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_accepts_a_self_satisfying_empty_graph[0m
[32;1m       START[0m [         ] (196/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_is_deterministic_regardless_of_input_order[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_is_deterministic_regardless_of_input_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.029s] (196/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_is_deterministic_regardless_of_input_order[0m
[32;1m       START[0m [         ] (197/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_names_every_member_of_a_cycle[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_names_every_member_of_a_cycle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (197/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_names_every_member_of_a_cycle[0m
[32;1m       START[0m [         ] (198/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_reports_missing_dependency[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_reports_missing_dependency ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.031s] (198/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_reports_missing_dependency[0m
[32;1m       START[0m [         ] (199/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_reports_version_mismatch[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_reports_version_mismatch ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (199/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_reports_version_mismatch[0m
[32;1m       START[0m [         ] (200/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_toposorts_a_diamond[0m

running 1 test
test manifest::plugin_dependency_tests::resolve_load_order_toposorts_a_diamond ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (200/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mresolve_load_order_toposorts_a_diamond[0m
[32;1m       START[0m [         ] (201/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_ord_matches_semver_precedence[0m

running 1 test
test manifest::plugin_dependency_tests::version_ord_matches_semver_precedence ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.058s] (201/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_ord_matches_semver_precedence[0m
[32;1m       START[0m [         ] (202/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_parses_valid_triples_and_rejects_malformed_input[0m

running 1 test
test manifest::plugin_dependency_tests::version_parses_valid_triples_and_rejects_malformed_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (202/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_parses_valid_triples_and_rejects_malformed_input[0m
[32;1m       START[0m [         ] (203/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_display_round_trips_through_parse[0m

running 1 test
test manifest::plugin_dependency_tests::version_req_display_round_trips_through_parse ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (203/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_display_round_trips_through_parse[0m
[32;1m       START[0m [         ] (204/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_caret_semantics_across_leading_zero_tiers[0m

running 1 test
test manifest::plugin_dependency_tests::version_req_matches_caret_semantics_across_leading_zero_tiers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (204/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_caret_semantics_across_leading_zero_tiers[0m
[32;1m       START[0m [         ] (205/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_exact_and_at_least[0m

running 1 test
test manifest::plugin_dependency_tests::version_req_matches_exact_and_at_least ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (205/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_exact_and_at_least[0m
[32;1m       START[0m [         ] (206/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_tilde_semantics[0m

running 1 test
test manifest::plugin_dependency_tests::version_req_matches_tilde_semantics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.010s] (206/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_matches_tilde_semantics[0m
[32;1m       START[0m [         ] (207/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_parses_all_five_grammar_forms_and_rejects_unknown_operators[0m

running 1 test
test manifest::plugin_dependency_tests::version_req_parses_all_five_grammar_forms_and_rejects_unknown_operators ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (207/266) [35;1msemio-framework[0m [36mmanifest::plugin_dependency_tests[0m[36m::[0m[34;1mversion_req_parses_all_five_grammar_forms_and_rejects_unknown_operators[0m
[32;1m       START[0m [         ] (208/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1madds_first_app_as_active[0m

running 1 test
test platform::tests::adds_first_app_as_active ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (208/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1madds_first_app_as_active[0m
[32;1m       START[0m [         ] (209/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mget_active_app_falls_back_to_first_when_active_id_unknown[0m

running 1 test
test platform::tests::get_active_app_falls_back_to_first_when_active_id_unknown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (209/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mget_active_app_falls_back_to_first_when_active_id_unknown[0m
[32;1m       START[0m [         ] (210/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mnotify_and_notify_chrome_increment_independently[0m

running 1 test
test platform::tests::notify_and_notify_chrome_increment_independently ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (210/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mnotify_and_notify_chrome_increment_independently[0m
[32;1m       START[0m [         ] (211/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mset_active_app_id_is_noop_when_unchanged[0m

running 1 test
test platform::tests::set_active_app_id_is_noop_when_unchanged ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (211/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mset_active_app_id_is_noop_when_unchanged[0m
[32;1m       START[0m [         ] (212/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mset_panel_visibility_is_noop_when_unchanged_else_bumps_chrome_generation[0m

running 1 test
test platform::tests::set_panel_visibility_is_noop_when_unchanged_else_bumps_chrome_generation ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (212/266) [35;1msemio-framework[0m [36mplatform::tests[0m[36m::[0m[34;1mset_panel_visibility_is_noop_when_unchanged_else_bumps_chrome_generation[0m
[32;1m       START[0m [         ] (213/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::append_run_log::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_append_identity[0m

running 1 test
test workflow::run_mutations::append_run_log::tests::metadata_has_the_canonical_append_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (213/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::append_run_log::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_append_identity[0m
[32;1m       START[0m [         ] (214/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::finish_run_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_finish_identity[0m

running 1 test
test workflow::run_mutations::finish_run_node::tests::metadata_has_the_canonical_finish_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (214/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::finish_run_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_finish_identity[0m
[32;1m       START[0m [         ] (215/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::seal_run::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_seal_identity[0m

running 1 test
test workflow::run_mutations::seal_run::tests::metadata_has_the_canonical_seal_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (215/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::seal_run::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_seal_identity[0m
[32;1m       START[0m [         ] (216/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::start_run::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_start_identity[0m

running 1 test
test workflow::run_mutations::start_run::tests::metadata_has_the_canonical_start_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (216/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::start_run::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_start_identity[0m
[32;1m       START[0m [         ] (217/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::start_run_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_start_node_identity[0m

running 1 test
test workflow::run_mutations::start_run_node::tests::metadata_has_the_canonical_start_node_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (217/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::start_run_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_start_node_identity[0m
[32;1m       START[0m [         ] (218/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::tests[0m[36m::[0m[34;1mdescriptors_follow_the_canonical_run_roster[0m

running 1 test
test workflow::run_mutations::tests::descriptors_follow_the_canonical_run_roster ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (218/266) [35;1msemio-framework[0m [36mworkflow::run_mutations::tests[0m[36m::[0m[34;1mdescriptors_follow_the_canonical_run_roster[0m
[32;1m       START[0m [         ] (219/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mapply_run_operation_checked_rejects_everything_after_seal[0m

running 1 test
test workflow::tests::apply_run_operation_checked_rejects_everything_after_seal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.014s] (219/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mapply_run_operation_checked_rejects_everything_after_seal[0m
[32;1m       START[0m [         ] (220/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mchecked_run_admission_matches_the_typed_diff_rejection[0m

running 1 test
test workflow::tests::checked_run_admission_matches_the_typed_diff_rejection ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (220/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mchecked_run_admission_matches_the_typed_diff_rejection[0m
[32;1m       START[0m [         ] (221/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_run_document_matches_schema[0m

running 1 test
test workflow::tests::empty_run_document_matches_schema ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (221/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_run_document_matches_schema[0m
[32;1m       START[0m [         ] (222/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_workflow_default[0m

running 1 test
test workflow::tests::empty_workflow_default ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (222/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_workflow_default[0m
[32;1m       START[0m [         ] (223/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_workflow_snapshot_matches_schema[0m

running 1 test
test workflow::tests::empty_workflow_snapshot_matches_schema ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.021s] (223/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mempty_workflow_snapshot_matches_schema[0m
[32;1m       START[0m [         ] (224/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mfinish_run_node_replacement_inverse_restores_the_original_node_order[0m

running 1 test
test workflow::tests::finish_run_node_replacement_inverse_restores_the_original_node_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (224/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mfinish_run_node_replacement_inverse_restores_the_original_node_order[0m
[32;1m       START[0m [         ] (225/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mmedia_contract_dsl_round_trips[0m

running 1 test
test workflow::tests::media_contract_dsl_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.022s] (225/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mmedia_contract_dsl_round_trips[0m
[32;1m       START[0m [         ] (226/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mplan_workflow_propagates_dirtiness_across_multi_hop_chain[0m

running 1 test
test workflow::tests::plan_workflow_propagates_dirtiness_across_multi_hop_chain ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.023s] (226/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mplan_workflow_propagates_dirtiness_across_multi_hop_chain[0m
[32;1m       START[0m [         ] (227/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mplan_workflow_skips_clean_nodes[0m

running 1 test
test workflow::tests::plan_workflow_skips_clean_nodes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.023s] (227/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mplan_workflow_skips_clean_nodes[0m
[32;1m       START[0m [         ] (228/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mremove_operations_backwards_restores_cascade_deleted_dependents[0m

running 1 test
test workflow::tests::remove_operations_backwards_restores_cascade_deleted_dependents ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.01s

[32;1m        PASS[0m [   0.022s] (228/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mremove_operations_backwards_restores_cascade_deleted_dependents[0m
[32;1m       START[0m [         ] (229/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_is_associative_with_empty_identity[0m

running 1 test
test workflow::tests::run_diff_absorb_is_associative_with_empty_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (229/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_is_associative_with_empty_identity[0m
[32;1m       START[0m [         ] (230/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_preserves_each_append_in_order[0m

running 1 test
test workflow::tests::run_diff_absorb_preserves_each_append_in_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (230/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_preserves_each_append_in_order[0m
[32;1m       START[0m [         ] (231/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_preserves_start_before_later_log[0m

running 1 test
test workflow::tests::run_diff_absorb_preserves_start_before_later_log ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (231/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_absorb_preserves_start_before_later_log[0m
[32;1m       START[0m [         ] (232/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_sequence_rejects_later_steps_without_mutating_the_base[0m

running 1 test
test workflow::tests::run_diff_sequence_rejects_later_steps_without_mutating_the_base ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (232/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_diff_sequence_rejects_later_steps_without_mutating_the_base[0m
[32;1m       START[0m [         ] (233/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_document_dsl_pack_round_trips[0m

running 1 test
test workflow::tests::run_document_dsl_pack_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (233/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_document_dsl_pack_round_trips[0m
[32;1m       START[0m [         ] (234/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_node_record_dsl_pack_round_trips_nested_tables[0m

running 1 test
test workflow::tests::run_node_record_dsl_pack_round_trips_nested_tables ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.028s] (234/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_node_record_dsl_pack_round_trips_nested_tables[0m
[32;1m       START[0m [         ] (235/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_operation_op_text_round_trips_every_variant[0m

running 1 test
test workflow::tests::run_operation_op_text_round_trips_every_variant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.029s] (235/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_operation_op_text_round_trips_every_variant[0m
[32;1m       START[0m [         ] (236/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_payload_serde_uses_exact_camel_case_and_rejects_unknown_fields[0m

running 1 test
test workflow::tests::run_payload_serde_uses_exact_camel_case_and_rejects_unknown_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.019s] (236/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mrun_payload_serde_uses_exact_camel_case_and_rejects_unknown_fields[0m
[32;1m       START[0m [         ] (237/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_flags_cycle[0m

running 1 test
test workflow::tests::validate_workflow_flags_cycle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (237/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_flags_cycle[0m
[32;1m       START[0m [         ] (238/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_flags_dangling_edge[0m

running 1 test
test workflow::tests::validate_workflow_flags_dangling_edge ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (238/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_flags_dangling_edge[0m
[32;1m       START[0m [         ] (239/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_ok_for_acyclic_connected_graph[0m

running 1 test
test workflow::tests::validate_workflow_ok_for_acyclic_connected_graph ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.017s] (239/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_ok_for_acyclic_connected_graph[0m
[32;1m       START[0m [         ] (240/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_snapshot_flags_unresolved_bindings[0m

running 1 test
test workflow::tests::validate_workflow_snapshot_flags_unresolved_bindings ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (240/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_snapshot_flags_unresolved_bindings[0m
[32;1m       START[0m [         ] (241/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_snapshot_requires_edge_xor_input_binding_on_required_ports[0m

running 1 test
test workflow::tests::validate_workflow_snapshot_requires_edge_xor_input_binding_on_required_ports ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (241/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mvalidate_workflow_snapshot_requires_edge_xor_input_binding_on_required_ports[0m
[32;1m       START[0m [         ] (242/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_diff_print_parse_and_encode_decode_round_trip[0m

running 1 test
test workflow::tests::workflow_diff_print_parse_and_encode_decode_round_trip ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (242/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_diff_print_parse_and_encode_decode_round_trip[0m
[32;1m       START[0m [         ] (243/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_media_port_dsl_round_trips[0m

running 1 test
test workflow::tests::workflow_media_port_dsl_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (243/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_media_port_dsl_round_trips[0m
[32;1m       START[0m [         ] (244/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_media_port_id_format[0m

running 1 test
test workflow::tests::workflow_media_port_id_format ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (244/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_media_port_id_format[0m
[32;1m       START[0m [         ] (245/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_operation_backwards_restores_pre_state[0m

running 1 test
test workflow::tests::workflow_operation_backwards_restores_pre_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.015s] (245/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_operation_backwards_restores_pre_state[0m
[32;1m       START[0m [         ] (246/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_operation_op_text_round_trips_every_variant[0m

running 1 test
test workflow::tests::workflow_operation_op_text_round_trips_every_variant ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (246/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_operation_op_text_round_trips_every_variant[0m
[32;1m       START[0m [         ] (247/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_snapshot_dsl_pack_round_trips[0m

running 1 test
test workflow::tests::workflow_snapshot_dsl_pack_round_trips ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (247/266) [35;1msemio-framework[0m [36mworkflow::tests[0m[36m::[0m[34;1mworkflow_snapshot_dsl_pack_round_trips[0m
[32;1m       START[0m [         ] (248/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::add_input::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (248/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (249/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::add_node::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.018s] (249/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (250/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::add_parameter::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (250/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::add_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (251/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::bind_input::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (251/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (252/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_output::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::bind_output::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (252/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_output::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (253/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_parameter_field::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::bind_parameter_field::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (253/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::bind_parameter_field::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (254/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::change_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::change_parameter::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (254/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::change_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (255/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::connect_ports::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::connect_ports::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (255/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::connect_ports::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (256/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::disconnect_edge::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::disconnect_edge::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (256/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::disconnect_edge::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (257/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::move_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::move_node::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (257/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::move_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (258/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::remove_input::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (258/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (259/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::remove_node::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (259/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (260/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::remove_parameter::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (260/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::remove_parameter::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (261/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::rename_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::rename_node::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (261/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::rename_node::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (262/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::tests[0m[36m::[0m[34;1mdescriptors_follow_the_canonical_workflow_roster[0m

running 1 test
test workflow::workflow_mutations::tests::descriptors_follow_the_canonical_workflow_roster ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (262/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::tests[0m[36m::[0m[34;1mdescriptors_follow_the_canonical_workflow_roster[0m
[32;1m       START[0m [         ] (263/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::unbind_input::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.016s] (263/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_input::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (264/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_output::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::unbind_output::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.013s] (264/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_output::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (265/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_parameter_field::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::unbind_parameter_field::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.011s] (265/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::unbind_parameter_field::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
[32;1m       START[0m [         ] (266/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::update_node_ports::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m

running 1 test
test workflow::workflow_mutations::update_node_ports::tests::metadata_has_the_canonical_identity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.00s

[32;1m        PASS[0m [   0.012s] (266/266) [35;1msemio-framework[0m [36mworkflow::workflow_mutations::update_node_ports::tests[0m[36m::[0m[34;1mmetadata_has_the_canonical_identity[0m
────────────
[32;1m     Summary[0m [   4.898s] [1m266[0m tests run: [1m266[0m [32;1mpassed[0m, [1m0[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-Pz7R7e[0m



 NX   Successfully ran target test-wire-retirement-native for project @semio-tech/framework-rs



 NX   Nx detected a flaky task

  @semio-tech/framework-rs:test-wire-retirement-native

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

