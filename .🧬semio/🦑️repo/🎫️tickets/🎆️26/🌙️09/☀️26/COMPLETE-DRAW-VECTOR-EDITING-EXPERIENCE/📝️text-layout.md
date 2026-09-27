# Text Layout and Appearance

## Findings

The inspector accepts multiline content, but Canvas2dHost paints the complete string in one fillText call. It also synthesizes black fill when fill is absent and ignores text gradients and strokes. Raster scene export repeats those paint limitations. Draw scene flattening discards the text body's x/y coordinates, while framing starts the same scene text at negative size. The canvas and semio-drawing bridge instead place its first baseline at positive size. Direct PDF export used baseline zero.

## Contract and Work

Text bodies use a local top-left origin; the first alphabetic baseline is one font size below it. Explicit CRLF, LF and CR line breaks advance by 1.2 font sizes, preserving blank/trailing lines. Scene flattening composes text x/y through the authored layer and parent transforms. Canvas and exports preserve authored paint. A shared framework 2D text iterator and font-independent fallback extent have Rust/TypeScript twins. The neutral fixture is checked against lines-and-columns in TypeScript and consumed by Draw native tests. Fallback extents remain approximations until shaped font metrics are supplied; this does not complete typography, font controls or direct canvas text editing.

## Pending Verification

Renderer paint regression handle 76920 tests multiline baselines, disabled fill with a stroke, and gradients before repair. Drawing framing regression handle 87006 tests the top-left origin and rotated multiline bounds before repair. Existing native suite 30810 and plugin build 43904 remain live; no restart has been made.

## Implemented

- Framework 2D now provides lazy Rust/TypeScript CRLF/LF/CR line iteration and one explicit fallback line extent. Neutral fixtures include empty content, consecutive breaks, trailing breaks, Unicode and varying font sizes. The TypeScript fixture test uses the installed lines-and-columns parser as its independent oracle.
- Canvas2dHost paints each nonempty line at its own baseline, preserves blank-line spacing, paints solid/gradient fills and strokes, and does not invent black fill. Raster scene painting uses the same line iterator while preserving its existing baseline-coordinate contract.
- Drawing scene conversion composes authored text x/y into its matrix. Layer bounds and shared editor/viewer framing use the multiline fallback extent at the text block's top-left origin.
- PDF emits per-line text matrices, uses explicit fill/stroke render modes, and clips gradient shading through glyphs. The semio-drawing bridge emits one positioned text leaf per nonempty line.

## Verification

- Renderer regression: **3 failed / 5 passed** before repair, isolating collapsed lines, unwanted fill and ignored gradient.
- Framing regression: **2 failed / 48 passed** before repair; text origin was -10 instead of 0, and rotated multiline bounds started at 94 instead of -8.
- Renderer after repair: **15 passed / 2 suites**, registered target `@semio-tech/framework-renderer-react:test`; output `🗑️generated/tests-text-paint-fixed.txt` (handle 35592 terminal success).
- Shared layout and Drawing framing: registered `@semio-tech/s-2d-js:test` and `@semio-tech/draw-js:test` both passed; Nx suppressed individual successful-task output, so no per-target counts are inferred. Output `🗑️generated/tests-text-layout-framing-fixed.txt` (handle 15868 terminal success).
- Native tests were added for the common line fixture, authored text coordinates and PDF multiline paint modes. They remain unverified while existing native/build handles 30810 and 43904 continue. The source was edited after those runs began; inspect compiled test rosters before treating either as coverage of these changes.
- Scoped whitespace validation passed.

## Browser Observation

The existing preview recovered from Loading plugins. Add Layer → Text → Execute produced two layers and a visible Text label after asynchronous command completion. The loaded component still has no Text Content / Text Size fields in Inspection, so this is evidence of the older component only. Screenshot: [text-layer preview](🗑️generated/text-layer-preview.png). A subsequent Fill Enabled click found no element during another shared-checkout reload; that paint toggle is unverified and was not repeated blindly.

## Remaining Limits

Font-independent bounds are approximate. The PDF writer remains Helvetica/WinAnsi without embedded Unicode fonts. The semio-drawing bridge's existing style contract drops gradients and lacks the authored font-size field. The later [SVG export checkpoint](🎨️svg-export.md) replaces that route for SVG with a direct typed serializer; its native and browser validation remain pending. Native renderer parity and browser verification of the rebuilt text controls remain outstanding. This checkpoint does not complete typography or the overall Draw goal.
