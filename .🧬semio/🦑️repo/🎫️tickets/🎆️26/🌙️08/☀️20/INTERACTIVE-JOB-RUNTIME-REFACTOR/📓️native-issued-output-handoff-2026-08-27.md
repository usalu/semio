# Native Issued Output Handoff

## Verified Boundary

Common Kernel patch-owner retirement passed twelve native tests after separate held-mutex REDs. Actor patch-receipt codec/outer tests passed three tests plus five lifecycle regressions. These are independent component boundaries, not proof that native host or guest lifecycle aggregation is complete.

## Actual Remaining Sources

`plugin/🖥️host/⏳️runtime.rs::convert_poll_success` moves WIT effects before constructing an empty Kernel patch collection. `plugin/🖥️host/🦀️component.rs::execute_turn` likewise returns an empty patch collection and drains `emit_patch_sink` without a consumer. A malformed effect can fail while the remaining original WIT result is only an ordinary local value. Neither conversion has a retained typed output cursor.

`plugin/🖥️host/🧵️shard/🦀️component.rs::to_actor_turn_result_in_place` creates a transport token even for an empty Kernel patch collection. Canonical no-patch output must instead be zero bytes with no receipt. The nonempty branch must validate the typed one-patch/one-receipt pairing before any move. Its existing transport token identifies native storage only; it is not the guest-issued patch receipt and cannot replace it.

`kernel::UiTurnPatchTransportLease::take_owner` currently frees the transport slot before the returned typed root retires. `UiTurnPatches::IntoIterator` and its generic callback transfer likewise cannot prove downstream descendant retirement. The live WIT converter uses the iterator; the WGPU host uses the callback transfer. These must become in-place, preadmitted typed-owner handoffs, not a callback that can unwind after receiving a raw patch.

## Required Retained Phases

1. Reserve the exact native output slot before calling the guest. Capture the entire returned WIT result in that slot before reading receipts, converting effects, or invoking callbacks.
2. Validate both receipt envelopes and the typed patch-count pairing without removing the captured result. Retain malformed or unknown output with a fault; absence of a receipt is not permission to discard a nonempty patch root.
3. Construct one `UiPendingPatch` under caller admission. Retain WIT input and typed output simultaneously while each field/page conversion advances. Whole WIT array lifting remains a separately accounted preexisting boundary; no per-step bound is inferred from it.
4. Transfer the exact typed owner, issued receipt, and native transport completion authority into a preadmitted consumer slot. A target failure leaves the source and cursor mounted. Do not hand a raw owner to arbitrary user callbacks.
5. ACK and rejection address the original guest lifetime and patch sequence. Surface/revision are additional payload checks. Stage feedback consumption and commit it only after the strict real-clock verdict; retries are idempotent and preserve the original root.
6. Native output and guest lifecycle release require exact child completion receipts. The host JS surface aggregate is a separate participant. Source-slot absence, numeric-instance absence, global arena emptiness, and generic Idle are not substitutes.

## Transport Contention Packet

Three new native laws preserve the exact owner, hold the old transport mutex for 100ms, and assert that producer Drop, lease Drop, and a normal close call do not wait. They recover and drain the original slot before assertion, so the expected RED does not intentionally strand payloads. The compiler owner has these queued against unchanged transport production.

The intended correction uses static fixed slots, pre-reserved atomic return/close handbacks, and try-lock operations. Blocked and poisoned states must be distinguished from terminal completion. The return message cannot be overwritten, and a checked-out slot cannot be reused until the exact returned owner or downstream completion witness is consumed. This lock correction alone will not establish the in-place consumer handoff described above.

## Scope Still Open

Actual guest open/capture registry, all-child preadmission, pending patch issuance and feedback, raw native WIT conversion, terminal release before the final clock, same-activation instance reuse, and full native/Wasm lifecycle gates remain required. No clean, deletion, or evidence relocation was performed by this lane.
