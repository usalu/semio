# Compact English PDF Verification

## Native Compile

The original English-light parametrization-en.tex compiled successfully through registered Tectonic 0.16.9 with the exact executable override, encoded bundle URL, shared ticket cache and search paths recorded in latex-variants.md. The input remained in the original assignment template directory. No checked source, font, icon, class or package was copied into the ticket.

## Independent PDF Checks

The retained verify-matrix-pdf.py passed against the actual compiled PDF using bundled pdfplumber and pypdf.

| Property | Actual result |
| --- | --- |
| Page count | 4 |
| Matrix placement | Both A and B on page 2 |
| Matrix shape | Two disjoint 9 by 9 grids |
| Blank writing rows | Eight per matrix, without text or artwork |
| Writing-row height | 6.14mm |
| Artwork minimum padding | 2.13 to 2.22mm across all sixteen icons |
| Maximum horizontal artwork-center offset | 0.008mm |
| Maximum vertical artwork-center offset | 0.004mm |
| Title | Finger Exercise: Parametrization |
| Author | Architecture |
| Catalog language | en-US |
| PDF size | 80,792 bytes |
| Overfull / underfull boxes | 0 / 0 |

The checker confirms native vector artwork in each header cell, uniform minimum padding across both matrices, a four-page document and no matrix spilling into the first graph page.

## Visual Review

The retained review-pdf.mjs rendered all four pages with PDF.js and canvas. It reported outside=[] for extracted text. The contact sheet and full-resolution page 2 were inspected.

Both tables fit together comfortably on the second page. The sixteen full icons are sharp and centered, their surrounding padding is visually consistent, and each blank writing row has a compact single-line height. The shared instruction paragraph, table headings and final reconstruction check remain legible. The first page preserves the worked boxes and criteria. Each graph page preserves eight icon vertices, the complete graph and all twenty-eight ledger entries. Headers, footers and page transitions show no clipping or collisions.

## Temporary Outputs

- 🗑️generated/compiled/parametrization-en.pdf
- 🗑️generated/compiled/parametrization-en.log
- 🗑️generated/review/parametrization-en-contact.png
- 🗑️generated/review/parametrization-en-1.png through parametrization-en-4.png
- 🗑️generated/review/parametrization-en.json

All generated files remain under the ticket generated directory for parent review and cleanup. The report is retained. No source edits were required during English compilation or visual review.
