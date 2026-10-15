# Result: PluginRuntime module split, cycle break and `loadPluginModule` decomposition

Executed 2026-10-15 by the Sonnet executor. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `PR` = `E/🔌️PluginRuntime`.
Seam map used: `SEAM-MAP-TSX-TIER2.md` section 2.E. Method and verification follow `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md`. `ShellHost` was not touched.

## 1. Outcome at a glance

| Item | Before | After |
|---|---|---|
| `PR/🟦️.tsx` | 4,582 lines, 70 declarations plus 5 export lists | 369 lines: header, imports, barrel re-exports, `pluginRuntimeTestDependenciesV1` and the in-source test hook |
| New children | 0 | 18 (`PR/<dir>/🟦️.ts`), 59 to 484 lines each, 5,062 lines in total (the surplus over the original is duplicated import blocks) |
| Largest single function | `loadPluginModule`, 1,145 lines | `createSpawnedJobDriver`, 225 lines; `loadPluginModule` itself is 19 lines |
| Public API of the parent | 90 exports | the same 90 exports, same names, same value/type kind (TypeScript checker, section 6.4) |
| `core` ↔ `typed-result` cycle | one residual two-cycle in the map | none: 0 SCCs of size greater than 1 over the parent and 18 children, and 0 cycles inside the PluginRuntime tree by dependency-cruiser |

## 2. The cycle between `typed-result` and `core`

Cause. The map's `🗃️core` was a 173-unit catch-all, and it put `typedOperationRoutersByActor` (the per-actor router state) on the core side while `TypedOperationRouter`, `withTypedOperationCall` and `consumeTypedOperationEffects` sat in `typed-result`. That gave the three typed→core edges the map counts. At the unit level (TypeScript-checker identifier references over the original file) the graph has no cycle at all except two same-module groups (`buildShardClientOptions`/`getShardClient`/`getActivationRegistry`/`handlePluginShardLost`, and `TypedOperationCall`/`TypedOperationRouter`), so the cycle was an artifact of the map's grouping.

Resolution (no `import type` band-aid, no interface indirection): the declarations are placed by dependency depth.

- `🪝️wire-codec` is the lowest leaf. It owns everything `consumeTypedOperationEffects` needs from the wire boundary: `WireVariant`, `WireTurnResult`, `decodeWirePack`, `coerceWireBytes`, `DIRECTORY_PROJECTION_RECEIPT_SCHEMA`, `shellFrameBytes`.
- `📬️typed-result` owns the whole typed-operation surface including the router state (`typedOperationRoutersByActor`) and depends on `wire-codec` only (4 edges).
- `teardownPluginActor`, which clears the router state of a dying actor, lives in `🧵️turn-dispatch`, which depends on `typed-result`, never the other way.
- `fnv1aHex` (used by `retainedSurfaceHash` and the transaction coordinator) moved with the wire helpers so `retained-ui-patch` has no edge into transaction code; `leftoverShellInvocationFrames` and `documentPortHotBytes` also sit in `wire-codec` so the settle and job code can read them without climbing.

Evidence: `🗑️generated/plugrt/graph.mjs` over the pre-split file with the final assignment: module edges listed in section 4.2, `SCCs>1: []`. `node_modules/.bin/depcruise` (no-circular, include-only `PR/`): "no dependency violations found (24 modules, 90 dependencies cruised)".

## 3. `loadPluginModule` decomposition

`loadPluginModule` is now a 19-line orchestrator in `🧩️module-loader`. It calls these phase functions in the order the original statements ran. All of them are plain top-level functions with explicit parameters and returns. Their bodies are the original statement ranges verbatim apart from the substitutions listed below.

| Phase | Function (child) | Lines | Original range |
|---|---|---:|---|
| 1 | `resolvePluginDescriptorAndRegistry` (`🧩️module-loader`) | 13 | descriptor resolution, `registerManifest` |
| 2 | `createPluginHandleScope` (`🪢️handle-scope`) | 43 | shard client, instance maps, counters, outcome broadcast |
| 3 | `createInstanceTable` (`🪢️handle-scope`) | 37 | `requireActorId`, `releaseInstanceMaps`, `submitTurn` |
| 4a | `createUiPatchAdmission` (`🧺️handle-ui-intake`) | 156 | intake lane, round admission, acknowledgement, surface install, publication announcement |
| 4b | `createUiSurfaceProjection` (`🧺️handle-ui-intake`) | 106 | maintenance, surface read-back, `ownedUiRefreshResponse` |
| 4c | `createUiInstanceClose` (`🧺️handle-ui-intake`) | 65 | `closeUiOwner`, close ladder, `retireInstanceLifecycle` |
| 4 | `createRetainedUiIntake` (`🧺️handle-ui-intake`) | 6 | wires 4a to 4c and returns the three entry points |
| 5 | `createRequestActor` (`🪢️handle-scope`) | 27 | lazily activated inbound-request actor |
| 6a | `createSpawnedJobProgressLane` (`🚚️handle-turn-drive`) | 27 | spawned-job progress listeners and coalescing clock |
| 6 | `createSpawnedJobDriver` (`🚚️handle-turn-drive`) | 225 | job drives, reserved-tool drives, `routeHostEffects` |
| 7 | `createTurnPump` (`🚚️handle-turn-drive`) | 157 | `runQueuedTurn`, `drainTypedOperations` |
| 8 | `createKernelHandle` (`🧩️module-loader`) | 113 | `createApp`, `destroyApp`, `enqueue`, `outcomes`, `dispose` |
| 9 | `createRefreshUi` (`🧩️module-loader`) | 61 | `refreshUi` |
| 10 | `createExtensionCompletionCapture` (`🧩️module-loader`) | 64 | `captureExtensionCompletion` |
| 11 | `createInboundInvoke` (`🧩️module-loader`) | 60 | `invoke` |
| 12 | `createDocumentPortBinder` (`🧩️module-loader`) | 63 | `bindDocumentPort` |

How the closure web was cut:

- Shared state is one explicit `PluginHandleScope` object (`pluginId`, `manifest`, `registry`, `shardClient`, `turnOutcomes`, the instance/UI/document maps and a `state` record). Each phase destructures what it reads at its top. Maps and sets keep their identity, so nothing is copied.
- The five mutable scalars of the old closure (`disposing`, `disposal`, `eventSeq`, `requestActor`, `extensionRequestSeq`) became fields of `scope.state`; every read and write was rewritten to `state.<name>`. The builder asserted the exact number of occurrences replaced in each chunk and aborted on any mismatch.
- Two ledgers that only the UI phases touch (`uiSurfaceInstalledAtMs`, `surfacePublicationListeners`) and `uiGrant` became local to phase 4a/4b, the spawned-job progress state local to phase 6a.
- The only true mutual recursion between phases is job completion → typed-operation drain → host-effect routing → job start. It is cut at one place, explicitly: `createSpawnedJobDriver` takes `turns: { drainTypedOperations }` and the orchestrator passes `(instanceId) => turns.drainTypedOperations(instanceId)`, resolved on first call after `turns` exists (the same late binding the old closure had through `const` hoisting).
- `hotBytes` became the module-level `documentPortHotBytes` (it closed over nothing). `nextGlobalInstanceId += 1` became `takeNextGlobalInstanceId()` in the module that owns the `let`, because an imported binding cannot be assigned.

Evaluation order and await semantics, argued from the code:

- The original function had exactly two `await`s at its own level: the descriptor load and `await adaptPluginHandle(...)`. Both are still inline in `loadPluginModule` at the same position. Phase 1 returns the in-flight promise of `withPluginLoadInFlightV1` itself (a non-`async` function), so the caller's `await` costs the same microtask hops as before; wrapping it in an `async` function would have added hops, which is why it is not one.
- Before the first `await` the original ran `moduleUrl = publishedPageUrl(moduleUrl)`; phase 1 runs it as its first statement, synchronously in the same call.
- After the descriptor await the original called `getShardClient()` first, then built maps and closures. `createPluginHandleScope` calls `getShardClient()` first. Closure creation has no side effects, so the order in which phases 3 to 8 are created is not observable; they are created in the original order anyway. `refreshUi`, `captureExtensionCompletion`, `invoke` and `bindDocumentPort` are created after `await adaptPluginHandle`, as before.
- Every `await` inside a closure moved verbatim, in the same textual order within its function.

Line-level proof (script in `🗑️generated/plugrt`, comparing the trimmed non-empty line multiset of the original `loadPluginModule` with the new phase functions): 36 original lines have no identical counterpart, and each of them is one of the planned substitutions (scalar → `state.*`, `hotBytes` → `documentPortHotBytes`, `drainTypedOperations` → `turns.drainTypedOperations`, `nextGlobalInstanceId` pair → `takeNextGlobalInstanceId()`, the descriptor `await`, the old `return`). Nothing else in the original is missing. The other 220 top-level units of the file appear verbatim exactly once across the new files (`coverage-pr.mjs`: 220 of 220, the 2 deviating units being `loadPluginModule` and the test-dependency bag below).

One required edit outside the function: `pluginRuntimeTestDependenciesV1` built a `testState` object with setters that assign the module-private `sharedActivationRegistry` and `sharedShardClient`. An imported `let` cannot be assigned, so the object now comes from `pluginRuntimeShardTestStateV1()`, declared next to the two variables in `🛰️runtime-singletons`. Same getters, same setters, same keys.

## 4. The module split

### 4.1 Children

Names are taxonomy-admissible folder names (`<emoji>️<kebab>`, RGI emoji with U+FE0F, unique among the siblings of `PR`, all 18 checked by script; longest path 170 UTF-8 bytes, limit 240). Each child opens with a JSDoc whose first token is its emoji and which links back to the parent. No comments were added inside definitions.

| Child | Lines | Units | Holds |
|---|---:|---:|---|
| `🧷️handle-contract` | 259 | 12 | `PluginWasmHandle`, extension-completion, document-binding, operation-completion and transaction types |
| `🪝️wire-codec` | 238 | 17 | WIT variant and turn-result shapes, pack and byte coercion, surface identity, shell-frame peel |
| `🪟️retained-ui-patch` | 213 | 7 | retained document, patch application, snapshot, hash, built-node projection, `wireEffectToFriendly` |
| `📬️typed-result` | 197 | 14 | typed-operation carriers, router and its state, `consumeTypedOperationEffects` |
| `🛰️runtime-singletons` | 321 | 35 | shard budget, shared shard client and activation registry, shard-loss recovery, isolated-job drive store |
| `🧵️turn-dispatch` | 358 | 33 | thunk queue, command-stall watch, plugin turn scheduler, lifecycle turns, crossing census, actor teardown |
| `🔁️turn-settle` | 446 | 34 | UI continuation limits, close ladder, command-ingress ceilings, `settlePluginTurn` |
| `🏁️operation-completion` | 112 | 10 | leftover-effect carriers, completion fan-out, terminal-output take, drain loop |
| `🖼️surface-refresh` | 393 | 21 | refresh requests to events, section values, refresh responses, retained surfaces per actor, guest presence |
| `🪦️instance-retirement` | 59 | 7 | retired-instance ledger and its typed predicates |
| `📥️invocation-frames` | 208 | 10 | `performInvocation`, `invocationFromFrames`, context menu, guest-ingress generation |
| `🔗️channel-adapter` | 347 | 7 | `adaptPluginHandle`, bounded history retry, shell actor identity |
| `🎫️transaction-coordinator` | 273 | 7 | `TransactionCoordinator` and its codecs |
| `🚦️dependency-boot` | 133 | 4 | `loadPluginModulesInDependencyOrder` and helpers |
| `🪢️handle-scope` | 183 | 6 | phases 2, 3 and 5, instance-id counter |
| `🧺️handle-ui-intake` | 371 | 4 | phase 4 (4a, 4b, 4c, aggregator) |
| `🚚️handle-turn-drive` | 467 | 3 | phases 6a, 6 and 7 |
| `🧩️module-loader` | 484 | 8 | `loadPluginModule`, phases 1 and 8 to 12, `resolveDescriptorBeforeRuntime` |

### 4.2 Module DAG

Verified by `graph.mjs` (identifier-level, with the checker) and by dependency-cruiser. Edges, children only: `typed-result`→`wire-codec`; `retained-ui-patch`→`wire-codec`; `operation-completion`→`wire-codec`, `handle-contract`, `typed-result`, `turn-settle`; `turn-dispatch`→`wire-codec`, `runtime-singletons`, `typed-result`; `turn-settle`→`wire-codec`, `runtime-singletons`, `typed-result`, `surface-refresh`, `turn-dispatch`; `surface-refresh`→`retained-ui-patch`, `wire-codec`; `invocation-frames`→`turn-settle`, `wire-codec`, `surface-refresh`, `operation-completion`, `typed-result`, `retained-ui-patch`; `channel-adapter`→`handle-contract`, `instance-retirement`, `invocation-frames`, `operation-completion`, `typed-result`, `retained-ui-patch`; `transaction-coordinator`→`wire-codec`, `handle-contract`; `handle-scope`→`runtime-singletons`, `instance-retirement`, `invocation-frames`, `operation-completion`, `turn-dispatch`, `wire-codec`; `handle-ui-intake`→`handle-scope`, `turn-dispatch`, `turn-settle`, `wire-codec`, `retained-ui-patch`, `surface-refresh`; `handle-turn-drive`→`handle-scope`, `handle-ui-intake`, `runtime-singletons`, `typed-result`, `turn-settle`, `wire-codec`, `operation-completion`, `turn-dispatch`, `invocation-frames`; `module-loader`→`runtime-singletons`, `handle-scope`, `handle-ui-intake`, `handle-turn-drive`, `turn-dispatch`, `channel-adapter`, `typed-result`, `turn-settle`, `surface-refresh`, `wire-codec`, `handle-contract`, `invocation-frames`; `dependency-boot`→`handle-contract`, `module-loader`, `runtime-singletons`. Leaves: `handle-contract`, `wire-codec`, `runtime-singletons`, `instance-retirement`. The parent imports children only; no child imports the parent.

### 4.3 Deviations from the seam map

| Map | Done | Why |
|---|---|---|
| `🗃️core` (1,171 lines, 173 units) | split into `runtime-singletons`, `turn-dispatch`, `turn-settle`, `operation-completion`, `surface-refresh`, `instance-retirement`, `invocation-frames`, `handle-scope` | the catch-all hid the cycle (section 2) and is not one cohesive responsibility |
| `🧩️module-loader` (1,387 lines incl. `adaptPluginHandle`) | `module-loader`, `handle-scope`, `handle-ui-intake`, `handle-turn-drive`; `adaptPluginHandle` to `channel-adapter` | the decomposition of `loadPluginModule` (section 3) gives each phase family its own child |
| `🔁️history-retry`, `🧲️actor-adapter` | history retry folded into `channel-adapter`; the actor-adapter units were placed by dependency depth into `runtime-singletons`, `wire-codec` and `turn-dispatch` | too small to stand alone, and `adaptPluginHandle` is the only consumer of the history retry |
| `🧷️plugin-runtime-core` | `🧷️handle-contract` | there is no `core` any more, so the old name would mislead; it also takes the transaction and operation-completion types that `PluginWasmHandle` names |
| `🧵️turn-scheduler` | `🧵️turn-dispatch` | `turn-scheduler` is a registered facet-kind slug (`🧵️`, taxonomy `semanticDirectoryKinds."turn-scheduler"`), so a folder of that name would be classified as that kind |
| `🎫️transaction` (implicit) | `🎫️transaction-coordinator` | `transaction` is a registered facet-kind slug with emoji `🔄️` |
| `🪝️wire-codec`, `🪟️retained-ui-patch`, `📬️typed-result`, `🧩️module-loader` | kept | |

`dependency-boot` is a child rather than parent code so the parent stays a pure barrel.

### 4.4 Taxonomy

Rule (same as in `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md` section 3): a child folder must be a member of a registered directory kind; `semanticDirectoryMemberKinds."members-of-members-of-elements"` applies under every element folder. Action: the 18 names were inserted as one hunk right after `🪟️window-view-store` in that `memberNames` array of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (a peer's names follow the same array and were left alone). Before inserting, each slug was checked against every exact-slug facet kind of the file (`turn-scheduler` and `transaction` collided, hence the renames in 4.3) and each emoji against `\p{RGI_Emoji}` / sibling uniqueness.

`bun ./📜️script.ts verify taxonomy report --scope PR` (log `plugrt-taxonomy-final.log`, run after the final write): `clean=false errors=12`, none of them on the parent or on any of the 18 children. The 12 are: 4 `directory-kind-unresolved` on the existing unregistered `⏱️command-stall`, `💼️job-ledger`, `📡️backbone`, `🫀️load-progress`; 1 on `🧪️tests/🧫️fixtures/🏁️job-completion-publication`; 2 `normalization-move-required` for two fixtures; 3 `reference-edit-required` on `🧪️tests/🔌️plugin-runtime/🟦️.tsx` (the fixture paths just named); 1 `reference-edit-required` on a Rust test; 1 `generator-preview-invalid` for the plugin registry. A first run in the middle of my work reported one more, `reference-preimage-unreadable` on the parent, because I edited its header comment while that run was reading it; the rerun above does not show it. I did not run a taxonomy baseline before editing, so I cannot show the number "before"; the 12 are all about pre-existing directories and files that this work did not create.

## 5. Source-text readers

The shared helper `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts` (`readModuleTreeSource`, from `RESULT-SOURCE-TEXT-READERS.md`) exists; it was used for both readers that needed the text.

| Reader | Old | New | Result |
|---|---|---|---|
| `E/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts:186` (`directoryHomeBootstrapOracle`) | `readFileSync` of the parent | `readModuleTreeSource` of the parent (the helper was already imported there by a peer) | The 10 `runtime.includes(...)` needles all hold on the tree text (script run, 10 of 10). The `shell` read in the same function was re-pointed afterwards, see section 9.2; the whole function now runs |
| `🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts:2746-2749` | `readFileSync` of the parent; after the split `new Worker(` is no longer in the parent, so the `SHARD_WORKER_URL` assertion would have passed vacuously | `readModuleTreeSource` of the parent (the wgpu bridge in the same loop has no children, so its text is unchanged) | vitest through `💻️os/🧪️tests/🎚️config` with `actor-transport` and the test name: passes (1 test) with `--testTimeout 120000` (its import phase alone takes 19.9 s; the default 5 s limit fails it for load reasons) |
| `🦑️repo/🔨️modules/📚️library/🖱️ui/📜️script.ts:160` `CHROME_I18N_ALLOWLIST` | listed the parent for its `options?.lane ?? "Interactive"` fallback line | lists `PR/🪢️handle-scope/🟦️.ts`, where that line now is | I replayed the lint's four regexes over the whole tree: the original file had exactly 1 hit; the new tree has exactly 1 hit, in `handle-scope`. Without this edit the lint would fail with a stale entry plus an unlisted hit. The lint itself was not run end to end |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts:14512` `bundleModule` | bundles the parent | unchanged | `Bun.build` of the parent with the gate's options (`define import.meta.vitest`): success, 1 output, 0 logs, 1.29 MB; the in-source test hook is dropped |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:217` and `🔌️plugin/📇️registry/🧪️tests/🧩️composition/🎚️config/🟦️.ts:7` | path of the in-source suite file | unchanged | the parent keeps the `import.meta.vitest` block, so the suite file is the same |
| `📓️print/🔮️oracles/🔣️.json:84` `reachableFrom`, `os-source-topology` fixture consumers | path lists | unchanged | the parent still exists and still reaches everything transitively |

## 6. Verification, baseline vs after

All commands ran in the foreground or as logged background jobs; logs are in `🗑️generated/plugrt-*.log`. Peers edited the tree during the work, so I compared against a baseline taken before my first edit and, for the test run, also against the original source served at the same moment.

### 6.1 Typecheck

`node_modules/.bin/tsc --noEmit -p 🗑️generated/plugrt-tsconfig.json` (a scoped program of `PR/🟦️.tsx` plus the seven suites that import it).

- Baseline (`plugrt-tsc-baseline.log`): 40 errors, 3 in `PR/🟦️.tsx` (TS2305 `Lane` not exported by the actor package barrel, TS2741 `maxFrames` missing in `DEFAULT_SHARD_BUDGET`, TS2345 `() => number` vs `DescriptorProgress`), 27 in `UiDocumentStore/🧪️tests/🧪️typedwire`, 2 in `WasmSessionLoader`, 8 in `ShellHost/🧩️shell-core-state` (a peer's, gone by the end).
- After the `loadPluginModule` decomposition alone, still a single file (`plugrt-tsc-decomp.log`): 40 errors, identical to the baseline once line numbers are stripped (sorted diff empty). With `--noUnusedLocals`, the only hits in the file are the four declarations that were already unused (`OwnedNativeUiPatchAuthority`, `steps`, `crossingStartedMs`, `payload`), no unused destructured name.
- Final layout (`plugrt-tsc-final.log`): 40 errors: 27 typedwire, 2 `WasmSessionLoader`, 6 in `World3dHost` (a peer's edit in flight at that moment), and 5 in `PR`. The 5 are the same 3 messages: `Lane` is now imported in `turn-settle`, `turn-dispatch` and `handle-scope`, each of which uses it, so the one pre-existing TS2305 shows three times; `maxFrames` is in `runtime-singletons`, `DescriptorProgress` in `module-loader`. No new kind of error. `Lane` does exist in `🎭️actor/🤖️generated/🎭️actor/🟦️.ts`; correcting the import belongs to whoever owns the actor package barrel, and I did not change it.
- Consumers (`plugrt-consumers-tsconfig.json`: react target, ShellHost, ShellHelpers, Shell, TaskManager, Interpreter, wgpu plugin bridge; final layout, `plugrt-tsc-consumers-final.log`): 34 errors, namely the 5 above, 27 typedwire and 2 `WasmSessionLoader`; no unresolved import or missing export anywhere.

### 6.2 Tests

Vitest through the react target's config, level `long`, `SEMIO_VITEST_POLICY` built by hand (`🗑️generated/tsx-vitest.sh`, set in `plugrt-vitest.sh`): the 14 files that import PluginRuntime or its in-source suite (the filter patterns were `PluginRuntime` (the in-source suite, 137 tests), `command-stall`, `directory-home-bootstrap`, `engine-contract`, `window-host-context`, `ingress-generation-gate`, `window-fault`, `plugin-load-progress`, `job-ledger`, `actor-backbone`, `more-work-drive`, `package-integration`, `wgpu-extension-dispatch`, `TaskManager` component, `surface-scene-lanes`; the last pattern matched no file in this config, so 14 files ran).

| Run | Test files | Tests |
|---|---|---|
| Baseline, before any edit (`plugrt-vitest-baseline.log`) | 4 failed, 10 passed (14) | 14 failed, 1,038 passed (1,052) |
| One-file decomposed `loadPluginModule`, before the module split (`plugrt-vitest-decomp.log`) | 4 failed, 10 passed (14) | 14 failed, 1,038 passed (1,052); failing set identical to the baseline |
| Module split, first version (`plugrt-vitest-split.log`) | 4 failed, 10 passed (14) | 14 failed, 1,038 passed (1,052); failing set and failure reasons identical to the baseline |
| Final layout (`plugrt-vitest-final2.log`) | 3 failed, 11 passed (14) | 11 failed, 1,041 passed (1,052) |
| Original source served at the same moment through a vite `load` hook (`plugrt-vitest-orig2.log`, `plugrt-vitest-orig.config.ts`; the hook was confirmed active by a `[DEBUG]` line, since removed from the config) | 3 failed, 11 passed (14) | 11 failed, 1,041 passed (1,052) |

The last two rows have identical failing tests (sorted diff empty) and identical per-file summaries. The drop from 14 to 11 failures against the first baseline is three `window-fault` tests that a peer's ShellHost work fixed in between; none involve PluginRuntime. The remaining 11 failures are pre-existing: 4 in `engine-contract` (procedural contributions `pageCount`, an `operator-fault` classification, two window-kind scoping tests), 1 in `more-work-drive` (a source needle in a generated worker script), 6 in `package-integration` (WGPU package catalog digest drift, Bun oracle errors). An intermediate run (`plugrt-vitest-final.log`) lost `engine-contract` and `ingress-generation-gate` to a peer's half-edited `World3dHost/🪟️window-view-store` (`createContext is not defined`); the original-source run at that moment lost the same two, and the rerun above, after the peer fixed it, ran everything.

### 6.3 Import cycles

`node_modules/.bin/depcruise` with a `no-circular`-only config (`🗑️generated/plugrt/depcruise.cjs`), TypeScript pre-compilation dependencies on, so type-only imports count.

| Scope | Before | After |
|---|---|---|
| include-only `PR/` | not measured (single file) | 0 cycles (24 modules, 90 dependencies) |
| include-only `PR`, `Shell`, `UiDocumentStore` | 2 violations (12 modules, 16 dependencies): `UiDocumentStore` ↔ its in-source test, `Shell` ↔ `PluginRuntime/🟦️.tsx` | 2 violations (30 modules, 111 dependencies): the same `UiDocumentStore` pair, and `Shell` ↔ `🧷️handle-contract` |
| include-only all of `E` (184 modules cruised after) | the big element web (72 modules after folding the 18 new children back into their parent, plus the 2-module `UiDocumentStore` pair); `PluginRuntime` is one node in it | the same web (82 modules); 11 `PluginRuntime` modules are in it (`handle-contract`, `invocation-frames`, `channel-adapter`, `operation-completion`, `transaction-coordinator`, `handle-scope`, `handle-ui-intake`, `handle-turn-drive`, `module-loader`, `dependency-boot` and the parent) |

The "before" for the whole-element scope is computed from the after-graph by folding my 18 children into the parent (`🗑️generated/plugrt/scc-wide.mjs`; the folded graph has the same members as the real original would, since the children's external imports are the original file's), not measured on the original tree. The set of modules in the web is unchanged; the single `PluginRuntime` node became 11 nodes because the web passes through the type-only imports of `Shell` types (`PluginManifest`, `ViewModel`) in `handle-contract`, `channel-adapter` and `invocation-frames`.

I also repointed one line of `E/🐚️Shell/🟦️.tsx` (line 77): `import { type PluginWasmHandle } from "../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts"` instead of the barrel. Without it the scoped run reported 23 violations (against 2 before), because every module reachable from the barrel joined the cycle; with it the cycle is `Shell` ↔ `handle-contract`, as small as before. Shell is not ShellHost, but it is another agent's file; the edit is one specifier.

### 6.4 Exports and completeness

- Public API: `getExportsOfModule` of the parent, original vs final (`🗑️generated/tsx-split/exports.mjs`): 90 vs 90 exports, identical name and value/type kind, 0 unresolved.
- Units: every non-export top-level unit of the pre-split file appears verbatim exactly once across the 23 files under `PR` (242 of 242; the 5 `export {...}` statements are rewritten as `export ... from` lines in the parent). Against the true original: 220 of 227, the 2 deviations being the decomposed `loadPluginModule` and the test-dependency bag (section 3), and the 5 export statements.
- Two comment-only differences: `//#region` markers inside `PluginWasmHandle` and `adaptPluginHandle` became blank lines (the tool strips region comments); the header docstring gained a list of children.
- Every new specifier resolves (tsc reports no TS2307 for the new files).

## 7. Files changed

New: the 18 `PR/<dir>/🟦️.ts` files in section 4.1.
Edited:
- `PR/🟦️.tsx`: barrel plus `pluginRuntimeTestDependenciesV1` using `pluginRuntimeShardTestStateV1()`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: +18 `memberNames`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/📜️script.ts`: allowlist entry.
- `E/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`: one reader.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`: one reader.
- `E/🐚️Shell/🟦️.tsx`: one import specifier (later superseded by section 9.1).
- Section 9.1: `E/🐚️Shell/🟦️.tsx` (type declarations removed, import changed), `PR/🧷️handle-contract/🟦️.ts` (declarations added) and the 16 importer files listed there. Section 9.2: the react target `📜️script.ts` (`directoryHomeBootstrapOracle`).

Tooling kept for traceability, all temporary and inside the ticket: `🗑️generated/plugrt/` (`decompose.mjs`, `decompose.template.txt`, `surgery.py`, `cfg-plugin-runtime.mjs`, `graph.mjs`, `coverage-pr.mjs`, `units.mjs`, `chrome-i18n.mjs`, `scc-wide.mjs`, `plan-report.mjs`, `depcruise.cjs`, `orig-PluginRuntime.tsx`, `work/`), the shared splitter `🗑️generated/tsx-split/{analyze,split,write,exports}.mjs`, and `🗑️generated/plugrt-*.{log,sorted,txt,json,sh,config.ts}`. No generated folder was deleted.

## 8. Not done, and findings for the owner

1. (Done after the coordinator's decision, see section 9.1.) The three `Shell` type imports (`PluginManifest` in `channel-adapter`, `ViewModel` in `invocation-frames`, both in `handle-contract`) are the only edges from `PluginRuntime` back into the shell, and they are what keep 11 of its modules inside the element-level import web. Measured on the dependency-cruiser graph: with those edges removed, 0 `PluginRuntime` modules remain in any cycle. The fix is the map's Phase 0 (b): move `ViewModel` (an alias of the external `PluginViewState`) and `PluginManifest` (built only from external types: `AppDefinition`, `ManifestExample`, `CommandDefinition`, `PluginDependency`, `TopicContribution`, `IconName`) from `Shell/🟦️.tsx` into `🧷️handle-contract` and re-export them from `Shell`. At the time of the first report I had not done it; section 9.1 records that it is now done.
2. `Lane`: the actor package barrel `🎭️actor/📦️packages/🟦️typescript/🟦️.ts` does not export `Lane`, which is the pre-existing TS2305 in 6.1.
3. `🧑‍💻dev/📤️distribution/🧾️manifest.json` lists the parent with a byte count and hash from before the split; it is generated and was not regenerated.
4. Not run: the browser or dev-server boot of the app (module evaluation order across the new files is evidenced only by vitest's SSR transform and the `Bun.build` of the parent), `verify interactivity`, the root chrome-i18n lint end to end, the full `directoryHomeBootstrapOracle` (stops at a ShellHost assertion, section 5), any Rust gate.
5. Still large: `createSpawnedJobDriver` (225 lines) and `createUiPatchAdmission` (156) are single closures with mutual recursion (job completion ↔ routing, intake ↔ round install); further cuts would need a state object per drive.
6. I added no new test: the existing in-source suite (137 tests) exercises the phases through `adaptPluginHandle`, `loadPluginModule` and the test-dependency bag. A structural guard (acyclic child graph, child size limit) would be a reasonable follow-up for the owner of the layering gate.

## 9. Follow-up after the coordinator's decision

### 9.1 `PluginManifest` and `ViewModel` moved into `🧷️handle-contract`

What changed.
- `ViewModel` (an alias of the external `PluginViewState`) and `PluginManifest` (built only from `AppDefinition`, `ManifestExample`, `CommandDefinition`, `PluginDependency`, `TopicContribution` and `IconName`, all external) were cut verbatim, with their doc comments, from `E/🐚️Shell/🟦️.tsx` and now live in `PR/🧷️handle-contract/🟦️.ts`. The six `@semio-tech/framework` type names went into that file's existing import block, `IconName` is imported from `@semio-tech/ui-react`.
- No alias or re-export is left in Shell. Shell imports `PluginManifest`, `PluginWasmHandle` and `ViewModel` from the contract like everyone else, and I removed the five type imports that only the moved declarations used (`ManifestExample`, `CommandDefinition`, `PluginDependency`, `TopicContribution`, `PluginViewState`).
- Importers found with a TypeScript AST scan over every tracked and untracked `.ts`/`.tsx` file outside the tickets (`🗑️generated/plugrt/find-shell-type-importers.mjs`; static imports, `export ... from` and `import("...")` types): 16 files. A codemod (`codemod-shell-types.mjs`) removed the two names from their Shell import, deleting the declaration where nothing else was imported, and added `import type { ... } from "<relative path to handle-contract>"`. Re-running the scan afterwards finds 0 importers of the two names from Shell. The files: the react target `🟦️.tsx`; tests `directory-home-bootstrap`, `plugin-runtime`; ShellHost children `focus-and-input`, `directory-bootstrap`, `contributions-refresh`, `shell-actions`, `shell-uri-documents`, `plugin-lifecycle`, `spawned-session`, `shell-core-state`, `built-node-stores`; ShellHelpers children `command-registry`, `plumbing`; PluginRuntime children `invocation-frames` and `channel-adapter` (the latter merged into its existing contract import by hand).

Evidence.
- Cruiser, `no-circular` only (`🗑️generated/plugrt/depcruise.cjs`): include-only `PR/`: 0 violations (24 modules, 91 dependencies). Include-only `PR`, `Shell`, `UiDocumentStore`: 1 violation, the `UiDocumentStore` ↔ typedwire in-source test pair (the `Shell` ↔ `PluginRuntime` violation of the baseline is gone); 28 modules, 106 dependencies. Include-only all of `E`, entries ShellHost, Shell, ShellHelpers and PluginRuntime together (192 modules, 24 of them PluginRuntime): a 69-module web plus the same two-module pair, and `PluginRuntime` modules in cycles: 0. Entering from PluginRuntime alone now reaches only 33 nodes instead of 210, because PluginRuntime no longer imports anything of the shell.
- tsc, scoped program (`plugrt-tsc-types.log`): 34 errors: 27 typedwire, 2 `WasmSessionLoader`, and the same 5 PluginRuntime ones as in 6.1. Consumer program of 7 entry files (react target, ShellHost, ShellHelpers, Shell, TaskManager, Interpreter, wgpu bridge; `plugrt-tsc-consumers-types.log`): the identical 34. No error mentions `PluginManifest` or `ViewModel`.
- vitest, the same 14-file set (`plugrt-vitest-types.log`): 3 failed, 11 passed files; 11 failed, 1,041 passed of 1,052 tests. The failing test list and the per-file summary are identical to `plugrt-vitest-final2.log`, i.e. unchanged by this move.

### 9.2 `directoryHomeBootstrapOracle` runs fully

The oracle (`E/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`, `directoryHomeBootstrapOracle`) read `ShellHost/🟦️.tsx` as one text. Its needles are spread over children (per `RESULT-SHELLHOST-SPLIT.md` section 9 and a search of the tree): `message.kind === "directory-event-page"` is in `🧵️backbone-worker`; `directoryHomeOwnerRef` in `🛂️directory-route`, `🚀️plugin-lifecycle`, `🎮️focus-and-input` and `🔒️identity-document-ports`; `openDirectoryHomeOwnerV1` in `🛂️directory-route` and `📇️directory-bootstrap`; the `visibleSession.viewState`, `directoryHomeOpeningRef.current.catch`, `identity: { userId: identity.userId` and `await refreshDirectoryHomeRef.current(active)` needles only in `🛂️directory-route`. One assertion is an absence check (`!shell.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl')`) that must cover every module, so the single `shell` variable of this function is now `readModuleTreeSource(join(contractRoot, "../🟦️.tsx"))`, the concatenated ShellHost tree (this also keeps the positive needles valid wherever a later split puts them). Only this function was changed; the two other functions in the file that read `shell` from the same path (`const shell = readFileSync(...)` near the `inviteToken` and the inference-port checks) were not in the request and are untouched.

Result: importing the script with bun and calling `directoryHomeBootstrapOracle("/Users/ueli/Documents/semio")` returns `checks 33`, the 33 checks the function declares, with no assertion failing (before this edit: `AssertionError` at the first ShellHost needle). I ran the function itself, not the `directory-home-bootstrap-check` script, so the two vitest runs that script also starts were not repeated here (the `directory-home-bootstrap` suite is part of the 14-file run above and passes, 15 tests).

### 9.3 The two other ShellHost-reading oracle functions

Same file, same re-pointing: in `directoryInviteCapabilityOracle` and `scopedPresenceOracle` the `const shell = readFileSync(join(contractRoot, "../🟦️.tsx"), "utf8")` is now `readModuleTreeSource(join(contractRoot, "../🟦️.tsx"))`, the concatenated ShellHost tree. I called all three functions through bun (`import` of the script, then each function with the repo root):

| Function | Result |
|---|---|
| `directoryHomeBootstrapOracle` | `checks 33`, no failure |
| `directoryInviteCapabilityOracle` | `checks 21`, no failure (its other reads, `SpaceAdministration` pane and the invite-capability child, were already valid) |
| `scopedPresenceOracle` | does not complete, for reasons that are not the ShellHost split (below) |

`scopedPresenceOracle`:
- One assertion did break because of the split and is fixed: `shell.includes('from "./👥️presence-scope/🟦️.ts"')` can only match a file sitting next to `presence-scope`, i.e. the old parent. The import now lives in the children `🧵️backbone-worker` and `✅️check-in-sync` as `from "../👥️presence-scope/🟦️.ts"`, so the needle is `'from "../👥️presence-scope/🟦️.ts"'`. All other `shell` needles of the function (presence peers, `portByRuntimeKey`, `operationRuntimeKey`, `inferencePortOwnerRef`, `owners.length === 1`, `inferencePortStatusRuntimeKeyV1(...)`, `retainInferencePortOwnerAfterCloseV1(...)`, `bindDocumentPort`, `receiveDocumentBackbone(...)`, the absence checks) hold on the tree text.
- Two pre-existing failures stop the function and are unrelated to ShellHost, so I did not change them:
  1. `ownedExport(repoRoot, "presence", "ScopedPresenceCloseV1")` throws "presence schema module publishes no export ScopedPresenceCloseV1". `💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/🔣️.json` no longer defines it; commit `d4597d0410a` removed it (the schema now has only `ScopedPresenceBindingV1` and `ScopedPresenceRejectionV1` among the scoped-presence defs). The test `🧪️tests/👥️scoped-presence/🟦️.tsx:46` references the same missing def.
  2. After the `shell` needles, `worker.includes('documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...inferencePort.scope }) === runtimeKey')` fails. `worker` is `🏪️store/👷️worker/🟦️.ts`; that text is not in it (nor in ShellHost, nor in HEAD of either). The only place with it today is `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/👷️worker/🟦️.ts:719`. Moving the reader there is a decision for the oracle's owner (a different source technology), so I left it.
- To show that nothing else in the function is broken by the split, I ran a scratch copy of the function (`scratchpad`, not in the repo) with schema validation stubbed to accept and the worker needle replaced by `true`: `checks 29`, the declared count. That run is evidence for the ShellHost reads only, not a pass of the oracle.
