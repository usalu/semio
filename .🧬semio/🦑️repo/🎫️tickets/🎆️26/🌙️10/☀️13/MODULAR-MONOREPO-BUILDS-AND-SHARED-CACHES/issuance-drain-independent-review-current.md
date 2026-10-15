# Issuance Drain Independent Review

Read-only Source review, 2026-10-15. No tests, Originals or production edits. Storage reports prototype66521 GREEN; that result is supplied evidence, not independently rerun here. Release and strict counterparts remain pending. Source paths below are relative to `🧰️framework/🔨️modules/🏃️process/`.

## Original lifetime gap is addressed in Source

`🧭️routing/📥️invocation/🏃️process/📥️issuance/🟦️.ts` 22–27 constructs a deferred task, registers it in the private parent's pending set synchronously, and installs rejection observation before its callback can execute. Loan entry 93–105 registers its complete nested settlement with its parent/birth. `withIssuedResourceOperationV1` 122–126 performs private identity/liveness/admission checks and registers real physical work before first async open.

`settle` 30–39 closes admission after body settlement, loops over all accepted work with allSettled, propagates body or accepted-child failure, and only then retires liveness. Nested loans therefore retain their parent authority while already accepted work settles. `admit` 16–19 rejects fresh work once birth or any parent is closing; existing authority checks intentionally use liveness rather than accepting, preserving admitted work during drain. That closed-admission choice is not a defect.

Birth returns the whole settle promise at 71–76. Underlying envelope `🏃️process/🟦️.ts` 47–48 removes timeout/SIGINT/SIGTERM handlers only after that promise settles, so accepted tracked work remains under the existing process cancellation lifetime.

Physical issuance `📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📥️issuance/🟦️.ts` 40–42 registers the operation before backend open. Actual accepted image reads register at 24–29 before observation begins. Body settlement closes image admission at 53, drains observations at 54, propagates accepted failure at 55–56, and only then retires the image token and awaits real close at 58. The resource-operation promise includes this finally, so parent drainage includes close. This removes the earlier open/body/close enrollment gap.

## P2: Async public loan wrappers create an unobserved rejection promise

Process issuance 93,109,115 declare async functions which adopt the internally tracked promise rather than return that same promise identity. The private `track` rejection handler at 26 observes its `work`, but not the distinct promise returned by `loan`, nor the distinct outer promise returned by `withIssuedResourceControlV1` / `withRequestedIssuedResourceControlV1`.

Consequently a caller that truly detaches an accepted failing loan without attaching a rejection handler can still produce an unhandled rejection even though parent failure storage/draining is correct. With another accepted child still pending, runtime unhandled-rejection behavior can occur before the parent reaches its final failure propagation. Tests that detach with `void child.catch(...)` do not cover that caller shape. This is a Source-level promise identity gap; no runtime termination was reproduced here.

To make the public accepted-work API itself own detached rejection observation, remove redundant async adoption layers and expose the same registered promise (handling validation refusal consistently), or explicitly observe each externally returned promise. Parent must still fail after drain; suppressing the detached rejection must not erase recorded failure. Physical `withIssuedNativeImageV1` already returns the registered promise directly, avoiding this particular wrapper gap.

## Remaining scope boundary

Body failure wins over child failure at settle 36–37 and physical 55–56; accepted child failure is surfaced when body succeeds. No aggregate-error requirement was inferred. Generic arbitrary asynchronous work started outside these admitted APIs remains caller-owned. Persistent image close uses its existing held lease release; the separate temporary macOS detach cleanup gap documented in `image-cancellation-cleanup-independent-review-current.md` is not resolved by registration alone.
