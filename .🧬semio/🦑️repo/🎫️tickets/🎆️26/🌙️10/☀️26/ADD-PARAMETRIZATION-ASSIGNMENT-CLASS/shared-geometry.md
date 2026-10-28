# Shared-Dimension Worked Example Geometry

The language-independent shared-geometry.case.json fixes box and pyramid to (L, B, H) = (2.5, 1.5, 2), and the elliptical cylinder to (3.5, 2.5, 3). Each dimension has two equal values and one different value; the first pair differs only by Form. The existing continuous domains, shared oblique axes, and native three-form drawings remain.

## Test-First Contract

shared-geometry.verify.cjs reads that fixture and checks the three native shape calls, all four table rows, the unchanged seven-argument table and six-argument graph interfaces, exact cylinder tangency formula, six directed K3 arcs, and preserved 53 mm K16 radius. Its expected table header height is 20 mm to retain 2 mm padding around the taller common-scale cylinder.

Native compilation and PDF review are coordinated by the parent agent. The independent product auditor owns third-party PDF validation.

## Source Verification

Before editing production geometry, the shared contract failed on the cylinder dimensions as expected. After updating the cylinder and B/H table rows, the same contract passed. The worked-example matrix now uses a 20 mm header; both capsule matrices retain 16 mm headers. Only four production replacements were needed. The cylinder uses the existing native affine ellipse projection and exact tangent sides, so the new dimensions retain the same drawing axes as the box and pyramid.

## Final German Dark Verification

After the localized two-line K3 captions were frozen, the retained k16-dark-derive.cjs used the official deriveDarkPrintTexSource implementation to create the dark driver. Registered Tectonic 0.16.9 compiled it successfully against the repository's pinned bundle and compiler-input resource projection. The final PDF is 134,621 bytes, three A4 pages, with native de-DE language metadata, zero overfull and zero underfull boxes, and no text outside page bounds.

Independent third-party checks passed through the ticket's updated verify-mixed-pdf.py, verify-directed-pdf.py, and verify-matrix-pdf.py. They confirm the three requested tuples, four parameter rows, native projected box/pyramid/cylinder dimensions, the 20 mm worked-example header, six curved K3 arcs with edge-local Form captions, 240 K16 directed arcs plus 20 selected directions, 266 native arrowheads, clear endpoints, the selected ring and four chords, and the ordered twenty-entry ledger. The actual dark K16 visible height is 126.243 mm (1.51008 times the old 83.6 mm reference). Both exercise matrices retain 6.14 mm writing rows, 2.53–2.61 mm icon minimum padding, and maximum center offsets of 0.031 mm.

The retained review-k16-pdf.py rendered all three final pages with bundled PDFium and Pillow. The contact sheet and full-resolution pages were inspected: the taller cylinder fits the common-scale header, all six K3 captions remain fully visible, the table and graph show the same three forms, both matrices are clear, and the full K16/ledger fit without clipping. Direct pixel comparison additionally confirmed identical navbar and footer-left artwork across all three rendered pages.

Retained inputs are shared-geometry.case.json, shared-geometry.verify.cjs, and shared-dark-check.py. The latter verifies dark metadata, three pages, zero layout warnings, and every PDF character's page bounds. Generated drivers, native build products, cache, and rendered PNGs are under the ticket's generated folder for parent-owned removal. No production changes were made after the source freeze.
