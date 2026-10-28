# K16 English Native PDF Verification

The original English assignment compiled successfully through registered Tectonic 0.16.9 after the final geometry and ledger-gutter changes. It uses the canonical derived compiler-reference input prepared by the parent, with that input directory first and last in the search-path list, the original absolute English main file, and no checked source or icon copies. Concurrent course, custom header, footer logo and AssignmentTable changes are preserved.

## Final Native Result

| Property | Result |
| --- | --- |
| Pages | 3 |
| PDF bytes | 114,182 |
| Title | Finger Exercise: Parametrization |
| Author | Architecture |
| Catalog language | en-US |
| Overfull / underfull boxes | 0 / 0 |
| Out-of-page extracted characters | None |
| Graph width | 170.001 mm |
| Graph height | 83.602 mm, preserved |
| Ellipse vertical radius | 32 mm |
| Canonical vector graph icons | 16, in matrix resource order |
| Complete graph base edges | 120 distinct straight segments |
| Marked edges | 16 distinct ring edges and 4 seeded middle chords |
| Blank ledger entries | 20, exactly matching marked edges |

## Independent Verification

Both native PDF validators passed the final compiled English PDF. The K16 validator independently reconstructed all sixteen degree-15 vertices, all 120 unordered pairs, the common ellipse equation, global labels, exact highlighted edge set and matching blank ledger. It also verified canonical vector resources, resource order, full text width, preserved graph height and native metadata.

The matrix validator independently found two disjoint 9 by 9 matrices on page two. Both retain eight blank writing rows at 6.14 mm. Icon minimum padding spans 2.13–2.22 mm; maximum center offsets remain 0.008 mm horizontally and 0.031 mm vertically. There is no matrix spill into the graph page.

The registered Bun source verifier also passed after the final geometry changes, including AJV mutations, d3-array complete graph and matrix oracles, the independent seeded shuffle, and mathjs mathematical checks.

## Visual Review

The retained render-k16-en.py uses the bundled PDFium renderer behind registered repo Python. It rendered all three pages, made a contact sheet, and independently checked metadata and page text using pypdf and pdfplumber. PDF.js exited without diagnostics on this host; Poppler is bundled but unregistered in the repo tool catalogue.

The final contact sheet and full-resolution third page were inspected. The first page preserves the worked example and three criteria; both compact matrices fit on page two. All sixteen sharp canonical icons are numbered once around a full-width horizontal oval on page three. The complete graph is faint, while the sixteen ring edges and four middle chords are clear. The twenty writable rules have distinct labels and comfortable gaps between columns. Reflection text and two response lines fit below them. Headers, footer logo, page numbers and page transitions show no clipping or collisions.

## Retained and Temporary Files

Retained input script: render-k16-en.py.

Temporary native PDF, TeX intermediates and log remain under the ticket generated/compiled directory. Final PDFium page images, contact sheet and metadata JSON remain under generated/review for parent inspection and cleanup.
