# Customizable Visualization Rendering

## Implemented Grammar

The shared primitive planner now consumes inferred layer tables rather than bypassing transforms and layouts. It renders bar, rect, heatmap, point, circle, symbol, line, area, band, trail, arc, pie, text, rule, link, path, polygon and ribbon marks. Data rows can supply conventional geometry columns without explicit encodings. Encodings resolve constant values, columns and scales for fill, stroke, opacity, symbol shape, area size, stroke width, dash, rotation, order and detail. Varying series styles produce separately styled segments, and trail size controls segment width.

Cartesian screen millimetres remain direct coordinates. Normalized Cartesian, transposed Cartesian and non-Cartesian coordinates project through the coordinate contract. Polar rectangular marks resolve as annular sectors. Layout-generated geometry remains absolute figure millimetres. Empty series produce no items; invalid positions split lines and skip point primitives rather than producing invalid numbers.

Axes support all four orientations, explicit offsets, categorical domain ticks, numeric and temporal format strings with explicit language, optional domain lines, label rotation, guide titles and styling. Grids emit only grid lines. Legends support color, size, stroke, dash and shape swatches. Localized titles and text/shape annotations resolve only with explicit document language.

Scene and TikZ emitters consume one shared plan. Plan-consuming exports prevent repeat data inference when both outputs are requested. Scene paint carries stroke opacity, dash, caps, joins, clipping and rotation; text carries anchor and middle baseline. TikZ plain text escapes every TeX special character. Paths expand Canvas arcTo geometry and split complete circles into two arcs; quadratic curves emit their equivalent cubic controls. TikZ supports clipping scopes, alpha, dash, cap, join and rotation controls.

Optional planner controls expose AbortSignal and progress callbacks. Cancellation is checked before layers, after progress notifications and periodically while rendering rows. The canonical asynchronous wrapper is responsible for scheduling phase boundaries; expensive data inference is being coordinated with its owner for the same control propagation.

## Verification Evidence

Authored language-neutral JSON fixture: `print/🧪️tests/🎬️render-scene/🔣️customization.json`, with 42 cases. The corresponding Gherkin outline and TypeScript adapter execute each case. Fixtures cover every mark family, row styles, empty data and gaps, normalized and polar coordinates, all four guide orientations, categorical guides, legends, localized annotations, escaping, clipping and complete arcs. Independent D3-shape and D3-chord contexts produce line, area, link, arc, symbol, ribbon and pie command oracles.

Executed the existing package script route through Bun: `bun ./📜️script.ts test quick render`. Latest result: **48 checks, 48 passed, zero failures, zero errors**. Existing bar positions remain covered; category axes now produce their previously omitted domain labels and the primitive census was updated accordingly.

Attempted Nx route: `bun x nx run @semio-tech/print-viz-kernel:test -- quick render`. It was blocked before test execution by unrelated repository graph errors: Rust package Cargo test policy escapes the workspace and an npm project dependency for `@asamuzakjp/css-color` is absent. Do not describe this route as passing until the parent resolves the graph and reruns it.

Ran the actual TypeScript compiler over renderer, probes and test adapter. It identified one renderer command-tuple cast and an old render namespace type reference; both were corrected. Other errors in the shared spatial implementation and transform oracle probes were sent to their owners. A subsequent compiler result is still required before declaring type checking complete.

Runtime-generated authored compiler input: ticket `🧫️renderer-specimen.tex`, containing all 42 resolved TikZ pictures. The runtime logged `[DEBUG] renderer emitted 42 TikZ specimen charts`. Parent/native verification owner must compile and visually inspect representative text escaping, complete arcs, polar rectangles and clipped dashed paths before reporting native verification complete.

## Architecture Coordination

The parent owns relocation of the previous computational kernel into canonical inference ownership. Rendering changes were saved at the original path until its announced safe migration boundary. No runtime external dependencies or additional standalone visualization runtime modules were introduced. No modifying Git operations were used.

Remaining audits should use the relocated inference paths, wire table-inference cancellation controls when available, and confirm the final TypeScript/Nx/native compiler results. Family presets are explicitly rejected by the generic planner and are dispatched by native print family inference rather than silently omitted.

## Canonical Inference and Native Guide Completion

Relocated ownership is `print/🧬️schema/💡️inferences/🖼️render/🟦️.ts`. Geographic projection uses the owned geographic inference; optional cancellation propagates to layer transforms/layouts and progress covers both data resolution and rendering. Native canvas anchors and baseline contracts are now consumed by the scene emitter. Strict TypeScript build through `bun ./📜️script.ts build` completed successfully and loaded 247 inference exports.

Compiled specimen QA found that TikZ absolute millimetre arc radii retain physical y-up angle semantics inside a y-down coordinate context. The raw emitter now negates its start/end angles, fixing disconnected pie sectors. The renderer has a numerical TikZ angle assertion and the parent recompiles the regenerated 42-chart specimen.

The native guide implementation now accepts schema guide style/color/font/rotation overrides, explicit legend values and tick format, all side orientations, placement/spacing, channel selection, area-size semantics, all owned symbol glyphs, and row scale values for color/dash/shape legends. Its gradient resolves authored scale color interpolation. Numeric timestamp labels use the common locale time formatter. A committed language-neutral Gherkin custom-controls scenario compiles the native fixture and compares actual ticks, German decimal label, UTC date label and RGB color against D3 packages.

Actual first custom-controls Tectonic compilation passed with tick positions [8,43,78] and German decimal label 0,5. Its PDF revealed preexisting zero-width horizontal legend labels measured under TikZ nullfont; measurement now enters pgfinterruptpicture. Recompiled PDF has nonzero label widths and no missing-character warnings. Additional temporal and color oracle compilation is being finished with the scale owner.
The complete custom-controls adapter was executed against Tectonic and passed all four independent projections exactly: ticks [8,43,78], German label ['0,5'], UTC timestamp ['2025-01-01 12:00'], RGB midpoint [128,0,128]. Latest renderer rerun remains 48/48, zero errors. Native guide measurements no longer emit nullfont missing-character warnings.
