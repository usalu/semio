# Native Course PDF QA

Both canonical language sources compile to three A4 pages and pass independent PDF inspection.

| Language | Pages | PDF Bytes | Maximum Header Center Offset X / Y | Minimum Capsule Inset | Overfull / Underfull |
| --- | --- | --- | --- | --- | --- |
| en | 3 | 1537138 | 0.0010 / 0.0312 mm | 2.0713 mm | 0 / 0 |
| de | 3 | 1537335 | 0.0010 / 0.0315 mm | 2.0711 mm | 0 / 0 |

- English PDF SHA256: 33a319248959dad0e7b1e26ac4c5314e8ead34fb49b00c1703dcbf67cb755246.
- German PDF SHA256: aaae52671dc2865143d548a7abac616bba3717c52302398feea82f70d7f9d813.

Both documents have the expected language/title metadata, no out-of-page glyphs, two compact 9×9 matrices on one page and sixteen canonical 2048-pixel capsule placements in the tables and graph. Root visual inspection of the German pages and English contact sheet / full graph confirms the approved layout, centered cells, complete directed graph, native silhouette chips and transition ledgers.

## Method

Retained inputs prepare-course-qa.ts and verify-course-qa.py stage only into the ticket generated folder. Preparation uses the production printLibrarySources inventory, printCompilerName, printSourceReferenceMapV1, printSourceReferenceTextV1 and renderPrintSvgResourceV1. All sixteen SVG hashes and all 205 consumed source bindings were rechecked against workspace bytes after compilation.

Registered repo tools exec tectonic selected the existing pinned Tectonic 0.16.9 through its tool override and the existing pinned local TeX bundle. PDF checks used pypdf, pdfplumber and PDFium through the registered Python tool, with bytecode disabled. These verification-only runtime overrides are absent from production.

An initial canonical-image pixel-limit failure exposed during this check was fixed in the reusable renderer. No layout or source-asset workaround was introduced.

## Boundary

These were native QA compiles. No artifact receipt or retained capability was fabricated. Managed build/publication remains blocked before target execution by repository source admission (see integration-delivery.md). All generated source copies, PNG resources, fonts, PDFs, logs and rendered pages are disposable and removed at ticket completion. Only authored verification inputs and reports remain.
