# Repository Cache Command Extraction

## Scope and result

This slice extracts the substantive cache command implementation from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` into thirteen anonymous TypeScript leaves under their cache concerns. The mandatory command file is now an 81-line router. It assembles dependencies, registers command owners, and retains only its focused test dispatch. It does not retain an extracted semantic declaration or export a compatibility facade.

The extraction registers 22 semantic contexts, a schema-first portable fixture and test, a cached Bun/Nx source target, and one seed/derived launch identity. The thirteen-owner graph has 14 internal edges, is acyclic, and has no owner-to-command-module back edge.

## Anonymous owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🎫️output/🟦️.ts`
   - ticket authority, repository containment, and generated-output allocation
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔍️discovery/📂️source/🟦️.ts`
   - cache policy coordinate and one-pass, no-follow source discovery
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧮️composition/🟦️.ts`
   - Nx command inventory, explicit artifact ownership, dependency-consumer joins, policy findings, and bootstrap coordinate
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/📋️orchestration/🟦️.ts`
   - audit and policy command orchestration
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts`
   - project-graph verification command
6. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts`
   - repository cache disk report command
7. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩺️environment/📋️inspection/🟦️.ts`
   - required toolchain inspection command
8. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts`
   - cache contract verification command
9. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🌐️workspace/🟦️.ts`
   - cache-area projection, target/build roots, progress, cancellation, and test-evidence pruning
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts`
    - dry report and protected cache-prune command orchestration
11. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🧭️package-inventory/🟦️.ts`
    - artifact package discovery and language-neutral implementation records
12. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts`
    - bounded subprocess capture, progress, signals, and cross-platform process-tree cancellation
13. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️package-orchestration/🟦️.ts`
    - artifact package contract command orchestration

Artifact behavior is directly under the cache artifact concern. No extracted implementation was placed under the language-package `📦️packages` boundary.

## Semantic contexts

The taxonomy registers these exact contexts:

- `repo-cache-domain`
- `repo-cache-ticket-output`
- `repo-cache-discovery`
- `repo-cache-source-discovery`
- `repo-cache-inventory`
- `repo-cache-inventory-composition`
- `repo-cache-inventory-orchestration`
- `repo-cache-graph`
- `repo-cache-graph-verification`
- `repo-cache-storage`
- `repo-cache-storage-report`
- `repo-cache-environment`
- `repo-cache-environment-inspection`
- `repo-cache-verification`
- `repo-cache-verification-orchestration`
- `repo-cache-pruning`
- `repo-cache-pruning-workspace`
- `repo-cache-pruning-orchestration`
- `repo-cache-artifacts`
- `repo-cache-artifact-package-inventory`
- `repo-cache-artifact-contract-capture`
- `repo-cache-artifact-package-orchestration`

Every implementation owner ends in the anonymous leaf `🟦️.ts`.

## Consumer, routing, and cache closure

The router directly imports its seven orchestration owners. The existing cache-contract test directly imports ticket output, source discovery, and inventory composition from their real owners. The generic repository library's source-data note now names the verification orchestration owner rather than the command router.

The current executable consumer population has one external cache-prune route: workspace cleanup invokes the mandatory cache command as `bun <cache-router> cache-prune`. The command remains registered at the router. The formerly present native Cargo scheduling consumer disappeared during a concurrent change; the current Cargo cleanup-boundary test declares that native Cargo commands do not schedule cleanup. This slice preserves that live boundary and does not restore a compatibility call.

Policy and bootstrap references use exact source-relative coordinates for `🔣️policy.json` and `🚀️bootstrap/📜️script.ts` at the inventory/source owners. No owner imports the command module.

The ordinary source route is:

- Bun: `bun ./📜️script.ts test cache-command-source`
- Nx: `repo:test-cache-command-source`
- launch identity: `🧪️test⚡️cache-command-source`

The target uses the exact `cacheCommandSources` named input. It contains the thirteen owners; router, project, policy, bootstrap, schema, fixture, and test; the existing cache-contract and workspace-cleanup consumers; lower cache owners; generic process/library and Nx policy/discovery sources; taxonomy; plugin source; shared library source-data consumer; and both launch catalogs. The seed and derived launch catalogs contain one matching entry each.

## Cancellation and output boundaries

Artifact contract capture bounds stdout and stderr to 64 MiB, emits ten-second progress, propagates SIGINT/SIGTERM, and rejects non-zero, signalled, timed-out, and output-limit outcomes. POSIX starts a detached process group and terminates the whole group with TERM followed by a bounded KILL fallback. Windows invokes the established `taskkill /pid <pid> /t /f` process-tree operation.

The portable native control first verifies exact bounded output and a direct timeout. Its hostile case launches a parent that launches a long-lived descendant, waits long enough to observe the descendant PID under concurrent Nx load, times out the parent group, and proves the descendant is no longer alive. It also admits the Windows branch statically so cross-platform tree policy cannot regress on this host.

## Test-driven evidence

The corrected schema-first red phase ran through `bun test` and produced one pass and ten failures: thirteen owners were absent, 22 contexts were absent, nineteen moved declarations remained in the router, and the registered route did not exist. The initial `bun <test-file>` attempt was an invalid invocation and is not counted as red evidence.

Final current evidence:

- Direct portable source contract: 11 tests, 169 assertions, zero failures in 20.77 seconds.
- Ordinary Bun router: 11 tests, 169 assertions, zero failures in 16.32 seconds.
- Actual isolated Nx target: 11 tests, 169 assertions, zero failures; test body 20.90 seconds and target 21.9 seconds, with cache explicitly skipped. Nx workspace data and cache were isolated under this lane's ticket scratch.
- Private lower pruning fixture: both its independent Python eviction oracle and real Cargo build/target/incremental, Vite, agent, cancellation, session-compaction, deletion, and empty-parent controls printed `PASS`. All fixture writes were under this lane's ticket scratch.
- Safe direct doctor route: Bun 1.3.14, Node 24.15.0, Cargo/Rust 1.99 nightly, Go 1.25.0, uv 0.11.15, .NET 10.0.300, and CMake 4.3.4 were observed.
- Direct router import completed with zero errors; current `loadCatalogTaxonomy()` completed with zero validation errors.
- Static current graph recount: thirteen owners, 14 internal edges, zero command-module back edges; the portable test also proves acyclicity and strict TypeScript compilation.
- Terra's independent audit identified the Windows descendant-cancellation gap, accepted the repair and source/cache structure statically, and retained its evidence in `📓️terra-cache-command-source-audit-2026-09-13.md`. The final registered result above was sent to that auditor after its pending-status draft.

One registered refresh initially reported ten passes and one test-control failure because the hostile child had not printed its descendant PID within 100 ms under concurrent load. It did not establish a product cancellation failure. The control now uses a bounded two-second setup window, observes the PID, and passes through direct, ordinary, and actual Nx routes.

## Limits

This slice did not execute live cache pruning, live test-evidence pruning, shared cache cleanup/reset, broad cache verification, live artifact package builds, package installation, or service operations. Read-only doctor and source checks plus the private pruning fixture provide the runtime evidence. The adjacent Cargo command API and library transaction/Go command behavior remain explicitly outside this accepted slice.

The private pruning fixture proves the existing lower implementation and its independent Python byte/accounting oracle. This extraction did not modify that lower test or its fixture. The concurrent removal of Cargo's former throttled prune scheduler is recorded as observed history and is not attributed here.

## Exact attribution

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🎫️output/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔍️discovery/📂️source/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/🧮️composition/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📇️inventory/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/✅️verification/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/📊️report/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩺️environment/📋️inspection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🌐️workspace/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🧭️package-inventory/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏃️contract-capture/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️package-orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧬️schema/🧱️command-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🧱️command-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧱️command-source/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-root-cache-command-extraction-2026-09-13.md`

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The project-manifest attribution is limited to `test-cache-command-source` and `cacheCommandSources`. The taxonomy attribution is limited to the 22 contexts listed above, the launch attribution to the singleton source entry, and the generic library attribution to the `CacheVerifyScript` source-data coordinate. Concurrent changes elsewhere in those shared files remain owned by their respective lanes.

No Git lifecycle, AGENTS, runtime dependency, live cleanup, shared cache mutation, Cargo script change, package installation, or service mutation belongs to this slice.
