# Compact German Dark Regression

The settled German source was derived through the official deriveDarkPrintTexSource implementation and compiled with registered Tectonic 0.16.9, the existing encoded local bundle, the four original product search paths and the ticket-local compiler cache documented in latex-variants.md. No checked source or resource was copied. The generated driver and all verification outputs remain under this ticket's generated directory for parent cleanup.

## Actual PDF Checks

The compiler exited successfully. The PDF contains 4 A4 pages, is 82,131 bytes and declares title Fingerübung Parametrisierung, author Architektur and catalog language de-DE. The final log contains zero overfull and zero underfull boxes. The retained check-matrix-dark.py expectation was updated from five pages to four.

The independent PDF geometry checker verifies both nine-column, nine-row matrices on page 2. All eight writing rows in each table measure 6.14 mm including their grid rules. Original vector artwork is centered within 0.3 mm, and its minimum cell inset ranges from 2.53 to 2.61 mm across all sixteen headers. The dark backing remains within the bounded frame. All answer rows are blank.

## Visual Review

The retained review-pdf.mjs helper rendered all four pages with PDF.js and canvas. It reported no extracted text outside any page. The contact sheet and full-resolution second page were visually inspected: both matrices share page 2, header artwork has balanced opposing margins, each writing row accommodates a single line, and labels and grid lines are legible without overlap or clipping. Both unchanged graph pages and their complete answer ledgers fit within the page chrome.

The page order is worked example, both matrices, graph A and graph B. The original sixteen vector PDF resources remain unchanged and preserve their proportions.

## Commands

- repo tools exec --light python -- <ticket>/check-matrix-dark.py
- repo tools exec --light python -- <ticket>/verify-matrix-pdf.py <ticket>/🗑️generated/compiled/parametrization-de-dark.pdf
- repo tools exec --light node -- <ticket>/review-pdf.mjs <ticket>/🗑️generated/compiled/parametrization-de-dark.pdf

Python used the bundled runtime through SEMIO_TOOL_PYTHON; the existing reviewer used the bundled Node package directory through NODE_PATH. The parent owns final delivery and removal of generated compiler, rendering and cache files.
