# Parametrization Assignment Content

## Current Authored Documents

The print catalog registers `parametrization-en` and `parametrization-de`. Both main documents explicitly select `type=assignment,theme=light,language=...` on the actual `semio` class. Sources live under `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/`.

- `parametrization-en.tex` and `parametrization-de.tex`: actual print-product class entry points.
- `parametrization-en.content.tex` and `parametrization-de.content.tex`: complete translated five-page teaching sequence.
- `parametrization-geometry.tex`: box example, exact Metabolism icon references, blank table, K8 and handwritten ledger.
- `parametrization-icons.json`: authored derivation plan for vector PDF resources from unchanged canonical SVG source assets.

## Corrected Exercise Subject

The exercise now uses the repository's original Metabolism dwelling-module icons. The previous drawn pill silhouettes and length/diameter fixtures have been replaced entirely in the authored document and tests.

Canonical source families are `💊️capsules` and `🥚️ellipsoids`. There are no eight cylindrical-capsule assets; the truthful descriptions are “Metabolism-Wohnkapseln und ellipsoide Varianten” in German and “Metabolism dwelling capsules and ellipsoid variants” in English. The unrelated four cylindrical tambours are not substituted.

The two tables cover all sixteen source icons exactly once, alternating the two families for each paired variant:

| Exercise | Paired Variant Order |
| --- | --- |
| 1 | backslash, slash, q, p |
| 2 | J, L, z, s |

Source paths, SHA256 hashes, SVG viewBoxes and vector path counts are explicit in the version-2 neutral fixture. Source SVGs remain in place and unchanged. The SVGs contain vector paths and no raster image elements. No icons are relabeled, rehandtraced, recolored or replaced.

## Valid LaTeX Asset Use

The shared shape macro uses `includegraphics[width=22mm,height=11mm,keepaspectratio]` on derived vector PDF resources. Table and graph rows consume the same family/variant/resource triplets.

The plan has fields `version:1`, `format:"svg-to-vector-pdf"`, `sourceRoot:"workspace"` and `resources:[{source,output,sha256}]`. Every `source` is the canonical repository-relative SVG path; every `output` is a local `resources/metabolism-<family>-<variant>.pdf` path. Both catalog entries list the plan among their authored sources. Root owns the actual native staging producer and compiler integration, which must produce these resources before compilation; direct SVG inclusion or a fake fallback is not used.

The user's latest explicit requirement is that the final main document be compiled through the real LaTeX print product with the new assignment class and its existing styling. The prior one-off ReportLab main-document script is historical ticket evidence and must not produce the final deliverable. No new main PDF was generated in this correction pass.

## Preserved Pedagogy and Writable Structure

Three common-axis boxes have dimensions (2,1,1), (3,1,1), (3,2,1). Length/width/height gives unique geometry and exact signed changes. Omitting width isolates ambiguity; prescribing an incorrect direct change isolates inconsistent transition rules; the invertible transform (L,L+W,H) isolates avoidable extra parameter changes.

Both exercise tables have exactly eight unnamed icon rows, one first-column Parameter header, five blank parameter headers and blank value cells. Table cells contain neither type names nor row numbers. Shape orientation and aspect ratio are preserved.

Each following graph repeats its table's icons in the same row order. A single authored list of all 28 unordered pairs serves both the K8 drawing and a two-column handwritten ledger. Row references occur only outside the tables. Reverse traversal negates changes, and closed-cycle sums must return to zero.

## Test-Driven Correction and Independent Oracles

Before replacing the document geometry, the updated resource-bound test failed with zero source icon rows where sixteen were required. After replacing the sources and adding the plan, `verifyParametrizationAssignmentContentV1()` passes.

The versioned neutral fixture is constrained by `🧬️cases.schema.json`, independently validated by existing test-only ajv. Existing mathjs independently evaluates the box coordinate changes, coupled coordinates and cycle sums. Existing d3-array independently enumerates the complete graph pairs.

The additional retained test `🔮️svg-provenance.py` independently parses every canonical source through bundled third-party lxml. It verifies the SVG namespace, exact viewBox, vector path count, no raster image elements, unchanged SHA256 and alternating family order for all sixteen icons.

Validation results:

- Semantic/source/bilingual cases: passed via registered repo Bun.
- Independent lxml provenance: 16 canonical Metabolism SVG icons verified via registered repo Python.

All commands use the repo interface. No source SVG, AGENTS file, git state, worktree, migration or application runtime dependency was modified by this correction.
