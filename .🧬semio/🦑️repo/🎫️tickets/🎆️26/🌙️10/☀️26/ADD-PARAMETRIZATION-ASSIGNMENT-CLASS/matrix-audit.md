# Matrix Correction Audit

## Scope

Audit the correction to one exercise, two large parameter matrices, nine rows per matrix, a first parameter column and eight icon columns, and one parameter strategy across all sixteen types. Root owns actual LaTeX compilation and PDF image inspection.

## Adjacent Course Metadata

The architecture course manifest `🎓️teaching/🏛️architecture/📚️course/🔣️.json` uses a generic bilingual parametrization description and language-specific published PDF references. It makes no claims about exercise, table, column or row counts; no change was needed. Searches of print/course source excluded generated output, installed dependencies and unrelated visualization scalar grids.

The print README describes the existing generic `AssignmentParameterTable` with five writable columns; that API remains unchanged, so this is accurate rather than obsolete. By coordination with the matrix author, added adjacent documentation for `AssignmentParameterMatrix`: eight icon columns plus parameter column, eight blank parameter rows plus header, and one exercise containing two matrices for all sixteen types.

## Changed Files Owned By This Audit

- `🧰️framework/🛍️products/📓️print/README.md`
- This audit report.

## Source Audit Status

Waiting for final matrix/geometry and bilingual content changes before final source review. No implementation tests are duplicated here.

## Reusable Matrix Source Review

The settled `semio-assignment.sty` defines a distinct `AssignmentParameterMatrix` with positive dimensions, eight type columns and eight parameter rows by default, 15 mm row heights, and 27 mm label width. Its width calculation includes all nine cells, eighteen cell paddings and ten vertical rules. The shared `parametrization-geometry.tex` explicitly requests 8 by 8 dimensions and builds the header from each canonical eight-type set. Matrix icons fit within 14 mm width and 11 mm height; graph icons retain their separate 22 mm width. The existing assignment class delegates to the existing semio class. No style/font/token/theme replacement was introduced.

## Independent Compiled PDF Check

Retained `verify-matrix-pdf.py` uses existing third-party pdfplumber and pypdf rather than production matrix internals. It expects five pages with matrices on pages 2 and 3, ten full-width horizontal borders, ten full-height vertical borders, eight blank data rows without text or icon curves, and vector artwork in every icon header cell. It checks native title/language metadata and obsolete numbered-exercise text. The previous published PDF was absent, so no meaningful pre-change PDF red result is claimed; source TDD belongs to the implementation agents. Actual final PDF verification is pending compilation.

## Settled Bilingual Content Review

Reviewed both settled language content files. Each has exactly one exercise section, two table subsections before two transition subsections, explicit shared parameters/rules across all sixteen types, and identical row naming/order between tables. The instructions place parameter names in the first column and values under header shapes, allowing unused parameter rows. Graph references now explicitly count shape columns left to right, 1–8, scoped separately to A and B. Shared geometry enumerates every pair once for each K8 and each 28-entry ledger. No obsolete independent second exercise or row-based graph instructions remain. No source issues found.

## Actual Final German PDF Verification

The actual compiled German PDF passed the retained independent checker: five pages, native title metadata and de-DE language, no numbered Exercise 1/2 or Übung 1/2 text, and both page-2/page-3 matrices each have nine columns and nine rows, eight vector icon header cells and eight entirely blank parameter rows. Size: 84479 bytes. SHA-256: `e4c18cf79176058387e60d6d5607d8d4269c8e6c8ce05fad9c3585634c259d8f`. Compiler overflow/underflow findings: none.

The first extraction grouped page chrome rules with identical matrix-width horizontal rules and incorrectly counted fifteen rows. Inspection showed actual table vertical rules are segmented per row. The verifier now finds the connected matrix through adjacent horizontal borders joined by aligned vertical segments, retaining strict ten-border/nine-cell checks in each direction. This fixes extraction identification without weakening the geometry requirement. Source audit and actual PDF checks found no implementation issues. Parent owns visual inspection of all rendered pages.
