# Independent Vite Input Source Review

2026-10-15, initial Root integration snapshot. Source only; no runtime/test/physical command or production edit. Caller adoption remains separately pending per census.

## Blocking Findings

1. **P1: Input helper receives no carried actual control.** `🌐️vite/📥️inputs/🟦️.ts` invokes `withArtifactStorageOperationV1(...,actual=>visit(0,actual))`. Actual imported `📦️artifacts/💾️storage/🟦️.ts:24` declares `run:()=>Promise<T>` and calls `run()`; it yields no control argument. Therefore `actual` is undefined. With no inputs, actual build/serve body receives undefined control and fails when using signal/control. With inputs, phase selection/reader admission fails or loses its intended supplied phase. This also produces an incompatible callback TypeScript signature. Small correction: callback `()=>visit(0,options.control)`; the storage wrapper tracks precisely that supplied phase and does not issue a replacement. Do not change the actual storage API merely to accommodate this caller.

2. **P1: Listener cleanup is skipped when initial final-drain rejects.** `🌐️vite/🟦️.ts` serve finally starts with `await Promise.all(environments.map(async environment=>{await environment.waitForRequestsIdle();await environment.depsOptimizer?.scanProcessing;}))` before entering its try/finally around listener/server close and cache rm. Any rejection there exits finally without closeAllConnections, socket destruction, server.close or cache cleanup. The input callback then rejects and nested SH leases unwind while the actual listener can remain live and still reference those input paths. Cleanup must be entered unconditionally, collect drain failures, stop admission/listener and close every environment, then propagate aggregate errors only after all accepted requests/optimizer work/close settle. Existing success-path enclosure is not enough for this failure path.

## Positive Source Observations

The helper imports actual firstparty reader/storage/control interfaces and no new runtime library. Requests are JSON-copied, closed-schema validated against the existing request schema, absolute/canonical, distinct by path and sorted before acquisition. Its own schema max8 is explicit. Reader SH plus accepted task recursion encloses the actual body, with each deeper layer using artifact.control; once the initial control defect is corrected, each reader checks/refines that phase through actual reader/task storage wrappers rather than fabricating a control. The precise global store active-frame count still needs an existing Source/runtime limit law; this review did not execute the cap8 path.

Build enclosure begins before config text read/compiler state, covers actual child runOwnedCommand completion and output publication, and unwinds only after compiler/writer cleanup. Output/input exact path equality is refused; overlapping ancestor/descendant requests are not comprehensively forbidden, so caller discipline and separate artifact resources remain relevant. No actual overlapping production caller was evaluated in this integration review.

Serve enclosure begins before Vite import/config initialization and covers initial optimizer scan, transforms, listener lifetime, final server.close and snapshot publication on its successful drain path. Transform admissions derive from the carried input control via withRequestedIssuedResourceControlV1(control,600000,...), and use that child for compiler capacity admission. Actual Vite transform itself takes no control argument; lifecycle relies on retained promise/close, not an invented abort API. Optimizer discovered-processing promises settle before capacity release.

Snapshot now derives from the carried control, skips when control.signal.aborted, and uses snapshotControl.signal for artifact collection and snapshotControl for publication. It does not create an unrelated fresh cancellation source. The ordinary stop path is cancellation, so it intentionally skips snapshot; this is Source behavior, not independently executed behavior.

## Remaining Qualification

Fix the two concrete defects before runtime admission. Verify empty and nonempty request lists, deepest carried control/refinement, cap8/global pending accounting, rejection while waiting/initializing/draining, no premature input EX availability, actual child terminal settlement, listener/socket shutdown and optimizer completion. Production adoption must include both census bypasses, not just generic build/serve calls. No runtime success, close guarantee or resource-limit pass is claimed here.
