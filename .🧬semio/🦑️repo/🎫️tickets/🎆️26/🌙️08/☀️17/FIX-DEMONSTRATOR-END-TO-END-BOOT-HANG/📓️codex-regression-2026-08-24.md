# Demonstrator Regression Continuation

## Goal

Restore the `mit-bestand` demonstrator end to end and verify every included app at runtime.

## Red Baseline

`bun nx run-many -t test-quick -p @semio-tech/framework-job-rs,@semio-tech/ui-contract-rs --skipNxCache --outputStyle=stream` failed on 2026-08-24.

The production-library blockers reproduced from the supplied log are:

- `semio-framework-job`: five `ManuallyDrop<Option<_>>` assignment mismatches and one rejected-page borrow-lifetime conflict.
- `semio-framework-ui-contract`: missing `SurfaceDoc` clone/equality support, a mutable/shared callback mismatch, and two iterator-temporary borrow conflicts.

The focused UI test build also exposes stale tests that still construct the former unbounded `UiText`, `UiFixedBytes`, binding-list, and builder APIs. These must be updated to the bounded contract before the target can become green.

## Verification Route

1. Make the two focused Nx test targets green.
2. Run the demonstrator Nx build from the registered launch path.
3. Start the registered demonstrator dev target on port `6029`.
4. Exercise all configured panes/apps and record browser/runtime evidence, including worker boot failures and console errors.

## Focused Fixes

- The job payload writer now preserves exact rejected-page ownership across `ManuallyDrop`, separates fallible reservation from grant creation, and retires pre-admitted faults only after close begins.
- The UI contract's bounded owners now use pre-sized heap slices instead of recursively nested multi-megabyte stack values. The arena itself is likewise heap-backed while retaining fixed capacities.
- Bounded lists, maps, bytes, and recursive UI values again admit and serialize populated schema fixtures with exact quota rejection.
- Retained patch removal now runs only after the candidate snapshot has been cloned and advances one subtree owner per opportunity.
- Text edit admission immediately returns byte credits after a failed single-page admission, and persistent rope edits reuse unaffected subtrees instead of scanning multi-megabyte roots.
- Stale tests were ported to the bounded constructors and ownership APIs; the generated TypeScript contract mirror was refreshed through its Nx target.

## Focused Green Baseline

- `@semio-tech/framework-job-rs:test-quick`: 11/11 passed.
- `@semio-tech/ui-contract-rs:test-quick`: 88/88 passed.
- `@semio-tech/ui-contract-rs:conformance`: 6/6 passed.
- `@semio-tech/ui-contract-rs:check`: generated schema mirror fresh.
- `@semio-tech/ui-contract-rs:check-wasm`: `wasm32-wasip2`, `wasm32-unknown-unknown`, and type-generation feature checks passed.

## Demonstrator Build Integration

- Updated the OS store artifact-fault path to use the bounded artifact envelope payload and the current interactive-job close lifecycle.
- Restored parsed-document text equality support and added coverage for closing an interactive job through the OS store.
- Updated the actor bridge to project the current bounded UI values and retain a payload page for each projected turn.
- Updated the World3D host freshness comparison to use a deterministic flat lexicographic comparison compatible with the current TypeScript target.

## UI Runtime Migration

- Replaced ordinary command cloning with explicit credited clones and reduced gateway tracking to command tickets.
- Migrated runtime text, children, operations, bindings, and state handling to the bounded UI contract types.
- Changed fixed-list and built-children backing storage to lazy or boxed slices so empty/default structures no longer overflow ordinary thread stacks.
- Boxed the runtime fixed-vector, semantic traversal, and retirement stacks and added size regressions for the reconciler, cursor, retained map, tree node, and UI node record.
- Raised the page capacity to 32 KiB so the bounded base node fits the page accounting invariant.
- Updated stale runtime tests to exercise the current bounded and credited APIs, including dynamic page-fault progression and bounded wide-tree traversal.

## UI Runtime Green Gates

- `@semio-tech/ui-runtime-rs:test-quick`: 81/81 passed on the default stack.
- `@semio-tech/ui-runtime-rs:check-wasm`: both Wasm targets passed.

## WGPU Integration

- Removed the obsolete toggle callback argument from the borrowed window-measure adapter.
- Separated retained-document key discovery from removal so incremental close no longer overlaps immutable and mutable table borrows.
- Added a regression proving retained-document close retires exactly one record per step and then completes.
- `@semio-tech/ui-rs:test-quick`: 145/145 passed.

## Fresh Plugin Build Integration

- Made the kernel activation event cloneable so manifest descriptors, activation decisions, and lifecycle events can retain it.
- Added a regression for retained activation-event ownership.
- Moved the private command-batch descriptor destructor assertion into its owning channel module and kept the public kernel test focused on exported owners.
- Updated the turn-patch transport test to unwrap without imposing an unrelated `Debug` requirement on the lease owner.
- Updated the action-bus wire-dispatch regression to observe the bounded payload page release before terminal completion.
- `@semio-tech/framework-rs:test-quick`: 191/191 Rust tests and 87/87 TypeScript tests passed.

## Demonstrator Component Restoration

- Restored the demonstrator schema, IO, editor, and viewer trait boundaries after the generated scaffold had applied asynchronous signatures to pure helpers and tests indiscriminately.
- Kept artifact composition, analysis, snapshot rebuilding, and actual editor/viewer lifecycle methods asynchronous while returning bounded component trees through the current UI assembly contract.
- Added the framework dependency required by the demonstrator's closed app enum and compiled the demonstrator with every foreign app crate in one `wasm32-wasip2` check.
- Restored the shared plugin scene-surface bridge, WGPU retained-paint traversal, host fault projection, and plugin-root exports needed by the foreign apps.

## Boot Scheduling and Build Selection

- Added language-agnostic launcher tests for runtime plugin build selection and delayed boot scheduling.
- Runtime build variants are deduplicated by plugin id, so the five demonstrator-backed routes share one component while the procedural generator retains its separate component.
- Overview boot now begins after a real delay and spreads background app activation over time; focusing a route pauses background activation so a visible app cannot be starved by five simultaneous component boots.
- `@semio-tech/mit-bestand-demonstrator:test`: 2/2 Vitest tests passed.
- `cargo check -p semio-s-plugin-demonstrator --target wasm32-wasip2`: passed with all six foreign app crates.

## Native Host Ownership Boundary

- The full build exposed a native-only `Send` failure in the retained shard drive future: pure `granted_budget` and `actor_lane` helpers borrowed the non-`Sync` Wasmtime instance table immutably across `await`.
- Made those helpers synchronous, kept outcome transport mutation on the shard's exclusive borrow, and removed the stale child-process `block_on` around synchronous registration.
- `cargo check -p semio-framework-plugin-host`: passed for the library and `semio-shard` binary.

## Descriptor and App-Routing Restoration

- Assigned every demonstrator and foreign editor/viewer an explicit shared/exclusive classification so descriptor generation no longer rejects current manifests.
- Restored the canonical Puzzle3D editor id in demonstrator metadata and removed the obsolete procedural command expectation.
- Made app-router loading dependency-first and let the shell use registry dependency manifests while a dependency is not itself a directly loaded pane plugin.
- Added a dependency-order regression and retained the six branded route rows: `generator`, `koordinator`, `aggregator`, `aussuchen`, `bearbeiten`, and `verfolgen`.
- Extended plugin-description diagnostics to report missing rendered descriptors and the producing process' stderr cause instead of timing out opaquely.

## Browser Stack-Budget Repair

- A live Aggregator boot first trapped while the browser initialized the reactor's large patch-tracker state.
- Boxed the two `ComponentTreeProducer` authorities that dominated the initialization frame.
- Added a 512 KiB state-size regression. Disassembly reduced the browser TLS initialization stack frame from approximately 11.86 MiB to 2.095 MiB; `PatchTrackerState` is 349,216 bytes.
- The next live load advanced beyond the reactor, proving the stack trap was removed.

## Jco Async Result ABI Repair

- The next live load reached the generated async callback adapter but decoded a non-empty task result as direct flat parameters, producing `invalid flat variant case`; after correcting that path it exposed the adapter's separate `ctx.memory` guard defect.
- A direct Jco 1.27 JSPI comparison reproduced both generated decisions byte-for-byte, so the materializer now post-processes only memory-backed task-return contexts: it changes them to the canonical indirect result pointer and checks the resolved `memory` value.
- Null-memory callbacks remain direct, and the rewrite is idempotent.
- The focused authentic-snippet regression passed: 1 passed, 43 skipped.
- A clean demonstrator-only rematerialization succeeded and contains no temporary diagnostics, no memory-backed direct callback, and the corrected memory guard.
- The next live Aggregator load advanced through the reactor and adapter into semantic panel rendering.

## Semantic Panel and Intent Bridge

- The shell panel cache now consistently carries authored `BuiltNode` trees rather than the removed recursive `UiNode` compatibility model.
- Centralized full-body `BuiltNode` to flat retained-snapshot reconciliation in `UiDocumentStore` and reused it from both shell window and panel hosts.
- Semantic panel bodies render through the real `UiDocumentStore` and interpreter path.
- Semantic `UiIntent` values now bridge to the existing plugin action channel, preserving scope, version, args, and input precedence instead of being reported and dropped.
- Focused semantic tree, intent, and history-panel regressions passed: 3 passed, 436 skipped.
- Full renderer quick suite passed: 439/439.

## Final Automated Gates

- Demonstrator plugin quick suite: 45/45 passed.
- OS development/materializer quick suite: 44/44 passed.
- Plugin descriptor quick suite: 17/17 passed, 2 skipped by profile.
- Demonstrator production Vite build: succeeded in 9.10 seconds through its Nx target.
- Scoped `git diff --check`: clean.
- Renderer typecheck remains a repository-wide red baseline with 407 current contract-migration errors across unrelated brand fixtures, Three renderer declarations, shell state, and backbone-worker types. The three touched fixture errors were corrected (410 errors before, 407 after); no diagnostic names the new snapshot or intent bridge functions. Runtime build and all renderer tests are green.

## Extension and Production Runtime Routing

- A six-route live-server asset audit resolved every pane through the generated catalog and found a remaining integration defect: all 13 consumed Flow/Process extension URLs returned the SPA HTML fallback instead of JavaScript.
- Moved the demonstrator's transitive runtime-layout calculation into its permanent `📜️script.ts` router and split ordinary plugins from extensions according to their catalog roles.
- The Vite config now mounts plugin crates at `/plugin-modules`, extensions at `/extensions`, and includes the pooled `_shard` runtime in production output.
- Added a red/green regression for the nine-plugin, thirteen-extension closure plus `_vendor` and `_shard`; the demonstrator suite passes 4/4.
- The corrected live server resolves all six default app ids from their staged descriptors and serves 27 required runtime assets with valid JavaScript/Wasm content types, including all 13 extensions.
- A recursive dependency crawl then found that installed extension components retained their build-tree `../_vendor` Preview2 imports. The extension publisher now rebases bare or staged shim imports onto the public `../../plugin-modules/_vendor` root instead of requiring an 8.4 MiB duplicate vendor tree.
- Added a red/green publisher regression and republished the demonstrator's 13 consumed extensions. The full OS-dev quick suite passes 45/45.
- The final live dependency crawl resolves 56 JavaScript modules and 22 Wasm cores with all 13 extension components rebased and zero HTML fallbacks.
- Forced the production build with Nx cache disabled; the final build succeeded in 8.90 seconds.
- The production dependency crawl resolves the same 56 JavaScript modules and 22 Wasm cores. The tree contains 11 plugin/runtime directories, 13 extension directories, the shared shard worker, and every `install.json` module URL it advertises.

## Current-Source Route Repairs

- Removed the duplicate Puzzle action declaration and migrated the React node-graph host from the retired selection/hover actions to the generic interaction actions.
- Gave the Flow evaluator an isolated receiverless fallback session and bounded the framework document, config, and interaction store owners.
- Corrected the shell's identity-config import boundary and the CAD proof factory identifier.
- Declared the exact bounded/resumable publication lanes for every Sourcing command used by the demonstrator.
- Replaced the tiled-map host's retired selection setters with the current `syncInteraction` Wasm ABI and added a focused host-mapping regression.
- Added a fully app-owned retained-command path for all fourteen GIS map commands, including exact publication contracts and bounded one-item artifact/config preparation.
- Added retained-factory ownership/contract regressions for Procedural3D, Sourcing, and GIS2D.

## Current Automated Gates

- `@semio-tech/framework-renderer-react:test`: 4 files and 461 tests passed.
- `@semio-tech/mit-bestand-demonstrator:test`: 2 files and 5 tests passed, including explicit-only staged-artifact reuse.
- `semio-s-plugin-stdio` current-source `wasm32-wasip2` library check: passed in 4m40s after the concurrent schema corrections.
- `@semio-tech/framework-os-dev:test-quick`: 49/49 passed, including explicit-build failure propagation.
- Scoped `git diff --check` for the route and renderer changes: clean.

## Explicit Build Failure Propagation

- Added a red/green regression requiring explicit plugin builds to reject incomplete catalogs after attempting all targets. Previously the command printed failed targets and returned success, allowing stale artifacts to look freshly built.
- Kept streaming dev builds best-effort so one failed optional plugin does not prevent the rest from being attempted.
- The first test attempt was blocked by the concurrent assets README taxonomy move. The registered `@semio-tech/assets:build` target restored its declared generated output and completed successfully with 286 deterministic outputs.
- Evidence: `🧪️explicit-build-failure-red-2.log`, `🧪️explicit-build-failure-green.log`, and `🧪️assets-taxonomy-prerequisite.log`.

## Fresh Launch Inputs

- Removed the demonstrator launcher's automatic reuse of any staged core and its implicit engine-build skip. Normal launches now build current plugin/engine inputs; the explicit `SKIP_PLUGIN_BUILD=1` and `SKIP_ENGINE_BUILD=1` iteration controls remain available.
- The launcher policy regression was red before the implementation and the complete 5-test demonstrator suite is green afterward. Evidence: `🧪️launch-freshness-red.log` and `🧪️launch-freshness-green.log`.
- The aggregate compile completed stdio, then exposed a Puzzle5D macro forwarding error. Changed the forwarded tool capture from `literal` to `tt`, matching the concrete publication-contract macro arms; the existing exact-contract test covers all four reserved tool values.
- The next aggregate compile passed that macro boundary and exposed four native-size byte counters assigned to schema-level `u64` checkpoints in Puzzle3D Config and GIS terrain preparation. Added explicit widening conversions at those four publication boundaries.
- Build 14 also confirmed the explicit-build failure propagation at runtime: the failed compiler now causes the Nx plugin target to fail instead of reporting success.

## Current-Source Preflight Gate

- New concurrent store/plugin edits invalidated shared dependencies again, so build 15 was intentionally interrupted before its expensive stdio link. Switched to `bunx nx exec -- cargo check -p semio-s-plugin-demonstrator --lib --target wasm32-wasip2 --message-format=short` using the same ticket cache.
- Check 16 caught eight newly introduced stdio errors: TIFF replace/remove-tag text parsers referenced a missing `u16_arg` closure, and JPEG quantization/Huffman plus PNG text-chunk mutations could not access schema-local diff helpers.
- Replaced the two missing parser calls with explicit standard-library `u16` parsing and widened only the six required diff helpers to `pub(in super::super)`, keeping them internal to their schema. Check 17 exposed the generated private `diff::component` wrapper, requiring the schema boundary two levels above rather than the immediate parent.
- Check 18 reduced the errors to two collection-level JPEG `is_empty` helpers, now widened to the same schema-local boundary. Check 19 is validating the complete correction before another full link. Evidence: `🧪️aggregate-current-check-16.log` through `🧪️aggregate-current-check-19.log`.
- Check 19 passed stdio and all six foreign app crates, then found two identical `usize` → schema `u64` checkpoint assignments in the demonstrator's own Playground preparation. Widened those boundaries; check 20 validates the final aggregate.

## Panel Surface Runtime Repair

- Provisional live Aggregator inspection rendered real geometry in both Top and Perspective views and produced no console warnings/errors, but opening its catalogue remained on the pending tree placeholder.
- Traced this to `PluginRuntime.refreshUi`: it submitted only window surfaces and explicitly omitted all panel bodies, despite the shell requesting them.
- Added window/panel body-key deduplication and shared retained-surface projection, including panel-only refreshes. A language-agnostic JSON fixture pins mixed, panel-only, and empty request ownership.
- Red evidence: `🧪️panel-surface-refresh-red.log`; focused green evidence: `🧪️panel-surface-refresh-green-3.log`. The first two green attempts hit suite startup time budgets under host load; selecting the owning inline suite completed in 5.49 seconds.
- The first live multi-surface request published only the window and inspector, then exhausted 4,096 continuations while waiting for the other four surfaces. The host deferred every acknowledgement until all requested surfaces arrived.
- Added per-turn retained-patch acknowledgement during settling, including patches returned by acknowledgement turns. The language-agnostic fixture also pins a one-unacknowledged-surface backpressure case. Red: `🧪️panel-patch-ack-red.log`; all 39 PluginRuntime tests green: `🧪️panel-patch-ack-green.log`.
- These provisional browser loads still use the older published core and are not final current-source proof. Concurrent repository-infrastructure edits repeatedly restart the dev server and interrupt descriptor fetches; final verification should use the production preview after both components are freshly published.
- The provisional production build succeeded in 15.55 seconds (`🧪️demonstrator-panel-preview-build.log`). Added an explicit Vite `import.meta.vitest` compile-time definition so inline test bodies and their fixture chunks are stripped from the final client build.

## Pending Runtime Evidence

### Current Continuation

- Check 20 caught new TXT mutation codec imports pointing at the schema rather than mutations, plus a native-size protocol offset. Corrected the two codec imports and the `usize` → `u64` boundary. A concurrent codec rewrite also introduced function-pointer method calls; both now call their `decode` fields explicitly.
- Build 16 compiled a prior TXT source snapshot and reported the already-corrected offset; stopped its known-red compiler and restarted. Build 17 encountered a temporarily missing canonical-store borrowed module during another writer's update; the file appeared without intervention, and build 18 is running.
- The final focused browser-runtime suite passes all 39 tests, including command-turn acknowledgement settling (`🧪️panel-runtime-final.log`).
- Production preview is now stable on `127.0.0.1:6036`. Its currently published older core still stalls on six requested Aggregator surfaces, publishing only main and inspector. This is not final proof.
- Added a six-shape language-neutral mounted-surface fixture and a test through `reserve_mounted` → `commit_source` → real producer/reconciler publication. An isolated ticket Cargo manifest compiles the actual production tracker source without the unrelated native engine dependency tree. Native UI-runtime and mounted-source checks are running; their results are not yet known.
- The mounted-source regression failed on a two-child document with the job still in `Drive`, no fault, and its reservation intact. The semantic census counted a separate opportunity per byte of fixed text capacity. Changed this accounting to pages bounded by the existing 32 KiB page limit, preserving total byte charges and cancellation/fuel boundaries. All six fixture surfaces now publish within the same 4,096-opportunity cap (`🧪️mounted-surface-source-red-2.log`, `🧪️mounted-surface-source-green.log`).
- Corrected the diagnostic command invocation: unqualified `nx exec` runs the command once per workspace project. Successful focused checks had therefore repeated; stopped that invocation and now specify `--projects=@semio-tech/mit-bestand-demonstrator` for every ad-hoc Nx command. Build targets already select their exact project and are unaffected.
- The pre-change native UI-runtime suite passed 79/82 tests. One 8 ms wall-time assertion observed 18.87 ms under host load; two later tests encountered unreleased global admission after earlier tests. The post-change suite is running separately. Build 19 is queued to include the page-census correction; build 18 remains the compile-baseline build without that correction.
- Adding actual action bindings to the same mounted fixture reproduced `PageBytes { actual: 227560, max: 32768 }` on a single interactive child. The census treated an entire fixed binding backing as one page, and also constrained cumulative node ownership to one page. Backing ownership is now charged across bounded pages, with the existing per-step page and aggregate surface limits retained. The interactive six-surface fixture is green (`🧪️mounted-interactive-surface-red.log`, `🧪️mounted-interactive-surface-green.log`); temporary fault logs were removed, and the fixture now independently pins its action wire data through serde JSON.
- Post-page-census native runtime suite: 80/82 passed; both remaining global-admission failures pass when run independently (`🧪️ui-runtime-cancellation-single.log`, `🧪️ui-runtime-ownership-single.log`).
- Full browser-renderer suite: 466/466 passed under the long profile. The quick profile reached its 30-second process budget under host load, without reporting an assertion failure. Evidence: `🧪️renderer-panel-final-long.log`.
- The latest frontend production build succeeded, strips the inline surface-refresh fixture chunk, and includes the referenced selectable cursor asset. It still needs freshly published aggregate and procedural Wasm cores for final live proof.
- Updated the previous single-node page rejection test to give that node an explicit aggregate byte quota; it still rejects before identity/record cloning and preserves the exact source owner. The focused quota test passes (`🧪️ui-runtime-aggregate-quota-final.log`). The final interactive mounted fixture with its serde action oracle also passes (`🧪️mounted-interactive-surface-final.log`).
- The independent Procedural component build is queued behind the aggregate cache lock (`🧪️procedural-current-source-20.log`). Do not use build 18 as final browser evidence: it predates the semantic accounting corrections. Build 19 and the Procedural build are the required publication candidates.
- The registered process-isolated UI-runtime gate passes all 82 tests in 4.83 seconds (`🧪️ui-runtime-registered-gate-2.log`). This confirms the two earlier libtest failures came from shared in-process admission state, not independent runtime regressions. The registered runner uses nextest; its artifacts are retained in `🧪️runtime-nextest`.
- Added an authored settings shape: one section, four labelled fields, and four action-bound numeric controls. It publishes through the actual mounted tracker within the existing cap, and serde JSON independently verifies the resulting values, steps, and action names (`🧪️mounted-settings-source-2.log`). The initial oracle used integer JSON literals for floating-point fields; the fixture now declares those values as floats.
- Added the default document-tree shape (19 nodes, 14 selectable rows, nested vortices, and six hide/lock row actions). It reproduced a second publication failure: `Credits` at node 10, 2,123,341 bytes, exceeding the old 2 MiB allowance (`🧪️mounted-document-source-red.log`). The fixed surface allowance is now 8 MiB with a four-surface aggregate budget of 32 MiB; the per-step page remains 32 KiB and the existing node/item limits remain enforced. The same document fixture publishes successfully (`🧪️mounted-document-source-green.log`), and its JSON pins all three byte limits.
- The broader tracker gate exposed obsolete identity assertions comparing an inline `UiText` address across a Rust move. Those refusal tests now compare the heap-owned child address, preserving their exact-owner invariant. The complete process-isolated tracker and UI-runtime gates are rerunning after these changes.
- The broader tracker gate also reproduced an actual close leak: retiring `SurfaceReconcileReadyPatch.patch` removed the surface ID used to find the remaining ready owner, leaving its credit/handback unreachable from subsequent close steps. `ReadySlot` now retains the owning instance independently of its retiring body. The close test pins ready/deferred/queued/active/terminal ownership in a language-neutral JSON fixture.
- Updated three stale tracker tests to measure their intended invariants: empty slot placement consumes no extra reconcile opportunities, the real aggregate allowance bounds admission, and identifier-cap rejection supplies an owned rejected tree. All 20 tracker tests pass after the close fix (`🧪️mounted-tracker-full-nextest-3.log`); the updated 8 MiB surface budget also passes all 82 registered runtime tests (`🧪️ui-runtime-registered-gate-3.log`).
- A two-second native compiler sample confirms build 18 is actively generating LLVM code, not deadlocked (`🧪️stdio-compile-sample.txt`). Its physical footprint was 26.4 GiB under shared host pressure. Builds 19 and 20 remain serialized behind its target-cache lock.
- After formatting and the JSON close fixture, the final tracker gate still passes 20/20 (`🧪️mounted-tracker-full-nextest-final.log`). Closed the stale browser proof tab to release its Wasm instance; the stable production preview remains running on port 6036. Create a fresh tab after both current-source components publish.
- Restarted only the two idle cache-lock waiters (Cargo PIDs 10242 and 40628, neither had child processes). The replacement aggregate build 21 and Procedural build 22 have four-hour bounded build budgets, so lock waiting cannot consume most of their compile window. The active baseline compiler was untouched. Current final-proof candidates are now `🧪️aggregate-current-source-21.log` and `🧪️procedural-current-source-22.log`.
- Paused only build 18's obsolete Bun wrapper, PID 82026, before its original one-hour timeout could discard the in-flight stdio code generation. Cargo PID 82291 and rustc PID 84290 continue normally; 255 incremental object files already exist. Resume wrapper PID 82026 with `SIGCONT` as soon as Cargo PID 82291 exits/becomes a zombie, so it can reap the child and finish. This is a temporary process-control measure for the owned baseline build, not a repository script change. Do not leave the wrapper paused after the Cargo stage completes; cancel the owned stage if it exceeds the replacement four-hour bound.
- Resumed wrapper PID 82026 after Cargo 82291 exited; both processes are now gone. Build 18 completed stdio, then found an incorrect relative module path for the newly added Flow retained owner. Corrected only that module reference from three parent levels to two, matching its actual location alongside Flow artifact/catalogue. Build 21 is compiling the current dependencies and will validate this boundary; build 22 remains queued behind it.
- Reviewing Generator's authored catalogue found four `outputExport` entries sharing one reconciliation key. Extracted its existing key calculation into the catalogue's identity unit and added a language-neutral serde JSON oracle. The regression failed with only four distinct keys for seven entries (`🧪️catalogue-identity-red.log`). Keys now include the semantic neuron, export format, or action variant; the remaining plain widget keys stay family-based. The live catalogue calls this same production unit, which an isolated ticket manifest tests without recompiling unrelated format codecs.
- The catalogue identity unit passes its seven-case JSON oracle (`🧪️catalogue-identity-green.log`). Refreshed launch/build gates also pass: demonstrator 5/5 and OS dev 49/49 (`🧪️demonstrator-launch-gate-final.log`, `🧪️os-dev-gate-final.log`).
- Added a host regression for a quiescent actor that has published its window but not its requested catalogue. It reproduced a successful partial response, which leaves the missing body on its loading placeholder (`🧪️idle-surface-red.log`, 39/40 passed). The host now rejects that incomplete terminal response with the exact missing surface IDs, while unchanged retained refreshes and instance-open still require no new publication.
- The complete renderer suite passes 469/469 after this correction (`🧪️renderer-terminal-surface-final.log`); focused host runtime passes 40/40 (`🧪️idle-surface-green.log`).
- Build 21 completed refreshed stdio code generation in about 17 minutes, then failed on 29 Flow errors. Several were stale compiled neural interfaces: the current source already contains the new retirement APIs and is changing channel maps to ordered storage. Corrected the still-invalid Flow imports to the actual `protocol::value::ordered` owner, declared its existing internal workspace dependency, and replaced two obsolete single-argument BTreeMap extraction calls with `pop_first`. Build 22 had resolved its dependency graph before the neural dependency was added and failed on the absent `protocol` import. Both builds have exited and published nothing. Current-source check 21 is now validating a freshly resolved dependency graph before the next full compilation.

- The earlier watched dev server on `127.0.0.1:6035` has been stopped; use the stable production preview on `127.0.0.1:6036` for final proof.
- The last published demonstrator core predates the CAD, Procedural, Process, Sourcing, GIS, and tiled-map fixes, so it cannot serve as final route evidence.
- The earlier stdio Wasm check was green. Build 13 completed stdio but stopped on the Puzzle5D macro error. Build 14 completed refreshed stdio and stopped on the four checkpoint counter mismatches. Build 15 was intentionally interrupted to run the faster current-source preflight described above. Evidence is in `🧪️aggregate-current-source-13.log`, `🧪️aggregate-current-source-14.log`, and `🧪️aggregate-current-source-15.log`.
- Complete the warmed build, confirm and republish the fresh Wasm component, then record DOM, canvas, loading-state, and console evidence for `generator`, `koordinator`, `aggregator`, `aussuchen`, `bearbeiten`, and `verfolgen` before closing this ticket.
