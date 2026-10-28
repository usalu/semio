# Shared Bilingual Example Content and English Native QA

Both content sources retain the original heading tree, continuous positive-real L/B/H domain, categorical Form domain, cycle explanation and Unique/Consistent/Minimal examples. The table uses identical dimensions (L,B,H)=(2.5,1.5,2) for samples 1 and 2 with different Form; sample 3 uses (3.5,2.5,3), incrementing all three dimensions by one. Four numeric K3 captions show complete ±(1,1,1) vectors and the associated Form change. The six-argument edge order remains 1→2, 2→1, 2→3, 3→2, 1→3, 3→1.

The initial single-line caption probe showed text reaching shape nodes. Following native evidence, both languages now use a two-line shortstack with the vector first and Form transition second, preserving the native 8.5pt caption font. No capsule, heading, K16 or ledger content changed.

A probe found malformed boundaries in the English source. Its complete original content was re-authored into a new temporary file and atomically replaced. Every later edit likewise authored a complete temporary file and atomically replaced its target. All expected sections, criteria and explicit page breaks were checked after writing.

## Frozen Content

| Source | SHA-256 |
| --- | --- |
| German | 6c48d0b5f438dcb8a65cb3c434689db506ea175824ff80519ecab7fb11e6e07d |
| English | f163181945152117051c072a0b9f743b3897467be11db9793d70784a95877b84 |

The English hash remained identical after final native compilation.

## English Native Compilation and Independent PDF Checks

The original English entry file compiled successfully through the registered Tectonic interface, using the prepared compiler-input search directory, the pinned 0.16.9 executable and encoded local bundle URL. Checked sources were compiled in place; none were copied into the ticket.

| Check | Result |
| --- | --- |
| Pages | 3 |
| PDF bytes | 133642 |
| Title / author | Finger Exercise: Parametrization / Architecture |
| Catalog language | en-US |
| Overfull / underfull boxes | 0 / 0 |
| Embedded fonts | All 9 PDF font resources |
| Capsule matrices | Both on page 2; each 9×9 |
| Blank writing-row height | 6.14mm |
| Capsule artwork minimum padding | 2.13–2.23mm |
| Directed complete graph | 240 cubic arcs, 20 selected directions |
| Native arrowheads | 266 including worked K3 |
| Complete-graph visible height | 125.603mm |
| Ledger | All 20 directed entries in canonical order |

The independent mixed-form verifier passes against the actual final English PDF. Its owner corrected the worked-table header expectation to nominal 20mm plus the 0.4pt matrix rule (approximately 0.14058mm); the measured header is 20.14008mm. The verifier also confirms all sample values, independently projected native shape geometry, categorical rows, six K3 arcs and vector/cycle captions.

## Full-Page Visual QA

All three final pages were rendered with PDFium and inspected at full image size.

Page 1 shows the correct three sample geometries and every numeric value. All four vector captions and six full categorical transitions remain readable and clear of shape nodes. The cycle, uniqueness collision, inconsistency and one-hot minimality examples are complete.

Page 2 retains both complete capsule matrices, uniform vector artwork padding and eight blank writing rows each.

Page 3 shows the closed 16→1 ring, twenty selected arrows, all ledger rows and final cycle prompt. The larger graph, text, header and footer fit cleanly. No glyph or artwork is clipped. Text extraction found no characters outside a PDF page; mathematical glyph overhang and the rightmost vertex label remain within the visible page gutter.

Temporary outputs are under the ticket generated directory: compiled-en/parametrization-en.pdf and review/shared-en-1.png through shared-en-3.png, plus the contact sheet and machine-readable review JSON. The parent owns final delivery and generated-output cleanup.
