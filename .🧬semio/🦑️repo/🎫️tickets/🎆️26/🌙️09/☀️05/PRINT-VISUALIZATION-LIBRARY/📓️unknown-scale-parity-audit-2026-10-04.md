# Authored Unknown Scale Inputs and Missing Geometry

The canonical first-party TypeScript scale inference now preserves authored unknown values directly, including null and boolean outcomes, instead of coercing missing inputs to zero or interpolating fallbacks as colors. Every supported family retains its existing tick, format, inversion, nice, bandwidth, step, or threshold methods. A first-party optional unknown getter supplies renderer fallback admission without duplicating scale math. Ordinal domains preserve their numeric and string identity and retain default implicit insertion.

## Independent Before Evidence

The registered Nx quick scale baseline ran before the scale implementation changed: 105 checks, 49 passed, 33 numerical mismatches and 23 runtime errors. Linear null mapped to zero while the independent D3 oracle returned undefined or the authored -5 fallback. Undefined, NaN and invalid strings produced NaN instead of unknown. Missing sequential/diverging/color inputs reached interpolation with NaN and threw. Numeric ordinal domains were coerced into strings and disagreed with D3. The initial rejection probe used an incorrect fixture projection; it was repaired to use each neutral entry.spec before final validation, and its initial passing result is not treated as evidence.

The independent D3 oracle establishes family distinctions: diverging(null) maps zero directly, while its undefined/NaN/invalid-string inputs use unknown; quantize/threshold invalid string maps the first bucket, while quantile uses unknown. Empty strings and booleans retain D3 numeric coercion. Authored ISO timestamp strings are admitted through the canonical UTC parser, while numeric-like strings (including empty strings) retain numeric coercion.

## Renderer and Admission

Authored numeric channel null/missing values create gaps by default; an explicit numeric fallback places them at that value. The renderer applies this explicit missing-channel policy to diverging without changing its direct D3 callable semantics. Point omissions and line subpath breaks are checked against independent D3 scale/shape calculations through the actual asynchronous worker. Nullable or missing fill/stroke fallbacks omit paint instead of producing the invalid strings null or undefined. Explicit authored encoding fields are read by presence rather than null coalescing.

Band and point expose no D3 unknown configuration. The canonical shared ChartScale schema rejects unknown on those kinds through allOf/anyOf, and their constructors reject it directly. Independent AJV and owned schema validation exercise rejection plus valid controls.

## Neutral Vectors and Runtime Evidence

The existing authored-color-scales test area contains 57 direct family/configuration cases, four band/point rejection specifications, and ten canonical worker rendered-gap/paint cases. Inputs cover null, absent values, NaN, invalid strings, numeric strings, booleans, empty strings, invalid dates and ISO timestamp strings. The existing differential adapter and inference probe harness register them; no external runtime dependency or additional permanent script was added.

Generated runtime evidence: generated/unknown-runtime.jsonl (temporary debug projections, removed with final ticket cleanup). Each line records actual subject and independent D3 oracle results with the [DEBUG] prefix. Native counterpart work and measured TeX gaps are retained in the native owner report.

## Changed Files

- 🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📐scale/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧪️tests/🎨️authored-color-scales/❔️unknown.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/🎨️authored-color-scales/❔️render.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/🎨️authored-color-scales/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧪️tests/🎨️authored-color-scales/🥒️.feature
- This retained audit.

## Verification

An intermediate post-fix quick scale/render run passed 181/181, including ten actual canonical worker render cases. A strict Nx build passed and confirmed 248 public exports. Further final expanded gates are being recorded below after completion.

Final registered verification completed on 2026-10-04:

- bun x nx run @semio-tech/print-viz-inference:test-exhaustive — 494/494 passed, zero mismatches/errors; scale 106/106 and render 76/76. Includes the existing 16 mutation checks with actual timer cancellation and browser worker execution. Run duration 11.2 seconds.
- bun x nx run @semio-tech/print-viz-inference:build — strict TypeScript succeeded, 248 public symbols, 34.8 seconds.
- Runtime debug projections regenerated after final source changes: 69 exact case records containing actual subject and D3/AJV oracle values. Band/point admission alternates false/true for rejected unknown configuration versus valid unconfigured controls.

The expanded boolean/empty-string vectors first failed seven checks: log(0) needed D3 interpolation arithmetic a*(1-t)+b*t rather than a+(b-a)*t, and diverging needed D3 midpoint reciprocal arithmetic to preserve the same RGB rounding boundary. Installed d3-scale/src/diverging.js and d3-interpolate/src/number.js supplied primary implementation evidence; the final registered numerical gate confirms the corrections.
