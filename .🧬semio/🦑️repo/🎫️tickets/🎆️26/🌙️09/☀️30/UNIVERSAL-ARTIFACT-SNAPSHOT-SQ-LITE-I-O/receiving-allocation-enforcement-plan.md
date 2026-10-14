# Receiving Allocation Enforcement Plan

Read-only design audit, 2026-10-13. No builds or source changes.

## Actual Gap

`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs::admit_allocation_bytes` and `allocation_stage` currently advance SQL allocation and forwarded counters when `allocation` is None. Final native/SQL settlement checks only aggregate totals. Unrelated original native header/close births can cover an unforwarded cold allocation numerically. `forwarded_owned_bytes` is not proof of native forwarding because that counter advances even without a bound port.

## Minimum Closure-Lent Contract

Add a private admission requirement to control state: standalone versus original-receiving. Expose one closure-lent method, for example `with_original_receiving`, that constructs a child `SqliteSnapshotControl` with required mode, the same borrowed ledger/callback/limits/recipient and inherited allocation port. It must never clear an already-required ancestor mode. Do not provide a public setter or a public reset/disable method.

At the beginning of every **nonzero** `admit_allocation_bytes`, reject required mode with no bound allocation port using a static literal cause, before advancing either SQL ledger or forwarded counter. Apply the identical pre-admission condition inside `allocation_stage`'s reservation callback, before invoking the producer's native allocation and before updating counters. Zero-byte admissions remain valid; a stage that performs no actual allocation need not be rejected merely because its port is absent.

`Port::receipt` / `admit_native_allocation_bytes` remain allowed: their actual producer already used original native allocation. `allocation_stage_native` remains a settlement path for actual native-paid stages. This distinction enforces nonforwarded SQL reservations, not a global ban on receipt reconciliation. It does not prove that every caller of a public native-settlement method genuinely paid; preserve original receiving settlement and explicit callsite audits. Do not rewrite cold admissions to native-settlement methods to bypass the guard.

## Exact State Propagation

Current explicit child constructors are `with_retirement_child`, `with_workspace_recipient`, and `with_original_allocation` (same file lines 172–195). All must inherit the required mode. `with_original_allocation` installs the actual port and uses local forwarded=0 for the start-relative native loan; required mode remains active. Workspace/retirement children inherit an existing port or remain required/unbound and reject nonzero admissions. Normal root `new` remains standalone. `new_forwarded` binds the real port but does not grant permission to disable an inherited requirement.

Retirement owner admission uses `admit_allocation_bytes` for its wrapper, so it automatically receives the check. Its existing refusal branch settles the reserved recipient slot to None; preserve that exact cleanup.

Keep admission mode as child-local immutable scope state. On closure Result error/success, propagate actual ledger work plus existing forwarded/limit changes as the current child constructors already do, then drop the loan. Parent mode is unchanged without restoration switches. On unwind, no requirement state was mutated on parent. Closure output must not borrow the child control beyond the loan.

`TypedFileSettlement::new` currently stores RefCell of the supplied control. Wrap each actual whole receiving entry in the new loan before constructing settlement; keep the loan alive through binder, close/publication callbacks and residual refusal settlement, not just until body return. Current consumers include direct export/import, both SnapshotHop branches and the SpaceHistory directional validator. Nested scopes must inherit required mode and share the actual ledger. TXT has its own receiving settlement and should use the same admission requirement at its receiving entry rather than a local reset.

## Expected Blockers And Correct Repair

Shared workspace P/R defaults already execute cold authored functions inside `with_encoding_receiver -> with_original_allocation`, so their admitted allocations are bound and should pass this guard. That does not solve arbitrary cold scratch custody.

Shared workspace directional validation defaults only checkpoint then invoke cold subset validation. Any validator that reconstructs a candidate, allocates diagnostics/indexes or otherwise uses SQL admissions will now refuse **before birth**, even if native slack formerly hid it. This is the intended actual frontier, not a reason to relax admission. SpaceHistory's newly authored borrowed paid validator avoids the candidate and uses Port receipts. Other domains need genuine borrowed/retained validation or an original allocation loan plus retained scratch custody; do not silently add fresh controllers.

Existing standalone controlled schema/physical/domain tests remain standalone unless entering a genuine original receiving operation. Requirement should not be applied indiscriminately to every `SqliteSnapshotControl::new`.

## Schema-First Laws

Add neutral closed fixture under the control's existing tests: initial SQL/native receipts; independent authored caller grant; required nested scopes; nonzero/zero request; actual port presence; expected exact cause; expected unchanged or admitted counters; success/error loan return.

Essential Native law deliberately gives original native unrelated birth slack greater than a cold SQL reservation. Inside required receiving, unbound reservation must still fail before allocator birth and leave SQL/native deltas unchanged. Cover direct admission and stage callback, both child kinds, original-allocation child acceptance, nested required loan, zero allocations, error return restoring parent's usable standalone policy, and same original cancellation recipient close. A negative producer candidate validator should expose the exact literal refusal; a Port-paid producer must pass and preserve aggregate settlement equality. Independent Source uses canonical grant schema and a small SQLite ledger model for immutable-before-refusal/admitted-after-port behavior, with source guards proving all child constructors inherit the requirement. No runtime qualification claimed.
