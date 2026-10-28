# Same Table Content

Both example tables use the same AssignmentTable environment as the capsule task tables. The existing seven-argument worked-table calls, bilingual introductory text, all six graph captions, cycle statements, three criteria, capsule matrices, and remaining content are byte-for-byte preserved after removal of the two wrapper lines. Source CRLF line endings are preserved.

The only source changes are a localized example wrapper around each existing worked-table call. Sources are frozen at the following SHA-256 hashes before native compilation.

## DE

- Before: 6c48d0b5f438dcb8a65cb3c434689db506ea175824ff80519ecab7fb11e6e07d
- Frozen: a63dad4f88f1c0844b4b61c9cb463100a3b404d53c62c0aa57b94ed5d5a56422
- Readback: 2473 bytes; full original content preserved.

## EN

- Before: f163181945152117051c072a0b9f743b3897467be11db9793d70784a95877b84
- Frozen: c570fe2d8300b8cc42515233e4c3de3bca8e2336a54688760587f96a5c782448
- Readback: 2326 bytes; full original content preserved.

## Native English Verification

The frozen sources compiled through repo tools exec with the pinned native Tectonic 0.16.9 and prepared local TeX resources. The compiler exited 0 and produced three pages, 134,139 bytes, with zero overfull and zero underfull boxes. All extracted text stays inside each page. Frozen bilingual SHA-256 checks passed again before PDF review.

Independent pdfplumber measurements found five header grid edges (four columns), nine horizontal lower boundaries (one header plus eight parameter rows), a 16 mm header, and forty 6 mm vertical row segments (five boundaries across eight rows). The first four rows contain the original Length/Width/Height/Form values; the last four rows contain no text. The numeric columns remain 2.5/1.5/2, 2.5/1.5/2, and 3.5/2.5/3.

All three PDFium-rendered pages were visually inspected at full size. The example shares the exercise table title tabs, outline, column divisions, and row geometry. Its three solids are centered with no header numbers. The six K3 edge captions, cycle, and three criteria remain readable and contained on page 1. The two capsule matrices retain their existing page 2 layout. The K16 graph, twenty marked edges and ledger fit page 3 with no clipped labels.

English PDF SHA-256: 235b6dde17346ea919a9b2eb2e4ae8d647a550969bc80f34b89924b942192915.

Retained verification inputs: same-table-content-check.py and same-table-grid-check.py. Temporary compiler output and full-page PNGs are under the ticket generated directory for parent cleanup.
