# Matrix Layout

The assignment matrix now transposes the sixteen canonical types into two tables. Each table has nine columns (a parameter label plus eight type icons) and nine rows (the icon header plus eight blank parameter rows). The tables share the same parameter strategy.

## Implementation

- Added the reusable AssignmentParameterMatrix environment to semio-assignment.sty with configurable type-columns, parameter-rows, row-height and label-width.
- Defaults are eight type columns, eight writable parameter rows, 15 mm row height and 27 mm label content width.
- Column widths use the current full line width, including all padding and vertical rules, with 2.5 pt padding per side.
- Shared parametrization geometry accumulates all eight canonical icons into each matrix header.
- Matrix icons fit a 14 mm by 11 mm box. Complete-graph icons retain their existing 22 mm by 11 mm sizing and graph scaling.
- Existing AssignmentParameterTable and AssignmentParameterRow remain available without changes.
- All sixteen canonical vector PDFs remain unchanged.

## Test-Driven Verification

A language-neutral matrix fixture declares the expected row and column counts, geometry and icon bounds. The independent AJV JSON Schema validator adjudicates the fixture and the source contract checks the TeX environment, declared options, icon header, row generation and blank cells.

The new matrix test first failed because AssignmentParameterMatrix did not exist. After implementation, the generic assignment suite passed all 17 tests with 57 assertions using:

repo tools exec --light bun -- test ./🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts

Final document compilation and visual verification are coordinated by the parent agent.

## Files

- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty
- 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts
