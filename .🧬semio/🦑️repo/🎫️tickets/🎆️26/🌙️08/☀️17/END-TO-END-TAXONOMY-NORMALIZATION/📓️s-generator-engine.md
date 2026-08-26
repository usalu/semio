# S-GENERATOR-ENGINE — Deterministic Generator Integration

## Outcome

The normalization engine now consumes the live schema-v7 `generatorContracts` registry and treats generated state as schema-owned state rather than ordinary renameable files. Planning remains read-only: it never invokes a generator or freshness target. Because the finalized schema has no preview target or deterministic expected-output manifest, an affected owned contract produces stable `generator-preview-protocol-missing` unresolved evidence instead of a guessed regeneration.

Production path changed:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts
```

Final production SHA-256:

```text
4bb788ea22d46d3f56893a45cfd8db20f0cb2ed13d71c7433cab86f77f6ec4cb
```

No schema, discovery, shared test, root script, manifest, Git, Compose, or temporary-Compose path was changed by this lane. No live generator was executed.

## Implemented contract

- Strictly loads ownership `owned | unsafe | unknown | external`, exact owner path, exact Nx target/check target, input globs, `tracked | ignored` output roots, and reason.
- Rejects malformed identifiers, arbitrary commands, duplicate roots, overlapping owners, nonliteral output roots, noncanonical paths, ownership/target mismatches, and any owner/input/output crossing the lexical opaque prefix.
- Includes ignored output state only by recursively walking exact registered `inclusion: "ignored"` roots. It filters opaque paths before filesystem access and never follows symlinks. It does not broaden discovery to arbitrary ignored files.
- Preserves exact schema-owned output roots. Noncanonical descendants or other output errors affect the owner, while generated files are never directly moved or reference-edited.
- Keeps ordinary directory source-to-normalized projections available to structured reference rewriting; only generated-owned outputs are excluded.
- Preserves recognized lexical references into `compose/**` as stable `opaque-reference-target` warnings without resolving, reading, or traversing the target.

The exported regeneration shape now carries exact deterministic state:

```ts
TaxonomyGeneratorNodeRecord {
  path;
  nodeKind;
  contentHash;
  mode;
}

TaxonomyRegeneration {
  id;
  contractId;
  cwd;
  command: ["bun", "nx", "run", target];
  verifyCommand?: ["bun", "nx", "run", checkTarget];
  outputRoots;
  inputs;
  preOutputs;
  outputs;
}
```

Every node array is unique and path-sorted. File hashes cover bytes, symlink hashes cover the lexical target, and every record carries its platform mode. Directory records make absent/new/stale directory state explicit. The regeneration ID is the first 24 hex digits of the canonical SHA-256 of its complete contract-owned record.

## Read-only planning behavior

For each contract, planning computes deterministic input and pre-output inventories and detects:

- a move/edit affecting a registered input;
- a move/edit overlapping a registered output;
- a generated descendant with a normalization or inventory error.

An affected `unsafe`, `unknown`, or `external` contract produces `generator-ownership-<state>` with stable input/output inventory digests. An affected owned contract produces `generator-preview-protocol-missing`, including its exact Nx target and inventory digests. No regeneration record is emitted without exact expected post-state bytes, modes, path set, and stale removals.

The missing schema/generator capability is precise: each owned target must expose a read-only preview protocol that writes nothing to the live repository and returns a canonical manifest containing complete expected output nodes plus removals. The manifest must be derived from the same generator implementation and inputs as the mutating target. A check target alone is insufficient because it cannot predict bytes before apply.

## Apply, verification, and rollback

Apply accepts only a regeneration whose contract is currently registered as owned and whose command, check command, owner cwd, roots, inputs, pre-state, post-state, and deterministic ID match the schema. The owner `📋️project.json` must contain the exact Nx project and targets.

Before mutation it verifies every input and the complete output-root pre-state. It rejects any output overlap with moves, edits, the taxonomy transaction tree, or an opaque path. For each regeneration it:

1. records the started owner in the journal;
2. backs up every pre-existing file and symlink and retains directory modes in the pre-state;
3. invokes the exact `bun nx run <target>` argv without a shell;
4. checks cancellation and progress between owners and command/check phases;
5. compares the complete output-root tree with expected paths, kinds, hashes, and modes, rejecting missing, stale, unexpected, byte-different, and mode-different nodes;
6. runs the exact registered check target when present;
7. includes the canonical output inventory in expected and actual affected-state digests.

On failure or cancellation, rollback removes every started output root, recreates the old directory tree/modes, restores all file and symlink preimages, and therefore deletes newly created and stale generator paths. Commit removes staging and backups while retaining the canonical journal. A resumed completed regeneration rechecks exact outputs and its registered check target.

## Evidence

Focused schema/load and engine scope:

```text
$ bun -e '<inventory + plan normalization scope>'
{"entries":7,"inventoryViolations":0,"moves":0,"edits":0,"regenerations":0,"unresolved":[]}
exit 0
```

Exact ignored-root inclusion and read-only preview blocker:

```text
$ bun -e '<inventory + plan actor generated root>'
entries:
  🧰️framework
  🧰️framework/🔨️modules
  🧰️framework/🔨️modules/🎭️actor
  🧰️framework/🔨️modules/🎭️actor/🤖️generated
  🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts
moves: 0
regenerations: 0
unresolved: generator-preview-protocol-missing
exit 0
```

All live owned command identities were checked against their owner manifests: 24 generate/check targets inspected, zero missing projects or targets.

Lexical opaque-reference preservation:

```text
$ bun -e '<inventory README.md and count opaque-reference-target>'
{"entries":1,"opaqueWarnings":35,"first":{"code":"opaque-reference-target","severity":"warning","path":"README.md","message":"markdown html-attribute:10:53@470 lexically targets excluded compose/asset/badge/🔣️site-🔣️play.svg"}}
exit 0
```

Focused normalization suite:

```text
$ bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' -t '^taxonomy normalization'
15 pass
196 filtered out
0 fail
182 expect() calls
exit 0
```

Bundle and diff checks:

```text
$ bun build '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts' --target bun --outfile '.🧬semio/.../🧪️generator-engine-build.js'
Bundled 15 modules in 16ms
exit 0

$ git diff --check -- '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts'
exit 0
```

Retained bundle artifact:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️generator-engine-build.js
SHA-256 f5717a9e944de461c8f2886ab66074ca1c61bf51949e776c861155c68b925ef9
```

Strict entry diagnostic:

```text
$ bunx tsc --noEmit --pretty false --strict --allowImportingTsExtensions --target ES2022 --module ESNext --moduleResolution Bundler --types node '.../🧹️normalization/🟦️.ts'
```

No normalization or discovery diagnostic was reported. Exit 2 remains solely from the two pre-existing transitive UI styling declarations: `ImportMeta.env` and `ImportMeta.glob`.

The Nx-owned `@semio-tech/repo-lib:test-quick` target was also attempted. It exceeded its 30-second quick budget and exposed unrelated existing failures caused by concurrent physical taxonomy/deletion work (old `📌️empty.md` fixture expectations, the intentionally removed Compose Neo4j expectation, an old Compose dependency-boundary fixture, and a historical styling path). The focused normalization packet above is green and is the relevant owned boundary.

## Acceptance status

- [x] Schema-owned ignored outputs are inventoried exactly, without broad ignored traversal.
- [x] Output ownership is unique and overlap/opaque boundaries fail closed.
- [x] Generated outputs are source/template-first and receive no direct moves or edits.
- [x] Planning runs no generator and does not predict bytes.
- [x] Missing preview support is a stable explicit unresolved contract.
- [x] Strong supplied records bind exact Bun/Nx commands, input/pre/post nodes, checks, digests, and modes.
- [x] Unexpected, missing, stale, byte-different, and mode-different output state fails verification.
- [x] Rollback removes new outputs and restores all preimages and directory modes.
- [x] Opaque digests are rechecked after generator/check execution.
- [x] Cancellation/progress boundaries and journal resume fields cover regeneration execution.
- [x] Existing focused normalization suite remains 15/15.
- [ ] Automatic regeneration planning awaits a schema-owned read-only preview manifest protocol; weakening determinism was intentionally rejected.
