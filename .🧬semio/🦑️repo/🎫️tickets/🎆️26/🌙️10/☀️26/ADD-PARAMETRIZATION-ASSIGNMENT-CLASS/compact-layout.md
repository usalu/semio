# Compact Matrix Layout

The reusable assignment matrix separates its icon header from its handwriting rows. It now defaults to a 16 mm header, 6 mm writing rows, 2 mm padding and a 27 mm label column. Both eight-row matrix bodies occupy 128 mm before borders and surrounding prose.

Each header cell uses an independent fixed-height vertical box with equal flexible glue above and below a horizontally centered content box. This removes the former baseline-dependent strut placement. Empty writing cells override the header preamble and use a symmetric 6 mm rule. The former AssignmentParameterTable implementation is unchanged.

AssignmentMatrixImage fits each original vector PDF proportionally within both the calculated cell content width and the header height minus twice the padding. Dark-theme image bounds subtract the existing 0.8 mm backing extent before sizing. The canonical PDF resources and complete-graph icon geometry are unchanged.

## Test-Driven Verification

The language-neutral JSON fixture and Gherkin scenario declare the independent header height, writing height, padding and row/column counts. AJV independently validates the JSON fixture. The modified fixture first failed against the old 15 mm matrix default. After implementation, the generic assignment suite passes all 17 tests and 64 assertions.

Command: repo tools exec --light bun -- test ./🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts

Actual PDF compilation, page geometry and visual inspection are coordinated by the parent agent and reported in compact-audit.md.

## Files

- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty
- 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🥒️.feature
