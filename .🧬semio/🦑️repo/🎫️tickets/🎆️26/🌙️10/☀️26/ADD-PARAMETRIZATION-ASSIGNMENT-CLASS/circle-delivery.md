# Closed Ring and Taller K16

The highlighted ring now runs continuously 1→2→…→16→1. Its previously reversed closing arc 1→16 has been changed to 16→1, including the matching answer label. All 240 directed arcs remain visible; exactly twenty directions stay highlighted, with the same four inner chords.

The K16 vertex spacing uses a 53 mm vertical radius. Its measured visible height is 125.603 mm, up from approximately 83.6 mm: +50.24%. Width remains approximately 170 mm. Canonical icons keep their original size and proportions. The K3, its six labelled directions, and both compact capsule matrices are unchanged.

All content still fits on three A4 pages. Both matrices remain together on page two; the larger K16, twenty writing rows and closing prompt fit on page three with clear separation from the footer. Bilingual instructions remain valid without added text.

## Verification

- Test-first neutral geometry case and version-nine schema/fixtures pass. Independent D3/mathjs checks traverse the complete directed ring and reject the old flipped closing arc. Schema mutations also reject the old 32 mm radius.
- Actual German PDF audits verify all 240 ordered arcs, twenty selected directions, their native arrowheads, the 16→1 closure and ordered ledger, 53 mm radius, full width and +50.24% visible height.
- Retained mixed-example, category/numeric cycle, vector provenance and two 9-by-9 matrix checks pass. Writing rows and ledger gutter are preserved.
- Native German light, English light and German dark builds pass. The dark graph measures 126.243 mm, approximately +51%, with its existing contrast panels. Final German log has no overfull, underfull or missing-character errors.
- All three final German pages were rendered with PDFium and individually inspected. English and dark renders were also reviewed without clipping, overlaps or an extra page.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages, 133408 bytes
- SHA-256: d32f94b7a1fd16d90727c69e5122e866bacb9a300223f07f35cedc89c908d830

Authoring scripts, neutral inputs and audit reports remain in this ticket. Temporary compiler and render outputs are removed after verification. Original icons, shared print chrome and concurrent repository work are preserved.
