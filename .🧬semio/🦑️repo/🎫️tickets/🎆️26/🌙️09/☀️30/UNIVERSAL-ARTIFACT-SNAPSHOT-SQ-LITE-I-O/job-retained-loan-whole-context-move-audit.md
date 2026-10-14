# Job Retained Loan Whole Context Move Audit

Read-only API/source audit, 2026-10-13. No test dispatch or runtime claim. Owner `🧰️framework/🔨️modules/🧵️job/🦀️.rs`.

Concrete authority defect: `OriginalRetainedLoan:949–950` retains a raw pointer to the original context slot's retained grant and unconditionally writes the old grant on Drop. `with_retained_loan:977–983` passes unrestricted `&mut Self` to public FnOnce. A safe callback can `std::mem::swap(cx, &mut second_context)` or `std::mem::replace(cx, replacement)`; original narrowed context moves out with its original progress/preview/ledger references, while the guard writes the original grant into a different context now occupying the slot. A replacement admitted under a zero/smaller grant therefore receives the first caller's larger ceiling and can spend it against its different recipient after return. The same unconditional restoration occurs during unwind after a swap.

Private fields do not prevent whole-value swap. StepContext public `new:954–956` creates another context with independently borrowed progress and preview references. Those borrows can share the same lifetime; the context has no PhantomPinned or restrictive receiver. There is no Drop/move check on StepContext. Even avoiding new authority construction, two existing valid contexts of the same lifetime can be swapped. This does not require unsafe caller code, cloning mutable references, changing selectors or fabricating a grant.

Illustrative safe callback shape (not compiled here):

```rust
original.with_retained_loan(requested, |cx| {
    std::mem::swap(cx, &mut second_context);
    Ok::<(), ValueError>(())
})?;
// original slot now contains second_context with original caller's restored ceiling.
```

API-compatible actual primitive guard: keep the original recipient identity (address of underlying RetainedCloneProgress), original preview identity and operation/generation/ledger identity; restoration must be conditional on the same original context custody still occupying the slot. Guard must verify before any write on normal return and unwind, and callback return must report invariant refusal if custody changed. Do not grant the replacement during error reporting or restoring. Nested loans must restore only their own verified ceiling. A recipient reference alone is a uniquely exclusive identity while both contexts exist in safe code, but whole context identity should include the existing ledger and cancellation/preview custody to match the claimed same-context law. This stops authority transfer but cannot stop safe movement itself; the narrowed moved-out context remains under its narrow grant.

Stronger type-level prevention requires a non-owning loan receiver that exposes the needed operations without DerefMut/as_mut to StepContext, or a genuinely pinned !Unpin StepContext with pinned job close receiver throughout. Passing `Pin<&mut StepContext>` while still handing ordinary `&mut StepContext` to existing close callbacks defeats pinning; PhantomPinned alone also does not prevent mem::swap through existing &mutSelf. Thus preserving real Job close `&mut StepContext` cannot provide a static immovability guarantee by just adding Pin at the loan method. The identity/conditional-restore fence is the direct shape preserving that actual close API; precise refusal and unwind laws are needed before qualification.

Existing Source6/0 and Native3 unrun frontier reported by root does not cover this whole-value movement law. Findings delivered to root and HighCodec agent; no producer edits by this audit.
