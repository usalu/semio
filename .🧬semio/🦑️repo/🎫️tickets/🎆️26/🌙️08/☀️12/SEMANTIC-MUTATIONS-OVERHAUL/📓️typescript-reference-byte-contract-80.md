# TypeScript Compiler Reference Byte Contract 80

## Frozen Boundary

This is implementation preparation only. It does not alter or rerun packet 75,
and adds no executable controller. Every future case is a self-contained
neutral record with `files: [{path, utf8}]`, `query`, and `expected`; fixture
construction must not branch on case ID.

## Closed 23-Case Roster

| Cases | Explicit aggregate source distinction | Expected |
| --- | --- | --- |
| `vcs-direct`, `vcs-named-alias`, `vcs-union-selected` | Direct `Leaf`, named alias, and `Leaf \| Delete` respectively; each imports the canonical leaf specifier. | bound |
| `gis-keyed`, `gis-union-selected` | `{ Key: Leaf }` and `{ Key: Leaf } \| { Other: OtherLeaf }`. | bound |
| `wrong-export`, `wrong-provider`, `missing-leaf`, `missing-import` | Imported export spelling differs; import resolves another virtual provider; canonical leaf bytes absent; no import declaration. | rejected |
| `wrong-discriminant`, `optional-discriminant` | Leaf literals are `"wrong"` and optional `mutation?: "rename"`. | rejected |
| `wrong-key`, `optional-keyed-property` | Key text differs; exact key has `?`. | rejected |
| `namespace-import`, `default-import` | `import * as N` and default import, never named import. | rejected |
| `incomplete-syntax` | Aggregate bytes contain an intentional parser diagnostic; query declares incomplete facts. | rejected |
| `shadowed-generic` | Imported `Leaf` plus `type Aggregate<Leaf> = Leaf`; query targets the use-site symbol. | rejected |
| `ambiguous-competing-alias` | Two named aliases of the same canonical leaf occur in the selected union. | rejected |
| `physical-omitted-surface` | Valid physical bytes plus query `requiredLanguageSurfaces` lacking TypeScript. | unsupported |
| `captured-vcs-six-member`, `captured-gis-two-member` | Exact current aggregate bytes and every direct imported leaf under their original canonical relative paths. | capture-only |

The remaining named 72 cases are retained in the first four rows: the roster
is exactly 23, with duplicate direct fixtures removed only where source bytes,
query, and expected result are identical.

## Required Algorithm and Read Set

Before any `Program`, guarded nofollow reads capture the controller, vectors,
schema, TypeScript package manifest, recursively referenced standard-library
bytes, and every fixture source byte. Program caches use `(path, sha256)` keys
and hosts return only those in-memory bytes. Parent starts draining stdout and
stderr concurrently before waiting; it clears its timer on either terminal
path, kills only its child on timeout, then awaits that child’s final exit.

For each non-capture-only case, the checker resolves the aggregate named import
at the selected union member, calls `getAliasedSymbol`, and requires its
declaration source path and exported symbol to equal the query’s canonical leaf
provenance. Literal members require the exact non-optional property and string
literal; keyed members require the exact non-optional key and type-reference
symbol. Exactly one selected member is required. Capture-only VCS/GIS queries
may parse and inspect the original export/import closure but cannot become
`bound`: current descriptors lack mandatory TypeScript binding metadata.

No descriptor path override, new parser, runtime dependency, or filesystem
fallback is permitted.
