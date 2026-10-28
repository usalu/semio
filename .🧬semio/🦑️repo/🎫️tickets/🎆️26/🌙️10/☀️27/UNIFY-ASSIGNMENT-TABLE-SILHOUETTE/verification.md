# Assignment Table Silhouette

The assignment matrix title is now rendered inside its table through the existing semio-table integrated title-row renderer used by the report tables. The shared chrome supplies the title/key tabs, baseline and corner joins; the original matrix supplies the grid. Both language documents group the existing matrix call inside AssignmentTable. The graph geometry, footer code and main published assignment PDF were not modified by this side task.

## Verification

- Test-first language-neutral composition vector:17 passed and the new composition case failed before implementation;18 passed afterward.
- AJV independently validates the silhouette fixture in English and German.
- Current parametrization content/math/source verification passed after the local caption replacements.
- Isolated registered-Tectonic table preview compiled successfully to one page with native xdvipdfmx output; no overfull or underfull diagnostics.
- The preview page was rendered and visually inspected. Both title caps and their keys join the grid outline directly.
- Independent pypdf/pdfplumber inspection confirms two nine-row matrices and zero measured caption-ink gaps at all four outer joins, accounting for the native stroke width.

## Preview

Output:output/pdf/Assignment-Table-Silhouette.pdf
Bytes:52954
SHA-256:fe34484eaf56f209f3aa59deb2d2974f3c58cf875c44e180b42ba6fe7e2df91c

## Files

- print/🖋️latex/semio-assignment.sty
- print/🧾️template/📝️assignment/parametrization-de.content.tex
- print/🧾️template/📝️assignment/parametrization-en.content.tex
- print/🧪️tests/📝️assignment/🔣️.json
- print/🧪️tests/📝️assignment/🟦️.ts
- print/README.md

Authored table-silhouette.tex, renderer and verification script remain in this ticket. Temporary compilation/cache/render files were removed after validation. No graph source, main-task ticket status, git state, worktree or AGENTS file was changed.
