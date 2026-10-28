# Retained Drawing Affine Paint Consumers

## Implemented

Fidelity preparation now accepts the retained ExportDrawingState. DrawingPath reads its local commands and paint/affine; DrawingOutline borrows its original shaped glyph segments and carries the supplied paint/affine. PaintPath keeps local commands, local flattened edges and original local stroke width together with its complete six-scalar affine and cached inverse. Existing world-space path producers use identity. World-space curve flatness prevents a small local curvature from becoming visibly coarse after scaling. Transformed culling envelopes include the local round-stroke ellipse; overflow remains conservative rather than dropping visible ink. Published path bounds are cached after preparation.

SVG paths and images publish full matrix transforms. Local path numbers and affine coefficients use shortest round-trip numeric strings, preserving tiny geometry before a large affine. PNG inverse-maps every world sample into the original local path for winding and round-stroke distance, preserving nonuniform scale, shear and reflection. RGBA image samples inverse-map into the original local rectangle before premultiplied bilinear filtering. Singular transforms have no raster area. The native and TypeScript scalar coverage cursors retain fixed metadata only and advance under an incoming edge/sample item grant.

The central image_affine(item) query returns Option<[f64;6]>, composing the optional outer affine after ordinary rotation about the exact converted local center. Root normalized Drawing images to the unit rectangle with their full f64 geometry in that affine. The central image_proxy(item,snapshot) query borrows the original inline or link-backed data URL. prepare_image_one takes the original immutable snapshot and mutates only the disjoint decoder/raster slot; it no longer needs a cloned linked proxy. SVG queries the same owner.

Negative and nonfinite stroke widths are refused at Drawing paint and Fidelity admission. Width0 disables stroke, matching SVG/CSS; no PDF hairline is introduced. Existing resource limits and the previously reported font/admission allocation gaps remain. This work does not claim the entire export renderer or VM allocation is fully bounded.

## Test-First And Actual Evidence

The affine schema and neutral local path/image/curve/precision witnesses were authored first. The independent oracle captured expected transforms, conservative bounds and raster probes using Three, Sharp/libvips, pngjs and Skia. All external types stay inside the test-only interface. Production helpers use only owned first-party types and system math.

Actual source execution passes6 path cases and1 RGBA image, across3 edge grants:28800 path pixels and1156800 bounded edge/sample events.24 interrupted coverage cases reset/replay successfully; zero grants do not read edges. Cases cover fill/alpha, nonuniform round strokes, shear, reflection, singular transforms and zero-width no-ink. Two transformed-curve flatness cases and4 invalid width admissions pass. Tiny/huge finite scale inverse round trips and conservative overflow envelopes pass.

Full path and image output is compared to independent Sharp SVG rasterization; maximum observed path channel difference is23 of255, with a mean-error gate3 and maximum gate48 for4×4 sample coverage. Paths also agree with Skia, maximum observed channel difference31. Skia canvas drawImage clips boundary pixels at their centers on this backend: it returns white where SVG/our16-sample area coverage returns partially covered color, with differences up to112. Therefore complete image boundary coverage is checked against Sharp; Skia independently checks image interpolation only on pixels whose four corners lie inside the transformed image, classified by Three. The interior check has maximum gate16 and mean gate3. This is an explicit oracle distinction, not a raster-equivalence claim for that Skia edge policy.

The targeted actual TypeScript closure has171 sources and0 strict diagnostics. Native preparation/sampling/source-loan/precision/width laws consume the same fixture, including exact original linked proxy pointer identity. Six native production/test files are grammar-clean; maximum checked path172. Native compilation and laws remain unrun because the authorized cached repo launcher is stale/unavailable.

## Files And Integration

Shared native fidelity: export/🎨️faithful-primitives/🦀️.rs. New paired scalar owner and neutral laws: export/🎨️faithful-primitives/📐️affine/{🔣️.json,🦀️.rs,🟦️.ts,🧪️tests/cases.json,🧪️tests/🦀️.rs,🧪️tests/🟦️.ts,🧪️tests/oracle.mjs,🧪️tests/oracle.d.mts,🧪️tests/run.ts}. SVG/caller updates: export/✅️validate-plan-and-svg/🦀️.rs. PNG-only culling update: export/📕️pdf-png-and-package-encoding/🦀️.rs; root owns its PDF/planning sections. Existing native fidelity test caller received the new source argument. Drawing planning paint admission was tightened in paired native/TypeScript files.

Root owns PdfItem/Job fields, normalized Drawing image publication and full-affine PDF operators. Core owns FramePlanAddress preparation and projection metadata extraction. The affinity API was handed off before their integration. Ticket export-affine-check.mjs retains the reproducible execution/grammar inputs and removes its generated outputs. No build/dependency/git/AGENTS changes were made by this scope.
