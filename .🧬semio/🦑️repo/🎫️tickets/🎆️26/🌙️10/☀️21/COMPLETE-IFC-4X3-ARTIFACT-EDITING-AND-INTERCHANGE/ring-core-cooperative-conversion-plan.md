# Existing ring core cooperative conversion audit and baseline

The actual shared IFC ring core is ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🕳️rings/🟦️.ts, beside Rust implementation/tests. This corrects the earlier profile exploration's incomplete search, which found the framework Rust triangulator but missed this first-party TS implementation. Reuse this core; do not create a second IFC ear-clipping algorithm or runtime external dependency.

## Existing semantics worth preserving

triangulateRings(vertices,input,tick) returns original-index triangles. It translates relative to the first outer point, normalizes by extent, computes dominant projection/Newell normal, verifies planarity, removes repeated closing coordinates, rejects degenerate/self-intersecting/intersecting/outside/nested rings, normalizes outer/hole winding, bridges visible hole occurrences, ear-clips, then verifies filled-domain centroids/coverage/boundary+interior incidence/triangle overlap. It reverses final triangles to the original outer winding. Generic projector calls this at geometry/🟦️.ts:162 and396 with its own tick budget. Retain these validations and deterministic triangle ordering, including repeated bridge endpoint occurrence handling.

Existing language-neutral cases and TS/Three ShapeUtils independent tests cover concavity/holes, reversed winding,1e±120 scales, collinear boundary segment retention, every original tick interruption, and oblique repeated-bridge regression across1e±90 scales. Existing test oracle compares area, not exact triangle topology. Neither source inspection nor the generic runtime's successful tests establish original-funded native execution.

## Exact hidden cold operations

- points Map normalization pass has no tick although each vector map is fixed3; add a continuation per map entry.
- input.map plus whole source spreads creates all ring copies without control. Copy one ordinal per step, retain closing-pop behavior.
- ring.reverse moves a source-sized ring in one action; swap at most one pair per continuation.
- filled uses rings.slice(1).every on every query; eliminate slice and preserve exact short-circuit order via generator helper.
- merged=[...rings[0]]; holes slice/map, then native Array.sort. Copy/leftmost scan per ordinal; sort with a bounded stable algorithm and preserve equal-key insertion order. Add original ordinal as explicit tie authority if using an unstable sorter.
- bridge creation concatenates four slices/spreads in one turn; build replacement merged roster per emitted ordinal, then exchange private arrays. Preserve duplicated h and merged[best] bridge indices exactly.
- merged.splice(ear,1) hides a source-sized shift on every ear. Shift one ordinal per turn or use an order-preserving linked roster with independently frozen topology; the simpler shift preserves exact current indexing.
- expected rings.reduce repeatedly calls area; boundary Set birth loops have no ticks. Convert both to explicit continuations. Edge/triangle loops already tick but their helper calls must themselves suspend.
- final triangles.map winding flip creates a source-sized result in one action; flip/push one triangle per continuation, or mutate each private triangle once before publication.

Most map/reduce/every/filter calls operating on2/3 coordinates are fixed bounded work and need not become separate scalar generators. The problematic variable arrays above must not be hidden behind one yield or one ring-level checkpoint. Intersection/ear containment/triangle-pair loops already tick per comparison; preserve those inner boundaries.

## Minimal API and semantics

Export triangulateRingsSteps(vertices,input,tick):Generator<void,RingTriangle[],void> from this domain module. Keep triangulateRings as a synchronous drain of that same generator. The module must not import IFC4x3 IO controls. Generator helpers for area,inside,filled,blocked must use yield* so nested source-sized comparisons remain suspendable; preserve short-circuit/evaluation order and diagnostic strings.

At each old tick site retain the callback call in its original semantic position, then expose a continuation; new metadata/copy/sort continuations should not silently add budget callback invocations if exact old tick counts are part of the generic projector's work policy. Alternatively intentionally revise that budget contract with explicit expected count changes and tests; do not call it unchanged. The IFC4x3 caller runs the same generator through runIfcIoSteps under original captured signal/progress and private source. It needs initial/interior/completion publication boundaries and generator return on cancellation. Do not asynchronously borrow caller-mutable arrays without existing private source capture.

The new continuation count may grow while old sync tick counts remain frozen; that is expected if scheduler work and existing projector semantic budget are separate. Array allocations and JS Map internals are conventional TS execution, not native five-axis receipt proof. Any new sort/replacement temporary remains private and unpublished on refusal.

## Actual immutable baseline

Authored ring-generator-baseline.ts imports CURRENT original triangulateRings (no source copy) and records36 successful cases: all five neutral families × reversed/normal × three scales, plus six oblique cases. Actual result:36/36 completed, zero errors. generated/ring-generator-baseline.json freezes exact triangle arrays and original tick counts; --compare reuses the same cases and writes a separate comparison. Bun process exited; no live source mapping remains.

Before integration, extend negative-vector baseline to exact errors for invalid index/nonfinite/range/zero edge/nonplanar/degenerate/outside/nested/touching/crossing. Keep every-old-tick exact thrown identity and add generator every-yield cancellation with no returned partial triangles, completion cancellation on actual IFC endpoint, large ring-copy/reversal/hole-sort/bridge-splice/boundary/final-winding frontiers. Existing independent Three area+coverage tests remain necessary; exact topology parity complements them rather than replacing the independent oracle. Root's current registered test run is separate evidence and not duplicated here.
