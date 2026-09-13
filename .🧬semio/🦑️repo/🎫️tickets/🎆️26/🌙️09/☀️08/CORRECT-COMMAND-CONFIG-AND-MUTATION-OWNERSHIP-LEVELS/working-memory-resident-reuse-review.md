# Resident Ledger Reuse Review

The earlier working-memory audit omitted two existing resident authorities. Root inspected both after that audit. Its conclusion that no inspected authority currently governs general FEM/Pack physical backing still holds, but choosing a new primitive before considering the shared resident module would duplicate substantial ownership machinery.

## Shared Value Resident Module

`🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs` already owns a domain-neutral `ResidentLedgerRoot`. Its root is allocation-free, capacity is explicit, data and control partitions are disjoint, and access uses a nonblocking atomic guard. Native consumer, admission and record nodes reserve aggregate capacity before direct allocation with an exact Rust Layout. Registrations carry generations and type identities. Release stages distinguish destruction, physical deallocation, refund and descriptor removal; aliases and exact original-root identity participate in close. The source has separate Rust/TypeScript implementations and closed shared contracts.

This is a strong reuse candidate for composition admission and cleanup frontier infrastructure. Its contract explicitly separates step work, resident admission and physical heap certification. The TypeScript logical metadata prices are not native allocation bytes.

It is not the missing general process backing authority as currently wired. Every `ResidentLedgerRoot::new` creates an independent balance. `reserve_record` reserves a caller-declared domain envelope plus the exact record-node Layout; `allocated_bytes` increases only by that node's Layout. It does not allocate or measure arbitrary nested Vec/PagedList backing, reconcile allocator capacity overgrant, or transfer physical credit with such backing. `ResidentRecord::handoff_into` moves a shell out of its slot; intrinsic cleanup can later refund an empty record independently of the handed-off shell's nested allocations. A concrete domain composition must preserve any required physical custody across that handoff. Registration generations protect resident cells; no built-in OperationId/Generation authority was found.

The proper next design must first evaluate extending/injecting this existing authority at the real execution root and binding concrete physical backing to its admission/custody lifecycle. It must not treat envelope reservation as measured allocation or claim a process aggregate merely because two owners share one local root. No resident source changes or native tests were performed in this review.

## UI Resident Permit

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🦀️.rs` is a distinct, older process-static UI authority. It has 64 fixed slots, per-surface and aggregate item/byte limits, epoch-protected affine permits, output obligation splitting and delayed return processing. Its amounts are admitted UI limits. `try_reprice` changes the reservation from supplied limits, and `close_step`/Drop return permit credit without independently measuring or freeing arbitrary physical backing. Its private contract also includes UI-specific static memory and action storage.

Those fixed UI policies must remain at the UI boundary. They are not suitable as general FEM/Pack allocation admission, and moving this module wholesale would carry its UI constants and permit-return semantics into the wrong level.

## Required Follow-Up

Before adding another allocation authority, have a read-only agent review the shared value-resident root and exact concrete consumer adoption. Resolve execution-root injection, operation/root identity, requested/reserved/actual accounting, allocator failure/overgrant custody, transfer without early refund and exact physical release. Reuse the existing resident machinery where those semantics match; extend the correct shared level where they do not. The required process-wide FEM/Pack acceptance cases in the earlier audit remain open.
