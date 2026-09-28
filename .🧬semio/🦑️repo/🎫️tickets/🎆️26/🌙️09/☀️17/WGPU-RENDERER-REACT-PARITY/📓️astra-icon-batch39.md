# Icon Export Batch Integration 39

## Current SVG Integration

The SVG file set is now coherent. Native40 stopped before compilation because the repository test wrapper forwarded a run-only `--success-output` option to `nextest list`. Native40b now invokes the supported `cargo nextest run` through scoped Nx exec, with fail-fast disabled and successful-test output retained in `🗑️generated/astra-runtime/gate34/native40b-export.log`. WASM30 stopped in its Cargo dependency prerequisite because the live manifests and lockfile were out of sync. Scoped offline `cargo metadata` completed successfully; WASM30b then passed that prerequisite and entered the actual renderer build. Its log is `wasm-30b.log`. Neither current build is a passing receipt yet.

Physical Shooting reference export remains unverified. Pointer targeting in the long Actions pane initially selected neighboring rows. Keyboard Tab reached the exact `Export Active Shot` button, and Return selected that action, but the browser download event did not arrive within 20 seconds. No export error was logged. This establishes keyboard reachability and an unresolved export/host-delivery observation; it does not prove either a successful download or a renderer serialization failure. No file was captured.

The Sol six-law UI attempt completed with 2 passed, 1 failed, and 3 skipped. Its failure was `same_size_moved_viewport_discards_the_old_origin_candidate_before_atomic_publication` at the first-origin unpublished assertion. Readback did not execute because that invocation stopped on the first failure. That run used an interactive terminal without a saved log; the verdict is agent-reported, not a root-inspected persistent receipt. The exact viewport-publication failure and a logged six-law rerun are assigned to Sol.

The Shell now validates the output format before fetch and sends decoded SVG assets directly to the first-party vector serializer. SVG uses `image/svg+xml`; PNG uses the accepted scene/GPU route. Both paths retire their request-owned resources before the save operation. Cancellation also drains SVG serializer and rejected-asset owners incrementally. These changes remain source-stage pending the coherent native gate.

Native39 finished with two compiler errors and no executed tests: its test target encountered the SVG module declaration before the agent's new native law file existed; its library target located the outstanding E0716 in the Shell export status's temporary localization key. The key now has a local binding. A coherent warm follow-up is required after the agent finishes the SVG file set.

The Projection label-order gap was subsequently reproduced and fixed in the shared React Tree stylesheet, with 2/2 tests and twelve Chromium order/direction comparisons passing. The live app also displays the corrected order. See `📓️astra-tree-text-flow40.md`; arbitrary-script WGPU shaping remains outside that focused receipt.

## Previous Checkpoint

The browser exact-request cancellation oracle completed with 2/2 passing tests (Nx exit 0). The renderer native37b and reserved UI six-law runs remain in the shared Cargo queue. No native export success is claimed.

The Shell now has an IconRenderExport effect arm, request-owned batch state, bounded scene/GPU/asset retirement before download, cancellable native save delivery, an incremental progress band, and an accessible cancel/status projection. The work is source-stage only and has not yet compiled or run. Neutral batch admission/cancellation fixtures and native laws were added before wiring. SVG delivery still requires the real vector serializer; it currently reports a failure rather than generating the wrong format.

The scene bundle retains World mesh owners through GPU readback. Shutdown explicitly drains the batch before the World close ladder. A save dialog cancellation stops the remaining batch; exact request tokens cannot cancel a recycled slot or another asset.

Admission still caps the WGPU pending item queue at 64, whereas the current React host iterates every item in an export effect. This is an explicit remaining batch-size parity gap: a bounded queue of request-owned batches is needed so a large `Export All Shots` request can advance and cancel incrementally without rejecting the entire request or bulk-dropping its remaining payloads.

Outstanding: compile and execute the new batch/asset/GPU/scene laws, connect SVG output, strengthen rejection and cancellation integration coverage, verify real Shooting export in both native and browser WGPU, and rebuild the comparison artifact. Full renderer parity remains open.
## Verification Update

The new batch schema/admission oracle passed 2/2 (Nx exit 0). Rustfmt parsed the linked batch module successfully; that is not type checking. Native37b completed after 49m30s with eight compile errors in new export code: CancelToken constructors, obsolete prepared-test constructors, input cleanup visibility, World build-context cleanup, rejection Debug, and one temporary borrow. Constructor and scene API repairs are on disk. A warm full renderer library/tests check is active with short compiler diagnostics retained in native39-check.log. New exact-request registry fault/delivery/abandonment laws are staged and unrun.
The Shell batch now advances at most 32 bounded substeps or 2 milliseconds per settle turn, preserving small readback/encoding pages without forcing one frame per page. A new unrun integration law sends the actual IconRenderExport effect through ShellState, paints its progress/control, activates the published accessibility target, and requires empty owners after cancellation. The GPU cleanup fault path retains its owner if retirement itself is blocked; that is a known diagnostic boundary to exercise in native verification.
Current follow-up: the former 64-item admission gap is repaired in source with scoped cancellation and a 6/6 independent oracle; native execution remains pending. See `📓️astra-export-queue41.md`. The browser wire and mesh transport gaps have fail-first and passing regression receipts; actual Chromium download-helper delivery passes, while the Shooting in-app browser saved-file observation remains unverified. See `📓️astra-export-wire41.md`. The notes below retain the earlier checkpoint for traceability.
