# big-number-display paint assertion

## Root cause
The test contract was wrong, not the TeX or the PDF capture. `big-number-display` (family `kpi`, mode `big-number`) is rendered by `\semio_viz_kpi_number:` (`semio-viz-charts-dashboard.sty`), which only emits TikZ text nodes: it is a text-only kind. The kind-paint contract (`glyphKinds`) listed only 7 text-only kinds, so `assertPrintKindPaintOwnership` demanded paths or images.

## Evidence
- `paint-oracle.ts` (plain pdfjs-dist, marker-scoped operator walk) over all 87 built gallery PDFs (light + dark): leaves with no path/image paint, all having glyph text ops:
  big-number-display (18), text (79), font-weight / font-size / text-rotation (74), plus the already-listed text-mark, word/tag/frequency-cloud, sequence-logo, motif-logo.
- `real-paint-check.ts` replays the real `readPrintKindPaint` (with the snapshot-equivalent operator normalization through `createPrintPdfOperatorCaptureV1`) against 160 built PDFs and applies the exact assertion per leaf: before the fix exactly 5 distinct slugs failed (glyphs=6 each, paths=0, images=0): `18/big-number-display`, `79/text`, `74/font-weight`, `74/font-size`, `74/text-rotation`. After the fix: zero failures.
- The `undefined`-argument capture change did not drop text operators: the glyph counts are non-zero through the real capture path.

## Fix
Authored contract (single source, schema-first) `glyphKinds` extended with the 5 source-backed text-only kinds (family and definition verified against the catalog and `.sty` by `glyph-kinds-check.ts`):
- big-number-display / kpi / semio-viz-charts-dashboard.sty / semio_viz_kpi_number
- text / grammar / semio-viz-showcase.sty / semio_viz_scg_texts
- font-size, font-weight, text-rotation / encoding / semio-viz-showcase.sty / semio_viz_sce_font_size, _font_weight, _text_rotation

File changed: `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🖌️kind-paint/🔣️.json`. No source files added or removed, so `🧫️command-boundaries.json` is unchanged.

## Tests
- Offline: `glyph-kinds-check.ts` all 12 true; `real-paint-check.ts` zero failures over 160 PDFs.
- Full run queued once in the background; see below for exit status.

## Status
Run log: `🗑️generated/bn-test-viz-full.log`, exit code: `🗑️generated/bn-test-viz-full.exit` (not yet written when this report was created; queued by the capacity gate).
