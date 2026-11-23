# IO Ports Final: control, control tests, sqlite-snapshot, txt, ifc

Prefix `🧰️framework/🔨️modules/🚪️io/`. Last full runs: `stale-p14.txt` (control, control tests, sqlite) and `stale-p15.txt` (txt lib). Build dir is the shared `native/build-domain`; every run waited 20 to 50 minutes on the cargo artifact lock.

| Target | Before | After |
| --- | --- | --- |
| `⏱️control` lib | 30 of 35 | 32 of 35 |
| `⏱️control` `typed-settlement` | did not compile | compiles, 1 of 1 |
| `⏱️control/🧪️tests` | 24 of 56 | 38 of 56 |
| `🪶️sqlite-snapshot` | 5 of 8 | 8 of 8 |
| txt lib | did not compile (28 errors) | compiles, 13 of 16 |
| txt `preparation_law` | did not compile | compiles, stalls (store law) |
| ifc 4x3 `any` | 3 errors | 121 errors, none in io (editor and templates) |

## Product fixes (root causes)

- `🌱️value/🛫️encode/🎟️effect`: `effect_allocated_with_work` handled a refused physical receipt by returning `Ok(None)`, which surfaced as `awaits an unfinished original reply`, left the settlement pending (Drop assertion) and never credited the caller wallet. New `NativeEncodeEffectRecovery::settle_refused` / `NativeEncodeControl::settle_refused_effect` finish the refused construction action (`finish_refused`), store the refusal as `failure.physical` and complete the settlement; the normal completion path then credits `local += submitted` exactly once. Fixes `first-copy-receipt` (octets), the sqlite `paged_text` refusal and the receipt tests that assert `recipient.submitted == wallet.progress()` together with `failure.receipt_refusal()`. The Source producer path still uses `settle_effect` and its paid refused-receipt transfer unchanged.
- Same function: a refused work or quote begin (`begin_effect_settlement` error) now calls `release_refused_effect` before returning, so a pre-work denial leaves no pending settlement (sqlite `paged_text_prework_denial_keeps_native_ownership_empty` passes).
- `NativeEncodeControl::record_effect_before_post` (new, wraps `NativeEncodeEffectSettlement::record_before_post` with the control's own `OriginalWorkTurn`) and `NativeSnapshotEncodeOwner::record_original_effect` (new): the missing work-recording API of `NativeSnapshotEncodeSettlement` (`record_effect` alone never set `submitted_progress`, so `settle_original_effect` could not run).
- Kernel feature `protocol-laws` now enables `semio-os-store-snapshot-clone/law`, so the `drive_one_item_preparation_law` the txt (and bmp, epw, semio) `preparation_law` targets import exists when the crate is built alone.

## Test ports and redesigns

- `🧪️tests/♻️settlement` (`typed-settlement`) rewritten around the current API: `OriginalWorkRecipient` / `OriginalConstructionRecipient` authorities, `begin_original_settlement`, `record_original_effect`, `settle_original_effect`, `take_receipt`, typed close through `EffectBank`, unwind retained by `retain_operation_unwind` and resumed by `resume_operation_unwind`. The physical-pending row models an interrupted accept after its verdict is recorded (`complete` then panic). The work-pending row has no interruptible callback any more (work commit has no recipient hook; only `admits` and `settle_partial` exist), so it runs the settled flow and asserts the same work accounting (`quote - actual == unspentWork`).
- Frames are retained until a funded close retires them (`with_retirement_owner` keeps the owner). Tests that asserted `!has_retirement_owner()` right after a successful nested receive now close first: nested-io dual-control (born plus close births equal releases, ledger `>=` born), nested cursor and nested receiving macros, encoder returned retirement, wallet short reborrows, txt seven shapes.
- Forwarded allocation ports in the tests crate model the full event vocabulary (`Admit` advances and asserts the prior total, `Quote` asserts it, `Post` sets owned plus actual) and every `NativeAllocationAuthority` is returned through `into_port` (its Drop asserts otherwise).
- Rich drive tests: fixture caller stock raised to the job testkit stock, decoder from `admitted_decoder`, turns driven through `refreshed_turn_with_decoder` until the receiving scope opens (`until_ran`), transient heap of the scope accounted like the job native-progress tests; the denied-work test runs against a control with a zero Work stock because a zero loan fuel never opens the scope.
- Other: `cursor_handoff` quote count 2 to 3 (the frame retirement adds a quote), `publication_slot` denial armed after the body quote instead of by absolute index, `body_receipt` decode test lends a Work stock, typed decode cursor no longer asserts `born == 0` when the cursor stays with the caller (the frame is born and retained), sqlite blob test returns its allocation authority through `into_port`.
- txt `🪶️sqlite/📸️snapshot/🧪️tests`: `mod lent` (open construction recipient, finite Work stock, full-vocabulary allocation port, funded encode and decode, heap-observed close helpers), `encoded` / `stage_failure` for the four-argument `encode_sqlite_snapshot_native`, `receive` with output slots, `receive_pack` with its output slot, forwarded control through `NativeAllocationAuthority::with_send_loan`. All six `txt_original_encoding_*` and paid-receiving laws are ported; four pass.

## Still failing

Control lib (3):

- `original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet` and `original_receiving_canceled_publication_retains_exact_io_error_fields`: a zero-grant decode close returns `Pending{work:0}` (`close_retirement_recipient`, grant without items or depth) and the following funded close under a new effect fails with `original work requires its original issuer and resolved quote`. The pending close invocation is not recoverable from a different effect; the product needs either a stateless unfunded return or a decode counterpart of `recover_close_retirement_recipient_into`.
- `original_receiving_publication_header_denials_retain_exact_output_backing`: the `zero` case copies 0 instead of the 1 intermediate byte; intermediate retirement now demands more copy than `publicationHeader.zeroCopyBytes`. The fixture layout (`🛫️snapshot/🧪️tests/🧫️fixtures/🔣️.json` and its TS twin) must be re-derived from the new demand.

Control tests crate (18), four groups:

- Job testkit transient allocation: the rich drive tests still see about 28 KB born and freed inside the receiving scope or in a failure close that no receipt reports (`(54947,28051)` vs `(26896,0)`, `(56662,56186)` vs `(392,0)`, `(214,63)` vs `(3,0)`). This is the unreceipted source-close allocation owned by the other agent (`source-close-alloc.md`); not touched here.
- Work-action flows that changed shape (`combined_allocation...`, `constructed_effect...`, `encoded_text_retains_original_allocation_post_refusal`, `cursor_advance_unwind...`, `encoder_owned_settlement...`, `work_only_guard...`, `frame_work_post_refusal...`, `publication_slot...`, `native_snapshot_cursor_retains_paid_original...`, `original_optional_and_deferred...`, `decoder_body_receipt_is_debited...`): need per-test re-derivation against `OriginalWorkTurn` and the `finite_work` verdict timing.
- `snapshot_primitive_original_loan...`: `with_original_work_loan` refuses (`callback replaced its initiating context or refilled its work`) once the scope is opened over several turns.

txt lib (3): the SQL progress callback cancellation inside `EncodeNative` and `DecodeNative` does not reach the call (`sqlite_snapshot_txt_native_controls_refuse_inside_copy_and_at_exact_limits`, second half of `seven_shapes`); `original_owner_cancellation_and_funded_close...` asserts `error.retained_progress() == owner.progress()` but the failure keeps the progress at the time of the SQL stage failure (3 items) while the owner later records the release of its intermediates (17 items, 140576 released). txt `preparation_law` stalls inside `drive_one_item_preparation_law` for every artifact (store crate).

ifc 4x3: the crate does not compile because of 121 errors in `🧬️schema/🏷️templates` and `✏️editor` (`app_testing` private, missing `RetainedCloneGrant` imports); none are in io.

## Notes

- No new build dirs. Temporary instrumentation (backtrace on large allocations in `⏱️trace/🧮️memory/🧪️testing/📥️requests`, decode close prints) was removed. The trace showed that the 26896 byte and 2544 byte blocks are the retained frame boxes themselves and are receipted (born equals progress); they only look unreceipted when a test asserts they are released in the same turn.
