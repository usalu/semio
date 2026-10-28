# One Shared Parametrization Exercise

Both explicit language sources now describe one exercise with a shared parameter strategy for all sixteen Metabolism shapes. Table A and Table B occupy the second and third pages; the two complete graphs and ledgers occupy the fourth and fifth pages. One exercise section owns all table and graph subsections. The worked boxes and the three isolated counterexamples remain on the first page.

Each table places eight shape icons across the header, preceded by the Parameter label. Eight blank parameter rows provide nine total rows and nine total columns. Students enter parameter names in the first column and values below the icons, retaining the same parameter names and row order in both tables. Graph references 1–8 follow each corresponding table's columns from left to right; Table A/B context distinguishes them.

The language-neutral fixture and JSON Schema are version 3 and express one exercise containing two tables, a shared strategy, exact 9×9 matrix dimensions and the five-page order. The Gherkin scenarios and TypeScript source contracts cover this structure. The SVG provenance oracle reads the same single-exercise fixture.

Test-first evidence: after updating the fixture, schema and source contract, verification failed because the old geometry lacked a transposed matrix. After the matrix implementation and bilingual source updates, the verification function passes. AJV also rejects independent strategies, duplicate table identities, ten-row matrices, seven type columns and reversed page order.

Independent checks passed:

- mathjs reproduces the coordinate transformation, changed-parameter counts and both cycle sums.
- d3-array reproduces every unordered graph pair and all 81 matrix cell roles.
- AJV validates the fixture and rejects the five malformed variants.
- The lxml SVG oracle verifies all sixteen canonical SVG namespaces, vector paths, view boxes and hashes. The final verification ran through the registered Python tool with the bundled Python executable override, which already contains lxml.
- The direct content verification also checks generated icon PDF signatures and embedded source hashes.

Commands used the repo tool interface exclusively. No Git mutation, worktree or AGENTS.md change was made. Root owns final TeX compilation and final delivery review; the requested additional German dark regression is documented separately in matrix-dark.md.

Files changed in this subtask:

- print template assignment parametrization-de.content.tex
- print template assignment parametrization-en.content.tex
- print test parametrization-assignment 🧫️cases.json
- print test parametrization-assignment 🧬️cases.schema.json
- print test parametrization-assignment 🥒️.feature
- print test parametrization-assignment 🟦️.ts
- print test parametrization-assignment 🔮️svg-provenance.py
- ticket single-exercise.md

## Temporary Dependency Audit

An earlier uv no-project run also passed, after the default system Python lacked lxml. That invocation performed no downloads or installations. A read-only recursive audit of all 32,234 entries under the existing uv cache found no entry modified on 2026-10-27, including during this task. No task-owned cache output was identified, so no unrelated cache was deleted. The final oracle was repeated with SEMIO_TOOL_PYTHON pointing to the existing bundled runtime. No runtime dependency or package manifest was introduced or changed.
