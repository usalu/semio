# K16 Independent PDF Audit

## Current User Steering

Retain the full K16 base graph: sixteen canonical capsule icons, all 120 unordered pairs, one ellipse at full text width, and the old vertical size. Select only twenty parameter changes: all sixteen outer-ring transitions plus four middle chords. Only those twenty selected transitions receive blank ledger entries, in two columns on page 3. The separate continuation page is removed; the final document has three pages and both compact matrices remain together on page 2.

The four random middle chords use the settled reproducible seed 20261027 and are (1,10), (2,12), (4,10), and (5,13). The graph draws the full base at 0.35pt and the twenty selected edges at 0.8pt.

## Baseline Measurement

Measured the previous published German PDF before replacement. Its K8 graph center coordinates span y=166.71843 to 348.13713 PDF points, exactly 64.000mm (32mm radius). Its icon node rectangles span 74.80058mm. Including outer numeric vertex labels, the graph spans y=140.01429904 to 376.99660904 points, or 83.60209mm. The approximate 76mm description excludes some visible numbering. Preserve the actual overall visible height and original vertical radius.

## Retained Verification

Updated verify-matrix-pdf.py to require three pages while retaining two separate 9 by 9 matrices on page 2, blank single-line parameter rows, centered vector header artwork, uniform minimum padding, and native metadata.

The new verify-k16-pdf.py uses independent third-party PDF parsers. It identifies sixteen degree-15 base edge endpoints, validates all 120 straight unique segments against K16, verifies common ellipse coordinates and 32mm vertical radius, checks the visible graph against full 170mm text width and measured old height, compares graph forms in canonical order against the sixteen matrix forms, and recursively rejects raster artwork within those canonical form resources. Concurrent page chrome imagery is outside that graph artwork assertion.

It additionally verifies twenty distinct stronger segments: sixteen outer-ring edges and the exact four selected middle chords. Twenty unique blank ledger labels must match those highlighted graph segments exactly. No fourth page or continuation ledger is allowed.

## Red Verification

Before the latest three-page steering, ran the initial K16 checker against the previous published K8 document. It correctly failed because only eight graph form resources existed. Final PDF verification is pending compilation.

## Adjacent Documentation

The print README contains no obsolete K8-only claim. Its generic assignment and matrix documentation remains accurate; no edit is necessary.

## First Actual Candidate Findings

The first successful German three-page PDF passed both compact matrix checks: writing rows 6.14mm, minimum artwork padding 2.13–2.23mm, maximum artwork center offset 0.030mm. The new page chrome changes the header rule's half-stroke endpoint distance to 0.37pt, so matrix border connection detection now allows 0.5pt; strict cell count, alignment, icon centering and padding checks remain unchanged.

The graph passed canonical sixteen vector resources, global numeric labels, all 120 straight unique base edges, exact twenty selected edges, 32mm vertical radius and 83.60209mm overall visible height. Two real geometry issues were reported for correction: visible width was only 165.18mm instead of 170mm, and first-column ledger lines overlapped long second-column labels by approximately 1mm. Final verification remains pending those corrections.

## Final Actual German PDF Verification

Both retained strict verifiers passed the final three-page German PDF. The graph uses all sixteen canonical matrix resources in global order, each global numeric vertex label 1–16 exactly once, all 120 unique straight base edges, and precisely twenty selected strong edges: all sixteen outer-ring transitions plus (1,10), (2,12), (4,10), and (5,13). The twenty blank ledger entries match those selected edges exactly. The corrected visible graph measures 170.001 by 83.602mm, preserving the original 83.60209mm visible height and 32mm vertical radius. Both prior geometry findings are resolved.

Both page-2 matrices contain nine columns and nine rows; eight blank writing rows measure 6.14mm each. Minimum vector artwork padding is 2.13–2.23mm and maximum center offset is 0.030mm. Three A4 page boxes, de-DE language and native assignment title metadata are present. All six used fonts are embedded. The compiler log contains no overfull boxes, underfull boxes, or missing-character warnings.

Final candidate size: 114937 bytes. SHA-256: 21470ff6110f23ee9fc612d32bee545d7e03ad1dbb11ab75a9453814c59d02d5.

Root owns visual review of all three rendered pages and publication. Root also reports independent final English and dark variants passed: English 170.001 by 83.602mm and dark 170.001 by 84.242mm, both with 32mm vertical radius. The dark height remains within the 1mm allowance against the original visible size.
