# Shared ring continuation independent audit

Current source: shared IFC 📐️geometry/🕳️rings/work.ts and 🟦️.ts synchronous drain. No new confirmed P1/P2 in this focused review. This supersedes the earlier recommendation to freeze tick counts: parent explicitly chose to charge newly exposed source-sized work, so increased ticks are intentional; topology and refusal/cancellation behavior remain the comparison contract.

## Actual preserved output

Reran immutable ring-generator-baseline.ts --compare against all36 old endpoint outputs. Exact original-index triangle arrays match36/36, including winding reversals, tiny/huge scales, collinear source boundaries and oblique repeated bridge endpoint regression. No errors introduced. Per-case tick deltas are independently retained in generated/ring-generator-comparison.json: increases33–819; central-square-hole case526→629(+103). Baseline file itself was not replaced.

## Source continuations

Normalized point Map pass, ordinal copying, in-place reverse swaps, hole leftmost scan/stable insertion ordering, each bridge roster append, ear removal shift, area/domain/blocked helper recursion, boundary construction/incidence and final winding flips all expose continuations. Inner intersection/ear/triangle-overlap comparisons also yield. Stable insertion sort compares <=0 before moving, preserving original equal-key hole order; it has quadratic worst-case hole ordering instead of native sort's faster asymptotic behavior, but every comparison/shift is bounded and the complete validator already performs pairwise geometry checks. Do not describe the whole evaluator as linear-time.

Remaining map/reduce/filter/spread uses are fixed two/three-component vector operations. The two-index bridge push and edge updates are fixed size. Dynamic array/Map allocation remains conventional TS runtime behavior, not physical native wallet proof. The generic generator borrows its vertices/rings; async caller must supply the existing captured/private/stationary graph rather than assume readonly types create ownership.

The sync wrapper drains this exact generator, calls original tick at each yielded boundary, and returns the generator in finally on callback refusal. Shared work.ts has no IO-control dependency. The existing runIfcIoSteps gives initial/interior/completion original cancellation/failure boundaries; no partially completed triangle list is returned.

## Independently executed control probes

Authored ring-generator-control-audit.ts against actual exports/shared runner. The central-square-hole case has629 yields. Every incomplete prefix0..629 was closed through generator.return and exposed no partial result; source coordinates/rings remained unchanged. All629 sync callback interruption points preserved exact thrown identity, alternating undefined and numeric23. Six original runner checks passed: Error identity and AbortController io.cancelled at initial/interior64/completion. generated/ring-generator-control-audit.json retains results.

These probes establish TypeScript source behavior only. They do not prove native execution/funding, a mounted new IFC polyline mesher, full schema validity, DOM lifecycle or MVD conformance. The parent's26-case independent Three/test suite is separate evidence and was not rerun here. Both Bun probes exited; no live audit source mappings remain.
