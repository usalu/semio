# Authored PDF Stroke Style

The PDF 1.7 page editor now carries line width, dash array and phase, cap, join, and miter limit from the content operators and ExtGState dictionaries into the canvas scene. The graphics stack restores these values through `q` and `Q`. The scene retains the transform for the stroke-time user space and expresses the path and stroke style in that space, preserving the shape of strokes under nonuniform scale and shear. Clipping stays in page coordinates.

Chromium uses the Canvas2D dash offset and miter limit. The native painter constructs the dashed outline in authored units and transforms its polygons into screen space. Hit testing maps the selection point back to the same authored space and checks the visible dash runs, caps and joins. For closed dashed PDF paths the scene emits the closing line with separate caps at the seam, matching the reference PDF behavior.

## Verification

- The shared `🧪️stroke-style.json` fixture covers dash phase and gaps, butt, round and square caps, round and bevel joins, the miter limit, a closed dash seam, nonuniform scaling, and restored state after nested `q` and `Q`. The page editor regression imports a genuine PDF assembled from its operators and checks scene wire values and selection against the same fixtures.
- MuPDF independently built and rendered a genuine PDF from the fixture and matched 20 interior or blank probes. Chromium ran the production canvas painter and matched all 20 probes. Visual comparison showed the same dash positions, cap and join silhouettes, narrow transformed stroke, and seam.
- The source-isolated Rust test compiled the production native path painter and PDF hit-testing geometry. All 11 tests passed, including the shared 20-probe stroke fixture and the earlier path, clipping, and viewport tests. The browser Canvas2D paint suite passed all 14 tests.
- The isolated tests exercise the exact stroke and hit-test algorithms and production Chromium painter. The full native GPU pipeline and editor import-to-scene unit suite remain unverified.

## Rendering Limits

The native renderer and selection approximate curves by tessellating them. Native stroking and selection still omit zero-length dash marks and degenerate subpaths that rely on round-cap dots. PDF zero-width hairlines use an approximate width, and stroke outlines of text glyphs still use the host's text-specific rendering path.
