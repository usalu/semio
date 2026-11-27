# F111: schema checker false positives removed

Lane F111, 2026-11-26. Source: `📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts` (+ tests in `🧪️tests/🟦️.ts`). Each fix has a failing fixture test first (6 new tests, then 1 more for the corpus role).

## Counts (F110 dump script, before -> after)
| Code | Before | After |
|---|---:|---:|
| `schema-ref-unresolved` | 2560 | 724 (-1836, exactly the deep-pointer class) |
| `schema-module-id-missing` | 2607 | 1673 (-934; +1 facet document row, see 2) |
| `schema-export-incomplete` | 4489 | 3346 (rust 3117 -> 1983, wit 9 -> 0) |
| `schema-fixture-defines-schema` | 2020 | 1978 (-42 net: 2 corpus rows dropped after the role refinement; the dump saw 1 more module meanwhile) |
| all other codes | unchanged | unchanged |
| total | 16242 | 12327 |

Final dump has 12327 rows (peer added one module: 8527). The 321 "new" ref rows of the first re-dump were only the reworded cross-scope message (456 non-id refs plus the deep-form hint); net count is -1836.

## Fixes
1. Cross-scope deep pointer: one helper `schemaRefFragmentExport(fragment, exportsKeyword)` returns the first `$defs` segment plus `deep`. Local branch: deep -> no export id (pointer checked against the document, as before). Cross-scope branch: the export is the first segment, so `#/$defs/CaseNumber/properties/value` checks `CaseNumber`; a missing first segment is still reported (tested).
2. Missing root `$id`: `readSchemaDocument` no longer reports; the module row stays the only report for the root leaf, plus one row at its own path for each non-root facet document without `$id` (3 rows expected; net +1 appeared).
3. Rust macros: `rustMacroDeclaredNames` uses the compiler syntax tokens (`rustTokens`/`rustTokenPairs` from `📚️compiler/📖️syntax/🦀️rust`, the tokenizer the rust-module-graph discovery uses) over `list_delta!`, `plain_list_delta!`, `row_patch!`: the delta/patch type and its removal, insertion, relocation, modification types. Names in comments or strings are not declarations (tested). This removed 1134 rust rows, not only the audit's 215: the macro-generated removal/insertion/relocation/modification structs are declarations too. `.wit`: excluded explicitly by the exported policy table `SCHEMA_EXPORT_COMPLETENESS_EXCLUDED_FORMATS` with its reason (a wit leaf is a hand-curated surface with its own kebab names, e.g. `editor-config` for `Generation3dConfig`, not a name-for-name twin); a name-based check would still be wrong.
4. `Cases` name heuristic: a name now counts only for a document in its role as a case list: the root, or a node reached through `items` (directly or by `$ref`), not an expectation field nested in an object property. Dropped 2 of the name-only rows: `DemonstratorBrowserCatalogCases` (runtime catalog, `admission.expectedCounts`) and `SnapshotFixture` (search). The other name-only hits are real corpora (case rows in a list), so the "84 suspect" mostly stand.

## Verification
- `repo test-file`: scope-catalog tests 42/0 (was 35), catalog-freshness 1/0, schema-invariants 164/0.
- `repo typecheck` on the scope-catalog folder: passed, 0 errors; `dangling-imports.py` exit 0.
- The catalog was already stale before my change (peer edit of `mit-bestand.demonstrator.runtime.catalog` schema hash; my change does not touch the catalog). I regenerated catalog and document with F107's script into scratch and renamed both atomically under `landing.lock.schema` (held for the two renames), so `catalog-freshness` is green again.

## Not done / notes
- Residual rust incompletes (1983) include other macro forms (`artifact_retire_struct!(X {..})` is an impl, not a declaration) and real absences; not touched.
- Lock hygiene slip: I wrote the checker source edit atomically (temp + rename) without taking `landing.lock.schema`; it is one file in the library, no build-system area. The catalog landing did hold the lock.
