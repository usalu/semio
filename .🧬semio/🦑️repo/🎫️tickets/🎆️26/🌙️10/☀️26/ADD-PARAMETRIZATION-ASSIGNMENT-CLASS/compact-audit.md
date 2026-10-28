# Compact Matrix Geometry Audit

The retained `verify-matrix-pdf.py` checker independently measures PDF vector curves and grid edges using pdfplumber, with document metadata checked through pypdf.

## Required Output

- Four pages total.
- Two vertically disjoint 9-column, 9-row matrices together on page 2.
- Eight blank writing rows per matrix, each 5.8–6.7 mm high.
- Artwork bounds centered within 0.3 mm of each header cell center on both axes.
- Every artwork minimum inset at least 1.8 mm, with a spread below 0.35 mm across all sixteen icons.
- Original vector artwork in every type header; no artwork or text in writing rows.

## Resource Measurements

All measurements below are PDF points; gutters refer to actual vector curve bounds relative to the MediaBox. These are independent of the eventual placement within cells.

| Resource | Canvas | Left | Top | Right | Bottom |
| --- | --- | ---: | ---: | ---: | ---: |
| Capsule, all eight orientations | 131.846 × 128.999 | 0.709 | 1.599 | 0.709 | 1.658 |
| Ellipsoid capsule backslash | 444.396 × 355.698 | 2.835 | 2.825 | 3.341 | 3.086 |
| Ellipsoid capsule J | 408.372 × 395.650 | 1.984 | 1.974 | 2.460 | 2.227 |
| Ellipsoid capsule L | 442.721 × 338.322 | 1.984 | 2.216 | 2.517 | 2.228 |
| Ellipsoid capsule p | 410.123 × 397.351 | 2.835 | 2.825 | 2.835 | 3.077 |
| Ellipsoid capsule q | 444.415 × 355.690 | 2.835 | 2.825 | 2.835 | 3.078 |
| Ellipsoid capsule s | 444.415 × 340.038 | 2.835 | 3.074 | 2.835 | 3.086 |
| Ellipsoid capsule slash | 408.422 × 395.657 | 1.984 | 1.974 | 1.984 | 2.234 |
| Ellipsoid capsule z | 410.123 × 397.350 | 2.835 | 2.825 | 2.835 | 3.077 |

Resource canvas centers and artwork centers are already nearly coincident. At a 20 mm displayed width, the maximum measured horizontal bias is approximately 0.012 mm. Final validation must nevertheless inspect placed vector artwork, because centering the resource canvas alone does not prove cell centering.

## Validation Status

The final compiled German PDF passed all assertions. It has exactly four pages and both full matrices on page 2. Every writing row is 6.14 mm high. All sixteen artwork minimum insets are 2.13–2.22 mm. Maximum horizontal/vertical artwork center deviations are 0.008/0.004 mm in matrix 1 and 0.008/0.003 mm in matrix 2. Native document language and title metadata passed.

Grid discovery allows 0.3 pt between horizontal rule centers and vertical segment endpoints, accommodating half the 0.4 pt stroke width. This tolerance affects edge association only; the artwork geometry remains subject to the independent 0.3 mm centering limit.
