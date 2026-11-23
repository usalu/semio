# Port of the `io/⏱️control/🧪️tests` crate (semio-framework-io-control-tests)

Build: `stale-cargo.sh ctt` (label `ctt`). Result: `cargo check --tests` passes; `cargo test` runs 56 tests, 24 pass, 32 fail (open, listed below).

## Before

59 errors: removed `NativeEncode/DecodeWorkRecipient`, `NativeEncode/DecodeConstructionRecipient`, `*Issuer`, `NativeConstructionRecipientLoan`, `RetireOwned::retirement`, `NativeSnapshotEncodeEffectQuote`, `crate::snapshot` path, missing dev-dependency `semio-framework-value-testing-admission`, old arities of `receive`, `drive_cursor`, `with_native_receiving`, `ControlledRetirement::step`.

## After (shared helpers in `🦀️.rs`, used by `🧾️receipt` and `🧭️drive`)

- `Cargo.toml`: dev-dependency `semio-framework-value-testing-admission`.
- Local recipients now implement `OriginalConstructionRecipient` / `OriginalWorkRecipient` (Context `()`, `quote.complete(..)`, work refusal via `settle_partial` with `check.delta()`), lent through `OriginalConstructionAuthority::with_loan` / `OriginalWorkAuthority::with_loan` (or `finite_work`). `ByRef` lends a recipient by `&mut` so it survives unwinds and can be read after the run.
- `Port` adapts a recording closure to `NativeAllocationAuthorityPort`; forwarded controls are built inside `NativeAllocationAuthority::with_send_loan`; port state moved to atomics and mutexes (Send).
- `CloseNative` (decode: birth step when required, then `close_retirement_recipient` in a bank effect; encode: `close_retirement_recipient_into` through `driven`), `StepOwner` (`ControlledRetirement::step_with`) and `close_cursor` (`RetirementCursor::close_into` with per-action heap observation) run each close inside one genuine original effect (`EffectBank`).
- `LegacyEncode` / `LegacyDecode` adapt the old closure-returning `receive`, `receive_nested` and `drive_cursor` onto the output-slot API (progress recorded through `body.record_progress`).
- `funded_encode` / `funded_decode` lend an open construction recipient and an unbounded work stock to an existing control (a bare control now refuses frames without an original Work and construction recipient).
- `NativeSnapshotEncodeEffectQuote` is gone: its tests use `NativeSnapshotEncodeOwner::effect` / `NativeSnapshotBodyWallet::encode_effect`. The encode allocation frame no longer exposes `expected`, `pending`, `collision`: assertions use `allocation_status()`; the capture-law test runs on the decode direction. `drive`: `frame_bytes` (private `NativeSnapshotReceiving` size) replaced by `freed>0`.
- Job turns: `with_native_receiving(&mut out, |native, grant|)` through a local `receiving` helper.

## Still failing (32): root causes found

- Encode allocation through a forwarded control: `charge` on a forwarded native with no quoted effect now refuses (`forwarded native storage requires its quoted allocation effect`, the encode product law restored in this ticket). Tests that call `native.allocate_vec` / `copy_text` / `charge` directly on a forwarded control need effect-based allocation (`allocate_vec_into` with an allocation effect): nested cursor receiving, returned retirement drain, decoder partial owner drain.
- Heap vs receipt equality: nested receive allocates an unreceipted block (2544 B in the encode direction, 148 B in the job turn, 26928 B in the decode cursor case); the 2544 B case matches the schema crate note (`io/⏱️failure-owner` nested receiver). Needs a product diagnosis, not a test change.
- Settlement and quote semantics changed: `encoder_effect_quote_*`, `cursor_advance_*`, `encoder_owned_settlement_*`, `work_only_guard_*`, `combined_allocation_*`, `encoded_*` rely on `resolve_original_post` / quote-run flows that no longer exist; the replacement flow needs re-deriving per test.
- Frame quote: `publication_slot_*` hits `snapshot body exceeds original recipient quote` (grant of the fixture is smaller than the frame quote).
- Job turns: three drive tests report `original receiving scope was not funded` (needs a fuel turn loop as in the job native-progress tests).
- Cursor retirement: `original_optional_and_deferred_child_receipts_*` needs the field cursor transfer re-derived (`ChildProgress` is no longer returned by a single close).

Failing tests:
- body_receipt_tests::combined_allocation_retains_operation_allocation_work_and_physical_refusals
- body_receipt_tests::constructed_effect_requires_original_quote_and_posts_its_grant
- body_receipt_tests::cursor_advance_submits_original_effect_receipt_before_refusal
- body_receipt_tests::cursor_advance_unwind_keeps_effect_and_refusal_until_original_close
- body_receipt_tests::encoded_effect_retains_work_and_recipient_refusals
- body_receipt_tests::encoded_text_denied_work_does_not_charge_or_allocate
- body_receipt_tests::encoded_text_retains_original_allocation_post_refusal
- body_receipt_tests::encoded_text_retains_prefix_and_original_receipt_refusal
- body_receipt_tests::encoder_owned_settlement_records_wallet_once_after_post_unwind
- body_receipt_tests::original_body_retains_both_copy_cancellation_and_submitted_receipt_refusal
- cursor_handoff_quote_denial_keeps_the_original_cursor_with_its_caller
- encoder_effect_quote_posts_after_mutation_even_when_work_and_receipt_refuse
- frame_work_post_refusal_still_publishes_its_actual_birth
- native_snapshot_cursor_retains_paid_original_on_cancellation_and_commits_same_output
- original_io_snapshot_receiving_wallet_survives_short_owner_reborrows
- original_nested_cursor_receiving_keeps_materialized_child_and_original_port
- original_nested_receiving_preserves_actual_child_in_one_original_recipient
- original_optional_and_deferred_child_receipts_share_one_existing_frontier
- original_snapshot_decoder_body_receipt_is_debited_before_cleanup_and_preserved_on_refusal
- original_snapshot_decoder_returns_partial_owner_and_drains_with_its_same_physical_port
- original_snapshot_encoder_returns_refused_retirement_and_commits_only_after_terminal_drain
- original_typed_decode_cursor_retains_canceled_backing_and_exact_supplied_receipts
- publication_slot_keeps_original_output_when_post_refuses
- returned_snapshot_retirement_keeps_original_allocation_port_during_physical_drain
- rich_drive_tests::original_job_encoder_keeps_the_actual_append_on_observer_refusal
- rich_drive_tests::original_job_recipient_gets_actual_body_before_refusal
- rich_drive_tests::original_job_recipient_gets_frame_before_body_refuses
- rich_drive_tests::snapshot_cursor_rich_outcome_conserves_original_work_causes_and_custody
- rich_drive_tests::snapshot_decode_frame_denied_work_preserves_original_cursor_and_backing
- rich_drive_tests::snapshot_encoder_primitive_borrows_the_original_job_work_loan
- rich_drive_tests::snapshot_primitive_original_loan_precedes_effect_and_preserves_failed_work
- work_only_guard_preserves_actual_effect_and_post_refusal_on_unwind
