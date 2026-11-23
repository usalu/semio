# Record Open: Phase Findings

## State

With the other agents' concurrent fixes, 3 of 4 `semio-os-store-member-open-store --lib` tests pass, including both receive tests (no "type, field or phase is invalid" refusal any more; the temporary backtrace prints on all three `invalid()` definitions never fired). Only `genuine_original_drawing_member_receives_closes_and_funds_exact_handoff` fails.

## Genuine test failure (root cause)

The close refusal is `InvariantViolated "schema close changed Source or Work"` raised by `close_dispatch` (`🧩️record/🔎️schema/🦀️.rs`) while the open is in phase Receive (6) and a schema matcher is still resident. A debug print of the three guard conditions showed: mode matches, but `invocation.source_turn()` is `None` for the one close attempt (336 receive-time calls had a Source turn).

`OriginalRecordMemberSnapshotOpen::close_step` (and its `Phase::CloseReceiving` step) close the receiver through a plain `cx.with_original_effect(..)` (`with_original_effect_at(None, ..)`), so the effect carries no Source turn. A mid-flight matcher close requires a receiving scope that carries the Source turn witness (`advance` gets it through `native.source_witness()`), or the decoder-owned close loan (`requires_original_control` / `close_original_birth_step`). The close happens after a cancel in the middle of Receive, so the open cannot close its resident matcher alone.

Not fixed: the fix is a design change in the close path (give the receiver close a witness-carrying effect, like `received(..)` plus a witness-based close on `NativeReceivingOriginal`, or route through the decoder's close loan). It touches `🧳️original`, `🔎️schema` close and the job context (`🧵️job/⏱️context/🛬️owned`, not mine).

An earlier run also showed an unexplained one-off heap mismatch at the first Owner admission step (heap 156 vs receipt 48, extra 108 bytes); it disappeared in later runs after other agents' edits and was not reproduced.

## Flatten ordinal law

`native_flattened_ordinals_refuse_ambiguous_declared_ids` was stale under the new ordinal semantics (see `record-field-match.md`): an ordinal key addresses the flatten field itself at its own index. Updated and renamed to `native_flattened_ordinals_address_the_flatten_field_itself` in `🔗️sealed-shared-receiver/🧪️tests/📖️public/🦀️.rs` (ordinal 0 gives Field(0), 1 gives Field(1), 2 Missing; name keys unchanged). No fixture or TS twin exists for this inline test.

## Cleanup

All `[DEBUG]` prints and the temporary allocation tap and fixture edits were removed. Remaining edits: the renamed law and a `Debug`-free test state identical to HEAD apart from earlier work.

## Results

`semio-framework-value-native-owned --no-fail-fast --test-threads=1`: all suites green, including `native_flattened_ordinals_address_the_flatten_field_itself` and `native_dsl_enum_ordinals_accept_only_non_negative_u32_numbers` (the latter passes single-threaded; its parallel failure was not reproduced, so it was not changed). The other three framework crates were not re-run in this pass.

## Close design (implemented)

A resident schema matcher closes only in an effect whose Work carries the Source's own turn. `JobContext::with_original_source_effect` already provides exactly that, so no parallel witness API was added:

- `🫴️record/🦀️.rs`: new `closed_on_source` (same as `closed` but over `with_original_source_effect`); `Phase::CloseReceiving` uses it for the receiver close.
- `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`: fixture `with_source_effect` next to `with_effect`.
- `🫴️record/🧪️tests/🦀️.rs`: `sourced` poll helper; `close_open` closes through it. The genuine test's cut points (0, 1, 12, 50, 200) are the mid-receive close law: each closes a resident receiver after that many turns, with denied-grant probes (no progress, no heap) before the funded close.
- `🛬️owned` was not edited.

## Verification status (incomplete)

- Cuts 0, 1, 12, 50 and 200 of the genuine test now close without refusal (observed in a run).
- The final cut (100000, a full open to Ready, then take, then close) did not finish within the time I could give it: shared-target lock contention and other agents' `[DEBUG]` prints in the host-stall area slowed each turn to seconds. So the genuine test, the other 3 record-open tests after this change, `-native-storage`, `-native-decoding`, `-value-derive` and `semio-framework-job --features testkit` were NOT re-run. `-native-owned` was green before this change (single-threaded).
