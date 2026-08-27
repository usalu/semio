# Renderer Boundary Review

## Current Continuation Checkpoint

The coordinator independently executed `NX_DAEMON=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache`: exit 0, **473 passed, zero failed, four files**, 21.27 s total and 6.09 s test time. This includes bilingual label resolution and six-kind action-semantics parity. The original `🧪️coordinator-renderer-react-full-r3-2026-08-27.txt` was read directly after completion, then disappeared from disk during an external artifact-loss event; the observed result is preserved here without recreating a fictional complete log. A later executor checkpoint has 474 tests after adding real Presence replacement coverage; coordinator verification of that changed tree remains pending.

The executor's full captured typecheck r8 is RED at 47 diagnostics, down from 133 at r7 and the coordinator's earlier 385 checkpoint. The coordinator read the r8 diagnostics: remaining groups include full tutorial local-state contracts, the obsolete World3d effect consumer, actual Wasm session loaders, a few fixtures/metadata, and five transitive CLI-discovery diagnostics. This is not a zero-error claim.

The remaining old World3d effect is being removed coherently from WIT, native/TypeScript schemas, host mappings and renderer code, with current Presence behavior tested first. BoardSession must stay puzzle-owned behind a registered product factory because the generic surface module does not export it. Both boundaries require fresh Wasm verification. The actual whole-document patch implementation below is still an independent open requirement.

## Measured Whole-Document Patch Diagnostic

The coordinator invoked the actual UiDocumentStore.applyPatch implementation through Bun/Nx, using valid 4,096-node and 20,000-node documents within the unchanged production limits. Each of 30 patches changed one scalar text node; all revisions advanced correctly and validation passed. Timed work includes the real draft clone, whole-document validation and notification scan, not fixture construction.

| Nodes | Calls above 8 ms | Median wall time | Maximum wall time |
| --- | --- | --- | --- |
| 4,096 | 13/30 | 7.195 ms | 231.164 ms |
| 20,000 | 30/30 | 90.086 ms | 348.977 ms |

This is a local Bun diagnostic during concurrent compilation, not an isolated benchmark, Chrome measurement, browser input-latency measurement, or attribution of every delay to application CPU. It nonetheless executes the real monolithic function and provides no evidence for an 8 ms bound. The implementation must be decomposed; final latency acceptance still needs isolated/native/browser/device runs.

Output: `🧪️coordinator-ui-patch-latency-diagnostic-r2-2026-08-27.txt`. The first attempt failed before execution due to Nx exec argument quoting and implicit project selection; it is retained as r1, not a performance result.

## Typecheck Gate

The coordinator's full r2 rerun remains RED with **385 diagnostics**, down 30 from r1: all 26 NodeGraph diagnostics and four affected fixture sites are cleared. The executor's earlier 112 count came from truncated output and has been corrected; the retained complete log is authoritative. The coordinator reviewed the NodeGraph diff and native first-separator handle grammar. Graph hover now decodes the actual node@port handle instead of reading a nonexistent portId field; source tests include eight strict fixture cases and an independent regex oracle. Log: `🧪️coordinator-renderer-react-typecheck-r2-2026-08-27.txt`.

The separate canonical React renderer typecheck is **RED: 415 TypeScript diagnostics across 31 files**, exit 1. Full functional tests passing does not establish type correctness. Counts are diagnostic locations, not unique root causes or new regressions. The largest groups are renderer tests (86), world/r3f (81), ShellHelpers (70), ChromePanels (33), and NodeGraph (26). Missing owned facade exports, schema/union narrowing, action definitions, mutation-envelope fields and test fixture drift appear in the output. No compiler strictness was weakened or file excluded.

Exact retained output: `🧪️coordinator-renderer-react-typecheck-r1-2026-08-27.txt`; machine-readable distribution: `📊️coordinator-renderer-typecheck-r1-distribution-2026-08-27.json`. This repair is a required renderer/integration gate, separate from the retained patch pipeline below.

## Full React Regression

Following the graph-host schema/fixture repair, the coordinator independently reran the entire suite: **470 passed, 0 failed, four files**, 22.35 s total/4.98 s test time, exit 0. Log: `🧪️coordinator-renderer-react-full-r2-2026-08-27.txt`. All previous tests still run; one pick-target conformance law was added.

The coordinator ran the complete React renderer `test-long --run` target through Bun/Nx: **469 passed, 0 failed, four files**, 12.91 s total and 4.38 s test time, exit 0. This includes the earlier seven graph-parameter/slider cases without filtering the rest of the suite. Log: `🧪️coordinator-renderer-react-full-r1-2026-08-27.txt`. These are mounted DOM/source/unit regressions with mocked Wasm boundaries, not fresh deployed Wasm, all-app authoritative edits, or an 8 ms maximum-envelope proof. The monolithic paths below remain open despite a green functional suite.

## Browser-Worker Gate

Coordinator rerun r2 **passes 32/32 tests in two files**, 49 ms test time / 622 ms total. The source-contract check now verifies exact admitted `try_into_retirement` handoff, rejected-owner restoration and terminal witness, and rejects four hostile substitutions. Production retirement was not weakened. This supersedes r1's stale-assertion failure, not the independent Wasm/live-browser and React monolithic-path gaps below. Log: `🧪️coordinator-browser-worker-r2-2026-08-27.txt`.

Coordinator ran `NX_DAEMON=false bun x nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache --args='--run'`: exit 1, 31 passed, 1 failed, 32 total across two files, 364 ms. The failure is the source-contract assertion at `🧪️browser-frame-transport.test.ts:292`, expecting the obsolete spelling `OsHost::into_retirement`. The current `🦀️browser_worker.rs` instead matches admission-aware `host.try_into_retirement()`, restores the exact host on rejection, then incrementally closes the admitted owner and checks terminal emptiness. The executor will strengthen the assertion around that current contract; reverting the production rejection ownership would be incorrect. Log: `🧪️coordinator-browser-worker-r1-2026-08-27.txt`.

This test does not instantiate current Wasm or exercise a live browser. The remaining 31 tests are transport/interactive-port checks, not all-app visual proof.

## Still-Monolithic React Patch Path

Source review finds a connected, production-reachable whole-document path in `PluginRuntime/🟦️component.tsx` and `UiDocumentStore/🟦️component.tsx`:

1. `applyRetainedWindowPatches` loops all incoming patches and `decodeWirePatchOps` loops all operations, including decoding arbitrary component/style/binding values.
2. `applyUiPatchToRetained` copies the operation array and calls synchronous `applyUiPatch`.
3. `applyUiPatch` estimates the full payload, copies the entire retained Map, applies every operation, and validates the entire graph before returning. Per-patch quotas are not a cursor or timing proof for the already-large retained graph.
4. `retainedUiRefreshResponse` recursively builds the entire BuiltNode tree, flattens all nodes to a snapshot, JSON-stringifies the whole snapshot, encodes it, and hashes it for each requested surface.
5. UiDocumentStore publication compares all old/new nodes and notifies all changed listeners in one call.

These functions remain synchronous in the React host path; successful worker transport tests do not certify them. A future executor packet must provide retained decode/apply/validation/tree-build/hash/notification stages with exact revision authority, atomic publication, bounded payload ownership, cancellation, and deep/wide graph fixtures. It must not merely insert timers around whole-map clones or whole-graph validation. No actual timing violation has been measured by this source review, but these paths cannot receive the required boundedness credit yet.

## Native Renderer Distinction

The current native renderer is further along than old status notes: `redraw_core` polls/submits a worker-owned frame, and `FrameTransaction` contains retained build/input/effect/authority phases guarded by operation/generation/base witnesses. The `FrameBuildJob` completed-session handoff does not recursively destroy its job: WorkerJobSession Drop transfers its pre-admitted retirement node to the bounded abandoned-session registry. This specific handoff is not a newly established bug.

Browser bootstrap still has whole font-atlas, icon-atlas, plugin-parse and shell-construction calls inside individual step branches. The existence of named stages and icon-count admission alone does not prove their maximum-envelope step time or complete cancellation lifecycle. Native renderer, bootstrap, fresh Wasm, live apps, input storms, accessibility and multi-device timing remain independent required gates.
