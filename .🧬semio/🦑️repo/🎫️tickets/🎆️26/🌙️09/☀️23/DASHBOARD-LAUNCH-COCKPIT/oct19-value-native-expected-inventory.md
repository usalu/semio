# Expected Value Native Runtime Inventory

Derived read-only from the current shared dispatch fixture. Native advertised route groups, including BorrowedObservation, are included. Source cases and repeated policy vectors are excluded. Exact package/library/full-law triples are deduplicated; shared laws advertised by several controls appear once. No runtime pass is claimed.

Expected: **66 exact laws across 9 package/library targets**.

| Package | Library | Exact laws |
|---|---|---:|
| semio-framework-value | semio_framework_value | 4 |
| semio-framework-value-edit | semio_framework_value_edit | 12 |
| semio-framework-value-native-decoding | semio_framework_value_native_decoding | 2 |
| semio-framework-value-native-owned | semio_framework_value_native_owned | 12 |
| semio-framework-value-persistent | semio_framework_value_persistent | 13 |
| semio-framework-value-registry | semio_framework_value_registry | 8 |
| semio-framework-value-retained-clone | semio_framework_value_retained_clone | 7 |
| semio-framework-value-shared-issuance | semio_framework_value_shared_issuance | 4 |
| semio-framework-value-tests | semio_framework_value_tests | 4 |

## Evidence Acceptance

For each package/library, retain actual Cargo test-body output naming every exact full law and a passing result. A successful wrapper, compilation, test listing, dispatch model, source oracle, filtered-out/zero-test run, or another crate reexport does not establish execution. Missing laws or failed/ignored laws remain uncovered. Compare full identifiers, not substring filters. Record the log path and matching pass line for each triple after the actual matrix completes.

## semio-framework-value / semio_framework_value

- [ ] `retained_utf8::tests::original_utf8_buffer_funds_transfer_and_every_cancellation_frontier` — controls: test-retained-utf
- [ ] `retained_utf8::tests::original_utf8_cursor_pages_match_portable_scalar_offsets_without_allocations` — controls: test-retained-utf
- [ ] `value::snapshot_projection::immutable_clone::list_tests::persistent_list_retained_clone_preserves_original_root_without_payload_copy` — controls: test-immutable-root-clone
- [ ] `value::snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody` — controls: test-owned-projection

## semio-framework-value-edit / semio_framework_value_edit

- [ ] `indexed_collection_commit::tests::indexed_collection_commit_copies_large_authored_fields_without_cloning_survivors` — controls: test-indexed-edit
- [ ] `indexed_collection_commit::tests::indexed_collection_commit_preserves_original_payloads_and_exact_cancellation` — controls: test-indexed-edit
- [ ] `indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody` — controls: test-fixed-request-owners, test-indexed-edit
- [ ] `indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded` — controls: test-fixed-request-owners, test-indexed-edit
- [ ] `named_edit::index::tests::admitted_named_index_preserves_original_opaque_keys_ranked_sources_and_cancelled_custody` — controls: test-named-key-index
- [ ] `named_edit::index::tests::admitted_named_index_reuses_persistent_originals_without_base_allocation_or_resorting` — controls: test-named-key-index
- [ ] `named_edit::index::tests::transfer::key_preserving_persistent_producer_checks_complete_original_utf8_and_foreign_sources` — controls: test-named-key-index
- [ ] `named_edit::index::tests::transfer::key_preserving_persistent_producer_transfers_original_directory_and_all_paid_cuts` — controls: test-named-key-index
- [ ] `named_edit::index::tests::transfer::receiving::original_named_receiving_checks_full_utf8_under_original_decoder_quantum` — controls: test-named-root-receive
- [ ] `named_edit::index::tests::transfer::receiving::original_named_receiving_publishes_independent_roots_and_closes_every_paid_cut` — controls: test-named-root-receive
- [ ] `named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded` — controls: test-indexed-edit, test-named-edit, test-named-key-index
- [ ] `named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody` — controls: test-indexed-edit, test-named-edit, test-named-key-index

## semio-framework-value-native-decoding / semio_framework_value_native_decoding

- [ ] `native_decoding::retirement_loan::tests::native_retirement_loan_closes_original_returned_payload_after_cancellation` — controls: test-native-retirement-loan
- [ ] `native_decoding::retirement_loan::tests::native_retirement_loan_preserves_original_cancelled_ledger_and_grant` — controls: test-native-retirement-loan

## semio-framework-value-native-owned / semio_framework_value_native_owned

- [ ] `owned::list_tests::native_persistent_list_later_row_refusal_retains_original_partial_owner` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_original_callback_refusal_preserves_partial_receiving` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_receives_original_rows_and_preserves_all_cancelled_custody` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_receiving_fits_original_interaction_quantum` — controls: test-native-persistent-receive
- [ ] `owned::original::tests::native_receiving_original_keeps_cancellation_and_denied_transfers_owned` — controls: test-native-persistent-receive
- [ ] `owned::original::tests::native_receiving_original_shell_requires_funded_input_and_receiver_custody` — controls: test-native-persistent-receive
- [ ] `owned::payload_enum_tests::native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier` — controls: test-native-payload-enum
- [ ] `owned::shared::tests::native_flattened_original_record_preserves_source_schema_and_all_partial_custody` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_record32_single_active_child_fits4096_and_preserves_every_original_prefix` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_shared_original_callback_refusal_retains_paid_child_authority` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_shared_original_receive_exact_prefix_cancellation_and_transfer` — controls: test-native-shared-receive

## semio-framework-value-persistent / semio_framework_value_persistent

- [ ] `list::persistent::build_tests::persistent_owned_builder_cancels_every_preparation_action_frontier` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_constructor_fits_authored_interaction_quantum` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_preserves_ranked_originals_and_every_receiving_frontier` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_refuses_missing_and_excess_without_consuming_input` — controls: test-persistent-build
- [x] `list::persistent::tests::borrowed_observations::borrowed_immutable_observations_preserve_original_rows_without_clone_or_heap` — controls: test-borrowed-observations
- [ ] `list::persistent::tests::traversal::persistent_table_traversal_preserves_original_rows_and_linear_zero_heap_scan` — controls: test-persistent-traversal
- [ ] `list::persistent::tests::traversal::persistent_table_traversal_projects_borrowed_records_and_controlled_wire_without_payload_clone` — controls: test-persistent-traversal
- [ ] `original_vector_concat::tests::original_vector_concat_preserves_original_rows_and_cancelled_custody` — controls: test-original-vector-concat
- [ ] `original_vector_concat::tests::original_vector_concat_wide_payload_and_chunk_directory_remain_bounded` — controls: test-original-vector-concat
- [ ] `retained_equality::tests::retained_typed_equality_cancellation_keeps_original_leases_and_child_frontiers` — controls: test-retained-equality
- [ ] `retained_equality::tests::retained_typed_equality_matches_portable_original_fields` — controls: test-retained-equality
- [ ] `slice_equality::tests::retained_slice_equality_matches_portable_bytes_without_allocations` — controls: test-slice-equality
- [ ] `slice_equality::tests::retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded` — controls: test-slice-equality

## semio-framework-value-registry / semio_framework_value_registry

- [ ] `exact_key_lookup::tests::exact_key_lookup_long_prefixes_and_cancellation_preserve_originals` — controls: test-exact-key-lookup
- [ ] `exact_key_lookup::tests::exact_key_lookup_matches_portable_opaque_identity_without_heap_work` — controls: test-exact-key-lookup
- [ ] `fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release` — controls: test-fixed-key-slots, test-fixed-request-owners
- [ ] `fixed_request_owners::tests::fixed_request_owners_match_portable_consumer_transitions` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::original_source_lookup_matches_portable_reordered_operands` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::provisional_request_consumers_match_portable_live_and_pending_outputs` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::request_selector_preserves_original_binding_until_verified_promotion` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::request_text_and_terminals_match_portable_boundaries` — controls: test-fixed-request-owners

## semio-framework-value-retained-clone / semio_framework_value_retained_clone

- [ ] `retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_clone_logical_body_survives_expanded_physical_headers` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_run_elements_obeys_every_grant_axis` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_cancels_at_every_turn_and_releases_everything` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_capacity_accounting_is_exact` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_turn_count_is_bytes_over_page` — controls: test-bulk-vec-retained

## semio-framework-value-shared-issuance / semio_framework_value_shared_issuance

- [ ] `frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_original_alias_return_slots_transfer_parallel_custody_without_heap` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_original_aliases_preserve_exact_payload_and_funded_header` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_stale_slot_refuses_a_distinct_original_issuance_at_same_address` — controls: test-sealed-frontier

## semio-framework-value-tests / semio_framework_value_tests

- [ ] `original_demand_tests::original_retirement_combined_quote_visits_terminal_leaf_once_with_actual_body_hint` — controls: test-original-retirement-demand
- [ ] `retained_clone::close_demand_tests::derived_original_record_and_enum_preflight_all_axes_and_close_exact_custody` — controls: test-close-demand
- [ ] `retained_clone::close_demand_tests::original_nested_close_preserves_paid_source_and_cancellation` — controls: test-close-demand
- [ ] `retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order` — controls: test-immutable-root-clone

Verified one of66triples: persistent lib borrowed-observation law, command32108 terminalrc0 confirmed by execution owner. Independently read e2e-persistent-borrowed-frontier-current-lock.log5262–70: exact named law passed,1passed0failed18filtered; DEBUG sixcases,3closurebudgets, nonCloneRows, zero comparison/debug heap, originals closed, zero terminal-drop heap, frontier payload4096. Remaining65triples and Source oracles are unverified.

## Current Projection-Carve Inventory (Supersedes Prior Owner Mapping)

Current authored dispatch inventory has69unique package/library/full-law triples across10owners. The prior borrowed observation proof remains one verified triple;68are pending. New quick projection3laws add coverage, and existing owned/root-clone laws follow their actual new physical owners. The prior66row table is retained as historical audit evidence, not the current completion denominator.

### semio-framework-value

Library: `semio_framework_value`

- [ ] `retained_utf8::tests::original_utf8_buffer_funds_transfer_and_every_cancellation_frontier` — controls: test-retained-utf8
- [ ] `retained_utf8::tests::original_utf8_cursor_pages_match_portable_scalar_offsets_without_allocations` — controls: test-retained-utf8

### semio-framework-value-edit

Library: `semio_framework_value_edit`

- [x] `indexed_collection_commit::tests::indexed_collection_commit_copies_large_authored_fields_without_cloning_survivors` — controls: test-indexed-edit
- [x] `indexed_collection_commit::tests::indexed_collection_commit_preserves_original_payloads_and_exact_cancellation` — controls: test-indexed-edit

  Independent runtime readback: `e2e-value-commit-readiness.log:5440–5492` identifies this exact package/library and both full laws above. Two tests passed, zero failed, 12 filtered, in 0.04 seconds. Controls confirmed owned command 91760 whole-command exit zero. Current verified inventory is four of 69 triples, with 65 pending; the supplemental String lifecycle law is separate and has not passed.

  Subsequent supplemental evidence: `e2e-string-lifecycle-explicit-close.log:8256–8262` identifies package `semio-framework-value-tests`, library `semio_framework_value_tests`, and exact law `retained_clone::string_output_tests::string_clone_output_waits_for_complete_owned_payload_and_closes_each_cut`. Independent readback shows one passed, zero failed, 67 filtered in 0.07 seconds, with DEBUG completion/cut/physical-receipt evidence. Controls confirmed command 75394 whole-command exit zero. This replaces the earlier supplemental pending/failing status, but does not alter the advertised inventory: four of 69 remain verified, 65 pending.
- [ ] `indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody` — controls: test-fixed-request-owners, test-indexed-edit
- [ ] `indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded` — controls: test-fixed-request-owners, test-indexed-edit
- [ ] `named_edit::index::tests::admitted_named_index_preserves_original_opaque_keys_ranked_sources_and_cancelled_custody` — controls: test-named-key-index
- [x] `named_edit::index::tests::admitted_named_index_reuses_persistent_originals_without_base_allocation_or_resorting` — controls: test-named-key-index

  Independent runtime readback: `e2e-value-persistent-close-rustup.log:5481–5512` identifies package `semio-framework-value-edit`, library `semio_framework_value_edit`, and this exact full law. It ran one test, passed with zero failures and 13 filtered in 232.91 seconds; DEBUG observations cover lengths 1, 17 and 65537. Controls confirmed owned command 17907 whole-command exit zero. Together with the earlier persistent borrowed-observation proof, two of the current 69 exact triples are verified; the other 67 remain pending. This does not count historical facade/derive bodies as current full-matrix completion.
- [ ] `named_edit::index::tests::transfer::key_preserving_persistent_producer_checks_complete_original_utf8_and_foreign_sources` — controls: test-named-key-index
- [ ] `named_edit::index::tests::transfer::key_preserving_persistent_producer_transfers_original_directory_and_all_paid_cuts` — controls: test-named-key-index
- [ ] `named_edit::index::tests::transfer::receiving::original_named_receiving_checks_full_utf8_under_original_decoder_quantum` — controls: test-named-root-receive
- [ ] `named_edit::index::tests::transfer::receiving::original_named_receiving_publishes_independent_roots_and_closes_every_paid_cut` — controls: test-named-root-receive
- [ ] `named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded` — controls: test-indexed-edit, test-named-edit, test-named-key-index
- [ ] `named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody` — controls: test-indexed-edit, test-named-edit, test-named-key-index

### semio-framework-value-native-decoding

Library: `semio_framework_value_native_decoding`

- [ ] `native_decoding::retirement_loan::tests::native_retirement_loan_closes_original_returned_payload_after_cancellation` — controls: test-native-retirement-loan
- [ ] `native_decoding::retirement_loan::tests::native_retirement_loan_preserves_original_cancelled_ledger_and_grant` — controls: test-native-retirement-loan

### semio-framework-value-native-owned

Library: `semio_framework_value_native_owned`

- [x] `owned::list_tests::native_persistent_list_later_row_refusal_retains_original_partial_owner` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_original_callback_refusal_preserves_partial_receiving` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_receives_original_rows_and_preserves_all_cancelled_custody` — controls: test-native-persistent-receive
- [ ] `owned::list_tests::native_persistent_list_receiving_fits_original_interaction_quantum` — controls: test-native-persistent-receive
- [ ] `owned::original::tests::native_receiving_original_keeps_cancellation_and_denied_transfers_owned` — controls: test-native-persistent-receive
- [ ] `owned::original::tests::native_receiving_original_shell_requires_funded_input_and_receiver_custody` — controls: test-native-persistent-receive
- [ ] `owned::payload_enum_tests::native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier` — controls: test-native-payload-enum
- [ ] `owned::shared::tests::native_flattened_original_record_preserves_source_schema_and_all_partial_custody` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_record32_single_active_child_fits4096_and_preserves_every_original_prefix` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_shared_original_callback_refusal_retains_paid_child_authority` — controls: test-native-shared-receive
- [ ] `owned::shared::tests::native_shared_original_receive_exact_prefix_cancellation_and_transfer` — controls: test-native-shared-receive

### semio-framework-value-persistent

Library: `semio_framework_value_persistent`

- [ ] `immutable_clone::list_tests::persistent_list_retained_clone_preserves_original_root_without_payload_copy` — controls: test-immutable-root-clone
- [ ] `list::persistent::build_tests::persistent_owned_builder_cancels_every_preparation_action_frontier` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_constructor_fits_authored_interaction_quantum` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_preserves_ranked_originals_and_every_receiving_frontier` — controls: test-persistent-build
- [ ] `list::persistent::build_tests::persistent_owned_builder_refuses_missing_and_excess_without_consuming_input` — controls: test-persistent-build
- [x] `list::persistent::tests::borrowed_observations::borrowed_immutable_observations_preserve_original_rows_without_clone_or_heap` — controls: test-borrowed-observations
- [ ] `list::persistent::tests::traversal::persistent_table_traversal_preserves_original_rows_and_linear_zero_heap_scan` — controls: test-persistent-traversal
- [ ] `list::persistent::tests::traversal::persistent_table_traversal_projects_borrowed_records_and_controlled_wire_without_payload_clone` — controls: test-persistent-traversal
- [ ] `original_vector_concat::tests::original_vector_concat_preserves_original_rows_and_cancelled_custody` — controls: test-original-vector-concat
- [ ] `original_vector_concat::tests::original_vector_concat_wide_payload_and_chunk_directory_remain_bounded` — controls: test-original-vector-concat
- [ ] `retained_equality::tests::retained_typed_equality_cancellation_keeps_original_leases_and_child_frontiers` — controls: test-retained-equality
- [ ] `retained_equality::tests::retained_typed_equality_matches_portable_original_fields` — controls: test-retained-equality
- [ ] `slice_equality::tests::retained_slice_equality_matches_portable_bytes_without_allocations` — controls: test-slice-equality
- [ ] `slice_equality::tests::retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded` — controls: test-slice-equality

### semio-framework-value-registry

Library: `semio_framework_value_registry`

- [ ] `exact_key_lookup::tests::exact_key_lookup_long_prefixes_and_cancellation_preserve_originals` — controls: test-exact-key-lookup
- [ ] `exact_key_lookup::tests::exact_key_lookup_matches_portable_opaque_identity_without_heap_work` — controls: test-exact-key-lookup
- [ ] `fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release` — controls: test-fixed-key-slots, test-fixed-request-owners
- [ ] `fixed_request_owners::tests::fixed_request_owners_match_portable_consumer_transitions` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::original_source_lookup_matches_portable_reordered_operands` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::provisional_request_consumers_match_portable_live_and_pending_outputs` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::request_selector_preserves_original_binding_until_verified_promotion` — controls: test-fixed-request-owners
- [ ] `fixed_request_owners::tests::request_text_and_terminals_match_portable_boundaries` — controls: test-fixed-request-owners

### semio-framework-value-retained-clone

Library: `semio_framework_value_retained_clone`

- [ ] `retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_clone_logical_body_survives_expanded_physical_headers` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_run_elements_obeys_every_grant_axis` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_cancels_at_every_turn_and_releases_everything` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_capacity_accounting_is_exact` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape` — controls: test-bulk-vec-retained
- [ ] `retained_clone::bulk_vec_tests::bulk_vector_clone_turn_count_is_bytes_over_page` — controls: test-bulk-vec-retained

### semio-framework-value-shared-issuance

Library: `semio_framework_value_shared_issuance`

- [ ] `frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_original_alias_return_slots_transfer_parallel_custody_without_heap` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_original_aliases_preserve_exact_payload_and_funded_header` — controls: test-sealed-frontier
- [ ] `tests::sealed_shared_stale_slot_refuses_a_distinct_original_issuance_at_same_address` — controls: test-sealed-frontier

### semio-framework-value-snapshot-projection

Library: `semio_framework_value_snapshot_projection`

- [ ] `snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody` — controls: test-owned-projection
- [ ] `snapshot_projection::tests::quick_immutable_snapshot_projection_bad_receipt_keeps_unadmitted_original_owner` — controls: test-immutable-projection
- [ ] `snapshot_projection::tests::quick_immutable_snapshot_projection_composed_record_every_prefix` — controls: test-immutable-projection
- [ ] `snapshot_projection::tests::quick_immutable_snapshot_projection_original_root_and_every_prefix` — controls: test-immutable-projection

### semio-framework-value-tests

Library: `semio_framework_value_tests`

- [ ] `original_demand_tests::original_retirement_combined_quote_visits_terminal_leaf_once_with_actual_body_hint` — controls: test-original-retirement-demand
- [ ] `retained_clone::close_demand_tests::derived_original_record_and_enum_preflight_all_axes_and_close_exact_custody` — controls: test-close-demand
- [ ] `retained_clone::close_demand_tests::original_nested_close_preserves_paid_source_and_cancellation` — controls: test-close-demand
- [ ] `retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order` — controls: test-immutable-root-clone

## Current Matrix Missing Retirement-Loan Bodies

Independent read during58654: `🧰️framework/🔨️modules/🌱️value/🛬️decode/📦️packages/🦀️rust/🦀️.rs` mounts the decoding root; its `♻️retirement/🦀️.rs` is the `native_decoding::retirement_loan` owner. That module ends with the close module/export and has no cfg(test) mount for its existing `🧪️tests/🦀️.rs`, which contains the exact two advertised loan laws. The package already owns required fixture/serde/allocator dev dependencies. Correct triples remain native-decoding/native_decoding::retirement_loan::tests; five passing retirement-continuation bodies cannot substitute. Controls informed to mount the actual two bodies and preserve inventory count69.

Controls additionally identified the latent loan fixture path: native-decoding manifest lives at decode/📦packages/🦀rust, so `../../../♻retirement/...` selects value-level retirement data, while the correct loan fixture is `../../♻retirement/...`. Independent path arithmetic confirms it. Mounting tests must include this exact correction plus Source resolution/shape evidence; merely finding an existing fixture file is insufficient.

## Selected String Close Runtime Acceptance

Independent current readback of `e2e-native-list-selected-string-close.log:5838` identifies package semio-framework-value-native-owned/library semio_framework_value_native_owned and exact law `owned::list_tests::native_persistent_list_later_row_refusal_retains_original_partial_owner`. Actual body passed1/0failed/18filtered in0.26s; DEBUG bad rows0/16/256 retain input/partial custody with exact birth=release203911/673648/7604240 and zero terminal drop. Controls confirmed69731 whole-command0. Current advertised inventory is now5/69 verified,64pending; supplemental String minimum-funded closure and full matrix remain separate/unverified. Earlier counts above are retained chronology.


Matrix20942 whole-command exit1 was reported by the owning Controls agent before Source/compiler/test bodies: cargo metadata --locked --offline rejected stale framework/Cargo.lock. It adds no advertised-law evidence. Current independently verified inventory remains 5/69; no whole-matrix success is claimed.


### Matrix1513 Independent Exact Readback

Regenerated69 unique package/library/full-law triples from current dispatch fixture plus borrowedObservations and matched exact Cargo package/library body sections in e2e-value-matrix-owned-allocator.log. Result61 passed,7 failed,1 not reached. The unstarted advertised law is semio-framework-value-tests / semio_framework_value_tests / retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order. The body active at cancellation is a different nonadvertised companion paged-combined law. Across all observed bodies125 passed,43 failed,1 started but unfinished. Controls confirms owned cancellation whole exit1; no whole-matrix success or missing-law coverage is claimed. Seven failing exact laws:

- semio-framework-value-native-owned / semio_framework_value_native_owned / owned::list_tests::native_persistent_list_receiving_fits_original_interaction_quantum
- semio-framework-value-persistent / semio_framework_value_persistent / retained_equality::tests::retained_typed_equality_matches_portable_original_fields
- semio-framework-value-shared-issuance / semio_framework_value_shared_issuance / frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier
- semio-framework-value-snapshot-projection / semio_framework_value_snapshot_projection / snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody
- semio-framework-value-retained-clone / semio_framework_value_retained_clone / retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes
- semio-framework-value-retained-clone / semio_framework_value_retained_clone / retained_clone::bulk_vec_tests::bulk_clone_logical_body_survives_expanded_physical_headers
- semio-framework-value-snapshot-projection / semio_framework_value_snapshot_projection / snapshot_projection::tests::quick_immutable_snapshot_projection_original_root_and_every_prefix


Native-owned focused37965 is confirmed whole0 by its owner. Independent package/library/fullname readback finds all19 actual bodies passed0failed0filtered12.13s, including all12 currently advertised native-owned triples. The previously failing4096 responsiveness law now passes with965 receiving turns≤2048 and exact physical birth/release708741; the typed default supplemental body also passes. Compared with matrix1513, this supplies one newly passing advertised triple (62 observed passing,6 prior failures,1 still not reached across retained commands), not a clean69-law rerun or whole-matrix success. Support28500 is separate supplemental evidence.


Shared-issuance focused64388 confirmed whole0 by owner; independent exact package/lib/body readback shows7 passed0failed0filtered0.01s, including unchanged advertised frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier and new supplemental denied-admission custody law. Actual DEBUG reports native child handoff1/nestedQueues0 and physical128/128. This converts the retained advertised frontier failure to observed passing:63 advertised observed passes,5 prior failures,1notreached across retained commands; no clean69 rerun or whole-matrix success.


### Complete Matrix64894

Independent current69 exact package/library/fullname comparison against e2e-value-matrix-selected-close.log confirms67 passed and2 failed; all69 were reached, no missing or unfinished advertised body. All package bodies total156 passed32 failed. Owner confirms whole exit101 without cancellation. Remaining failures:

- semio-framework-value-tests / semio_framework_value_tests / retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order
- semio-framework-value-snapshot-projection / semio_framework_value_snapshot_projection / snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody

This supersedes the prior aggregate observed-pass count for this coherent run, but is not a passing matrix. Derive4 and support1 remain separate supplemental successes.


### Stable candidate source safety review

The current private ProjectionOutput seal closes the external safe custom-output initialization hole. A remaining concrete derive constraint was identified: generated placement constructs `&mut MaybeUninit<Field>` from a record field address; a `repr(packed)` record can make that reference unaligned. Requested schema/Syn negative cases rejecting packed and packed(N) before generation. Candidate ownership, denied placement owner restoration, pointer-sized handoff, and partial-field closure were reviewed; no runtime pass is inferred.

Packed-layout blocker correction freshly read: derive now parses every repr attribute as Syn Meta and rejects packed/packed(N) before any generation, including empty records. The private output seal and this explicit alignment constraint close both identified safe-initialization issues. Stable candidate/field/macro source review has no remaining concrete blocker; actual native compilation and all advertised laws remain pending.


RetainedFieldCursor selected-close source diagnosis independently confirmed: wrapper close admission queries an unstarted child before its own child.begin_close effect. The wrapper needs an explicit parent-owned child_closing state and its own depth-one start event. Readiness/take must reject closing; advance quote phase checks should match actual advance ordering. Controls owns schema-first correction and actual proof.

Affected stable-output gate97847 settled whole1 before compiler/body under unchanged262144-byte Source bound (bulk-vec tests22366B versus19115remaining reported by owning Controls). No new body result. Recommended semantic ownership carve keeps private scalar matrix at contract and moves all-public lifecycle/compound/normal-demand integration to existing companion; every real test mount/assertion and exact advertised triple must remain accounted for.

Public integration carve source reread accepted: existing companion mounts scalar_lifecycle_tests (3public laws) and normal_demand_tests (1public law); strict child is created through3real public advance calls with exact grants/heap, private scalar state matrix remains at contract owner. No advertised69 mapping changes or cap increase; actual extent/compiler/body results pending.

Dedicated public retained-clone integration owner lock projections read: framework adds exactly one local0.1.0 package and five intended fixture/facade/allocator/serde/serde_json edges, no registry/removal delta; paired Root projection has no semantic package/registry/edge delta. Owning commands37679/68495 confirmed whole0 separately. Four new public laws remain supplemental; advertised69 unchanged.


Affected11411 owning whole101 reached133bodies/121pass12fail. Exact record failure quote5128copy/release2576 matches the actual sealed Cursor transfer formula sizeof<Option<Arc<T>>>+2sizeof<Option<T>>, releasing Arc allocation sizeofT+16 for inline Nested2560. Stable candidate Box handoff alone does not remove the wide original payload final transfer. Requested exact caller scope label before repair; preserve physical4096 law, no lowered accounting/padding/cap increase.

Strict public fixture failure is its8turn terminal assertion, not a preclose child query panic. Bounded type/lastquote/receipt context is needed before treating this as a production stall; paid binding closure can consume multiple events. Generic normal driver confirms scalar-vector completes every cut before DEBUG, then failure occurs in Vec<String> denied action heap50; actual top-level Vec/String quote admission guards already precede effects. Requested row/cut/axis/demand/result context outside allocator observation, no blanket error normalization. Other12failure repairs remain Controls-owned.
