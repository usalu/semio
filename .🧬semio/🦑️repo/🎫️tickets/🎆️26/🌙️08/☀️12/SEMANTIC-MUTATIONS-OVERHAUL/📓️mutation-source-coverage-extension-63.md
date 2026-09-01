# Mutation Source-Coverage Extension 63

## Scope

Read-only inventory/API preparation. This report does not add a membership collector, expand a source capture, invoke an inventory, or claim any declaration is a mutation.

## Current Domain Boundaries

### Membership Is Already Canonical

[TaxonomySourceInventory](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:337) extends the canonical admission output: one status, observed rows, diagnostics, repository root, taxonomy path/hash, and membership digest. Its row has raw `sourcePath`, physical kind/mode, origins, index tuples, generator outputs, and `repositoryBoundary`; it has no parser or mutation-identity field.

The schema-owned form is [normalization sourceAdmission](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json:351) and [sourceAdmission output](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json:395). It must remain membership/observation-only: putting parser support or mutation census facts there would create a second semantic authority at the wrong layer.

### Mutation Index Is the Only Capture Boundary

The root-private [MutationTaxonomySourceIndex](/Users/ueli/Documents/semio/📜️script.ts:20687) already retains exactly one `admission`, its selected `files`, captured `bytes`/`contents`, captured taxonomy/descriptor schemas, `sourceRoster`, and `sourceTreeDigest`. [mutationTaxonomySourceIndex](/Users/ueli/Documents/semio/📜️script.ts:20790) obtains/injects the admission once, then reads only its existing evidence projection.

The public [MutationTaxonomyInventory](/Users/ueli/Documents/semio/📜️script.ts:20619) returns a closed v2 object with `sourceTreeDigest`, `roots`, `sourceRoster`, mutation-root `records`, `unresolved`, and `violations`. There is no coverage row today.

## Exact Coverage Extension Point

Add a private, admission-derived coverage projection to `MutationTaxonomySourceIndex`, then expose its serialized rows next to `sourceRoster` on `MutationTaxonomyInventory`. It must derive from the already-captured admission and the already-captured taxonomy bytes; it must not call `inventoryTaxonomySources`, walk a root, or resolve any provider.

Proposed output vocabulary, subject to root schema review:

```ts
type MutationTaxonomyParserSupport = "supported" | "unsupported" | "unclassified";
type MutationTaxonomySourceCoverageState = "complete" | "incomplete" | "unknown" | "unclassified";
interface MutationTaxonomySourceCoverageRow {
  readonly sourcePath: string;
  readonly fileKindId: string | null;
  readonly parserSupport: MutationTaxonomyParserSupport;
  readonly state: MutationTaxonomySourceCoverageState;
}
```

The row retains the raw admission spelling. `fileKindId` is the existing taxonomy classifier result, not a name-based language guess. `supported` means a specific owned structural/declaration inspector was actually selected; only that inspector may yield `complete` or `incomplete`. `unsupported` yields `unknown`; `unclassified` yields `unclassified`. No row carries a provider, mutation name, or identity conclusion.

For initial all-repository coverage, classify every admitted regular non-gitlink file that is captured by the index, not only six ecosystem languages. A language-focused later parser can select its supported kinds from these rows, while JSON/schema/grammar/document/test and future registered kinds remain explicit `unsupported`/`unknown` rather than silently disappearing.

## Existing Schema and Consumer Surfaces

| Surface | Current shape | Required later change |
| --- | --- | --- |
| Root public types | `MutationTaxonomySourceRecord` and `MutationTaxonomyInventory`, `📜️script.ts:20617–20627` | Add the output row/type and define ordering/digest participation. |
| Root private capture | `MutationTaxonomySourceIndex`, `📜️script.ts:20687`; evidence selection, `:20799` | Derive coverage selection from the same admission and taxonomy capture; do not create another collector. |
| Root serializer | `runMutationTaxonomyCli`, `📜️script.ts:21058–21072` | It serializes the returned object through `taxonomyCliWriteJson`/`taxonomyCliPrintJson`; no separate serializer exists, but emitted JSON changes automatically. |
| Inventory output schema | [mutation inventory schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🛂️schema/🔣️inventory.json:1) | Add a closed `sourceCoverage` definition/property and decide schema-version handling. Its current `sourceRoster.role` enum is already narrower than root’s type: it omits `taxonomy-schema` and `mutation-descriptor-schema`. |
| Inventory fixture | [consumer fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️consumers.json:1) | Add neutral raw paths for supported, unsupported, and unclassified rows without generating expected facts from implementation. |
| Direct inventory test | [index.test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:6848) | Update its inline closed Ajv output schema, currently `schemaVersion: 2`, and assert deterministic coverage rows. |
| Consumer/digest test | [index.test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:6873) | Extend fixture-schema validation and source-tree digest/change assertions for coverage bytes. |
| CLI terminality test | [index.test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:7019) | Preserve plan/apply behavior while asserting coverage rows do not manufacture moves or mutation records. |

## Can Capture Widening Be Isolated?

Not completely. `before.files` drives the current consumer graph and `sourceRoster`; `sourceTreeDigest` hashes that roster at `📜️script.ts:20818` and protects the before/after retry at `:21014`. Merely widening `files` would change the public roster/digest and may make unsupported source paths flow through the existing graph loop, even before coverage rows are exposed.

A safe staged boundary is therefore:

1. add private `coverageFiles`/`coverageBytes`/`coverageContents` derived from the same admission and taxonomy capture, with a coverage digest included in the stable index digest; do not feed it into structural mutation-root/consumer logic;
2. add public `sourceCoverage` and its schema/tests in the same output-version change; and only then let supported parser facts populate `complete`/`incomplete`.

This retains one membership authority. It is not a second traversal: every coverage row begins with an existing admission observation and only captures leaf bytes under the source-index no-follow boundary.

## Neutral Cases for the Future Schema

1. A captured Rust row with an actual supported structural fact is `supported`/`complete` or `supported`/`incomplete` according to the inspector, never according to a filename.
2. A TypeScript row with a parser recovery fact is `supported`/`incomplete`; the raw source path and catalog kind remain intact.
3. A classified Go/Python/.NET/JavaScript row with no owned parser is `unsupported`/`unknown`.
4. A registered non-source kind such as JSON/schema is explicitly `unsupported`/`unknown`, not omitted from an all-captured coverage report.
5. An unregistered suffix has `fileKindId: null`, `unclassified`/`unclassified`; no language fallback is guessed.
6. Symlink, directory, absent, unobserved, nonregular, or gitlink-boundary admission rows are not byte-captured and cannot claim parser completeness.
7. Two NFC-equivalent paths retain their raw spellings and byte ordering; classification normalization does not rewrite output identity.
8. A source-byte change in a covered but unsupported row changes the coverage/index digest and forces the existing before/after retry, without creating a mutation record.

These cases remain coverage evidence only. They do not establish concrete mutation identity, source reachability, codec law adequacy, or whole-repository semantic completeness.
