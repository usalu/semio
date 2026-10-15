# Job Native Test Access Classification

Read-only source audit, 2026-10-15. Concurrent decomposition is active. No build/test execution, lock probe, source edit or git mutation. Paths below are relative to `🧰️framework/🔨️modules/🧵️job`. Existing actor-growth-audit-current.md and current core/worker Cargo manifests were read. No ticketjob-crate-decomposition-current.md was found by filename inventory. This is an access classification, not a passing Source attestation.

## Concrete Decision

Do not preserve the current core unit-test assembly by adding a worker dependency: that reverses the new ownership DAG. Public Original caller/outcome/source laws belong in independent manifest `[[test]]` binaries importing the compiled core or worker library. The fixture caller already supplies real source birth, one stock/recipient, original cancellation, borrowed initiating turn, fuel/deadline/clock, unwind recovery, cumulative receipt and paid root retirement. Use `turn`, `receipt`, `cumulative_receipt`, `born_bytes`, `admit_alias`, `return_alias`, `retire_step`, and `retire`; do not manufacture fresh context/ledger for every turn.

A test file that reads private fields is not movable unchanged merely because its asserted behavior is public. Preserve all law bodies and denied-axis/frontier loops. Separate white-box invariants from black-box fixture construction; private assertions below must remain with their true owner or gain only the narrow capability described below.

## Independent Public Integration Targets

Already registered core targets: `original-turn-failure` (⚠️failure/🧪️tests/🦀️.rs), `original-close-context` (♻️retirement/🎟️context/🧪️tests/🦀️.rs), `original-weak-return` (⏱️context/📦️return/🪶️weak/🧪️tests/🦀️.rs), `original-fixture-caller` (⏱️context/🧪️testing/🎟️caller/🧪️tests/🦀️.rs). Worker already registers `original-payload` (👷️worker/📬️payload/🧪️tests/🦀️.rs). Existing required-features=testkit registrations are observations, not permission to omit laws from the normal configured test run; required testkit must be enabled by that run.

Direct public behavior candidates: work-grant laws; publication and typed-fault-publication laws; paged-source law; normal-worker-failure; worker native preparation and staged metadata laws; portable reconcile contract. Replace `use super::*`, private TEST_RETAINED_POLICY, and old shared fixture helpers with explicit imports and locally owned fixture constants. Keep global allocator observations supplied by Trace. These are candidates after compilation confirms every helper import; this audit did not compile them.

## Private Boundaries and Minimal Capabilities

| Owner / files | Actual access blocking unchanged integration | Minimal legitimate boundary |
|---|---|---|
| Core caller source, child source and capture tests | `parent.original`, `Arc::as_ptr(&actual.payload_ledger)`, `authority.original()` ledger pointer, `returned.original...` custody | Opaque comparable source/ledger identity token or `same_payload_authority` boolean; read-only returned semantic loan preserving lifetime. No Arc/raw ledger return. Receipt refusal already has public accessors. Keep exact internal reference-count/cursor checks private. |
| Core payload builder | `builder.original.as_ptr()`, initialization state, published slot and ledger observations | Opaque builder backing identity and read-only initialized/published status if these are real conformance needs. Production `initialization_demand`, `advance_initialization`, `append_original`, `seal`, retirement APIs drive all actual effects. |
| Core outcome loan/descriptor/fault-binding | Direct `JobPayloadOperationLedger::new/reserve`, page storage writes, `payload.pages/page_count/length/ledger` fixture assembly | Rebuild the identical payload through real admitted builder/publication inside OriginalJobFixtureCaller turns; preserve pages/bytes/denial expectations. This needs no exported mutable payload internals. If exact ledger identity remains, opaque equality only. |
| Core payload retirement and retained ownership first six payload laws | private ledger creation, `with_payload_ledger`, `process_share_bytes`, writer.staged, payload internals | Keep corruption/mismatched identity constructor law private. Read-only accounted-byte count and opaque authority identity suffice for observation; create source pages through real API. Never export reserve/release or atomic counters. |
| Core retained loans and context owner | raw StepContext constructors and exact recipient/context replacement and unwind checks | Public fixture caller handles ordinary loan/unwind tests. Owner lifetime/reference identity checks can stay core-private. Refusal injection must remain private if it deliberately violates receipt/authority invariants. |
| Worker authority, budget worker sections, worker clock-stride section | `WorkerJobAuthorityOwner`, `issued_retained`, receipt pending mutation, `job`, `params`, `payload_ledger`, `preadmitted_fault`, `quarantined_outcome`, overrun state | Canonical worker white-box tests. Do not expose mutable authority. Read-only opaque authority identity, receipt view, quarantine status, overrun summary could support public assertions, but direct injection and paid internal close ladder remain worker-private. |
| Worker retained ownership identity/panic/quiet-wake/slot laws | `mounted.session.inner.authority`, checked_out.authority pointer, preadmitted fault page, private sequence exhaustion, slots | Canonical worker-private. Public outcome/receipt/resume/close tests may split off. Exact identity uses opaque equality token; exhaustion injection stays private. Slot exclusivity fixture is synchronization, not a raw slot export. |
| Worker terminal authority/session and parked wake retirement | `session.inner`, private SessionHandle clones, `session.retirement` header/slot, WORKER_JOB_RETIREMENT_SLOTS and node cast | Keep white-box ownership SCC in worker. Public read-only phase and opaque identity/alias witness may cover observations; never export raw node/slot/Arc or core→worker reverse edge. |
| Session return and erasure/preview | sealed SessionHandle and native session source preparation/private scaffolds | Canonical worker owner. Public erased driver behavior can become worker integration; exact partial-allocation/scaffold/refusal injection remains internal. |
| Scope and registry portions of retained ownership | JobScope/child registry and FixedOperationRegistry | Scope and registry crates own these law bodies, with downward core dependency for actual caller. Never restore them to core through imports. |

## Source-Bounded Grouping

Use one existing domain law file per independently compiled integration target first, not a single integration root that includes all laws. Public core targets: original-work-grant, original-source-publication, original-typed-fault-publication, original-paged-source, original-caller-source, original-child-source, original-caller-capture, original-outcome-loan, original-outcome-descriptor, original-fault-binding, original-payload-builder; the latter eight need the narrowly identified fixture/access rewrites. Public worker targets: original-normal-failure, original-native-preparation, original-staged-metadata, original-worker-turn, original-preview-driver, original-erased-preview-driver. Split the three preview laws into separate roots if their compiled closure exceeds a limit. Do not duplicate production code with path-included component roots.

Core-private closure must retain only true ledger/context/refusal-injection invariants. Worker-private closure must retain true authority/retirement/scaffold/refusal-injection invariants, split into real narrower owners only where the ownership SCC permits. Multiple `mod` test files inside one `#[cfg(test)]` library remain one closure; this alone cannot cure the reported 605KB/84-file core or 263KB worker test frontier. A public integration target compiles against the library without its unit-test modules, so it removes that aggregate, but each library production Source and each target still requires the unchanged 262KB/32-file/4000-line census. No count here claims qualification; moving a private law to an integration target through a public giant test runner is not independent decomposition.

## Exact Existing Test Inventory

Every name below is an existing #[test] body in this snapshot. Inclusion here does not imply public access. File ownership and specific private dependencies are above. Keep all these laws active during moves.

### `♻️session-return/🧪️tests/🦀️.rs` (5121 bytes)

- `original_session_frame_return_has_exact_custody`

### `⚠️failure/🧪️tests/🦀️.rs` (20468 bytes)

- `original_turn_failure_domain_admission_preserves_full_source_and_paid_close`
- `original_turn_failure_debit_preserves_both_owned_causes`
- `original_turn_failure_paid_retirement_preserves_denied_sources`
- `original_turn_failure_preserves_full_diagnostic_custody`
- `original_turn_failure_preserves_three_complete_owned_causes`

### `⚠️failure/📦️domain/🤝️closure/🧪️tests/🦀️.rs` (6109 bytes)

- `original_source_closure_full_pair_same_caller_denials_and_paid_close`

### `🔎️reconcile/🧪️tests/🔬️contract/🦀️.rs` (1437 bytes)

- `neutral_reconcile_envelopes_match_language_agnostic_vectors`

### `🎟️context/🪙️loan/🧪️tests/🦀️.rs` (11266 bytes)

- `original_retained_loan_matches_neutral_inputs_and_nested_ceiling`
- `original_retained_loan_physical_failure_preserves_owned_cause_and_paid_close`
- `original_retained_loan_unwind_restores_parent_without_new_context`
- `original_retained_loan_swap_and_replace_preserve_both_original_causes`
- `original_retained_loan_swap_unwind_never_promotes_replacement`

### `🧪️tests/🔬️fixed-operation-registry/🦀️.rs` (11498 bytes)

- `maximum_plus_one_and_saturation_return_the_exact_owner`
- `stale_generation_interrupted_close_and_aba_preserve_exact_authority`
- `maximum_registry_backing_initializes_inside_one_interactive_ceiling_under_concurrent_load`

### `🧪️tests/📏️close-demand/🦀️.rs` (10266 bytes)

- `worker_close_demand_queries_preserve_exact_physical_grants`
- `worker_close_receipt_preserves_all_original_currencies_without_heap_effects`
- `original_failed_job_close_preserves_actual_allocated_receipt_before_refusal`

### `🧪️tests/🧪️fixed-operation-registry-cases/🦀️.rs` (9978 bytes)

- `language_neutral_fixed_operation_registry_laws_execute_production_registry`

### `🧪️tests/🔬️clock-stride/🦀️.rs` (7402 bytes)

- `a_clock_stride_reads_about_once_per_interval_and_bounds_its_overshoot`
- `a_new_step_starts_at_its_drivers_reading_and_keeps_the_calibrated_stride`
- `a_worker_step_reads_the_clock_once_on_entry_and_once_on_exit`
- `the_logical_clock_advances_one_microsecond_per_read_per_thread`

### `🧪️tests/🔬️retained-ownership/🦀️.rs` (66797 bytes)

- `retained_payload_physical_close_preserves_short_pages_until_the_exact_backing_grant`
- `retained_writer_physical_close_preserves_staged_and_rejected_backing`
- `mounted_close_on_a_short_byte_grant_walks_every_named_phase_to_terminal`
- `payload_ledger_identity_must_match_the_exact_step_context`
- `retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned`
- `retained_state_and_output_have_separate_credits_and_close_one_page_per_grant`
- `retained_writer_and_reader_advance_exactly_one_page_per_opportunity`
- `child_registry_max_plus_one_stale_duplicate_exhaustion_and_parent_completion_are_exact`
- `worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout`
- `worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact`
- `worker_pool_rejection_returns_exact_job_before_resume`
- `worker_panic_and_quiet_wake_publish_one_durable_terminal_intent`
- `worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed`
- `batch_session_advances_exactly_one_external_opportunity`
- `checked_out_and_worker_begin_close_transitions_report_exact_zero_release`
- `worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned`
- `hostile_retained_box_and_arc_require_complete_independent_physical_grants`
- `mounted_original_cancel_alias_returns_only_to_its_still_borrowed_matching_root`

### `👷️worker/🎟️turn/🧪️tests/🦀️.rs` (14635 bytes)

- `original_worker_turn_receipt_preserves_narrow_callback_and_exactly_once_recipient`
- `original_batch_receipt_and_mutable_job_fence_preserve_none_semantics_and_original_authority`
- `original_worker_admission_preserves_sources_metadata_zero_and_exact_native_birth`
- `original_context_cancel_alias_preserves_same_root_paid_metadata_zero_and_return_witness`

### `👷️worker/⚠️failure/🧪️tests/🦀️.rs` (6000 bytes)

- `original_normal_worker_failure_loans_both_causes_after_receipt_and_paid_close`

### `👷️worker/🚪️admission/🏗️preparation/🧪️tests/🦀️.rs` (12875 bytes)

- `original_native_preparation_writes_final_fields_and_preserves_every_paid_cancellation_frontier`

### `👷️worker/🚪️admission/🚚️staged/🎟️metadata/🧪️tests/🦀️.rs` (7521 bytes)

- `original_staged_source_preserves_metadata_zero_exact_ranges_and_native_custody`

### `👷️worker/🔀️erasure/🧪️tests/🦀️.rs` (13876 bytes)

- `original_native_session_erasure_keeps_refused_birth_and_every_partial_return`

### `👷️worker/📬️payload/🧪️tests/🦀️.rs` (4810 bytes)

- `original_worker_payload_denials_and_actual_paid_lifetime_match_portable_contract`

### `🏃️work/🎟️grant/🧪️tests/🦀️.rs` (6708 bytes)

- `original_retained_work_funding_preserves_axes_and_physical_receipts`
- `original_step_context_forwards_independent_work_funding`

### `⏱️context/📦️owner/🧪️tests/🦀️.rs` (7523 bytes)

- `retained_step_context_owner_turns_use_original_ledger_without_heap_birth`
- `retained_step_context_owner_preserves_original_ledger_until_context_and_writer_return`

### `⏱️context/📦️return/🪶️weak/🧪️tests/🦀️.rs` (6947 bytes)

- `original_native_weak_return_preserves_denied_source_and_posts_exact_metadata`
- `original_native_weak_nested_unwind_and_context_swap_expire_scope`

### `⏱️context/📸️capture/🧪️tests/🦀️.rs` (8254 bytes)

- `original_caller_capture_preserves_paid_aliases_supplier_identity_and_exact_receiving`

### `⏱️context/🎟️source/👶️child/🧪️tests/🦀️.rs` (9436 bytes)

- `original_child_source_issues_business_identity_once_and_closes_every_paid_prefix_under_same_parent`

### `⏱️context/🎟️source/🧪️tests/🦀️.rs` (9570 bytes)

- `original_owned_caller_source_loan_preserves_static_roots_actual_turn_and_paid_parent_return`

### `⏱️context/🧪️testing/🎟️caller/🧪️tests/🦀️.rs` (13468 bytes)

- `original_fixture_caller_keeps_one_stock_recipient_fuel_and_paid_root_close`
- `original_fixture_caller_restores_narrowed_loan_after_unwind_and_deadline`
- `original_fixture_caller_keeps_monotonic_clock_observation_across_turns_and_root_close`
- `original_fixture_caller_work_loan_preserves_actual_fuel_deadline_clock_and_recipient`

### `📬️outcome/⚠️fault-binding/🧪️tests/🦀️.rs` (2908 bytes)

- `original_fault_binding_keeps_actual_page_and_same_spent_recipient_without_clock`

### `📬️outcome/🤝️loan/🧪️tests/🦀️.rs` (8051 bytes)

- `original_semantic_outcome_loans_preserve_payload_identity_fixed_grants_and_current_receipts`

### `📬️outcome/📦️builder/🧪️tests/🦀️.rs` (19267 bytes)

- `original_empty_payload_header_initialization_preserves_zero_copy_at_every_native_prefix`
- `original_in_place_payload_builder_preserves_fixed_grants_source_pages_and_all_cancellation_frontiers`
- `original_payload_authority_admits_once_and_borrowed_contexts_preserve_original_ledger_and_receipts`
- `original_payload_pages_preserve_narrow_capacity_exact_extents_and_native_conservation`

### `📬️outcome/📖️source/🧪️tests/🦀️.rs` (4608 bytes)

- `original_job_paged_source_preserves_header_authority_bytes_and_zero_heap_borrows`

### `📬️outcome/📤️publication/⚠️fault/🧪️tests/🦀️.rs` (7775 bytes)

- `original_typed_fault_publication_streams_canonical_wire_without_truncation_and_preserves_native_custody`

### `📬️outcome/📤️publication/🧪️tests/🦀️.rs` (7530 bytes)

- `original_source_publication_preserves_native_identity_paid_stages_and_every_cancellation_frontier`

### `📬️outcome/🎟️descriptor/🧪️tests/🦀️.rs` (5819 bytes)

- `original_outcome_descriptors_keep_same_payload_and_receipt_until_paid_ack`

### `🔭️preview/🎟️drive/🧪️tests/🦀️.rs` (22033 bytes)

- `original_preview_driver_preserves_native_receipts_fault_loan_and_paid_closure`
- `original_erased_preview_driver_preserves_native_receipts_fault_loan_and_paid_closure`
- `original_cancelled_preview_erasure_returns_original_source_before_paid_closure`

### `♻️retirement/📄️payload/🧪️tests/🦀️.rs` (14044 bytes)

- `job_payload_retirement_original_variants_page_and_ledger_release_match_actual_heap`
- `job_payload_retirement_staged_writer_preserves_original_ledger_after_page_release`
- `job_outcome_slot_retirement_keeps_original_header_until_separate_admitted_metadata`
- `job_payload_retirement_original_fault_slot_closes_in_place_under_fixed_body`

### `♻️retirement/🎟️context/🧪️tests/🦀️.rs` (3052 bytes)

- `original_close_context_denied_work_and_deadline_preserve_owned_source`

### `♻️retirement/🔔️wake/🧪️tests/🦀️.rs` (16232 bytes)

- `worker_original_typed_wake_retirement_preserves_native_lease_aliases_and_physical_payload`
- `worker_original_typed_wake_live_session_keeps_custody_through_wake_and_all_paid_frontiers`
- `worker_original_typed_wake_parked_session_keeps_original_arc_and_separate_node_release`
- `worker_original_typed_wake_native_thread_issuer_preserves_actual_frame_and_tls_lease`

### `♻️retirement/👷️authority/🧪️tests/🦀️.rs` (28706 bytes)

- `worker_terminal_authority_retirement_preserves_original_ledger_then_in_place_backing`
- `worker_terminal_job_retirement_keeps_original_inline_job_for_in_place_removal`
- `worker_terminal_params_retirement_preserves_original_cancellation_and_separate_cursor`
- `worker_terminal_session_retirement_preserves_original_arc_aliases_and_funded_scaffolds`
- `original_source_preparation_keeps_failed_effects_in_same_incoming_recipient`
- `original_destination_initialization_preserves_paid_page_custody_at_every_abandonment`

### `⏱️budget/🧪️tests/⏱️budget/🦀️.rs` (37039 bytes)

- `microsecond_zero_expired_or_empty_fuel_never_enters_job`
- `microsecond_language_neutral_deadline_boundaries_and_overflow`
- `microsecond_retained_worker_admits_half_ms_and_rejects_missing_or_overflow_clock`
- `microsecond_platform_clock_and_real_half_ms_worker_progress`
- `microsecond_exact_callback_quarantine_retains_original_output_and_session_identity`
- `descheduled_step_over_the_ceiling_is_recorded_and_the_job_keeps_running`
- `every_step_over_the_ceiling_is_quarantined_within_the_sustained_threshold`
- `microsecond_browser_clock_fraction_and_invalid_source_vectors`
- `original_job_step_context_conserves_actual_independent_retained_wallet`
- `original_worker_job_retains_actual_step_receipt_across_producer_panic`
- `original_job_failed_receipt_ingress_keeps_actual_external_recipient`

