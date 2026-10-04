# Final Scale and Customization Contract Audit — 2026-10-04

Independent read-only review of the current canonical print chart contract. Only this report was created. No source edits, new dependencies, process interruption, or Git mutation occurred. The coordinator's latest registered 494/494 exhaustive and strict 248-export build evidence was read in the retained unknown-scale audit; those commands were not rerun here.

## Material Findings

1. **Declared nice method loses authored tick controls.** `schema/inferences/📐scale/🟦️.ts` lines 894–898 bind authored tickValues, ticks and tickFormat to the initial scale. The numeric continuous nice method (line 200) returns a fresh raw `scaleContinuous`; the authored binding is absent on that result. An actual Bun source invocation of linear domain [0.1,0.9], range [0,10], options {tickValues:[0.2,0.8],ticks:2,tickFormat:'.2f',unknown:-5}, then nice(2), produced beforeTicks [0.2,0.8], afterTicks [0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1], beforeFormat '0.20', afterFormat '0.2'; unknown remained -5. Preserve authored controls when deriving the niced scale and cover the method result in the existing neutral scale tests. The owned VizScale currently declares no copy method; copy preservation is therefore unverified and is not an implemented API claim.

2. **Declared labelRotate is ignored in canonical numerical inference.** Shared schema line 25073 declares labelRotate as tick-label rotation in degrees. Native `semio-viz-guide.sty` registers labelRotation and labelRotate against the same rotation register (lines 316 and 344). Numerical renderer line 135 reads only labelRotation. An actual asynchronous canonical inferVizChart request with en, width100, height80, no layers/tables, linear x domain[0,1]/range[0,100], axis tickValues[0,1], options {labelRotate:45}, completed successfully with no diagnostics and both text rotations 0. Resolve the current authored aliases consistently, including deterministic precedence when both are authored, and add neutral worker/geometry validation.

These are supported contract defects, not requests for additional hypothetical options.

## Confirmed Source Closure

- Shared ChartScale allOf/anyOf admits unknown on the thirteen supporting kinds and forbids it on band/point. Both constructors explicitly reject a present unknown option. Four existing neutral schema vectors compare owned validation with AJV; the retained latest exhaustive result covers them.
- Numeric null/absent channel values become NaN gaps rather than zero. Explicit encoding values are read using property presence. Diverging direct D3 null coercion remains distinct from renderer missing-channel policy. Typed paint omissions avoid the strings null/undefined. The final worker admits the complete plan/scene through the owned closed inference schema; failed admission publishes empty TikZ, diagnostics, complete false, and no partial IR.
- Ordinary caller timer cancellation can run while the owned worker computes. Worker lifetime, AbortSignal, listener cleanup, completion-before-publication cancellation and observer failures are covered by sixteen registered mutation checks. The retained async audit includes real Chromium Web Worker success and independent timer cancellation, plus bundled Node v24.14.1 worker_threads success and after-layout timer cancellation. Current constructor retains the literal bundler-discoverable Worker URL pattern. This audit inspected that source and evidence; it did not rerun browsers or Node.
- Numerical runtime ownership remains under schema/inferences. Repository source search found no standalone viz-kernel imports or project references. One historical oracle identifier viz-kernel-twin-parity remains; it is metadata, not a runtime module dependency.
- Inference package lists D3 as devDependencies only. Runtime chart numerical leaves use owned source, canonical JSON, and platform interfaces. External numerical imports found in the package are in differential probes. Node worker_threads is a system interface. No added external runtime dependency was identified.

## Existing Fixes Retained

The earlier minor-tick/domain-line audit findings are closed in current renderer source: explicit minor options override the typed flag, subdivision/size are applied, domain/domainLine precedence is supported, grids default to no domain, and actual range endpoints govern domains. The retained guide report records seventeen neutral worker vectors and measured native minor positions. The closed-result report records strict owned plan and scene variants, arities, frame/theme/paint admission and physical TeX-point to millimetre text conversion.

## Remaining Execution Gates

The native High owner is still executing shared unknown/typed-row refinements and native grammar verification. This audit does not certify that gate from source review or TypeScript results. The all81-document light/dark catalogue gate requires162 independently consumed PDFs and final visual QA; it is also pending coordinator execution. The two findings above require closure before final contract completion.

## Evidence and Scope

Read the retained unknown-scale parity, guide-control parity, closed-result, async-inference, and customization audit reports; inspected canonical chart schema, scale dispatcher/methods, renderer, worker, inference boundary, native guide keys, owned inference package declaration, and repository source references. Two small runtime vectors were executed through Bun in the canonical workspace. Their exact output is retained above. Temporary source vectors were streamed to Bun rather than creating a separate permanent script.
