# Matching Worked-Example Table

The worked example now uses the same AssignmentTable frame, numbered title, centered icon header, borders, padding and eight parameter rows as the task tables. Its three example columns retain the four filled L/B/H/Form rows; four additional rows are blank. Header numbers were removed to match the unnamed task headers.

The shared header is 16 mm high, writing rows are 6 mm, padding is 2 mm and the label column is 27 mm. Native worked shapes use a common 0.5 scale so the tallest cylinder fits without changing proportions. The example is numbered Table 1.1.a; the task tables retain 1.2.a/b.

An expandable AssignmentParameterMatrixBlankRows command supplies the blank rows in both the example and the task matrices through the existing shared grid. The numeric values, six K3 labels, both K16 cycles, twenty highlighted directions and three-page layout remain unchanged.

## Verification

- Version-twelve schema, neutral fixtures and differential checks pass. All eighteen generic tests pass with 75 assertions.
- Native PDF checks verify the four-column, nine-row example grid, four exact filled rows, four entirely blank rows, unnumbered header and common 0.5 shape scale. Its ruled header interval matches both task headers within 0.02 mm after accounting for border strokes.
- Both page-two nine-by-nine matrices, all directed graph paths and native table numbering pass their retained audits.
- German light, English light and German dark native builds pass. All final German pages were rendered and individually inspected; English and dark pages were also reviewed. The final German log has no overfull, underfull or missing-character errors.
- K3 labels and all three criteria remain readable on page one. The task matrices and taller K16 with twenty answer rows still fit on pages two and three. Frozen content hashes remain unchanged.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages, 134586 bytes
- SHA-256: 2e17efa6e4b9a3f640d9a59333c70733a6b033f07c1b1ea216960442bd901890

Authored inputs and audit reports are retained. Temporary build outputs and renders are removed after verification. Original resources, existing styling and concurrent work are preserved.
