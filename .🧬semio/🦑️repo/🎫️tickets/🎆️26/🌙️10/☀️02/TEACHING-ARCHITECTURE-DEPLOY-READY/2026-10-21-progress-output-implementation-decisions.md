# Pending Progress Coalescing Implementation Decisions

Updated 2026-10-20T23:27:29.215Z. These decisions refine the independent design review. They authorize preparation only while TTY75119is live. Do not edit the current Source cohort or execute a second build. Implementation will follow the actual current native outcome and fresh Source reread.

Use the existing progress schema, corpus, actual-writer AST/SQLite tests and Original settlement tests. Restrict new pacing to the four measured dependency activity families, with eleven schema-fixed slots. Do not add a broad bare-running fallback or change producers, callbacks, guards, yields, CLI resource grants or guest output limits.

Use one shared5000ms presentation frame for those slots. This slower presentation cadence leaves all asynchronous progress publication and cancellation checks intact. Within a stable frame, eleven paired rows per5seconds gives190080 newline records over12hours before initial/final/boundary terms; this is a design bound for this category, not a universal guest acceptance proof. Unknown diagnostics, genuine resets, other typed categories and product output still require actual bounded whole-workload evidence.

The first valid measured observation opens the shared window and emits once. Other first-seen slots receive no extra immediate entitlement. On a due observation, replace its slot with the current original row before draining the latest rows in last-observation order. Unit/family alternation never resets or flushes the shared frame. Keep only the latest original raw/human pair in each fixed slot; do not regenerate its message at flush time.

Keep ordering arithmetic bounded by rank-compressing the at-most-eleven live observation positions. A fixed slot-index tie break may be defined explicitly, but an indefinitely increasing Number is not sufficient. No timer, background task, unbounded owner map or queue is needed.

An actual owner/command/Script-stage boundary drains all older pending evidence in order, then resets the corresponding frame state. Immediate unknown/malformed/old-family rows preserve older ordered evidence but do not reset the new frame clock. Explicitly test interleaving with existing source-provider/cache/tooling categories so the new pacing does not alter their established rules.

Use only producer-proven counter reset semantics. Ownership unit kinds share one completed-work progression; scan and ledger are independent; replay elapsed zero and repeated preservation counts are not resets. A genuine computation reset drains only the affected family, emits its new original row and advances the shared frame clock, preserving other slots. Repeated genuine resets remain an explicit immediate-evidence exception.

Backward or nonfinite clocks drain older evidence, emit the current original row and enter an explicit declared scheduling state. They cannot reject or suppress the underlying work.

For sink failure, dequeue each attempted row immediately before its first write; never replay an ambiguously written raw/human pair. Keep untouched later rows pending, propagate the sink failure and allow a later flush to attempt those untouched rows. A raw write followed by a throwing human write is not atomic; document the at-most-once attempt semantics. A successful final flush is idempotent. Existing Original settlement must preserve the exact primary failure, including falsy thrown values, when a final flush also fails.

Required vectors include due-slot replacement, dense eleven-slot sweep, alternating ownership unit kinds, alternating families, independent counters, repeated counts/elapsed-zero replay, family reset with other slots pending, immediate diagnostics without a pacing reset, owner/command/stage boundaries, clock reversal/nonfinite clock, multi-slot partial sink failure, repeated flush and Original cancellation/falsy-error settlement. Preserve existing test titles and add an exact focused selection only after actual Source review and new seal.

Historical replay can quantify a declared timing scenario and retained record ordering. Since the old log lacks a universal callback wall clock, do not call a synthetic schedule a measured real-time replay. Only actual Linux whole-workload terminals/logs can prove the fixed output budget.


## Interface Readback Accepted

Root accepts the two concrete constraints in2026-10-21-progress-output-interface-addendum.md. Existing provider/cache/tooling callbacks keep their existing schedulers: only rows those schedulers actually emit can form immediate barriers for the new slots. Do not flush new slots on every old-family callback. Ordering is latest-observation order within each drain; an affected-family-only reset intentionally permits unrelated older latest rows to remain pending, so no globally chronological callback-log claim is made. The exact five existing permanent files and two focused test titles are listed in that addendum. Current Source remains unchanged pending the actual75119outcome.
