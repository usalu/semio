# Inner Cycle German Dark Review

The official deriveDarkPrintTexSource driver compiled with pinned Tectonic 0.16.9 into a three-page German dark PDF. No production source was changed for the review. The PDF skill was applied; the parent had already recorded the artifact marker.

## Independent PDF Verification

- PDF size: 134,584 bytes; catalog language de-DE; three A4 pages.
- Zero overfull and zero underfull boxes. Every text character remains within its page bounds.
- verify-directed-pdf.py passes: 240 directed cubic base arcs, 20 selected directed arcs, 266 native arrowheads, icon/endpoint clearance, ordered 20-entry ledger, and directed K3 example.
- Inner cycle is precisely 1 → 5 → 10 → 13 → 1. Each selected inner vertex has one incoming and one outgoing arc, the index winding covers exactly one turn, and the final four ledger entries match.
- Dark graph visible height: 126.243 mm (the contrast backing adds approximately 0.64 mm); 53 mm center radius and 170 mm text-width fit are preserved.
- verify-mixed-pdf.py passes: box/pyramid (2.5, 1.5, 2), cylinder (3.5, 2.5, 3), native projected geometry, 20 mm header, all six category captions, numeric cycle and form constraints.
- verify-matrix-pdf.py passes: two separate 9×9 matrices on page 2; 6.14 mm writing rows; icon padding 2.53–2.61 mm; maximum center offset 0.031 mm.
- shared-dark-check.py passes all three pages, language, diagnostic and text bounds assertions.

## Visual Inspection

Rendered through bundled PDFium and Pillow using review-k16-pdf.py. Inspected the contact sheet and all three full-page PNGs. Page 3 shows the four additional highlighted paths as one closed inner loop, with no inner-loop self-crossing and clear arrowheads. The outer selected ring is intact. The 20-entry two-column ledger shows 1 → 5, 5 → 10, 10 → 13, 13 → 1 as its last four entries with a clear center gutter. The full graph and sixteen icon labels fit. Page 1 retains the worked example and edge-local two-line captions without overlap. Page 2 retains both aligned matrices, all sixteen vector icons, and blank parameter rows. Headers, footer logo and page numbering remain clear.

## Retained Inputs and Temporary Outputs

The retained neutral case and verifier are inner-geometry.case.json and inner-geometry.verify.cjs; geometry implementation evidence is in inner-geometry.md. The existing official derivation and verification helpers were reused. All compiled PDFs, logs, auxiliaries, renderer images and isolated cache are under the ticket generated folder. The parent owns generated-output cleanup and final ticket closure.
