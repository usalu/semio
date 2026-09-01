# Gitlink Inventory Preclassification Refusal Law

## Contract and bounded footprint

Read [`📓️admission-gitlink-57-contract.md`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-57-contract.md) in full. This packet adds only ticket-owned neutral vectors/schema and [`📜️script.ts`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/📜️script.ts); it did not edit normalization source, schema, S/N/D, planner/apply, or run a native/real nested-repository operation.

The controller extracts the actual current `inventoryTaxonomyWithSourceParentPruning` declaration, transpiles that exact captured declaration, and invokes it with closed service stubs. The admission stub has one `repositoryBoundary` observation; classification/content/projection stubs record and throw if the function reaches them. Ajv 2020 validates the neutral vectors before invocation.

## Frozen neutral cases

| Case | Observation | Desired assertion |
| --- | --- | --- |
| `initialized-boundary` | directory + `repositoryBoundary: "gitlink"` | Intentional repository-boundary error immediately after the one collection; no authored-directory, content, or projection call. |
| `absent-boundary` | absent + `repositoryBoundary: "gitlink"` | Same refusal; absence must not bypass the terminal Git-index boundary. |
| `ordinary-source` | regular file + `repositoryBoundary: null` | Reaches the first authored-directory classification sentinel, proving the guard is not a blanket inventory refusal. |

The vectors and their Draft 2020-12 schema are at [`🧫️fixtures/🔣️vectors.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/🧫️fixtures/🔣️vectors.json) and [`🧫️fixtures/🔣️schema.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/🧫️fixtures/🔣️schema.json).

## Retained actual RED

Command executed through scoped Nx:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/📜️script.ts' red
```

It exited `1` intentionally after writing [the receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/🧫️runs/red-d4e707a4-0d4d-40a6-be1a-3dcab2d818fb/🔣️result.json) and exact source/declaration captures in the same run directory. The current result has 12 assertions, 3 cases, and 4 desired-contract failures:

- initialized Gitlink reached `DOWNSTREAM:canonicalDirectory` once;
- absent Gitlink reached `DOWNSTREAM:projection` once;
- both lacked the required intentional repository-boundary error;
- the ordinary control reached only `DOWNSTREAM:canonicalDirectory`, as required.

The capture had no source drift. Input digests are:

- normalization source: `d0e84c70d4ae32cdf5035b5d94fda44c7c1d2420bead3d8b51b36b839e4f50bf`;
- extracted declaration: `b2b0fdb260b73fa6503aeb8adc9c2afa1dfeb9f7fa77c1908943242ec15f0417`;
- controller: `0b9452c5910abeea1e0b8c8954de41953058fe01f5eee29219f8ccffaadf821d`;
- neutral schema: `7894d916e93925318e1466319cddc020f51acb76eba4d3d98c72db5312d5eb7a`;
- vectors: `d4a4f102f26afcf7a902f3b9667882f94b4a7b8cbd36b16209dd90c2cfc9a1e7`.

This is source-level preclassification evidence only. It intentionally does not claim filesystem, nested-repository, planner/apply, compiler, or runtime behavior.

## Hardened pre-mount RED

The controller now has two explicit modes: `red` retains a nonzero desired-law failure before the guard is mounted, while `check` exits `0` only when every desired law passes and all captured inputs remain stable. It no longer forces a failure merely because the laws pass.

Every injected admission is now validated by Ajv 2020 against the actual canonical source-admission schema at [`normalization/🧬️schema/🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🔣️.json): both the input candidate and the full output admission are schema-valid. The Gitlink cases carry one tracked stage-0 `160000` entry with a 40-hex object ID, `repositoryBoundary: "gitlink"`, and empty `generatorOutputs`; the absent case uses the canonical absent tuple. No output observation carries `unsafeAncestor`.

The refreshed `red` invocation retained [a new receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-inventory-refusal-59/🧫️runs/red-e927fe70-c274-44d7-858c-1deb2a2c581d/🔣️result.json) with 18 assertions, the same three desired laws, and the expected four failures. It also captures and re-reads, with nofollow ancestor checks, the normalization source, canonical admission schema, neutral schema, vectors, and controller itself. Its stable digests were:

- normalization: `aece45f7980f07b393f23e2b0b3cacf7cd1aa8d857d2a63998f7361410a703be`;
- canonical admission schema: `1b88f7dfd1cd8f4809e690225af22251c798f7fac4526d993301eedca04afbc4`;
- neutral schema: `7894d916e93925318e1466319cddc020f51acb76eba4d3d98c72db5312d5eb7a`;
- vectors: `d4a4f102f26afcf7a902f3b9667882f94b4a7b8cbd36b16209dd90c2cfc9a1e7`;
- controller: `a9b6f615321efc9c6e02749f9f6c13d85cf8d1e019e7b0ce567aee964c68dd38`.

This remains pre-mount source evidence. The `check` command has intentionally not been called while the actual guard is absent, so no green result is implied.
