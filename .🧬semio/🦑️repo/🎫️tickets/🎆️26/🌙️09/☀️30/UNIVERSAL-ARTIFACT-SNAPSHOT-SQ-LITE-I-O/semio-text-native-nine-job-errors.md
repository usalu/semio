# Semio Text Native Nine Job Error Reconciliation

Read-only Native90438 log lines59366–59497 and concurrent source, 2026-10-13. No edits/replay; High currently mutating Job spans, so compiler locations describe captured invocation rather than final current source.

Actual floor is nine errors before selected Semio Text runtime laws. Prior statement that OriginalPreviewSession grant close was a valid isolated low-level scope is historical: its trait still had grant/plain result while its actual BatchJobSession implementation now delegates to cx/Result, which the fresh compiler rejects. The erasure boundary must migrate too; merely preserving old trait spelling does not make its concrete owning calls valid.

| Captured location | Actual mismatch | Original authority path required |
|---|---|---|
| Job root2966 | worker core invokes InteractiveJob.close_step(grant), now expects cx | Thread supplied original caller StepContext into worker closure core and call actual job with it |
| Job root2967/2976 | invokes admit on Result and matches removed Interactive Refused | Match canonical Result first, retain whole OriginalTurnFailure; on Ok validate step receipt/terminal witness; no truncation to kind |
| Job root3285 versus2385/2529 | WorkerSession bare-grant/plain result while admitted session wrappers already pass cx and expect Result | WorkerSession original close must accept same cx and return canonical Result unchanged through wrappers |
| worker/erasure38 | OriginalPreviewSession impl passes grant and returns plain Worker step into BatchJobSession cx/Result | Migrate original erased trait, typed impl, erased impl and forwarding tests consistently to supplied cx/Result; no compatibility overload |

Existing session/job context is original operation/generation/retained recipient/cancel/fuel authority. Erasure should borrow it, not create an internal StepContext/clock/deadline or reset receipt. Lower pure retirement primitives may still accept independent child grant obtained from that same context; the job-owning boundary cannot remain grant-only when executing semantic child closure.

Canonical child InteractiveJob close already accounts actual receipt through cx according to its implementation. Worker/session wrapper must not debit the same child progress again. Receipt admit checks actual step and terminal witness without replacing grants; any resulting ValueError should become canonical failure without treating an already-recorded receipt as new child effects. Whole primary/receipt_refusal crosses typed/erased layers intact. Pending/Blocked retain original authority at same SESSION_CLOSE state; error also returns/retains exact authority before propagation.

Current source read encountered partially edited worker fragments during High's active transformation; these are not classified as final source defects. Owning compile must run only after coherent author completion. Semio Text Source8046 actual3/79 green is separate from this native compiler floor, and Native90438 has zero selected text runtime laws in this evidence.
