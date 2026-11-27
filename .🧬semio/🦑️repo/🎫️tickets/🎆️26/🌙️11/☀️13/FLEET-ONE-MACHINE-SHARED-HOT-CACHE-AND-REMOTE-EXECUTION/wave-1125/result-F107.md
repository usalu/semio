# F107: one schema-definition predicate, catalog regenerated, freshness gate in the suite

Lane F107, 2026-11-26. schema-invariants now **164 pass / 0 fail** (was 163/1).

## 1. One predicate, one walk (generator and rule)
Shared code lives in `📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts` (exports, schema-first policy: a JSON document naming a json-schema.org `$schema` IS a schema definition):
- `isSchemaDefinitionDocument`, `parseSchemaDefinitionText`, `schemaDefinitionModulePaths(files, moduleDirName, readText, excluded)`, `walkSchemaTree(repoRoot, taxonomy, under)`.
- `inventorySchemaScopes` uses them and returns the new `definitionModules`; a definition module with no scope id now always raises `schema-module-id-missing` (before, only modules with canonical documents did). No new diagnostic code.
- The rule side (`🧪️test/📐️schema-contracts`) delegates: `schemaModuleDirectoriesOnDisk` -> `schemaDefinitionModulePaths`; `isJsonSchemaDefinition`/`readSchemaDefinition` -> shared; `schemaTreeFiles` -> `walkSchemaTree`. The two walks had different skip lists (rule: build/out/obj/bin/.cache; generator: pkg/temp/coverage/client/...); there is now only the generator's.
- Why the four folders had no row (root cause): the generator derives a scope only from a conforming `$id` (never from the path). acceptance had `schema://semio/repo-test-acceptance/v1`, providers had a `urn:`, inventory had no `$id`, and `🧪️test/🧬️schema` has no root document (its three gate schemas sit in non-facet subfolders). Fix is schema-first data: handcrafted `$id`s `.../repo/test/acceptance/schema.json`, `.../repo/test/inventory/providers/schema.json`, `.../repo/test/inventory/schema.json`, and a new root `🧪️test/🧬️schema/🔣️.json` (`.../repo/test/schema.json`, scope identity only, no exports; the three gate documents are untouched). New scopes: `repo.test`, `repo.test.acceptance`, `repo.test.inventory`, `repo.test.inventory.providers`.
- Laws (in `schema-scope-catalog/🧪️tests/🟦️.ts`, fixture tree with ok/no-id/urn/nested-only/non-definition/collection/node_modules/dot-dir modules): generator set == rule set; every definition module is a row or a `schema-module-id-missing` finding and every row is a definition module; rule walk == generator walk (incl. `under`); the four test-platform modules are catalogued. 35/0 (was 31).

## 2. Catalog regenerated (same code path as F105's render script)
`T/🧰️scripts/F107/regenerate-catalog.ts` renders with `inventorySchemaScopes` + `renderSchemaCatalog` + `renderSchemaCatalogDocument` into scratch, then atomic temp+rename under lock `landing.lock.schema` (held for the three renames only).
- `🔣️schema-catalog.json`: 5924 -> 5972 scopes; **0 removed**, 48 added (44 pre-existing gaps: bim, gis terrain, norm din/eurocode, puzzle5d, shooting, trinity, pdf, kernel/surface, plus the 4 above), 2844 changed rows = 2842 `hashes`, 14 `dependsOn`, 13 `exports` (2840 content hash changes, 2 hash key-set changes). Provenance keys unchanged. sha256 prefix 790feed5cedb4d58 -> 85752f97c2784884.
- `📓️schema-catalog.md` regenerated from the same catalog (verify checks it too; it was stale as well).
- `🗣️dsl/✨️derive/🗂️schema-document-index.json` (derived index keyed by `$id`) regenerated with its own `mutationSchemaDocumentIndexProjection`: was identical before; the diff is only the four changed/new ids. Its stability test 5/0.

## 3. Why 2844 rows drifted, and the gate
- `schema generate --check`, `schema check` (`schema-catalog-stale`) and `schema verify` all exist, but nothing runs them in a suite: `schema-check` returns exit 1 on ~14k unrelated repo-wide findings, so nobody can use it as a gate, and `schema-generate` writes with a plain non-atomic `writeFileSync`. The test-platform rule only compares module path sets, never hashes. Schemas were edited for weeks without regeneration and no test failed.
- New suite `schema-catalog-fresh` (budget 300000, all three platforms) with the member `schema-scope-catalog/🧪️tests/🧮️catalog-freshness/🟦️.ts`: byte-compares catalog and document with a fresh render (`schema generate --check` as a test, 85 s under load 20, 1/0). Suite table + aggregate list + target `test-schema-catalog-fresh` (inputs `schemaSources`, catalog, document) landed together; route table law `⚡️caching/🗂️suites/🧪️tests/🟦️.ts` 5/0 (budget classes allow only 15000/300000, so no 600000).
- Consequence: any schema edit (or a peer's new module) now fails this suite until `repo ./📜️script.ts schema generate` + `schema docs` run. Remedy is named in the test comment.

## Verification
- `repo test-file`: schema-invariants 164/0, schema-scope-catalog tests 35/0, catalog-freshness 1/0, suites route table 5/0, schema command tests 21/0, schema-authoring 24/0, derive schema-index-stability 5/0.
- `repo typecheck` (library + schema-contracts folders, and the five touched .ts files) passed, 0 errors; `dangling-imports.py` exit 0 (scope-catalog, schema-contracts, test `🧬️schema`, suites).

## Not verified / notes
- `🎯️acceptance/🧪️tests/🎯️goal-gate` fails before the first test (Ajv2020 `addSchema` of a draft-07 `$schema`: "no schema with key http://json-schema.org/draft-07/schema#"). It reads `schema.$id` dynamically; not caused by this change, not fixed here.
- Repo-wide debt remains and is now explicit: 7957 definition modules, 5972 rows, 1985 definition modules without a row (1569 flagged `schema-module-id-missing`, the rest owner-ineligible/ambiguous). That is the generator's diagnostics list, unchanged in kind.
- `schema generate` itself is still a non-atomic `writeFileSync`; I did not touch the command.
- The freshness gate is race-prone by design under concurrent schema edits; the catalog is a snapshot at landing time (~17:3x).
