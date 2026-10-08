# Erased Read Lease Custody and Portable Law Audit

Read-only source inspection on October 8. No native execution performed; session 69778 remains a separate runtime receipt. No production edits.

## Actual defining custody

`🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:26–31` refuses zero items, zero depth, or insufficient exact concrete `size_of::<ReadLease<T>>()` capacity before allocating, returning the original typed lease. Successful admission boxes that same lease and records one item and exact frame capacity. No root reconstruction, raw pointer cast, or replacement registry is introduced. Lines 12–16 and 33–38 delegate typed root, lease identity, commit authority and source admission to the retained concrete lease.

The original typed custody is `🔗️read/🦀️.rs:147`: original root Arc, original registry Arc, optional controlled registry close cursor, original slot/generation identity. Source admission at 153–155 clones original aliases into RegistryAuthority; refusal drops only those new aliases while the lease remains live. Its source pins therefore follow the original registry and root. This is a source argument, not an executed erased-source witness.

## Exact closure and terminal boundaries

Erased lines 39–42 delegate actual copy/capacity/release/depth observers. Item demand is the close contract's one admitted item. An emptied inner lease exposes only concrete frame release bytes and depth one. Lines 49–53 release that frame in a separately funded step returning Progress, while a subsequent absent-frame call returns Complete. Active child closure at 55–57 checks the full progress grant and rejects Complete while the child still retains custody through `🧬️retained-clone/🦀️.rs:202–207`; the outer retained frame always keeps the outer result Progress.

No sentinel maximum or swallowed Result occurs in the erased observer interface. The RetirementCursor bridge at 69–70 maps errors to None because that interface still uses Option; its depth/copy observers preserve Result. The aggregate `demands(0)` used by individual observers also queries the other currencies. No concrete refusal inconsistency was established because the retained typed lease supplies all four observers. Do not claim this generic aggregate pattern independently validates each currency under arbitrary observer failures.

The only unsafe operation, line 80, drops ManuallyDrop after the option is already empty. Dropping live custody asserts; unwinding deliberately retains its ManuallyDrop contents. This avoids uncontrolled owner release but does not provide a recoverable cancellation API. Normal cancellation must drain explicitly. Typed lease Drop has the same boundary at `🔗️read/🦀️.rs:174`.

## Original neutral fixture and test scope

`🔗️read/🧬️schema/🔣️.json` is a closed Draft 7 object requiring the original sparse/wrap/starvation cases, work budgets, text corpus, alias counts and erased cancellation points. The actual fixture retains capacity 1024, workBytes [1,3,64], aliases [1,3], erasedCancelAt [0,1,5].

`🔗️read/🧪️tests/🦀️.rs:73–84` exercises nine erased cancellation combinations. It proves one-short capacity admission returns the original String pointer without heap events; successful boxing reports physical frame birth; typed lookup preserves pointer and rejects a wrong type; serde_json independently interprets the scalar text; every close step reports exact physical births/releases within its five-currency grant; full read plus authority drainage conserves original allocation plus all new births. The shared drain at 6–15 probes zero items, one-short capacity and release, and forbids stalled exact funded closure. This is currently mounted test intent, pending genuine owning receipt.

## Narrow original-test additions worth making

1. Publish a literal generation/revision before issuing the root, record original ReadLeaseId, and assert the erased lease preserves id and accepts exactly that authority while rejecting changed generation/revision. Current erased test never publishes or checks either.
2. Through ErasedReadLease, assert wrong-type source request refuses without heap changes; one-short source admission preserves pointer/id/authority; exact source admission retains the original borrowed pointer. Close erased lease and authority while keeping the source live, then prove their release remains short of the total until that source is explicitly closed. Existing typed source test at 35–49 does not execute the erased forwarding methods.
3. Extend the same admission test with zero-item and zero-depth grants, returning and reusing the exact original typed lease. Capacity refusal alone does not execute these two branches.
4. At inner-terminal/outer-frame-retained state, assert release one-short and depth-zero cannot free the frame, exact release emits Progress and empties it, and the next call is Complete(Default). Current aggregate conservation can pass without explicitly checking this two-stage terminal protocol.
5. A false-Complete child law belongs beside the canonical `admit_retained_clone_close` helper or a module-private injected LeaseView test, not a new production compatibility constructor. Check retained false completion rejection and overgrant progress rejection. Current real String path does not manufacture false child completion.

No concrete erased provider defect was proved by this inspection. Missing above witnesses are test scope gaps, not evidence that original custody is lost.
