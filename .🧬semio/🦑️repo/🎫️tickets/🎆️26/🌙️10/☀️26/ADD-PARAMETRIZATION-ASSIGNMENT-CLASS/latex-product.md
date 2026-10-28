# LaTeX Product Integration

The actual assignment sources now use the literal `assignment` document class. The new class delegates to the existing `semio` class with `type=assignment`, so fonts, design tokens, theme and page chrome remain the print product's own implementations. Both language mains and the plain-text preamble emitter select this class. No replacement page document was created with ReportLab.

The sixteen symbols in the assignment are the canonical Metabolism SVG assets named in `parametrization-icons.json`. A retained ticket-only authoring script reads the original SVG bytes, verifies each SHA-256, converts the vector geometry with svglib/reportlab, normalizes physical page dimensions to the SVG's point dimensions, and writes vector PDF assets under the actual product template's `resources/` directory. Each PDF carries `SourceSVG` and `SourceSHA256` metadata. This conversion is an authoring operation; no production runtime dependency was added.

Both registered document source lists explicitly include all sixteen derived PDF resources. The existing print source staging interface therefore retains, copies, hashes and rechecks these actual document inputs through its ordinary bounded source lifecycle.

## Checks

The icon verifier was first run before resource creation and failed on the missing first PDF. After conversion and catalog registration, independent pypdf inspection passed all sixteen source SHA-256 values, PDF source metadata, exact physical SVG-to-PDF page dimensions, nonempty vector path operations, absence of raster image XObjects, and membership in both document source lists. The product's content test now also verifies every derived PDF header and source-hash metadata against the original canonical SVG fixture.

The assignment schema/AJV suite passed 13 tests after the public preamble switched to `assignment`. The shared content/mathematical/original-SVG/PDF-provenance verification passed after the actual language mains switched to the new class.

The parent ticket owner runs and visually reviews the actual LaTeX compiler output. This report does not claim a completed compile before that parent verification.

## Exact Files Changed In This Continuation

- `🧰️framework/🛍️products/📓️print/🖋️latex/assignment.cls`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-en.tex`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-de.tex`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json`
- `🧰️framework/🛍️products/📓️print/README.md`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-backslash.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-backslash.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-slash.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-slash.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-q.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-q.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-p.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-p.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-J.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-J.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-L.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-L.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-z.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-z.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-s.pdf`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-s.pdf`

Retained ticket authoring and verification inputs: `assignment-icons.cjs`, `convert-assignment-icons.py`, `verify-assignment-icons.py`, and this report.

## First Actual Compiler Feedback

The parent ran the actual registered Tectonic against the product's original class/packages and original verified font files without copying source into the ticket. The first German compile exited 0 and produced an approximately 82 KiB PDF. Its reported overfull answer-line paragraphs and underfull ledger paragraph were traced to missing paragraph boundaries. Both language content files now put each full-width answer rule in its own paragraph, and the shared `AssignmentLedger` ends its TikZ paragraph explicitly. Colors, fonts, rule widths and intended spacing are unchanged. The content/math/icon/PDF-provenance verification passes after this correction. The parent owns final recompilation and page-image review.

Additional source files changed for the compiler-feedback fix:
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-en.content.tex`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-de.content.tex`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex`

## Dark Theme Contrast

The reusable class now exposes `AssignmentContrastImage`. In the light theme it emits the original `includegraphics` unchanged. In the dark theme it places that same canonical vector image on the existing `semio-chrome-light-canvas` color with 0.4 mm of local padding. The original icon strokes and every font/token/theme definition remain unchanged. The 11 mm maximum image height plus 0.8 mm total padding fits within a 14 mm worksheet row. Both worksheet rows and graph vertices call the same helper.

The language-neutral contrast fixture and source contract pass as part of the 14-test assignment suite. The updated content verification also passes. The parent owns actual dark-PDF compilation and visual verification.

Additional affected files:
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🥒️.feature`
