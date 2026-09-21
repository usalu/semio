# Draw Native14 Bootstrap Deadline Audit

## Finding

The two Draw Native14 initializer failures have a source-level, deterministic owner-lifecycle cause. A deadline crossing after the outer bootstrap-job admission check is currently converted into a bootstrap fault even though no domain allocation failed.

The narrow correction is for `DrawingMutationArenaPoolBootstrap::step` to return `Ok(false)` when `cx.should_yield()` is true. That preserves the retained `Building` owner and lets the next admitted turn continue. It must not set `fault`, retire the owner, or report a generic bootstrap error.

## Exact Current Path

- [owned.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1033) returns `Err("drawing-store.mutation-arena-bootstrap-budget")` immediately when its inner yield check observes a deadline. Unlike every real allocation/domain failure, this branch does not call `self.fail` and leaves `fault == None`.
- [owned.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1267) maps any `Err` from that method to `DrawingMutationArenaProcessTransition::Retire`, without distinguishing the scheduler-boundary result from a domain fault.
- [owned.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1280) later chooses `bootstrap.fault.unwrap_or("drawing-store.mutation-arena-bootstrap-fault")`; the missing fault marker therefore becomes the exact generic refusal seen in Native14.
- The initializer maps the final job fault to its terminal refusal at [owned.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:5230). Both reported end-to-end tests therefore share this bootstrap gate rather than independently proving envelope or candidate corruption.

The outer job check at [owned.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs:1224) cannot make the inner check redundant: acquiring the process mutex and entering `step_locked` happens between them.

## Deterministic Fail-First Seam

`StepContext::new` accepts an injected `fn() -> Option<u64>` clock at [job.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:942). Its deadline law is `now >= deadline` at [job.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:975), and `should_yield` combines it with fuel exhaustion at [job.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:993).

The existing Draw fail-first test uses its thread-local incrementing clock in [retained-mutation-authority](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs:425) and calls the real `DrawingMutationArenaBootstrapJob::step_locked` in [the deadline law](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🦀️.rs:438). The neutral fixture specifies reads `[0, 1]` with deadline `1` in [mutation-admission](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🧫️fixtures/🧮️mutation-admission/🔣️.json:3).

That produces this exact state sequence on the current production code:

1. The explicit outer guard reads `0`, so admission proceeds.
2. The inner `PoolBootstrap::step` guard reads `1`, so it returns the budget `Err`.
3. The job changes `Building(bootstrap)` to `Retiring(bootstrap)`, consumes one unit, and returns `Pending`.
4. The retiring owner has no recorded fault; after close it terminalizes as `Fault("drawing-store.mutation-arena-bootstrap-fault")`.

The required sequence is `Building(owner, allocation unchanged, fault None)` followed by `Ready` after ordinary admitted turns. The test records exactly that as `["yielded", "resumed"]`, allocation delta zero. It directly proves the corrected scheduler behavior, without relying on timing luck or a text-only source check.

## Bounded Repair and Regression Law

Change only the deadline branch in `DrawingMutationArenaPoolBootstrap::step` from `Err(...budget)` to `Ok(false)`.

This is semantically correct because `Result::Err` in this API is reserved for a retained owner’s terminal domain failure: all allocation, capacity, overflow, false-terminal, and injected-fault branches call `self.fail` before returning `Err`. A scheduling yield is a nonterminal `false`, already represented by the function’s result type and already mapped to `DrawingMutationArenaProcessTransition::None`.

The native law must verify all of the following through the real job, as the current law is designed to do:

- inner deadline after the accepted outer guard returns `Pending { advanced_items: 1 }`;
- the state remains `Building`, with zero allocation delta and no fault;
- later unlimited-budget turns reach `Ready`;
- retirement does not surface either the generic fallback or a budget refusal;
- the existing allocation fault matrix still produces its original explicit domain error and cursorized close.

The neutral fixture and the current React scheduler oracle are the third-party counterpart for the same yielded-then-resumed contract. No runtime test was run in this audit; parent reported the Draw15 native fail-first test was being run when this report was written.

## Confidence

High. The two clock readings, the distinct error-to-retirement conversion, and the generic fallback are all direct source paths. This establishes the scheduler-boundary defect. It does not assert that every previous Draw14 failure was caused by this interleaving until the newly registered deterministic law and the two original Native14 rows are rerun after the narrow correction.

