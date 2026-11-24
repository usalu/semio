# IO Ports Final 2: control lib, control tests crate, txt

Prefix `🧰️framework/🔨️modules/🚪️io/` (control) and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt` (txt). Builds ran through `stale-cargo.sh` (labels `f2a` to `g10t`); runs waited 30 to 90 minutes on the shared artifact lock, two died of `ENOSPC` while the disk was full, one cargo hung in its job queue (killed) and two failed on other agents' in-flight edits (`original_debug_tag`, `guest_linear_memory_over_refusal_ceiling`, the job testing caller).

## Results

| Target | Before | After (last run) |
| --- | --- | --- |
| `semio-framework-io-control` lib | 32 of 35 | 35 of 35 (`stale-g6.txt`) |
| `semio-framework-io-control-tests` | 38 of 56 | 51 of 56 (`stale-g6.txt`) |
| txt lib | 13 of 16 | 15 of 16 (`stale-g8t.txt`), the 16th has a test-helper fix in the tree that is not built yet |
| txt `preparation_law` | stalls | no longer stalls, now fails in the close accounting (see below) |

## Goal 1: control lib (all 35 pass)

### Zero-grant decode close followed by a funded close (4 tests, one root cause in a test helper)

The `Pending{work:0}` theory in `io-ports-final.md` was wrong. A temporary backtrace on `invalid()` in `🌱️value/🎟️control/🧬️contract/🎟️work-action/⚙️operation/🦀️.rs` (removed again) showed that the failing `prepare_for_action` is reached from `decode_close_installed` in `🛫️snapshot/🧪️tests/authority.rs`: the helper ran the failure-birth step (`close_original_birth_step`) INSIDE `with_effect`/`invoked`, i.e. while the effect's own Work action was already open. The birth step is documented, and `close_retirement_recipient` enforces, that it precedes the general effect, so a second `prepare_for_action` on the same Work owner is correctly refused with "original work requires its original issuer and resolved quote". The product is right; the helper now runs the birth outside the effect and then `with_effect` for the ordinary close. A zero grant stays stateless (`refill_original_failure_storage` returns `Progress(default)` before it touches the Work owner), so the later funded close resumes under the same owner. Fixes `original_receiving_nested_io_decode_...`, `..._encode_...`, `..._dual_control_...` and `original_receiving_canceled_publication_retains_exact_io_error_fields`. No product change.

### Header-denials fixture and TS twin (re-derived from the real demand)

Measured with a `[DEBUG]` print of each case's progress (run directly from the built test binary, removed again): the intermediate retirement of an untouched `Option<()>` slot is a ladder of reply shaped steps, not one header copy. Copy demands in order: 425 (424 core start plus the 1 byte header, 392 capacity), 48, 2, 40, 1, 32, 48, 32 for the `receive` entry (total 628); the `cursor` entry adds the 1 byte cursor handoff and a 663 byte ladder (total 664). The 24 byte `Option<String>` publication header follows. Accepted totals: `receive` 652, `cursor` 688.

`🛫️snapshot/🧪️tests/🧫️fixtures/🔣️.json` `publicationHeader`: `intermediateBytes` is replaced by `intermediateHeaderBytes: 1` (the `size_of::<Option<()>>` witness) and `layouts[].copyBytes.{receive,cursor}.{zero,oneShort,exact}` = `{628,651,652}` and `{664,687,688}`; a denied case must have copied exactly the full intermediate retirement. The 32 bit layout rows were removed here and in the retirement-copy fixtures below: their ladders are struct sizes that cannot be measured on this host, so a 32 bit build now fails loudly on the missing row instead of asserting an invented number. TS twin `🛫️snapshot/🧪️tests/🟦️.ts` cross-checks the table with SQLite (`bun test ./🟦️.ts`: the publication test passes; three other tests of that file fail on source string probes that predate this work). The denial message `snapshot publication exceeds original physical grant` never existed in the product; the single site (`publication_progress`) says `... original item grant` for every axis, and the test asserts that text.

## Goal 2: control tests crate (51 of 56)

### The "28 KB born and freed without receipt" in the rich drive tests: neither a leak nor a retained frame

A throw-away allocation tap in `⏱️trace/🧮️memory/🧪️testing/📥️requests` (size histogram plus a raw `backtrace()` on the 632 byte requests, both removed again) showed 70 requests of 632 bytes: the BTreeMap leaf nodes of `serde_json::from_str` inside `fixture_grant()`. `turn()` and `close()` in `🧭️drive/🦀️.rs` evaluated `super::fixture_grant()` inside the `observe_backing` closure, so the test's own JSON parse was counted as an unreceipted turn allocation (28 KB per call, twice in `close()`). The frame (26896/26912 B) is receipted exactly. Fix: parse the grant once before the observed window. No product allocation is unreceipted.

### Per-test derivation

| Test | Cause against current semantics | Change |
| --- | --- | --- |
| `constructed_effect_requires_original_quote_and_posts_its_grant` | `admit_constructed` needs an original Work quote first | lend a finite Work recipient |
| `encoded_text_retains_original_allocation_post_refusal`, `combined_allocation_...` (assertions) | the allocation outcome moves into `failure.encode_effect().allocation` (carrier status `Absent`); the operation cause is `effect.primary`, not `allocation.primary`; `marker.performed` is reset after settlement | read from `encode_effect()`, assert `Absent`, assert the cause at `effect.primary` |
| `encoder_owned_settlement_records_wallet_once_after_post_unwind` | the old `record_effect` never set the submitted progress | `record_original_effect`; the physical recipient completes its verdict then panics (same model as `typed-settlement`); close the receipt |
| `work_only_guard_...` (renamed `..._commits_the_actual_effect_exactly_once_on_operation_refusal_and_unwind`) | plain `prepare_work`/`commit_work` has no recipient verdict any more (`commit` admits `actual <= reserved`; only effects with a performed receipt call `settle_partial`), so a "work post refused" cannot occur | the guard commits the actual effect once in the ok, operation-refusal and unwind rows |
| `frame_work_post_refusal_still_publishes_its_actual_birth` | same: the refusal now surfaces at the first retirement effect of the in-turn ladder, after the body ran | body runs once, `turn.work == 2`, the recipient's first receipt is the birth and its totals cover the owner's accepted progress |
| `publication_slot_keeps_original_output_when_post_refuses` | the 3rd post is no longer the publication (the ladder posts in between) | refuse the post by content (items 1, copy `size_of::<Option<String>>`) |
| decode body, encode cursor copy tests | aggregate copy includes the ladder | `encode-owner/🔣️.json`: `retirementCopy` 747 (3 + 720 ladder + 24), `cursorRetirementCopy` 859 (3 + 24 handoff + 808 ladder + 24); cancelled totals unchanged |
| the same tests, typed decode cursor | a successful receive keeps its frame until a funded close | close first, tally births and releases |
| `original_typed_decode_cursor_...` | cut 0 cancels before any frame exists; Ok copy adds 24 + 808 + 24 | an owner exists iff progress was accepted; `🛬️cursor.json` `headerTransfers.intermediateRetirementBytes {"8":808}` |
| `original_optional_and_deferred_child_...` | every cursor close is reply shaped (`replyHandoffCopyBytes` 64, `childRetirementCopyBytes` 33 in the 64 bit `originalFieldChild` row); scalar child steps return `Progress` receipts; the optional cursor needs one more step after its child transfer | grants carry the handoff, loops accept `Progress`, optional closes its remaining cursor |
| `snapshot_cursor_rich_outcome_...` | a terminal child release now always pays copy (>= 16) | the release step is asserted under its own copy credit instead of "without copy credit" |
| `original_job_recipient_gets_*` | `caller.receipt()` includes the turn's 15 refresh items | compare the callback's `business_receipt()` |
| `cursor_advance_unwind_...` | the unwound effect keeps its original Work issuer, so recovery needs the same authorities | one pair of authorities for unwind and close |

### Not resolved (5 tests)

- `cursor_advance_unwind_keeps_effect_and_refusal_until_original_close` and `combined_allocation_..._refusals` (the closing step): both end with an encode effect settlement retained in a frame or carrier that nothing recovers at close ("original effect settlement remains unresolved", "original owner requires recovery before retirement"). This looks like a product gap rather than a stale test: `NativeSnapshotReceiving::terminal_is_empty` and `close_step` do not account for `action_marker` (the retained `NativeEncodeEffectSettlement` after an unwound or work-refused effect), so the physical receipt of the unwound effect never posts at close. Fix belongs in the frame retirement (recover the marker through the control's `recover_close_retirement_recipient_into` path); not done, needs the effect owner's decision.
- `snapshot_primitive_original_loan_...`, `snapshot_encoder_primitive_...`, `original_job_encoder_keeps_the_actual_append_...`: `with_original_work_loan` refuses ("callback replaced its initiating context or refilled its work") when the receiving scope opens inside the loan, and the later close turns find 0 fuel (`[DEBUG] work quote denied ... fuel=0`, a print of the job-context owner). These are the receiving-scope refund and fuel changes in `🧵️job/⏱️context/🛬️owned` that another agent is landing; re-run after it lands.

## Goal 3: txt (15 of 16, plus law)

- SQL progress cancellation (`sqlite_snapshot_txt_native_controls_refuse_inside_copy_and_at_exact_limits`, SQL half of `seven_shapes`): `TypedFileSettlement::observe` (`🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/⚖️settlement/🦀️.rs`) dropped every native event until the body had finished, so the SQL callback never saw a body-time event. It now forwards body events to the control's checkpoint without settling allocation (`observe_body`, mirroring `ReceivingSettlement::observe_body`), keeps the first refusal and fails the body at `finish_body`; `settlement.refused(...)` now maps the native cancellation to the SQL cause in `decode_typed`, `encode_typed` and the txt encode and decode, so the failure's primary cause is the SQL one. Both targets pass (`stale-g8t.txt`).
- Retained progress vs owner progress: the body failure carried body time progress while the frame then retired its intermediates in the same turn. New `stamp_published_failure` (same module) stamps the published failure with the owner's final cumulative receipts (used by the four sites above). The stale clause `accepted.released_bytes == 0` in `txt_original_encoding_original_owner_cancellation_...` is now `>= extent`: the frame releases the canceled prefix in the same turn (140576 B released, equal to the observed heap). That test then stopped at its zero-grant close helper `lent::close_encode`, whose bank stock could not fund the reply handoff; the helper now sizes the bank with `invocation_extent` (in the tree, not built: the last runs hit other agents' compile errors).
- `preparation_law` stall: the law opened the receiving scope with the step's own 1 item grant, and a scope needs setup items plus control items plus its quantum (`receiving-floor-verification.md`), so `with_native_receiving` never ran and every turn was `Blocked`. `drive_one_item_preparation_law` now opens the scope under the production interactive floor (64 items, 1 MiB copy, 16 MiB capacity, 1 MiB release, depth 4096) raised to the step grant; the scope opens at turn 4 and the law reaches `Prepared`. It then fails in the first quoted close turn: heap (1,693,030 B requested, 856,729 B released) against receipt (0, 64). Not analysed further; either the close turn pays a Source birth inside the observed window (as the testkit's `served` births are excluded from its receipt) or the preparation close allocates unreceipted. The same law is used by bmp, epw and semio.

## Files

- Product: `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/⚖️settlement/🦀️.rs`, `🏪️store/🧬️snapshot-clone/⚖️law/🦀️.rs`, txt `🪶️sqlite/📸️snapshot/🦀️.rs` and `🛫️encoding/🦀️.rs`.
- Tests and fixtures: `⏱️control/🛫️snapshot/🧪️tests/{authority.rs,🦀️.rs,🟦️.ts,🧫️fixtures/🔣️.json,🧫️fixtures/🛬️cursor.json,🧫️fixtures/🎟️encode-owner/🔣️.json}`, `⏱️control/🧪️tests/{🦀️.rs,🧾️receipt/🦀️.rs,🧭️drive/🦀️.rs}`, txt `🧪️tests/🦀️.rs`.
- Every `[DEBUG]` instrument of this pass was removed (the memory tap, the backtrace in `invalid()`, the retire print, the receipt prints, the law print). The `[DEBUG] tags` and `[DEBUG] work quote denied` lines in the logs belong to the job-context owner.
