# Final German Dark K16 Regression

The settled German dark variant compiled successfully with registered Tectonic 0.16.9 after the graph width and ledger gutter fixes. The official deriveDarkPrintTexSource function produced the driver. Verification used the established printSourceReferenceMapV1/TextV1 projection for the newly added default footer resource, with the derived compiler input first and last in Tectonic search paths. The ASCII directory junction preserves the original PNG bytes and file link count. No production class, style, icon or checked source file was copied or changed by this regression.

## Actual PDF Results

- Three A4 pages, 115,381 bytes; catalog language de-DE; title Fingerübung Parametrisierung; author Architektur.
- Zero overfull and zero underfull boxes.
- Two compact nine-column, nine-row matrices remain together on page 2. Writing rows measure 6.14 mm; canonical vector icons remain centered within 0.030 mm, with minimum insets of 2.53–2.62 mm.
- Page 3 contains all sixteen canonical vector icons in matrix order, 120 distinct straight K16 base edges, the sixteen ring edges plus four chosen middle chords, and exactly twenty matching blank ledger entries.
- Actual label-inclusive graph bounds measure 170.001 by 84.242 mm. The vertical center radius is 32 mm; the small dark contrast backing adds 0.64 mm to the light graph's 83.602 mm outer height. The horizontal dimension fills the 170 mm text width without vertical scaling.
- Twenty writing rules use two columns of ten rows, a 6.3 mm pitch and approximately 68 mm line lengths. The label areas and column gutter remain clear.
- Independent PDF character bounds inspection found no text outside any of the three pages.

## Visual Review

The retained generic review-k16-pdf.py helper rendered all three pages through the bundled PDFium and Pillow libraries. The contact sheet and full-resolution third page were inspected. The oval spans the text width, all sixteen numeric labels remain readable, the ring and four selected chords stand out from the faint K16 base, and the ledger has twenty clear writing slots with a visible gutter. Both matrices, page headers, footer logo and page numbers fit without overlap or clipping.

The old Node review-pdf.mjs helper exited silently with status 1 on this Windows host; the independent PDFium renderer completed successfully. No source change was made for that auxiliary renderer.

## Retained Inputs and Commands

The ticket retains k16-dark-derive.cjs, the three focused diagnostic TeX probes and check-matrix-dark.py with its current three-page expectation. The parent owns final delivery and generated-output cleanup.

Registered Tectonic used the pinned encoded local bundle and existing product, LaTeX, font and assignment search paths, with the derived compiler-input directory first and last. Bundled Python ran check-matrix-dark.py, verify-matrix-pdf.py, verify-k16-pdf.py, review-k16-pdf.py, and a direct character-bounds check. Temporary outputs are under generated/compiled and generated/review-k16-dark; their contents are removed by the parent when the ticket closes.
