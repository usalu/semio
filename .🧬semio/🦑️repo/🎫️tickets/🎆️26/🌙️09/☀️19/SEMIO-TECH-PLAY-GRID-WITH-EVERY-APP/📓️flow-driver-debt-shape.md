# 📓️ `debt`: the shape a retained driver needs when it out-grants its own caller

Written by the play fleet's `flow` topic (2026-09-22, ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP) at the
coordinator's request, for FL3 / the `End-to-end repo completion` peer. It describes the field the six framework
drivers are missing, the invariants it must satisfy, and the law that pins it.

## 1. The situation

Since the `next_close_byte_demand()` contract landed, a driver that holds a nested frontier does this:

```rust
let demand = self.retirement.next_close_byte_demand().map_err(str::to_owned)?;
Ok(match self.retirement.close_page(maximum_items, maximum_bytes.max(demand))? {
    SnapshotRetirementStep::Pending { released_items, released_bytes } =>
        SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) },
    step => step,
})
```

Six sites in the framework have exactly this shape:

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2746` (`close_page(1, maximum_bytes.max(demand))`, result dropped)
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:842`
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:952`
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:643`
5. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:774`
6. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs:66`

`.min(maximum_bytes)` **clamps and discards**. The bytes above the page were physically freed and are then never
reported to anyone. The caller's ledger is therefore not a conservation law any more: it under-reports by exactly
the excess, silently, and only when a nested cursor out-demands the page — i.e. only on the large documents, which
is where a byte ledger is worth having.

Two things follow that are worse than the arithmetic:

- **A `Complete` can arrive with bytes still unaccounted.** `terminal_is_empty()` forwards the frontier's answer,
  so a driver reports terminal emptiness on the same turn it discarded 30 KiB of release. A caller that fails
  closed on "released < payload" cannot distinguish that from a leak.
- **The clamp now discards PAYLOAD, not allocation.** Before FL3's correction the excess was mostly
  `capacity * size_of::<T>()` and discarding it was arguably right. Now `release_root_backing` charges
  `owner_backing_payload` and draws it down through `root_backing_credit`, so everything above the page is real
  payload. The same applies to `FlowCopyAllocationBudget::charge_release`
  (`🧵️retained/📑️copy/🦀️.rs:358`): its `released_bytes.saturating_sub(page_bytes)` now moves PAYLOAD into
  `returned_bytes`, and its law `assert!(cursor.allocation().returned_bytes() > charged)` passes for the wrong
  reason. `charge_release` needs the same treatment or its own restatement.

## 2. The shape

One `usize` per driver that owns a nested frontier, plus one shared helper. Reference implementation, in
production in `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/♻️retirement/🦀️.rs` since
2026-09-22 and driving all four flow-plugin retirements:

```rust
/// 🎟️ Bytes a nested frontier already freed above the caller's page, still owed to the caller's
/// accounting. Amortizes the ACCOUNTING; it never delays the free.
debt: usize,

pub(crate) fn close_frontier_page(
    domain: &mut FlowRetirement,
    debt: &mut usize,
    maximum_bytes: usize,
) -> Result<SnapshotRetirementStep, String> {
    if *debt > 0 {
        let paid = (*debt).min(maximum_bytes);
        *debt -= paid;
        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: paid });
    }
    let demand = domain.next_close_byte_demand().map_err(str::to_owned)?;
    let step = domain.close_page(1, maximum_bytes.max(demand))?;
    let SnapshotRetirementStep::Pending { released_items, released_bytes } = step else {
        return Ok(step);
    };
    *debt = released_bytes.saturating_sub(maximum_bytes);
    Ok(SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) })
}
```

`released_items: 0` on an instalment turn is deliberate: no additional owner was released, only accounting was
settled. Every `released_items <= 1` bound in the tree still holds.

## 3. Invariants

1. **Grant.** The nested frontier is granted `maximum_bytes.max(demand)`; the caller is charged at most
   `maximum_bytes` in any one turn. (Unchanged — this part the six drivers already do.)
2. **Conservation.** Over a whole close, `Σ reported released_bytes == Σ bytes the nested frontier freed`. This
   is the invariant `.min()` alone breaks and the only reason `debt` exists.
3. **Exclusivity.** While `debt > 0` the driver does **not** advance the frontier, take a root, move an owner or
   change phase. Debt is therefore bounded by one turn's overshoot (`freed_chunk − page`) and never accumulates
   across owners; one field suffices, no queue.
4. **Liveness.** An instalment turn reports `released_bytes = min(debt, maximum_bytes) >= 1` whenever
   `maximum_bytes >= 1`, so it is never a `Pending { 0, 0 }`. It cannot trip
   `FLOW_RETIREMENT_CLOSE_STALL_BOUND` / `FLOW_COPY_CLOSE_STALL_BOUND`, and it cannot be mistaken for
   back-pressure by a driver that counts idle turns.
5. **Never `Blocked`, never `Complete`, while owing.** An instalment turn answers `Pending`. Answering
   `Complete` while `debt > 0` is the bug this whole field exists to prevent.
6. **Terminal rule.** `terminal_is_empty()` **must** be `frontier.terminal_is_empty() && debt == 0`, and every
   `Drop` assertion must use that same conjunction. Forwarding the frontier's answer alone lets a driver claim
   terminal emptiness with an unpaid ledger — and lets a cold drop assert successfully over it.
7. **Demand rule.** `next_close_byte_demand()` returns `1` while `debt > 0` (an instalment needs one byte), and
   forwards the frontier's demand otherwise. Forwarding a large demand while merely owing makes every driver
   above over-grant for nothing.
8. **Reset rule.** `debt` is decremented only by payment. It is *not* reset by `push`, re-seeding or a new root —
   unlike `FlowRetirement::root_backing_credit`, which tracks the CURRENT root and correctly resets on `push`.
   A driver that resets `debt` on a phase change silently discards again.

## 4. The law that pins it

A clamp-and-discard driver passes every existing test; only a **1-byte page over a known payload** catches it.

```
Given a frontier holding a payload of exactly P bytes and a driver paged at 1 byte:
  * every turn reports released_items <= 1 and released_bytes <= 1
  * no turn reports Pending { released_items: 0, released_bytes: 0 }
  * terminal_is_empty() is false until the final byte has been REPORTED, not merely freed
  * Σ released_bytes over the close == P    ← fails on a clamping driver, by (P − turns)
  * the source Arc is gone and the driver's Drop assertion holds
```

The flow plugin already owns four instances of this law, and they are the ones that went red under the
uncorrected frontier (§7.6 of `📓️flow.md`):
`retained::artifact::snapshot::tests::child_typed_handoff_preserves_mismatched_owner_then_retires_exact_scene`
(P = 16 388 at a 1-byte page) is the cleanest single case; the presence law
(`flow_presence_store_owners_preserve_readers_and_retire_neutral_byte_grants`, P = 16 388) drives the same shape
through a real `PresenceStore` disposer. Add one of that shape next to each of the six drivers — the vcs and
mutation-frontier ones have no such law today, which is why the clamp went unnoticed.

## 5. Scope note

This is an accounting contract only. It changes no ownership, no free order and no grant bound, and it is a
strict no-op for any driver whose nested frontier never overshoots the page (`debt` stays 0 and every branch
behaves exactly as today) — so it can land without touching the drivers' callers.
