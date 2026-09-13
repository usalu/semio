# Library Cargo, Transaction, and Go Command Ownership Extraction

## Outcome

The bounded Cargo command API and repository-library transaction/Go implementation slice now has nine anonymous implementation leaves, twelve registered semantic directory contexts, seven acyclic internal owner edges, and no owner back-import to either command module. The Cargo command module only registers the native router. The library package command delegates transaction verification and Go input selection to their owners while retaining its literal route table.

This slice deliberately does not claim the whole library package command is implementation-free. `WorkspacesScript` and the historical ticket-important-FEM generator/preview/check bodies remain in that command and are assigned to the separate `📓️library-workspaces-historical-handoff-followup-2026-09-13.md` packet.

## Owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts`
   - `runOwnedCommand`
   - `startNativeProgress`
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🎛️native-input/🟦️.ts`
   - `artifactRustCargoArguments`
   - `validateNativeCargoArguments`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`
   - `buildCargoArtifacts`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`
   - `NativeScript`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📁️run-allocation/🟦️.ts`
   - `TRANSACTION_V2_RUN_OWNER_RELATIVE`
   - `transactionV2BundleRoot`
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🧾️provenance/🟦️.ts`
   - `retainTransactionV2Record`
   - `transactionV2Identities`
   - `transactionV2IdentityPaths`
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts`
   - `TRANSACTION_V2_DEFAULT_FILTER_WAVES`
   - `TransactionV2ShardOptions`
   - `TransactionV2ShardOutcome`
   - `TransactionV2ShardResult`
   - `runTransactionV2Shards`
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts`
   - `runTransactionV2`
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️execution/🐹️go/🟦️.ts`
   - `GoInputSelectionOperations`
   - `GoTestScript`
   - `projectGoInputPackages`

The source fixture binds every owner to its ordered semantic context chain and compares the context-chain directory names with the owner path. The union must equal the twelve new contexts, so an unrelated list of valid taxonomy rows cannot satisfy the contract.

## Preserved behavior

- Generic owned commands retain progress, bounded timeout, SIGINT/SIGTERM handling, POSIX process-group retirement and the established Windows `taskkill /pid ... /t /f` branch.
- Cargo input admission rejects caller-controlled workspace/package/manifest and non-test target expansion.
- Native builds retain Cargo JSON receipt parsing, primary source-project containment, linked dependency staging, shared compiler intermediates, private capture output and terminal cleanup.
- The native command owner retains locked metadata, component manifest validation, component header validation, Cargo build/check/test routing, and the internal `@iarna/toml` parser boundary without exporting parser types.
- Transaction allocation now belongs to the active ticket and admits creation only of its fixed generated/run ancestors. Every traversed ancestor is checked with `lstat`; links and non-directories are rejected.
- Transaction receipts identify allocation, provenance, shard execution, orchestration, suite, normalization, discovery, taxonomy, and the two real library-level fixture authorities. The package router is no longer used as a behavior identity.
- The 62-case transaction filter partition remains source-owned by shard execution. Shard output, PID/boundary registries, timeout and descendant retirement remain separate from bundle/result orchestration.
- Go input projection handles virtual replacement owners and delegates overlay planning and native execution to the existing `canonicalGoPlan` and `runCanonicalGoTests` authorities.
- The former scheduled Cargo cache-prune API consumer remains absent. The workspace-cleanup executable `cache-prune` route and the Trunk executable hook remain command relationships.

## Consumer and cache closure

The source contract records fourteen distinct production Cargo/process consumers: eleven native-build consumers and four owned-process consumers with artifact Rust as their one overlap. It also records the Cargo router, the library router, the private publication test, and the cache-contract dynamic/source-data test, for eighteen consumer records total. Static imports are parsed and checked against the declarations of the exact owner they resolve to; dynamic source-data rows must contain the exact owner path. No production import of the former Cargo command API remains.

The caching command-boundary producer now points at the native-build owner. `cacheCommandSources` includes native input/build/orchestration, owned execution, and the command-boundary fixture/schema. The library package project now has explicit `cargoTransactionCommandSources`, `libraryTransactionCommandSources`, and `libraryGoCommandSources` named inputs, used by the source, transaction, Go projection, Go dispatch, and Go command targets as appropriate. The generic native producer continues to derive the Cargo router's complete relative import closure; a current comparison found ten Nx inputs and ten esbuild inputs with zero missing or extra paths.

The new registered route is `@semio-tech/repo-lib:test-cargo-transaction-command-source`, called by package script and both seed/derived launch entries as `🧪️test🦀️cargo-transaction-command-source`. Existing Go and transaction route identities remain unchanged.

## TDD and runtime evidence

The corrected schema-first red run passed 1 test and failed 9 tests with 13 assertions before the owners, contexts, route, and consumer migration were complete.

Final current-source checks:

- Direct portable source contract: 10 tests, 270 assertions, 0 failures, 5.32 seconds.
- Ordinary package command `bun ./📜️script.ts test cargo-transaction-command-source`: 10 tests, 270 assertions, 0 failures, 3.75 seconds.
- Isolated registered Nx target with daemon/plugin isolation, lane-private workspace/cache/artifact/temp roots, and cache skipped: 10 tests, 270 assertions, 0 failures; Bun body 4.27 seconds, Nx target 4.6 seconds.
- Strict installed-TypeScript owner program after formatting: 1 selected test, 17 assertions, 0 failures; nine owners and seven internal edges, with no cycles or owner-to-command back edge.
- Cargo router and library router Bun compilation: two outputs, zero compiler logs.
- Cargo cache closure against installed esbuild: 10 actual inputs, 10 expected inputs, zero missing and zero extra.
- Private native artifact publication: Python content oracle, Cargo offline executable oracle, shared compiler-intermediate retention, private deliverable staging/retirement, and synthetic WASM success/failure/missing-output controls all passed.
- Focused native-preparation control: native argument/progress/runner behavior and Cargo metadata production/test dependency closure passed; two deliverables staged in the private fixture.
- Ordinary Go projection: 1 test, 13 assertions, 0 failures, 3.41 seconds.
- Isolated Nx Go projection: 1 test, 13 assertions, 0 failures; Bun body 1.35 seconds, Nx target 1.5 seconds.
- Selected Go dispatcher: 2 tests, 9 assertions, 0 failures, 19.87 seconds. The native Go oracle passed its command, cancellation and spawn-error cases; the outer cancellation control observed six descendants across three process groups, returned within its bound, retired all observed processes, and removed its overlay directory.
- Prior cache-command source regression after source-data/input rebinding: 11 tests, 169 assertions, 0 failures, 3.69 seconds.

One intermediate retry was blocked by three concurrently moving OSdev generator project authorities. That was retained as a current external checkpoint, not a Cargo/library result. After the OSdev owner paths were corrected, full taxonomy validation and every final route above passed.

## Limits

The full transaction-v2 aggregate was not executed. Its existing suite materializes Git repositories and the task forbids modifying Git. The portable source gate instead exercised current-ticket no-follow allocation, exact source identities, the 62-case partition, owned descendant cancellation, and registered route/input closure. This report does not claim a Windows runtime cancellation result; the Windows tree-kill branch is retained and checked statically while descendant runtime evidence is macOS/POSIX.

No live transaction, workspace cleanup, cache prune/reset, service, installation, scaffold/apply, or publication outside private fixtures ran.

## Exact attribution

Created product and contract files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🎛️native-input/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📁️run-allocation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🧾️provenance/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️execution/🐹️go/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-transaction-command-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️cargo-transaction-command-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts`

Updated command and production consumer files:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🔷️dotnet/📜️script.ts`
- `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🦀️native/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`

Updated source-data, verification, routing, cache, and registration files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧫️cases.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

Ticket records created:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-library-cargo-transaction-owner-plan-2026-09-13.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-library-cargo-transaction-extraction-2026-09-13.md`

All temporary logs, compiler data, Nx state/cache, and private native outputs were kept inside `🗑️generated/sol-library-cargo-transaction-extraction` during verification and are disposable after this report.
