# German Dark Matrix Regression

The German dark driver was regenerated from the settled canonical German driver with the product's official deriveDarkPrintTexSource implementation. The registered Tectonic 0.16.9 compiler used the encoded local bundle and four official search paths documented in latex-variants.md. All generated compiler and review files remain under this ticket's generated directory for parent cleanup.

## Actual Compile

The settled-source retry completed successfully. The actual PDF is 85,033 bytes, contains exactly five A4 pages, has title Fingerübung Parametrisierung, author Architektur and catalog language de-DE. The final TeX log contains zero overfull and zero underfull box diagnostics.

The retained check-matrix-dark.py helper independently verifies these properties with the existing bundled Python runtime and pypdf. verify-matrix-pdf.py additionally verifies both actual table pages: each has nine columns, nine rows, eight original vector icon headers and eight blank parameter rows. Data rows contain no prefilled text or artwork. No obsolete exercise numbering appears.

## Visual Review

The retained review-pdf.mjs rendered all five pages with PDF.js and canvas and reported outside=[] for extracted text. The resulting contact sheet and full-resolution second page were visually inspected. Both large matrices fit within their pages; the first column accepts parameter names, the remaining columns align below the eight icon headers, and all eight answer rows remain blank. The table grid, canonical vector icons on light pads, subsection bars, page chrome and footer fit without clipping or overlap. Both graphs and complete ledgers also fit their pages.

The five-page order is worked example, Table A, Table B, transitions for Table A, transitions for Table B. One exercise section owns all table and graph subsections. The graph numbers remain local to their corresponding table columns.

## Initial Concurrent Snapshot

An initial run failed before emitting pages because the shared semio-window.sty begin-document hook observed an undefined SemioChromeNavbarLabelsCompute alias. The later shared source contained the proper definition, and fresh probes of the unmodified originals passed. No product workaround, fallback, alias repair or shared-source edit was made for this diagnostic. The exact same derived dark driver compiled successfully on the settled-source retry.

## Retained Inputs and Temporary Outputs

The retained new helper is check-matrix-dark.py. The report is matrix-dark.md. The temporary PDF, TeX log, generated dark driver, rendered pages, contact sheet and extraction JSON are confined to the ticket's generated directory. The parent owns final delivery and cleanup of those generated outputs.
