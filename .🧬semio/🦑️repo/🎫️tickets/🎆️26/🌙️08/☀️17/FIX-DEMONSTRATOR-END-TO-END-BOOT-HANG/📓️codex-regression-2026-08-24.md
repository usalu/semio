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

## Pending Runtime Evidence

- A fresh Vite server is ready on `127.0.0.1:6029` with the final rematerialized plugin.
- The controlled in-app browser's generated connection-error tab was subsequently closed; no controlled tabs are currently open. Its security policy had rejected reloading that document and explicitly forbade alternate navigation workarounds.
- The user must manually reload/open `http://127.0.0.1:6029/#aggregator`; after that, record console/DOM evidence for all six focused routes before closing this ticket.
- Final blocked-state audit: the Vite session remains alive and `HEAD /` returns HTTP 200, but the in-app browser tab list is still empty on the third consecutive goal turn. No policy-compliant browser surface exists until the user opens the URL, so the repo ticket remains open and the goal is blocked solely on the six-route DOM/console pass.
