# Shared Compiler Store Maintenance

General cache pruning previously serialized only other pruners. Its Cargo build and target units were marked unlocked without excluding live compiler profiles. The explicit incremental compactor already acquired its exact profile before rescanning; that did not protect the general pruning command.

The schema now declares independent shared compiler-store access and exclusive maintenance access. Every controlled Cargo build/test profile acquires its shared store lifetime before the existing exclusive profile lifetime. Different profiles still build in parallel. General maintenance acquires cache-prune exclusion followed by exclusive ownership of the same actual canonical compiler store, then rescans and applies its policy. Release unwinds profile before store, and every failure/cancellation releases admitted ownership. The original cancellation reason is preserved across queued store admission.

The neutral fixture admits prototype and release simultaneously, proves maintenance exclusion through independent SQLite, checks zero deletion on original cancellation and while either profile remains, then uses the production scanner/planner/deleter after both release. Fast-glob independently checks the physical retained/deleted files. The existing incremental compaction fixture also checks current, working and foreign-profile preservation. Unique temporary lease databases use the short runtime lease store and are removed after each fixture; captured test evidence stays under this ticket's generated directory.

`cargo-store-prune-red.log` physically fails because the store admission API is absent. The first implementation probe refused an overlong fixture lease path. The next probe exposed replacement of an original custom cancellation reason by Node's timer AbortError; store admission now returns the original signal reason. The prototype third probe passes both behavior laws but the added broad orchestration TypeScript check reaches unrelated repository/UI/test barrels and fails after 115 seconds. That broad check is not claimed green.

A narrow first-party Cargo lease facade and direct Cargo directory-owner imports remove unnecessary compiler/component imports from pruning. The permanent focused strict gate checks the pruning core and its real source dependencies. `cargo-store-prune-release4.log` passes both actual laws (10 and 19 assertions) plus strict TypeScript, through the pinned SDK's actual Nx task runner, 13.5 seconds. The current prototype gate `cargo-store-prune-prototype5.log` also passes both laws and strict core checking, 4.0 seconds. Full orchestration/project and monorepo checks remain required.

The existing finite original deadline/cancellation/progress ports are carried through maintenance admission. No canonical compiler profiles were removed while developing this change. Explicit inactive private profile reclamation is recorded separately in the existing headroom report.

The additional cross-process oracle `cargo-store-node-release6.log` runs actual Node system SQLite under the original controlled child process, while both Bun-owned profile leases remain admitted, and after maintenance closes. It confirms exclusive denial followed by reacquisition. Both laws and strict core checking pass (12 and 19 assertions), 4.8 seconds through actual Nx.

At 00:36 UTC on October 14 the actual host reported three minutes uptime, no old compiler/SDK daemon processes, and 42 GiB disk availability. The previous kernel compiler probe failed with ENOSPC before its retirement law; that failure is retained and needs a current retry. The complete prototype/release graphs and accepted warm launch latency remain unverified. The goal and ticket remain active.

Exact parent-owned files created or updated:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🌐️workspace/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🔒️store/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🔒️store/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🔒️incremental/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

The Nx executor owns the headroom dispatcher/target extensions. Their retained inputs reference the original canonical source and copy no source files.
