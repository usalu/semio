# Issued Patch Receipt Transaction Audit

## Scope

Read-only review of the current reactor pending-patch receipt transaction in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧪️tests/🩹️receipt.rs`

No build was run in this audit. The root source oracle was reported green; native qualification was still compiling when reviewed.

## Current transaction boundary

The boundary is coherent for the output conversion that currently exists.

1. `poll_kernel_output` builds a retained `TurnResult`.
2. It validates the one-patch/one-receipt relation, stages the exact receipt in the pending slot, and calls `prepare(&TurnResult)` while the patch remains borrowed by `UiTurnPatches`.
3. WIT preparation reads the result and constructs independent WIT values; it does not consume the patch owner.
4. The lifecycle real-clock verdict runs before `commit_emission`.
5. On an ordinary preparation failure or a lifecycle finish/clock failure, `try_transfer_one` returns the exact patch to the reserved pending turn. `hand_back_turn` clears only an uncommitted issuance.
6. Only the successful, infallible publish path marks the issued receipt committed.

This order avoids an ACK becoming valid for an output that failed WIT preparation or failed its later lifecycle clock verdict. A reissued patch obtains a later receipt sequence; an ACK for the provisional sequence remains inert.

The pending authority now also drains acknowledged slots in its normal `close_step`, before close-lifecycle reservations. That means successful ACK/rejection does not consume all fixed pending slots. My earlier intermediate read missed this current branch; it is not an outstanding capacity P0.

## P1: Rejection loses the reconcile generation after exact receipt matching

`PendingPatchAuthority::apply_issued_rejection` first matches the complete issued tuple: lifetime, patch sequence, surface, and revision. For a reconcile-owned slot it then calls the closure in `turn` that invokes `PATCHES.mark_rejected(&surface)`.

`PatchTracker::mark_rejected` selects only by surface. In contrast, the ACK route consumes `SurfaceReconcilePublishedPatch` into `SurfaceReconcilePublishedAck`, then calls `mark_published_ack`, which checks the retained reconcile generation. The rejected route therefore lacks the publication generation that its own `slot.published` still owns.

This is not a forged receipt bypass: the outer receipt selection is exact. It is a same-surface ordering hole if two retained publication generations can coexist. A valid rejection for receipt B can reset whichever current tracker slot is selected by surface rather than B's generation.

Minimal repair:

- Have the rejection branch derive the generation from the exact matched `SurfaceReconcilePublishedPatch` before invoking the tracker.
- Add a generation-qualified tracker operation such as `mark_rejected_exact(surface, generation)`; it must leave both owners untouched on a surface/generation mismatch.
- Keep the issued slot pending when the tracker cannot accept that exact generation. Do not convert a mismatch into an unscoped reset.

Required native law: retain two reconcile publications for one surface with distinct generations, issue two different actor receipts, reject B, and assert only B's tracker generation becomes terminal/reset. A's publication and receipt must still accept its own ACK or rejection.

## Intentional unmatched receipt behavior, and its required proof

The turn currently ignores a `false` result from `apply_issued_ack` or `apply_issued_rejection`. For a foreign, duplicate, uncommitted, or stale receipt this is the safe behavior: it cannot release or reset a pending slot merely because a different receipt named the same surface.

The cost is that a live unmatched feedback frame leaves its emitted slot awaiting either the exact later ACK/rejection or the exact lifecycle close. That is a protocol liveness condition, not a reason to auto-free/reset it. A transport diagnostic may be useful, but it must not become an unscoped reset mechanism.

The fixture covers the individual acceptance predicates, but needs a production ordering law:

1. Issue parallel slots A and B.
2. Send a foreign or duplicate ACK/rejection for A; assert neither slot is retired and B is unchanged.
3. Send the exact feedback for A; assert only A enters bounded ordinary slot retirement.
4. Continue normal `PendingPatchAuthority::close_step` until A is absent and capacity is restored; B remains usable.

The planned 65 sequential successful-feedback law should drive the real normal `close_step`, not the test-only direct `close_instance_step` helper. It proves the permanent-queue concern is closed without pretending a malformed feedback frame can authorize release.

## Output and late-clock retention laws still missing

The existing uncommitted handback test calls the pending authority directly. It validates its local mechanics, but does not exercise the production `prepare -> finish_turn -> hand_back` path.

Add these native laws against the real `poll_kernel_output`:

1. **Preparation fault:** make the borrowed WIT preparation fail after staging. Assert exact patch bytes and pending sequence are returned, provisional receipt is not accepted, and a retry emits the same patch under a later receipt.
2. **Late clock with another live patch:** run live actor A and lifecycle actor B. Emit A's patch while B's lifecycle ACK or close transition is focused; inject the real-clock deadline failure at B's `finish_turn`. Assert A's patch returns to its exact pending slot, its staged receipt is uncommitted and inert, and B's receipt/owner remains mounted. Retry the same B event, then emit/ACK A under its fresh receipt.
3. **Lifecycle retry matrix:** force the first real-clock verdict to fail for a captured ACK, accepted-close ACK, and retired ACK. For each, repeat the identical event and assert `admit`, `stage_ack`, close admission, and terminal release remain idempotent; phase changes only once after the successful verdict. The current lifecycle APIs support this: matching opens return non-fresh, equal staged ACKs are allowed, and exact close admission is idempotent.

The WIT converter presently borrows `TurnResult`, copies patch data into WIT values, and can fail before lifecycle finish. That is compatible with the handback design. Effects and presence have separate retry/ownership frontiers and are not claimed transactionally safe by this audit.

## Obsolete Exact Feedback Must Retire Its Own Owner, Not the Newer Surface

There is a retained-owner liveness hole once the exact issued patch is older than the tracker’s current generation.

`PendingPatchAuthority::apply_issued_ack` first locates the full issued tuple, then consumes its `SurfaceReconcilePublishedPatch` into a `SurfaceReconcilePublishedAck` before it asks the tracker to advance ([`📨️pending/🦀️.rs:154-163`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs:154)). `PatchTracker::mark_published_ack` returns `false` unless a live surface slot has the same generation ([`🩹️patches/🦀️.rs:616-630`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:616)). The explicit ABA test advances the same surface slot’s generation, observes that false result, and asserts that the original acknowledged owner is still retained ([`🩹️patches/🦀️.rs:1559-1583`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:1559)). Because `PendingPatchSlot.acknowledged` remains false, that owner cannot enter its normal bounded `close_step` drain.

The separately exercised reopen trace confirms that an old terminal and a newer same-surface generation can coexist, and that the old owner may not mutate the reopened slot ([`🩹️patches/🦀️.rs:1720-1734`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:1720)). Today the only non-test `mark_rejected` call is behind the exact issued feedback route, so this audit did **not** find a currently exercised production path that independently supersedes a live pending receipt. The structural proof is nevertheless real, and the planned generation-qualified rejection path makes the disposition explicit rather than relying on that incidental call topology.

### Required disposition

Replace the boolean tracker callback with one crate-private, generation-qualified disposition shared by ACK and rejection. It must inspect the exact reconcile owner’s `(surface, generation, revision)`; never infer it from the actor receipt alone.

| Tracker evidence for the matched pending owner | ACK/rejection action | Pending slot action |
| --- | --- | --- |
| Same surface, same generation, and current revision | Apply the exact ACK or exact rejection. | Clear `issued`, mark acknowledged, then let ordinary `close_step` retire the owner. |
| Same surface with a strictly newer generation | Do not advance, reset, cancel, or otherwise touch that newer slot. | Treat the feedback as obsolete-but-valid for the old owner: clear `issued`, mark it acknowledged, then retire only that old owner. |
| No current surface, but an exact `(surface, generation)` terminal witness remains | Do not revive or mutate the terminal/current state. | Same obsolete retirement. |
| Same surface with a lower generation, same generation with a non-current revision, or no exact current/terminal witness | Do not mutate the tracker. | Return false and retain the issued owner for a later exact response or lifecycle close. |

The last row is important: generation is monotonic, so a lower tracker generation cannot be a legitimate response from this tracker; absent state has no proof that the pending owner became obsolete. Neither is authority to free it. Duplicate feedback is already excluded by `!slot.acknowledged` and remains inert.

The smallest API is an enum such as `PublishedFeedbackDisposition { Applied, Obsolete, Retain }` exposed only inside the reactor patch/pending seam. For ACK, retain the present order: first `acknowledge_into` moves the unforgeable published owner only after bounded admission succeeds; then ask the tracker for the disposition using the resulting `SurfaceReconcilePublishedAck`. `Retain` leaves that ACK owner in place. For rejection, derive the generation from the still-retained `slot.published` and call the same tracker decision before clearing the issued tuple. This keeps a tracker error, capacity refusal, or future/absent mismatch owner-preserving.

`PatchTracker::mark_rejected(&surface)` cannot be reused as that current action: it selects solely by surface and resets that slot ([`🩹️patches/🦀️.rs:575-613`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:575)). Its replacement must first require `slot.surface == surface && slot.generation == issued_generation`; only then may it create a terminal/reset for the current generation. A terminal-witness branch needs exact surface identity too. `TerminalSlot` currently stores a key, instance, and terminal authority but not a surface, so add the source `SurfaceId` when each terminal is constructed rather than guessing from numeric instance or a global generation.

### Required native laws

1. Produce reconcile patch A, retain its committed actor receipt, then advance the same tracker surface to a later generation using the real rejection/reopen owner path. The exact late ACK for A must retire A through normal `PendingPatchAuthority::close_step`; the later slot’s generation, revision, and cancel state stay unchanged.
2. Repeat with exact late rejection for A. It must retire A but must not invoke the new generation’s rejection/reset path.
3. With a current generation lower than the issued one, and with no exact current/terminal witness, an otherwise exact actor receipt returns inert false and preserves the issued owner byte-for-byte. This prevents a fabricated/future tracker claim from freeing it.
4. Keep the existing foreign-then-exact law: an unmatched receipt changes neither owner; the later exact receipt takes the relevant `Applied` or `Obsolete` branch once.

These laws need one true `PendingPatchAuthority` reconcile owner plus `PatchTracker`, not two external patch test doubles: only that combination demonstrates that an obsolete response closes the old `SurfaceReconcilePublishedPatch`/ACK authority while the newer render owner survives.

## Acceptance status

The staged/committed receipt ordering and ordinary slot retirement are source-coherent. Do not claim the full issued-patch path qualified until the generation-qualified rejection plus obsolete-feedback repair and the production preparation/late-clock laws above pass.
