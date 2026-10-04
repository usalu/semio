# Customization Contract Audit — 2026-10-04

Read-only source audit of the current canonical chart snapshot, guarded diff/mutation, inference renderer, scale dispatcher, Rust inference emitter, and native guide grammar. No source modifications or fresh runtime tests were performed in this audit.

## Actionable Findings

1. **Authored guide minor ticks are silently ignored by TypeScript inference.** `VizGuideSpec.minor` is a typed boolean, and the canonical schema describes drawing minor ticks between major ticks. The guide loop in `🧬️schema/💡️inferences/🖼️render/🟦️.ts` reads ticks, tickValues, tickSize and grid but never reads minor. Rust emits the authored minor setting. Native `semio-viz-guide.sty` implements it in `semio_viz_guide_minor_draw:` with minorTicks=4 and minorSize=0.7 defaults, interpolating positions between consecutive major tick positions. The same chart therefore has different geometry across implementations. Add a neutral authored minor fixture and independent subdivision oracle, then implement the existing semantics in the canonical renderer.

2. **Native domain-line customization has a different key and default from TypeScript.** Native axis options register both `domain` and `domainLine` aliases, reset the domain boolean to true, and draw the domain through the existing axis routine. TypeScript only draws a domain when `guide.options.domain === true`; it never reads domainLine and otherwise omits it. The extensible authored guide.options record accepts domainLine, and Rust forwards that key unchanged. This is a concrete native option interoperability gap rather than a request for a new API. Use the existing native keys/default as the owned contract and add explicit true/false plus omitted-option neutral geometry cases.

## Reviewed Without a Confirmed Defect

- Guarded mutation replay clones before applying, enforces exact before-value preconditions, validates the completed chart, and rejects atomically. Array indexes are canonical/in-range; removal splices; explicit null differs from absent undefined. Mutation inverse restores the parent array for array edits, avoiding a shifted-index inverse. Diff inverse derives a guarded reverse snapshot diff. These are source observations, not fresh test claims.
- Numeric chart math resides under canonical schema/inferences; this audit found no new standalone runtime chart-math module. Theme paint parsing uses the existing print-owned design-token paint interface.
- Scale dispatch reads unknown only for ordinal; continuous handling returns NaN for NaN input. The public type permits unknown on a shared scale-options object but does not document applicability per kind. This alone does not establish a continuous-scale supported-option defect; no new test was run, so broader unknown applicability remains a contract clarification limit.

## Verification Limits

The coordinator reports latest 408 exhaustive/browser checks. This audit did not rerun them and does not certify full native grammar or 162-PDF catalogue completion. Native execution and catalogue agents own those live gates. Open option records permit arbitrary primitive keys; this review cannot certify every family-specific key from a bounded source scan. Remaining guide style keys (for example native labelRotate versus renderer labelRotation) warrant a future explicit shared vocabulary audit, but are not promoted to a separate completion blocker without checking their authored contract and accepted existing aliases.

## Files Read

- `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📐scale/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-guide.sty`

Only this report was created.

