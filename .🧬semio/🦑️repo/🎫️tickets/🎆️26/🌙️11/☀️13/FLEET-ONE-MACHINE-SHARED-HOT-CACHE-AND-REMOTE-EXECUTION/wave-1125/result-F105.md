# F105: schema-invariants 163/1, catalog stale for four test schema folders (not fixed)

Lane F105, 2026-11-26. Test `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` via `repo test-file`: **163 pass / 1 fail** before and after (no edit landed).

## Findings
- Failing case: "the test platform's own subtree carries no schema-contract finding". Four `schema-catalog-stale` findings, each "carries a schema definition and no catalog row names it":
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧬️schema`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧬️schema`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🧬️schema`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema`
- Rule: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/🟦️.ts` (`schemaMeasurementDisagreementDiagnostics`), compared against the catalog `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`.
- The generator (`repo run repo-schema-commands:schema-generate`, rendering `inventorySchemaScopes` + `renderSchemaCatalog`) emits **no row** for any of the four paths. The live catalog has none either. Their `🧬️schema` folders hold a `🔣️.json` the generator's walk does not treat as a scope, so the test's on-disk walk and the generator disagree. Hand-adding four rows would pass the test with a catalog `schema generate` would never write, so I did not do it.
- A full generator render (read-only, via `🧰️scripts/F105/render-catalog.ts`) shows the live catalog is broadly stale across the repo: 44 scopes missing and 2844 rows differing. A full regenerate is therefore not a four-folder fix, and it would rewrite that whole file.

## Not done
- No catalog write, no landing, no lock taken. `schema-catalog.json` is byte-identical to its snapshot (sha256 prefix 790feed5cedb4d58).
- `repo run` for the regenerate stalled on the graph read (`door.script`, 30+ min); cancelled by SIGTERM and TaskStop, nothing else touched.

## Needed from coordinator
Decide between (a) the test-domain folders get a catalog-visible shape the generator recognises (a schema-owner change in those folders), or (b) the rule's on-disk walk excludes `🔣️.json`-only folders like the generator does. Both change owned files outside this lane; the brief said not to change the rule.
