# Admission-Derived Source-Kind Selection 63

## Scope

Read-only preparation only. No source tree census, filesystem walk, schema change, parser invocation, or mutation-identity conclusion occurred.

Captured inputs read:

- root [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts:20687), SHA-256 `5eb9cbfff2f505be52eef456cb6c26a310622f0fabff291a5277306c47d779e4`;
- public [discovery classifier](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:1180), SHA-256 `3aa680154aef6893065150a5072b0a78c3b9165e9225a92ad0e81ca0f6edc520`;
- taxonomy [file-kind registry](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:989), SHA-256 `84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce`.

## Existing Captured Authority

`mutationTaxonomySourceIndex(repoRoot, options, injectedAdmission?)` at root `📜️script.ts:20790` stores one `MutationTaxonomySourceIndex` with:

- `admission: TaxonomySourceInventory`, including every projected `observation` with exact `sourcePath`, `observedKind`, `worktreeMode`, origins, index tuples, generator identities, and `repositoryBoundary`;
- `files`, `bytes`, and `contents`, currently a smaller evidence projection;
- captured taxonomy and mutation-descriptor schema byte records.

`mutationTaxonomySourceFiles(admission)` at `📜️script.ts:20763` is already a pure admission projection: it rejects non-complete admission and returns byte-sorted observations only when `repositoryBoundary !== "gitlink"`, `observedKind === "file"`, and mode is `100644` or `100755`. It performs no path walk or content read.

The current `evidenceFile` at `📜️script.ts:20799` then narrows this set to a root descendant or a handwritten suffix regex. Consequently `index.files`/`bytes`/`contents` do not currently cover every registered language source kind, even though the full `index.admission.observations` retains their captured row facts.

## Existing Public Classification

`fileKindIdForSourcePath(path, taxonomy)` at discovery `🟦️component.ts:1181` normalizes only for comparison, matches the longest registered extension chain, and returns one unambiguous global file-kind ID or `null`. This is the usable pure classifier.

`scopedFileKindIdForSourcePath(path, taxonomy, context?)` at `🟦️component.ts:1191` is deliberately different: it only resolves owner-scoped evidence suffixes that satisfy a path-pattern and optional parent kind. The catalog probe returned `null` for every ordinary language source example, so it must not be used as the generic language-source selector.

The schema owns the six-language union through `Object.values(taxonomy.ecosystems).flatMap((ecosystem) => ecosystem.sourceFileKindIds)`. At the current taxonomy this deduplicates to:

| File kind | Registered physical chains |
| --- | --- |
| `rust-source` | `.rs` |
| `typescript-source` | `.ts`, `.tsx`, `.mts`, `.cts`, `.d.ts` |
| `javascript-source` | `.js`, `.mjs`, `.cjs` |
| `go-source` | `.go` |
| `python-source` | `.py` |
| `dotnet-source` | `.cs` |

## Proposed Pure Projection

The future census can receive only `{ admission, taxonomy }`, and select one row iff all predicates hold:

1. `admission.status === "complete"`;
2. `row.repositoryBoundary === null`, `row.observedKind === "file"`, and `row.worktreeMode` is `100644` or `100755`;
3. `const kindId = fileKindIdForSourcePath(row.sourcePath, taxonomy)` is non-null and belongs to the schema-derived ecosystem-source union above.

Sort selected rows by UTF-8 byte order of their raw `sourcePath`; retain that raw spelling, all admission evidence, and the resolved `kindId`. The classifier's normalized comparison must not replace the physical spelling retained in the output. This adds neither roots nor skip lists and does not infer a mutation/provider declaration from a selected source row.

If later parser content is required, the existing index must capture bytes for this same pure selected set while building the one index. It must not perform a second observation or filesystem traversal.

## Current Gaps and Limits

`evidenceFile` currently includes `.rs`, `.ts`, `.tsx`, JSON/schema/support formats, manifests, and mutation-root descendants. It omits `.mts`, `.cts`, `.js`, `.mjs`, `.cjs`, `.go`, `.py`, and `.cs` outside selected mutation roots. `.d.ts` happens to match its terminal `.ts` alternative but should be selected by the taxonomy classifier, not that incidental regex overlap.

`.jsx` is not one of the six current schema source-kind chains: the public probe produced `fileKindIdForSourcePath("fixture/🟦️.jsx") === null`. It cannot be admitted as TypeScript or JavaScript by this proposal without a separate schema-owned extension registration. This report does not authorize such a registration or a guessed fallback.

The default `loadTaxonomy()` runtime probe was blocked by unrelated missing `wgpu-frame-worker` generator outputs. A catalog-only `loadCatalogTaxonomy()` probe completed and observed the table above; it did not traverse a source tree. That classifier observation is not a complete admission run.

## Neutral Captured-Row Cases

1. One regular stage-zero captured row for each of `.rs`, `.ts`, `.tsx`, `.mts`, `.cts`, `.d.ts`, `.js`, `.mjs`, `.cjs`, `.go`, `.py`, and `.cs` selects its catalog kind with unchanged raw path spelling.
2. A regular `.jsx` row is retained in admission but excluded with an explicit `unclassified-source-extension` result; no fallback kind is invented.
3. A `.json` regular row is excluded because its `json` kind is not in the derived ecosystem source-kind union.
4. A symlink, `other`, directory, absent, unobserved, executable/nonregular, or `gitlink` boundary row is excluded before file-kind classification.
5. A non-complete admission is rejected before inspecting any row or kind.
6. NFC-equivalent path spellings classify identically while the output preserves the exact captured raw spelling and deterministic byte ordering.

These are admission/file-kind selection laws only. They do not prove language parser completeness, concrete mutation identity, or a complete off-facet census.
