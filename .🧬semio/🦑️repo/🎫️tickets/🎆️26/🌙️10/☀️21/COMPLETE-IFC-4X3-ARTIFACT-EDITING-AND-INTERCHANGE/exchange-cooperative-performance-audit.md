# Exchange Cooperative Performance Audit

## Finding And Recommendation

The registered five-second timeouts are consistent with material cooperative scheduling overhead, rather than evidence that source admission can be removed. Preserve source capture, all structural/value/reference checks, progress visibility and cancellation. Prefer bounded throughput batches for large success fixtures and separately budget exhaustive cancellation tests. No production edits or test semantics were changed during this review.

One focused read-only reference-TypeScript probe executed from the retained ticket input exchange-performance-probe.ts. Its generated result is under 🗑️generated/exchange-performance-probe.json. This is local wall-clock evidence under concurrent fleet load, not a stable benchmark, native execution, or a claimed fix. No duplicate complete suite was run.

## Measured Reference TS Evidence

| Fixture | Entities | Capture generator steps | Synchronous capture | Async capture batch 256 | Async capture batch 4096 |
| --- | ---: | ---: | ---: | ---: | ---: |
| triangulated | 35 | 2,346 | 13.1 ms | 135.7 ms | 16.3 ms |
| alignment | 3,161 | 195,666 | 211.8 ms | 1,919.7 ms | 698.1 ms |

Alignment capture includes 85,932 copyOwned, 82,772 native-shape admitInput and 26,962 validateBase steps. Progress callback count is unchanged between async batch configurations: all 195,666 steps are still observed. The batch changes host yield frequency, not structural validation or callback/cancellation checks. Default alignment parse separately took 2,079.0 ms; it observed 26,962 references-phase callbacks plus parse/value/source checkpoints. STEP roundtrip additionally captures, validates text and writes in bounded chunks, so a five-second total budget is fragile.

A timer-only probe measured 16 sequential setTimeout(0) turns in 17.3 ms and 64 in 172.5 ms during this session. Variability and fleet contention mean these are not universal timer costs, but they demonstrate that zero-delay timers are not free. Alignment's default capture alone requests floor(195666/256)=764 timer turns; batch 4096 requests 47. The timing difference is therefore credible scheduling overhead, not permission to weaken admission.

## Source Bottlenecks

runIfcEditAsync invokes steps.next, original cancellation checks and progress for every generator step; only host setTimeout scheduling is batched. The private source path intentionally makes multiple complete passes: descriptor-safe copying, native schema shape validation, normalization and graph identity/reference/value admission. IfcGeometryGraph.capture then constructs its index in separately bounded batches. STEP writing uses the same central capture and then text validation and bounded output. This multiplies necessary small operations and timer opportunities.

nativeShapeSteps repeatedly resolves immutable schema $ref strings with split/reduce and selects discriminants; these allocations/comparisons repeat over many graph values. A bounded internal pre-resolved static schema lookup could reduce that overhead while keeping schema-first admission and avoiding another library dependency. It must retain exact domain validation and cannot become a mutable source-value cache.

copyIfcOwnedSteps explicitly uses Reflect.ownKeys per container. The complete key roster is one eager host allocation, especially material for a very large entities/aggregate array. Increasing batch size does not bound this burst. Replacing it must preserve refusal of symbols, accessors, nonenumerable unexpected fields and sparse arrays; simply iterating numeric indices or for-in would weaken admission. This is a known source-capture frontier, not evidence of a newly confirmed five-second defect for these small fixtures.

Progress reporting constructs generator tick records per step and calls the wrapper even when host yielding is batched. That is real overhead, but throttling callbacks would change observable cancellation/test semantics. Any future optimization should retain original per-step checks and explicitly preserve callback behavior.

## Bounded Changes Worth Considering

1. Use the existing admitted maximum batchSize/yieldEvery=4096 for large complete-success throughput fixtures and benchmark registered outcomes. This retains finite bounds and every callback, but it does not help intentionally batchSize=1 tests. If made the production default, validate event-loop latency and cancellation responsiveness across native Windows/macOS/Linux and devcontainer environments first; measured synchronous cost is not a cross-platform bound.
2. Put host task scheduling behind a repository-owned cross-platform yield interface, then evaluate an actual task-queue implementation with a timer fallback. Awaiting an already resolved promise alone is insufficient because it can starve timer-driven cancellation and input events. Preserve checks before/after callback and after the actual host yield.
3. Cache immutable schema reference resolution behind the validator interface, preserving current schema and admission behavior; measure before broadening. Do not skip copy/shape/reference passes just because the graph was previously observed.

## Test Budget Policy

The 855-position alphabet test intentionally performs hundreds of independent full reader operations, each with initial/final host checkpoints. Splitting that loop into nine alphabet-page tests retains all 855 vectors and all assertions while giving each test an honest finite scope. Alternatively provide a justified larger explicit aggregate timeout. Running all cases concurrently without a bound would obscure cancellation/fairness and increase memory contention.

The central every-yielding-phase test intentionally pays batch-one scheduling to verify cancellation at original boundaries. Retain those semantics; a five-second default is not a useful bound for a full exhaustive multi-phase matrix. A scoped explicit timeout based on its measured worst-case test workload is appropriate alongside throughput improvements. Do not raise every suite's timeout to hide a semantic regression.

Official alignment and Khronos NegativeScale cases must preserve genuine full fixture/output comparisons. Their timeout policy can be scoped to the large case while keeping an independent throughput/latency check. This audit did not independently profile NegativeScale, so it does not claim which of native geometry/reference validation/foreign oracle loading dominates that case. The real BIM Axis/RefDirection bug is independent of these timing issues and requires its semantic fix.

## Boundaries

No build-system, compiler ingress, global scheduler or production source was changed. Native flags remain false. Probe-generated outputs stay under the ticket's shared generated folder and were not deleted. Source inspection and local TS timings cannot prove native original-grant receipts, controlled retirement, full semantic conformance or registered publication.
