# 🧾️ Mutation Source File Facts 64 Plan

## Boundary

This is schema-first preparation only. The proposed production pure projection is `mutationTaxonomySourceFileFacts(admission: TaxonomySourceInventory, taxonomy: Taxonomy)`, adjacent to [`mutationTaxonomySourceFiles`](../../../../../../../../📜️script.ts) in the root script. It must first reject a non-`complete` admission. For every admitted observation whose `repositoryBoundary` is not `gitlink`, `observedKind` is `file`, and `worktreeMode` is exactly `100644` or `100755`, it emits one row:

```text
{ sourcePath, fileKindId: string | null, fileRole: FileKindSpec.role | null }
```

Rows are byte-sorted by the raw `sourcePath`. The projection composes the existing `mutationTaxonomySourceFiles` selection with `fileKindIdForSourcePath`; it performs no IO, traversal, new scope/root/skip selection, provider identification, or parser invocation. An unresolved extension is retained as `{ fileKindId: null, fileRole: null }`. The taxonomy's existing NFC normalization and longest extension-chain resolution remain the resolver's authority; the emitted `sourcePath` preserves its physical spelling. In particular, `.jsx` remains unresolved until the taxonomy registers it.

The resolved role is copied from the taxonomy's full `FileKindSpec.role` vocabulary: `source`, `schema`, `specification`, `configuration`, `documentation`, `test`, `asset`, `generated`, or `marker`. The test set will use actual taxonomy entries rather than a six-ecosystem surrogate.

## Canonical Test Surface

On approval, the only new canonical L test files will be:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-file-facts/🔣️.json`

The schema closes the vector object and each observation/expected row. It models complete and rejected admissions separately, raw physical paths, mode/kind/boundary inputs, and exact expected rows. The test validates vectors with Ajv Draft 2020-12. It will call the actual root projection through its exported declaration; before that export exists, the subject lane must fail nonzero rather than treating absence as a pass.

The independent test-only oracle will use installed `minimatch` only to choose the longest matching suffix from an explicit immutable extension-chain table constructed from the captured taxonomy. Its independently selected suffix must agree with the actual projection's kind/role rows; it is not a runtime dependency and does not determine admission eligibility.

## Neutral Cases

The fixed vectors will cover:

- all currently registered source chains, including `.mts`, `.cts`, `.js`, `.mjs`, `.cjs`, `.go`, `.py`, `.cs`, C/C++ headers and source forms, shell/PowerShell, Swift, SQL, Cypher, and assembly;
- schema, specification, configuration, documentation, test, asset, generated, and marker roles through actual taxonomy file kinds;
- longest-chain precedence (`.d.ts`, `.grammar.semio`, and `.dsl.semio`) plus raw NFD spelling with NFC comparison;
- an unknown `.jsx` source retained with both classification fields null;
- executable regular `100755` accepted alongside regular `100644`;
- nonregular, absent, and gitlink observations excluded; a rejected admission throws before projecting any row;
- virtual `CoMpOsE` exclusion exercised only through a preprojected rejected admission, never through a real workspace path.

This packet deliberately makes no semantic mutation-owner, content, or parser claim. It is a complete-admission file-fact projection contract only.

## Current Evidence And Requested Next Step

`mutationTaxonomySourceFiles` currently exists at root [`📜️script.ts:20763`](../../../../../../../../📜️script.ts:20763) and already has the intended regular-file/gitlink filter. `fileKindIdForSourcePath` is the public taxonomy longest-chain resolver at [`discovery/🟦️component.ts:1181`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:1181); its `FileKindSpec.role` is the complete role union at [line 81](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:81). Ajv and `minimatch` are currently resolvable installed transitive test tooling, not direct root/P manifest dependencies; the test oracle must fail explicitly if either becomes unavailable. Neither is a runtime dependency.

No canonical test file, root script, production source, launch configuration, or Census has changed. Awaiting review before authoring the neutral files, running the reference, or capturing the missing-export RED.
