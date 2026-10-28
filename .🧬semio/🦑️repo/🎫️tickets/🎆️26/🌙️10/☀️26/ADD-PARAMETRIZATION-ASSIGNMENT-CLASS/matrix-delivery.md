> Superseded by compact-delivery.md: both compact matrices now share page2; the final PDF has four pages and centered icons with measured uniform padding.

# One Exercise With Two Transposed Matrices

Delivered 2026-10-27: `output/pdf/Fingeruebung-Parametrisierung.pdf`.

The assignment now contains one shared exercise for all sixteen original Metabolism types. Tables A and B use the same parameter names and row order. Each transposed matrix has exactly nine columns and nine total rows: the header contains Parameter and eight unnamed vector icons, followed by eight blank parameter rows. Names belong in the first column; values go below the icons. Both tables occupy consecutive pages before their two K8 graph/ledger pages. Every graph number follows the corresponding table's type columns from left to right.

## Artifact And Actual Verification

- Final PDF:84,479 bytes; SHA-256 `e4c18cf79176058387e60d6d5607d8d4269c8e6c8ce05fad9c3585634c259d8f`.
- Five A4 pages: example, Table A, Table B, graph/ledger A, graph/ledger B.
- Actual LaTeX print product, `assignment` class and original design tokens/fonts/chrome retained.
- Native metadata: Fingerübung Parametrisierung, Architektur, de-DE, LaTeX with hyperref, xdvipdfmx.
- No overfull or underfull diagnostics. PDF.js reports no out-of-page text. The parent individually inspected all five latest German page images, including both complete matrix grids and graph ledgers.
- Independent pypdf/pdfplumber inspection confirms ten aligned grid borders in each direction per table, nine rows and columns, eight vector icon header cells and eight empty parameter rows. No obsolete Übung1/2 or Exercise1/2 remains.
- English light and German dark also compile to five pages, pass the actual 9×9 matrix checker and metadata/log checks, and were rendered and visually reviewed.

## Tests And Concurrent Source State

The generic assignment suite passes seventeen tests after a meaningful missing-matrix red run. The version3 language-neutral content fixture describes one exercise, two matrices and a shared strategy. AJV rejects five malformed structures; mathjs reproduces the box/cycle calculations, d3-array reproduces all graph pairs and81 cell roles, and lxml plus PDF/hash checks reproduce the sixteen canonical icon sources.

Initial native compiles encountered an undefined shared navbar alias while concurrent source work was settling. Fresh diagnostics and final compiles succeeded against the current original shared sources without any shared-navbar stylesheet edit or fallback. The retained diagnostic input imports original geometry/content rather than copying tracked source. There is no new application runtime dependency. The source admission issue affecting the full managed build is unchanged; this report claims the completed registered-Tectonic document compiles.

See `matrix-layout.md`, `single-exercise.md`, `matrix-audit.md`, `matrix-english.md`, `matrix-dark.md` and `matrix-navbar-fix.md` for exact checks and retained inputs.

Temporary compiler output, caches, generated drivers, PDFs and review PNGs are removed after successful verification. Authored input scripts, diagnostic source, Markdown reports, original source assets and the final output PDF remain.
