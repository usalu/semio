# F110 audit: `schema check` findings, classified

Lane F110, read-only audit, 2026-11-26. No source, schema, catalog or git change was made. The findings are the exact set `schema check` prints (inventory diagnostics + placement + catalog freshness), 16242 rows, produced by the checker's own functions without `repo run`.

Reproduce: `bun 🧰️scripts/F110/dump-schema-findings.ts <out.jsonl>` in the ticket folder (imports `inventorySchemaScopes` from `🧾️schema-scope-catalog/🟦️.ts` and `loadCatalogTaxonomy`). The raw JSONL is not kept (T/🗑️generated/F110 deleted); rerun the script to regenerate it.

Areas are the first three path segments. Codes are sorted by count.

## Summary by class

Exact per-code split (the sum is 16,242). Full code table is at the end.

| Code | Total | (a) mechanical | (b) decision | (c) checker bug |
|---|---:|---:|---:|---:|
| `schema-export-incomplete` | 4489 | 4253 (regenerate twins) | 12 (twin file absent) | 224 (215 rust macro, 9 wit) |
| `schema-module-id-missing` | 2607 | 1669 (needs path-to-id rule) | 0 | 935 (duplicate row) |
| `schema-ref-unresolved` | 2560 | 696 (cascade: 240 uncatalogued ids, 456 non-id refs) | 28 (cascade of export ids, mutation targets) | 1836 (deep pointers) |
| `schema-fixture-defines-schema` | 2020 | 0 | 2020 (84 name-heuristic rows are suspect c) | 0 |
| `schema-placement-forbidden-filename` | 884 | 0 | 884 | 0 |
| `schema-export-id-invalid` | 677 | up to 677 (PascalCase rename; acronym titles need a decision) | acronym subset | 0 |
| `schema-export-id-duplicate` | 654 | 0 | 654 | 0 |
| `schema-owner-ineligible` | 583 | 0 | 583 | 0 |
| `schema-document-id-unaddressable` | 571 | 571 (path rule) | 0 | 0 |
| `schema-scope-ambiguous` | 542 | 0 | 542 | 0 |
| `schema-document-id-duplicate` | 365 | 0 | 365 | 0 |
| `schema-dialect-not-draft-07` | 182 | 182 (verify 2020-12 keywords) | 0 | 0 |
| `schema-mutation-leaf-id` | 91 | 91 (rewrite to printed id) | 0 | 0 |
| `schema-mutation-aggregate-id` | 6 | 6 | 0 | 0 |
| `schema-module-id-inconsistent` | 10 | 10 | 0 | 0 |
| `schema-catalog-stale` | 1 | 1 (regenerate last) | 0 | 0 |

Totals (approximate, export-id-invalid counted as (a)): (a) about 8,150; (b) about 5,060; (c) about 2,995; cascades 28.

## Checker bugs (fix these first; about 3,000 of the 16,242 rows)

1. `🧾️schema-scope-catalog/🟦️.ts` lines 620-621. The cross-scope branch sets `exportId = fragment.slice("/$defs/".length)` with no check for a further `/`. The local branch (line 606) does check it. So `$id#/$defs/CaseNumber/properties/value` is read as the export `CaseNumber/properties/value`, and line 858 reports it as "references export ... which ... does not declare". 1,836 of the 1,863 "target export not declared" rows are this bug: the first segment IS a `$defs` key of the target in all 400 sampled. Fix: apply the same `!includes("/")` guard; resolve deep pointers against the target document instead of the export set.
2. Lines 556 and 734. A root `🔣️.json` without `$id` is reported twice: once per document (556, `schema-module-id-missing` with the file path) and once per module (734, with the folder path). 935 of 2,607 rows are this duplicate. Fix: keep one (the module row at 734), and keep the document row only for non-root facet documents (3 rows).
3. `🧾️schema-scope-catalog/🟦️.ts` lines 407-417 (`schemaFormatDeclaresExport`). The Rust check only accepts `struct|enum|type|trait|fn` and `pub use`. Types declared inside the `protocol::list_delta! { pub EquationEdgesDelta { ... } }` macro are invisible. 215 rows are such declarations present in the file (`rs-macro-decl`). Fix: accept the macro form (or reuse the twin generator's own declaration parser).
4. Same lines. The keyword table has no entry for the `.wit` format, so `declared` is undefined and the check always returns false. 9 rows.
5. Line 447, `namedCorpus` regex (`Fixture|Corpus|Cases|Vectors?`). It matches export titles by name, e.g. `DemonstratorBrowserCatalogCases` in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📇️catalog/🧬️schema/🔣️.json`, a runtime catalog input, not a test corpus. Suspect: needs a human review of the 84 name-only hits before any fix.
6. Around lines 872-876 (export-incomplete source read). A missing twin file is swallowed as `""` and reported as "carries no declaration" (12 rows). Report "twin file missing" instead (naming only).

## Mechanical rules (a)

- Export id (`schema-export-id-invalid`, 677). Rule: `exportIdPattern` is `^[A-Z][A-Za-z0-9]*$` (taxonomy `schemaExportResolution`). Convert each `$defs` key and each root `title` by splitting on non-alphanumerics, capitalising each word, keeping inner capitals (`graph` -> `Graph`, `Axis-Angle Rotation` -> `AxisAngleRotation`), then rewrite every `#/$defs/<key>` and `#/$defs/<key>/...` pointer repo-wide and regenerate twins. Acronym titles (GIS, STEP, ZIP, IDML, WFC, 3D/2D) stay in (b) because the spelling is a naming decision.
- Export missing from twins (`schema-export-incomplete`, 4,253 real absences: rust 2,902, graphql 462, typescript 452, protobuf 437). Rule: twins are generated from the JSON Schema by `repo ./📜️script.ts schema twins generate`. Regenerate, then `schema twins check` must be clean. Not verified by running (out of scope for this lane). Twins with list-delta macros may not be emitted by the generator, so verify before claiming completeness.
- Module id missing (`schema-module-id-missing`, 1,669 unique modules). Rule still to be written. The generator never derives a scope from a path (F107 finding), so there is no existing path-to-id function. Proposed rule to verify against the 5,972 conforming catalog rows before use: `$id = https://json.schemas.assets.semio-tech.com/` + scope path + `/schema.json`, where the scope id is the dotted form of the owner path with emoji prefixes stripped, and the standards, subsets and version segments dropped (`app.stdio.semio.config` from `🗿️artifacts/🧿️semio/.../✉️base/✏️editor/🎚️config`). Framework modules use the `repo.<...>` prefix F107 chose (`repo.test.acceptance`). Acceptance criterion: the rule reproduces every existing catalog row's scope id.
- Document id unaddressable (`schema-document-id-unaddressable`, 571). Non-dotted `$id` forms: `semio://...` (some), `schema://gis.gismap/TilePlanV1`, `gis:...`, `urn:` (34), dotted `s.wfc.wfc2d...v1` (96). Same path rule as above, plus the `$id` must be under idBase.
- Ref cascade (`schema-ref-unresolved`, 240 target `$id` not catalogued, 456 non-id cross-scope form). Rule: resolved once the missing `$id`s above are written; rewrite non-id cross-scope refs to `<target $id>#/$defs/<Export>`.
- Dialect (`schema-dialect-not-draft-07`, 182: `null` 106, `2020-12` 75, `https` variant 1). Rule: set `$schema` to `http://json-schema.org/draft-07/schema#`. Caveat: 2020-12-only keywords (`prefixItems`, `unevaluated*`, `dependentSchemas`) change meaning under draft-07; compile each rewritten document and send those to (b).
- Mutation ids (`schema-mutation-leaf-id` 91, `schema-mutation-aggregate-id` 6, `schema-module-id-inconsistent` 10). Rule: the checker already prints the expected `$id` in each message (`mutationLeafSchemaId`, `mutationsFacetDirName`). The writer examples show a stutter (`app/writer/writer/window/main` expected `app/writer/main-window`). Rewrite `$id` to the printed expected value.
- Catalog stale (`schema-catalog-stale`, 1). Rule: run `repo ./📜️script.ts schema generate` and `schema docs` after the above (F107 gate `schema-catalog-fresh` enforces it).

## Human or design decisions (b)

- `schema-fixture-defines-schema` (2,020): 1,631 "defines schema authority inside examples" under fixture/example folders, plus test-corpus names. Decision: either declare the specimen inert (the `inertSchemaData` route) or move it out of examples. Include the line 447 review above.
- `schema-placement-forbidden-filename` (884): `**/*.schema.json` (754) and `**/🧬️schema.json` (130). Moving to the owner's `🧬️schema` module requires choosing the owner (628 are under test folders). Not mechanical.
- `schema-owner-ineligible` (583): owner paths are not declared `schemaScopeOwnerLevels` levels (e.g. `🏭️bridge`, `🛠️tools`, `🐚️shell`). Decision: add a level to the taxonomy or relocate the module. Taxonomy is the authority; do not patch the checker.
- `schema-export-id-duplicate` (654) and `schema-scope-ambiguous` (542), `schema-document-id-duplicate` (365): the same scope or export declared twice (e.g. `FemCombination` in `🔣️.json` and `📸️snapshot/🔣️.json`; `app.wfc.2d.config` in both `✏️editor/🎚️config` and `👁️viewer/🎚️config`; `app.stdio.semio.config` in 38 modules). The checker is right; which copy owns the id is a design decision. Unique scope ids: 165 ambiguous, 157 duplicate document ids.
- `schema-export-incomplete` for 12 twin files that do not exist: decide whether the format is still wanted for that scope.

## Per-code breakdown (exact)

| Code | Findings |
|---|---:|
| `schema-export-incomplete` | 4489 |
| `schema-module-id-missing` | 2607 |
| `schema-ref-unresolved` | 2560 |
| `schema-fixture-defines-schema` | 2020 |
| `schema-placement-forbidden-filename` | 884 |
| `schema-export-id-invalid` | 677 |
| `schema-export-id-duplicate` | 654 |
| `schema-owner-ineligible` | 583 |
| `schema-document-id-unaddressable` | 571 |
| `schema-scope-ambiguous` | 542 |
| `schema-document-id-duplicate` | 365 |
| `schema-dialect-not-draft-07` | 182 |
| `schema-mutation-leaf-id` | 91 |
| `schema-module-id-inconsistent` | 10 |
| `schema-mutation-aggregate-id` | 6 |
| `schema-catalog-stale` | 1 |
| **total** | **16242** |

### `schema-export-incomplete` (4489)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 981 | `✏️s/🔌️plugins/📕️norm` |
| 870 | `✏️s/🔌️plugins/🏛️architect` |
| 594 | `✏️s/🔌️plugins/🗄️stdio` |
| 401 | `✏️s/🔌️plugins/🏙️bim` |
| 268 | `✏️s/🔌️plugins/🧩️puzzle` |
| 231 | `✏️s/🔌️plugins/🧱️block` |
| 191 | `🧰️framework/🛍️products/💻️os` |
| 154 | `✏️s/🔌️plugins/📏️layout` |
| 113 | `✏️s/🔌️plugins/🏗️fem` |
| 105 | `✏️s/🔌️plugins/📐️cad` |

Examples:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`: s.equation.equation declares export EquationEdgeInsertion in 🔺️diff/🔣️.json; 🟦️typescript carries no declaration of it. Provide it, or annotate the export with x-semio-formats to declare restricted support honestly.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql`: s.wfc.grid3d.1.any.artifact declares export Grid3dSolve in 💡️inferences/🔣️.json; 🔗️graphql carries no declaration of it. Provide it, or annotate the export with x-semio-formats to declare restricted support honestly.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/📜️.wit`: app.procedural.2d.config declares export Generation2dConfig in 🔣️.json; 📜️wit carries no declaration of it. Provide it, or annotate the export with x-semio-formats to declare restricted support honestly.

### `schema-module-id-missing` (2607)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 846 | `🧰️framework/🛍️products/🦑️repo` |
| 455 | `🧰️framework/🛍️products/💻️os` |
| 248 | `🧰️framework/🔨️modules/🏃️process` |
| 183 | `🧰️framework/🔨️modules/🌱️value` |
| 90 | `🧰️framework/🔨️modules/🧵️job` |
| 60 | `🧰️framework/🔨️modules/📡️replication` |
| 59 | `✏️s/🔌️plugins/🗄️stdio` |
| 51 | `🧰️framework/🔨️modules/🖱️ui` |
| 47 | `✏️s/🔌️plugins/🖍️draw` |
| 46 | `✏️s/🔌️plugins/📏️layout` |

Examples:

- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema`: ♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json must declare the $id that names this module's scope.
- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json`: Declare $id as https://json.schemas.assets.semio-tech.com/<scope path>/<facet>.json.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🧬️schema`: ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🧬️schema/🔣️.json must declare the $id that names this module's scope.

### `schema-ref-unresolved` (2560)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 1977 | `✏️s/🔌️plugins/📕️norm` |
| 275 | `✏️s/🔌️plugins/🗄️stdio` |
| 56 | `✏️s/🔌️plugins/🖍️draw` |
| 45 | `✏️s/🔌️plugins/🔋️energy` |
| 28 | `🧰️framework/🛍️products/🦑️repo` |
| 26 | `🧰️framework/🛍️products/💻️os` |
| 17 | `✏️s/🔌️plugins/📋️forms` |
| 16 | `✏️s/🔌️plugins/🌍️gis` |
| 15 | `✏️s/🔌️plugins/🗒️note` |
| 14 | `🧰️framework/🔨️modules/🏃️process` |

Examples:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`: /properties/geometry/$ref references export "geometry", which ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json does not declare.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`: /properties/graph/$ref references export "graph", which ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json does not declare.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`: /$defs/EquationEdgeInsertion/properties/row/$ref references export "edge", which ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json does not declare.

### `schema-fixture-defines-schema` (2020)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 623 | `🧰️framework/🛍️products/🦑️repo` |
| 314 | `🧰️framework/🔨️modules/🌱️value` |
| 261 | `🧰️framework/🛍️products/💻️os` |
| 134 | `✏️s/🔌️plugins/🗄️stdio` |
| 123 | `🧰️framework/🔨️modules/🏃️process` |
| 87 | `🧰️framework/🔨️modules/🧵️job` |
| 82 | `🎓️teaching/🏛️architecture/🏗️bim` |
| 28 | `🧰️framework/🔨️modules/🖱️ui` |
| 27 | `🧰️framework/🔨️modules/📡️replication` |
| 23 | `✏️s/🔌️plugins/📸️remodel` |

Examples:

- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📇️catalog/🧬️schema/🔣️.json`: DemonstratorBrowserCatalogCases defines a test corpus contract. Examples use actual domain contracts and cannot own separate schemas.
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🔣️.json`: ♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🔣️.json defines schema authority inside examples. Ordinary parser specimens require an explicit inert data declaration.
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🛑️cancellation/🧬️schema/🔣️.json`: ♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🛑️cancellation/🧬️schema/🔣️.json defines schema authority inside examples; contract identity cannot be declared inert.

### `schema-placement-forbidden-filename` (884)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 212 | `🧰️framework/🔨️modules/🌱️value` |
| 139 | `✏️s/🔌️plugins/🗄️stdio` |
| 102 | `🧰️framework/🛍️products/💻️os` |
| 92 | `🎓️teaching/🏛️architecture/🏗️bim` |
| 82 | `🧰️framework/🛍️products/🦑️repo` |
| 66 | `🧰️framework/🔨️modules/🧵️job` |
| 28 | `🧰️framework/🔨️modules/🖱️ui` |
| 22 | `🎓️teaching/🏛️architecture/⚡️energy` |
| 22 | `🧰️framework/🔨️modules/🏃️process` |
| 9 | `🎓️teaching/🏛️architecture/🏢️nakagin` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/⚙️engine/💼️job/📤️publication/📦️owner/🧪️tests/frontiers.schema.json`: **/*.schema.json ceases to exist: move the contract into its owner's 🧬️schema module.
- `✏️s/🔌️plugins/🀄️wfc/⚙️engine/🔍️search/🏃️batch/🧪️tests/🧫️phase/🧬️schema.json`: **/🧬️schema.json ceases to exist: move the contract into its owner's 🧬️schema module.

### `schema-export-id-invalid` (677)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 183 | `🧰️framework/🛍️products/🦑️repo` |
| 93 | `🧰️framework/🛍️products/💻️os` |
| 49 | `🧰️framework/🔨️modules/🌱️value` |
| 34 | `✏️s/🔌️plugins/🗄️stdio` |
| 32 | `🧰️framework/🔨️modules/🖱️ui` |
| 30 | `✏️s/🔌️plugins/🖍️draw` |
| 25 | `🌎️hub/📇️directory/🧬️schema` |
| 21 | `🧰️framework/🔨️modules/🎭️actor` |
| 20 | `✏️s/🔌️plugins/🖨️raster` |
| 19 | `✏️s/🔌️plugins/📏️layout` |

Examples:

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`: $defs key "expression" is not a PascalCase export id.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`: $defs key "node" is not a PascalCase export id.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`: $defs key "edge" is not a PascalCase export id.

### `schema-export-id-duplicate` (654)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 391 | `✏️s/🔌️plugins/📕️norm` |
| 116 | `✏️s/🔌️plugins/🏛️architect` |
| 48 | `✏️s/🔌️plugins/🏗️fem` |
| 35 | `✏️s/🔌️plugins/🗄️stdio` |
| 34 | `✏️s/🔌️plugins/📏️layout` |
| 19 | `✏️s/🔌️plugins/🏙️bim` |
| 6 | `✏️s/🔌️plugins/📐️cad` |
| 3 | `✏️s/🔌️plugins/🔱️trinity` |
| 2 | `✏️s/🔌️plugins/🧩️puzzle` |

Examples:

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json`: Export FemAnalysisSettings is already declared by ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🔣️.json; one scope resolves each export id to exactly one file.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json`: Export FemCombination is already declared by ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🔣️.json; one scope resolves each export id to exactly one file.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json`: Export FemCombinationTerm is already declared by ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🔣️.json; one scope resolves each export id to exactly one file.

### `schema-owner-ineligible` (583)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 88 | `✏️s/🔌️plugins/🗄️stdio` |
| 74 | `🧰️framework/🛍️products/💻️os` |
| 54 | `✏️s/🔌️plugins/🖍️draw` |
| 39 | `🧰️framework/🔨️modules/🏃️process` |
| 30 | `🧰️framework/📦️packages/🟦️typescript` |
| 23 | `✏️s/🔌️plugins/📏️layout` |
| 22 | `🧰️framework/🔨️modules/🖱️ui` |
| 21 | `✏️s/🔌️plugins/🖨️raster` |
| 20 | `✏️s/🔌️plugins/🔋️energy` |
| 16 | `🧰️framework/🔨️modules/🌱️value` |

Examples:

- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema`: ♻️mit-bestand/🧺️demonstrator/🐚️shell is not a declared schemaScopeOwnerLevels level.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🧬️schema`: ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill is not a declared schemaScopeOwnerLevels level.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/🧬️schema`: ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge is not a declared schemaScopeOwnerLevels level.

### `schema-document-id-unaddressable` (571)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 233 | `🧰️framework/🛍️products/🦑️repo` |
| 79 | `🧰️framework/🔨️modules/🏃️process` |
| 56 | `🧰️framework/🛍️products/💻️os` |
| 34 | `✏️s/🔌️plugins/🖍️draw` |
| 24 | `🧰️framework/🔨️modules/🌱️value` |
| 16 | `✏️s/🔌️plugins/🗄️stdio` |
| 11 | `🧰️framework/🔨️modules/⏯️tool-run` |
| 11 | `🧰️framework/🔨️modules/◻️2d` |
| 11 | `🧰️framework/🔨️modules/🖱️ui` |
| 11 | `🧰️framework/🔨️modules/🧵️job` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🧬️schema/🔣️.json`: $id "s.wfc.wfc2d.fill.tick.v1" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🧬️schema/🔣️.json`: $id "s.wfc.grid2d.fill.tick.v1" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/.
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🌡️fill/🧬️schema/🔣️.json`: $id "s.wfc.bitmap.fill.tick.payload.v1" does not resolve to a dotted scope id under https://json.schemas.assets.semio-tech.com/.

### `schema-scope-ambiguous` (542)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 352 | `✏️s/🔌️plugins/🗄️stdio` |
| 60 | `✏️s/🔌️plugins/📕️norm` |
| 20 | `✏️s/🔌️plugins/🀄️wfc` |
| 12 | `✏️s/🔌️plugins/🧱️block` |
| 8 | `✏️s/🔌️plugins/🪐️space` |
| 5 | `✏️s/🔌️plugins/🌊️flow` |
| 5 | `🧰️framework/🛍️products/🦑️repo` |
| 4 | `✏️s/🔌️plugins/🌿️vcs` |
| 4 | `✏️s/🔌️plugins/🎞️animate` |
| 4 | `✏️s/🔌️plugins/🎥️shooting` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema`: Scope app.wfc.2d.config is declared by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json and by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema; ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema`: Scope app.wfc.2d.presence is declared by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🔣️.json and by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema; ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standard
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema`: Scope app.wfc.grid2d.config is declared by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json and by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema; ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/

### `schema-document-id-duplicate` (365)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 280 | `✏️s/🔌️plugins/🗄️stdio` |
| 30 | `✏️s/🔌️plugins/📕️norm` |
| 10 | `✏️s/🔌️plugins/🀄️wfc` |
| 6 | `✏️s/🔌️plugins/🧱️block` |
| 4 | `✏️s/🔌️plugins/🪐️space` |
| 2 | `✏️s/🔌️plugins/🌿️vcs` |
| 2 | `✏️s/🔌️plugins/🎞️animate` |
| 2 | `✏️s/🔌️plugins/🎥️shooting` |
| 2 | `✏️s/🔌️plugins/🎪️demonstrator` |
| 2 | `✏️s/🔌️plugins/🎬️sequence` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🔣️.json`: $id https://json.schemas.assets.semio-tech.com/app/wfc/2d/config/schema.json is already declared by ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json; a document $id is the resolution key and is unique repository-wide.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/🔣️.json`: $id fixture.neutral-host-fixture.counter is already declared by 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🔣️.json; a document $id is the resolution key and is unique repository-wide.

### `schema-dialect-not-draft-07` (182)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 51 | `🧰️framework/🛍️products/🦑️repo` |
| 41 | `🧰️framework/🛍️products/💻️os` |
| 22 | `🧰️framework/🔨️modules/🌱️value` |
| 14 | `✏️s/🧑‍💻dev/🎭️variants` |
| 8 | `🧰️framework/🔨️modules/🧵️job` |
| 7 | `✏️s/🔌️plugins/🗄️stdio` |
| 4 | `🧰️framework/🔨️modules/🎒️pack` |
| 4 | `🧰️framework/🔨️modules/📡️replication` |
| 3 | `🧰️framework/🔨️modules/🎭️actor` |
| 3 | `🧰️framework/🔨️modules/🏃️process` |

Examples:

- `♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json`: $schema must be http://json-schema.org/draft-07/schema#, got null.

### `schema-mutation-leaf-id` (91)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 41 | `✏️s/🔌️plugins/🀄️wfc` |
| 30 | `🧰️framework/🛍️products/💻️os` |
| 12 | `✏️s/🔌️plugins/📕️norm` |
| 5 | `✏️s/🔌️plugins/✒️writer` |
| 2 | `✏️s/🔌️plugins/🗄️stdio` |
| 1 | `✏️s/🔌️plugins/🔱️trinity` |

Examples:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧬️schema/🧬️mutations/⚙️editor-settings/🧬️schema/🔣️.json`: $id "https://json.schemas.assets.semio-tech.com/app/writer/writer/window/main/config/mutation/set-editor-settings/schema.json" must be https://json.schemas.assets.semio-tech.com/app/writer/main-window/config/mutation/set-editor-settings/schema.json: a mutation leaf is its own scope, not a facet of i
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/📄️replace-part/🧬️schema/🔣️.json`: $id "s.stdio.idml.mutation.replace-part" must be https://json.schemas.assets.semio-tech.com/<enclosing scope path>/mutation/replace-part/schema.json: a mutation leaf is its own scope, not a facet of its enclosing module.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/📦️replace-package/🧬️schema/🔣️.json`: $id "s.stdio.idml.mutation.replace-package" must be https://json.schemas.assets.semio-tech.com/<enclosing scope path>/mutation/replace-package/schema.json: a mutation leaf is its own scope, not a facet of its enclosing module.

### `schema-module-id-inconsistent` (10)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 6 | `✏️s/🔌️plugins/🀄️wfc` |
| 2 | `✏️s/🔌️plugins/🎥️shooting` |
| 1 | `✏️s/🔌️plugins/📜️imperative` |
| 1 | `✏️s/🔌️plugins/🗒️note` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json`: $id "https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any/inferences/schema.json" must keep this module's scope path https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any/artifact/ and vary only the facet filename; a facet document never deepens the scope.

### `schema-mutation-aggregate-id` (6)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 4 | `✏️s/🔌️plugins/🀄️wfc` |
| 1 | `✏️s/🔌️plugins/📜️imperative` |
| 1 | `🧰️framework/🛍️products/📓️print` |

Examples:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔣️.json`: $id "https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any/config/mutations.json" must be https://json.schemas.assets.semio-tech.com/app/wfc/2d/config/mutations.json: an aggregate is the mutations facet of its own module and never deepens the scope.
- `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🔣️.json`: $id "https://semio.tech/schema/print/chart/mutation" must be https://json.schemas.assets.semio-tech.com/print/mutations.json: an aggregate is the mutations facet of its own module and never deepens the scope.

### `schema-catalog-stale` (1)

Top 10 areas (first 3 path segments):

| Findings | Area |
|---:|---|
| 1 | `🧰️framework/🛍️products/🦑️repo` |

Examples:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`: The catalog does not match the schema modules on disk.

