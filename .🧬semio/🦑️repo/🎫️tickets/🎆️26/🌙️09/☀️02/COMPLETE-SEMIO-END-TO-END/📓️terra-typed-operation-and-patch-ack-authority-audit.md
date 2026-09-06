# Typed Operation Token and Patch ACK Authority Audit

## Typed-operation renderer ACK: no demonstrated in-process ABA

The absence of a guest-lifetime field in `TypedOperationResultToken` is not, by itself, an ABA
bug in the current process-local path. The actual acceptance chain is narrower than a bare
receiver id:

1. The renderer wire parser accepts only the exact magic plus its fixed 25-byte token body
   ([`🔌️plugin/🦀️.rs:13285`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13285)).
2. The turn accepts it only when the `Shell` source's numeric instance equals
   `token.receiver` ([`⚛️reactor/🔄️turn/🦀️.rs:269-272`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:269)).
3. `plugin_acknowledge_typed_operation_result` then resolves the *current* runtime app cell for
   that receiver ([`🔌️plugin/🦀️.rs:32073`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32073)). A closing app has been moved out of that registry, and a
   reopened app begins with an empty typed-operation registry.
4. `VcsArtifactApp::acknowledge_typed_operation_result` requires the live receiver, exact
   `u64` operation key, full page-token equality, and the unique `AwaitingAck` page
   ([`:24836-24856`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24836)). The
   fixed registry uses the modulo index only to locate a slot and then compares the full operation
   id ([`:15683-15722`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15683)).
5. One operation owns at most one result page; an ACK clears it and advances its checked result
   sequence ([`:16436-16508`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16436)). A duplicate or a prior-page token cannot consume the successor.

Production typed-command admission obtains `operation` from the process-wide atomic allocator,
not the wire or a per-app counter
([`:23103-23120`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23103),
[`⏱️trace/🦀️.rs:645-650`](../../../../../../🧰️framework/🔨️modules/⏱️trace/🦀️.rs:645)). That static has no
observed reset inside a process. Together with `generation`, `sequence`, and the exact current
page, it prevents an old ACK from an earlier operation or an old numeric app instance from
acknowledging a current page. The current native result-page laws already exercise delayed exact
ACKs and stale sequences at the mounted-operation layer
([`🔌️plugin/🦀️.rs:17694-17754`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:17694)).

Two qualifications, neither evidence of a current receiver-reuse exploit:

- `NEXT_OPERATION_ID.fetch_add` wraps despite its “never 0” doc claim. After `2^64` allocations it
  can reuse an operation id, including zero ([`⏱️trace/🦀️.rs:647-650`](../../../../../../🧰️framework/🔨️modules/⏱️trace/🦀️.rs:647)). That is a genuine exhaustion/identity defect, but not a
  lifecycle counter reset. Its only ACK trace requires a hostile old token retained for that many
  allocations and all remaining fields to recur. It merits a later checked atomic allocator that
  permanently refuses exhaustion; it does **not** justify adding guest lifetime to this token now.
- A process restart resets the static allocator. I found no surviving process-crossing Shell ACK
  channel that can replay an old renderer message into a fresh `PluginRuntime`, so cross-restart
  ABA is unproven. If restart-resilient renderer delivery is later introduced, the transport must
  fence it with a runtime/session epoch (or the existing guest lifetime) before replay—not infer
  safety from the current process-local allocation rule.

`attempt` is currently always `1` in `next_token`; `publication_attempt` instead counts internal
publication retries ([`:16433`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16433),
[`:22509-22520`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22509)). That makes
the field semantically redundant today, but full page-token equality does not rely on it for
protection.

## Patch ACK: current receipt is a concrete ignored authority

This path is different. `Event::PatchAck` carries an `ActorUiPatchReceipt`, whose wire format
already includes activation generation, instance, guest lifetime, and a nonzero patch sequence
([`🎭actor/🚪️lifetime/🩹️patch/🦀️.rs:13-55`](../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs:13)).
The WIT decoder preserves it ([`⚛️reactor/🦀️.rs:1290-1297,1344`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1290)).
Yet the reducer discards it:

```rust
Event::PatchAck { surface, revision, .. } => { ... }
```

at [`⚛️reactor/🔄️turn/🦀️.rs:215-219`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:215).
`PendingPatchAuthority::apply_published_ack` locates its owner only by `surface` and `revision`
([`📨️pending/🦀️.rs:110-116`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs:110));
the later tracker check uses the internally held generation, not the event receipt
([`🩹️patches/🦀️.rs:615-631`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:615)).

Concrete hostile trace: while a valid patch for `7:map` revision `R` is published, a caller can
send `PatchAck { receipt: <any syntactically valid foreign or stale receipt>, surface: "7:map",
revision: R }`. The pending owner is consumed into its ACK owner and the current patch generation
is marked acknowledged. No comparison establishes that the receipt was issued for that patch.
`PatchRejected` also discards its receipt/revision and calls `mark_rejected(surface)`; it needs the
same fence or a foreign event can cancel/restart a live surface.

### Actual output capacity

`UiTurnPatches` is a one-patch owner per `TurnResult`
([`🎠kernel/🦀️.rs:1700-1735`](../../../../../../🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1700)). It is **not** a
global “one outstanding ACK” rule: `PendingPatchAuthority` has a fixed multi-slot queue, marks a
slot emitted, and can emit later slots in later turns before earlier ACKs
([`📨️pending/🦀️.rs:11-93`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs:11)).
The repair must preserve that bounded existing concurrency rather than accidentally throttling all
surfaces to one global outstanding patch.

### Smallest bounded repair

Make the existing exact `PendingPatchSlot` own an `issued: Option<IssuedPatchAck>`, where the
private fixed-size record is exactly:

```rust
struct IssuedPatchAck {
    receipt: ActorUiPatchReceipt,
    surface: ui_contract::SurfaceId,
    revision: ui_contract::UiRevision,
}
```

Keep a private `PendingPatchTurnKey { sequence }` beside the one patch while it is transferred
from `turn_handback` into `UiTurnPatches`; `sequence` already identifies the bounded pending slot.
After a successful one-patch transfer, mint the receipt from the matching live lifecycle slot and
install the full triple only in that exact pending slot. This avoids ambiguous matching by a
duplicate surface/revision across pending slots.

On `PatchAck` and `PatchRejected`:

1. Require that `receipt.lifetime` names a currently Live native lifecycle cell, and that the
   exact pending slot’s stored triple equals `(receipt, surface, revision)`.
2. Run the existing pending-owner transition plus `PatchTracker` advance/rejection while the
   issued record remains retained.
3. Consume/revoke the issued record only if that transition succeeds. A tracker contention,
   insufficient acknowledgement backing, wrong tuple, duplicate, close, or foreign lifetime
   leaves the exact owner unchanged.

Close cleanup must revoke all issued records for its exact native key before a new lifetime can
reuse the numeric instance. The records are bounded by the existing pending-slot capacity; no new
unbounded ACK map or generic receipt registry is required.

Because the generic output lowerer is still fallible, issuing must happen only after the patch has
an actual successful outbound transfer, or must have a paired exact rollback that returns the
patch and erases the issuance on lower failure. Otherwise an unsent patch could permanently own an
ACK triple. This is the same output-transaction boundary recorded in
[`📓️terra-reactor-wit-lifecycle-output-transaction-audit.md`](📓️terra-reactor-wit-lifecycle-output-transaction-audit.md), applied to the patch owner.

## First executable laws

1. Real native `TestRuntimeApps`: issue a patch for instance A, then ACK its surface/revision
   with a valid receipt from instance B. Assert neither `published` owner nor tracker ACK state
   changes; the exact A tuple succeeds once.
2. Close/reopen the same numeric instance. A stale old-lifetime receipt paired with a current
   surface/revision must not move the current owner; the current receipt succeeds.
3. Produce two bounded pending slots, retain both issued tuples, and prove each ACK consumes only
   its own tuple. A duplicate after consumption and a cross-slot same-surface/revision attempt are
   inert.
4. Send a valid `PatchRejected` tuple for A and foreign/stale tuples for B; only the exact one may
   call `mark_rejected`.
5. Force the output lowerer to fail after a patch is prepared. Assert the patch is handed back and
   no issued tuple can be ACKed. Then retry and complete the same exact patch once.

No build or runtime test was run for this audit.
