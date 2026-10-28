# Mixed-Form Worked Example Geometry

The final worked example uses one box, one rectangular pyramid and one elliptical cylinder. The language-independent fixture mixed-example.json fixes their respective dimensions to (L, B, H) = (2.5, 1.5, 2), (2.5, 1.5, 2), and (3.5, 1.5, 2). Thus only Form changes on 1→2; L and Form change on 2→3 and 1→3.

## Source Design

The shared API is AssignmentWorkedExampleTable with seven arguments (four parameter names followed by three localized form categories), AssignmentWorkedExampleGraph without arguments, and AssignmentWorkedExampleLedger with three complete localized change descriptions. The table reuses the existing AssignmentParameterMatrix component with three type columns, four parameter rows, a 16 mm header, 6 mm rows, and 2 mm padding. Shape headers use a common 0.6 scale. The graph is native K3 with 12 mm vertical center radius and measured horizontal adjustment to the available text width. The ledger has three full-width rows at 6 mm pitch.

All three forms share the same 6 mm drawing unit and oblique projection (x, depth, z) ↦ (x + 0.45 depth, z + 0.35 depth). The pyramid has a rectangular base and an apex above its center, with hidden rear edges dashed. The cylinder top and base are affine projections of a horizontal ellipse with diameters L and B; vertical sides join exact x-extremum tangency points. H remains the extrusion height. These are source-native TikZ paths, with no raster images or runtime library dependency.

The existing box drawing remains. The previous AssignmentWorkedBox API is removed. Capsule macros, the two compact matrices, all K16 edge enumerations, the K16 graph, and its twenty-entry ledger precede the replacement boundary and are unchanged.

## Verification

The retained mixed-geometry-check.cjs reads the language-independent mixed-example.json fixture. Before the implementation it failed on the missing AssignmentWorkedExampleTable contract; after the implementation it passed. It checks all three dimension tuples, the four matrix rows, generic macro names, the three ledger mappings and preserved K16 entry points.

German dark native compilation and visual review passed on the final exactly-one-active constraint version; complete results follow below.

## Changed and Retained Files

Production: 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex.

Ticket inputs: mixed-example.json, mixed-geometry-check.cjs, edit-mixed-geometry.cjs, and this report. All compiler artifacts and rendered review images remain under the ticket's generated folder for final removal by the parent.

## Final German Dark Verification

The final exactly-one-active Form constraint version compiled successfully with native Tectonic 0.16.9. The three-page output is 115,631 bytes, retains de-DE metadata, and has zero overfull and zero underfull boxes.

The independent verify-mixed-dark.py validator uses pypdf and pdfplumber behind the ticket test interface. Reading the same language-independent fixture as the source test, it confirmed one header plus four parameter rows, every continuous numeric value, the three localized categorical values, exactly three native K3 edges, four elliptical curve paths, and all page character bounds.

Existing verify-matrix-pdf.py and verify-k16-pdf.py also passed on the settled dark PDF: both compact nine-column matrices preserve 6.14 mm writing rows and at least 2.53 mm icon padding; the third page retains sixteen canonical vector nodes, 120 unique K16 edges, sixteen highlighted ring edges, four highlighted middle chords, and twenty matching writing entries. Graph outer bounds remain 170.001 × 84.242 mm with 32 mm vertical radius.

The bundled PDFium/Pillow renderer produced the final three-page contact sheet and full-resolution page 1. Visual inspection confirmed recognizable and consistent box/pyramid/elliptical-cylinder drawings in the table and K3, legible four-row parameter values, clear three-row change descriptions, and unclipped header, footer and page boundaries. The two exercise matrices and full K16 also remain visually intact.

Retained final validator: verify-mixed-dark.py. Source and language-independent fixture tests, native compilation, independent third-party PDF checks, and rendered visual review all pass. The parent owns delivery and generated-output cleanup.
