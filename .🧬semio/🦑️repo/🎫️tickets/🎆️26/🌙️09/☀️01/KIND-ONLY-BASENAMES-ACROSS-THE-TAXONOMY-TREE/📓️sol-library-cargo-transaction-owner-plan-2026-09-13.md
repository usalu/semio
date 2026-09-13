# Library Cargo, Transaction, and Go Owner Plan

## Live source map

This plan was retained before moving behavior.

The live Cargo command module `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts` has four concerns:

1. generic owned-process progress, timeout, signals, and process-tree retirement;
2. Cargo selector admission and artifact-router argument projection;
3. Cargo JSON receipt capture, source-project containment, dependency deliverables, and artifact staging; and
4. native command selection for metadata, component, build, check, and test.

The live production API population is fourteen modules:

- artifact-build consumers:
  - `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🦀️native/📜️script.ts`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️publication/🟦️.ts` is a separate private native test consumer.
- generic owned-process consumers:
  - `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts`
  - `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`

The production count is eleven artifact-build importers plus four process importers with artifact-Rust as their single overlap: fourteen distinct production modules. The publication test, cache-contract dynamic/source-data inspection, and Trunk executable hook are separate relationships. The former scheduled-prune import is absent and will remain absent.

The repository-library package router has two bounded behavior areas:

- `GoTestScript` projects one requested filesystem input to a canonical Go compiler package and delegates to the existing canonical planner/runner.
- `transactionV2BundleRoot` and the `test transaction-v2` branch allocate a no-follow run, capture input identities, build the suite/normalization bundles, execute four bounded shard selections, retire descendants, verify exact boundary coverage, and retain before/after/outcome receipts.

The transaction branch currently points at two absent package-local fixture paths. The actual authorities are the library-level ledger and harness fixtures, and the transaction suite already reads them.

## Planned anonymous owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts`
   - `startNativeProgress`, `runOwnedCommand`
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🎛️native-input/🟦️.ts`
   - `validateNativeCargoArguments`, `artifactRustCargoArguments`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`
   - `buildCargoArtifacts`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`
   - `NativeScript`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📁️run-allocation/🟦️.ts`
   - current-ticket no-follow run allocation and `transactionV2BundleRoot`
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🧾️provenance/🟦️.ts`
   - actual owner/test/normalization/taxonomy/fixture source identities and immutable receipts
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts`
   - exact 62-case shard selection, process observation, bounded cancellation, and boundary registry collection
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts`
   - bundle construction, before/after identity comparison, shard/result coordination, and final boundary receipt
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️execution/🐹️go/🟦️.ts`
   - explicit Go input-to-package admission and `GoTestScript` delegation to the existing canonical Go plan/runner

The Cargo and repository-library command modules retain only mandatory command routing. No behavior owner is placed under a literal `📦️packages` directory.

## Consumer and source-data closure

- Rebind all fourteen current production imports to the owned-process or native-build owner by meaning.
- Split artifact-Rust's five imported operations among owned execution, native input, and native build.
- Rebind the publication test directly to native build.
- Rebind the cache-contract dynamic/source-as-data assertions to the three actual behavior owners while retaining the Cargo command module only for executable command coverage.
- Keep Trunk and project command strings on the mandatory Cargo command route.
- Rebind the transaction harness fixture and installed-TypeScript source inspection to allocation and shard owners. Identity receipts must name all four transaction sources rather than hashing the package router.
- Correct both transaction fixture coordinates to their library-level authorities.
- Keep the existing Go planner/runner in `📚️library/📦️packages/🟦️typescript/🟦️.ts`; the new owner imports and delegates without copying it.
- Preserve the existing `go-test`, `test-go-input-projection`, `test-go-dispatch`, and `test-transaction-v2` command and launch identities.

## Validation boundary

A new schema/fixture/source test will precede behavior movement and cover the nine owners, complete consumer classes, contexts, acyclic imports, exact command/source inputs, transaction fixture/source identities, portable argument and Go-selection vectors, owned-process cancellation, and seed/derived launch entries. The installed Ajv and TypeScript implementations remain the independent schema/compiler oracles.

Native evidence will use the private Cargo publication fixture and the existing selected Go dispatcher under ticket-private output roots. The full transaction aggregate is excluded because it performs Git fixture mutation and retained normalization transaction work. Its source, selection, cancellation, and output topology will be checked without executing it.

No live transaction, cleanup, pruning, shared reset, installation, service operation, Git mutation, AGENTS change, or compatibility facade is authorized.
