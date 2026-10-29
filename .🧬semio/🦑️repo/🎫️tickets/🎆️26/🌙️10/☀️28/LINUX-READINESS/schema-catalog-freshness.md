# Current Schema Catalog Freshness

The existing owned Root commands completed successfully:

- `repo ./📜️script.ts schema generate`: terminal0,3626scopes,20071inventory diagnostics.
- `repo ./📜️script.ts schema docs`: terminal0, canonical index regenerated.

Only the derived catalog and its canonical index were regenerated. No schema assets, authority declarations, eligibility laws, validators, parsers or references were changed. No Source files were copied to this ticket. The generated before-state artifact contains fingerprints and identity metadata only.

## Projection Changes

| Measure | Before | Current |
|---|---:|---:|
| Catalog scopes |3656|3626|
| Added identities | |44|
| Removed identities | |74|
| Changed surviving rows | |709|
| Surviving identities with different owner paths | |39|
| Current catalog owners physically missing | |0|

Of74removed identities,71old owner directories are physically absent. The other3are the puzzle2d/3d/5d presence modules: their actual current JSON `$id` declarations use `s.puzzle.*.presence`, and those3identities appear among the44added rows at the same paths. This is projection of current authored identity; no asset was renamed or migrated.

The generator admits catalog rows only for modules with both a declared scope ID and an eligible owner level. Current rows use13declared levels, including2869mutation-leaf rows. Ambiguous scope claimants remain diagnostics and the existing deterministic claimant rule remains unchanged.

Evidence: `🗑️generated/schema-catalog-before-fingerprints.json`, `schema-catalog-delta.json`, `schema-catalog-generate.log`, `schema-catalog-docs.log`.

## Same Harness Measurement

The actual taxonomy declares the testing domain at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. The earlier triage report's shorter framework-module path was incorrect; that different script is a forwarded receiver and correctly refused an absent Original. No receiver guard changed.

The exact baseline issuing command was then run once:

`repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json`

It produced the complete JSON and terminal1, retaining the failing gate. Diagnostics decreased from23031to22612, a net419. Fixtures remain empty in both measurements; this does not prove any schema fixture stage passed.

| Diagnostic family | Before | Current | Difference |
|---|---:|---:|---:|
| Catalog malformed |110|0|-110|
| Catalog stale / unindexed module |1943|1754|-189|
| Export unknown |197|1|-196|
| Cross-scope dependency forbidden |2|0|-2|
| Export incomplete |5042|5112|+70|
| Export parser missing |1272|1290|+18|
| Export format undeclared |146|146|0|
| Referenced schema unresolved |841|842|+1|
| File missing |9|9|0|
| Internal reference broken |1|1|0|
| Owner ineligible |820|821|+1|
| Dialect not draft07 |518|521|+3|
| Fixture defines schema |3775|3777|+2|
| Placement outside module |523|523|0|
| Forbidden filename |256|256|0|
| Mutation input UI |3462|3445|-17|
| Mutation payload parity |664|664|0|
| Mutation editability |2|2|0|
| Fault notice |3448|3448|0|

These are before/current measurements during concurrent Source work, not an isolated causal estimate attributing every change to generation. Refreshed catalog authority exposes some additional export/parser obligations. Existing missing implementations and authored references were retained.

## Why1754Unindexed Findings Remain

Every remaining `schema-catalog-stale` finding says a physical module carries a definition but no catalog row names it. None is a current catalog-row checksum mismatch. Applying the actual owned eligibility and ID readers to those exact1754paths gives:

| Current authority condition | Count |
|---|---:|
| Ineligible owner path |591|
| Missing canonical root document ID |731|
| Root ID rejected by the canonical scope-ID reader |317|
| Valid scope already indexed at a different owner |115|

The classification uses `schemaScopeModuleOwnerPath`, `schemaScopeOwnerLevel`, `schemaScopeCollectionPath` and `schemaScopeIdFromDocumentId`, with current catalog taxonomy. It does not invent a filename-only schema classifier. The115duplicate claims require authored ownership decisions; inserting all claimants into an index keyed by scope ID would erase that boundary.

The9missing-file findings name absent format implementations of declared facets, for example presence/config diff TypeScript/GraphQL/protobuf files. They remain after a fresh projection; generation cannot supply those implementations. The one remaining unknown export is `valueType` from `framework.value.type`, referenced by the trinity jack schema.

Full evidence: `🗑️generated/schema-diagnostics-catalog-current.json`, `schema-catalog-diagnostics-comparison.json`, `schema-catalog-coverage.json`. The reusable handwritten classification input is `schema-catalog-coverage.ts`.

The catalog/index are refreshed. The complete schema gate remains failing and its residual authority, dialect, format, parser, fixture, domain UI/payload and reference obligations remain open.


The final owned read-only freshness command, `repo ./📜️script.ts schema generate --check`, completed terminal0 after the diagnostic measurement. The catalog still matches the current canonical inventory (3626scopes); the1754authority omissions above are not generator-output byte drift. Evidence: `🗑️generated/schema-catalog-freshness-check.log`.
