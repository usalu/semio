# Direct Canvas Manipulation

Native suite session46142 has now terminated at compilation with nine incorrect NoConfig value constructions in the lasso/drag tests. Those fixture calls were repaired to use NoConfig::default(), and the current full suite is session48654 (`tests-convert-all.txt`). The registered-editor drag regression and all direct-drag runtime claims remain unverified until this suite executes.

Implemented the first object-drag path through the retained gesture authority. An unmodified pointer-down with the direct-selection tool now starts the existing incremental hit-test query. Candidates retain their bounded tree path; the selected target captures its original transform and composed parent matrix. The statechart has a moving_layer state, so the retained operation persists across pointer commands. Modified clicks retain additive/subtractive selection behavior.

Movement updates only a transient translation preview. Canvas scene records apply the world displacement to the selected leaf's transform without modifying the document. Release converts world displacement through the inverse parent linear transform and commits one UpdateLayerTransform mutation. Returning to the initial point, movement below the threshold, Escape, and a cancelled pointer release produce no document edit. Canonical revision or utility changes continue to retire the existing gesture owner.

The translation core has Rust/TypeScript twins and shared language-neutral cases for root, scaled-parent, rotated-parent and singular-parent transforms. Three.js Matrix3 inversion independently validates the parent-space displacement. The preimplementation test failed for a missing module; `ts-drag.txt` passes18 tests/366 assertions,42 field-patch cases and28 publication routes. Native tests now cover ephemeral previews, exactly one release mutation, nested transforms, cancellation and subthreshold clicks. They have not yet executed.

Current checks: native library job session80134 (`check-fill.txt`) completed successfully in17m20s after the direct-drag production code was added. Full native suite session46142 (`tests-lasso-all.txt`) remains live; its source-capture timing matters because drag changes were made after launch. Do not start a duplicate while it is queued/building. `git diff --check` passed for Draw.

A production-editor regression now drives SetSnapshot, SetCamera, pointer-down, batched pointer-move and pointer-up through the registered retained factories. It reads the actual Canvas2dScene before and during the drag, checks the world displacement in the rendered transform, verifies no snapshot edits or revision publications during the preview, and checks cancellation versus exactly one release publication. This new test is authored but has not yet executed; the live full suite may have captured its sources before this addition.

The Draw-only catalog attempt failed preflight because the committed descriptor hash differs from the component-dev deliverable. A targeted `describe materialize-dev` run for @semio-tech/draw-plugin is now live in session55411 (`draw-describe-drag.txt`), so the subsequent catalog can validate one matching component and descriptor. This is not browser runtime proof.

Remaining scope: this first drag path moves one hit leaf using the direct-selection tool. Multi-selection dragging, group movement, resize/rotation handles, snapping, and direct node/handle dragging still require implementation. No browser drag workflow is verified. Rendering still flattens the complete scene, so large-document projection needs a separate retained-work audit.

## Native Canvas Runtime Confirmed

The isolated registered-editor test `direct_drag_projects_without_editing_and_publishes_only_on_release` passed (one executed, 326 filtered), including both cancelled and committed gestures. `tests-drag-diagnostic.txt` records actual runtime DEBUG observations after pointer-down, movement, rendered preview and release. The test compares Canvas2dScene transform coordinates, verifies the complete document remains unchanged during preview, checks cancellation leaves it unchanged, and asserts one artifact publication on release with the expected transform. The fixture performs retained envelope admission and acknowledgement, not an unexecuted host reset effect. Transient revision receipts are allowed; pre-release artifact publication is forbidden. Temporary per-stage diagnostic logs have been removed; the final verification observation remains.

A preceding all-suite run hit the repository's 15-second execution budget without reporting an assertion failure. The isolated drag test completed in 0.13 seconds. A final all-suite rerun (`tests-regressions-final.txt`, session 6077) is active; browser behavior remains unverified because the hub build failed.

### Final Native Result

Session 6077 completed successfully: **327 tests run, 327 passed, zero skipped**, with assertions taking 2.541 seconds and the Nx task taking 10 seconds. Evidence: `🗑️generated/tests-regressions-final.txt`. This supersedes the prior pending/full-suite-failure statuses. `git diff --check -- ✏️s/🔌️plugins/🖍️draw` also passed. Nx labeled the task flaky after the earlier execution-budget timeout; no claim is made that the timeout itself has been diagnosed.

The user goal and ticket remain open. Remaining work includes multi-layer/group transforms, resize/rotate/snapping, direct canvas node handles, joining separate path layers, fuller text/image editing and a successful browser workflow audit. The isolated hub bootstrap remains blocked by its Rust build errors; no browser editor pass is claimed.

## Multi-Selection and Group Movement Implementation

Gesture payloads now retain the framework interaction snapshot. A direct drag on a selected layer, or on a descendant of a selected group, preserves the full selection. An unselected hit starts a replacement selection. A retained preparation cursor visits one document layer per turn, resolves selected ancestors once, validates visibility/locking and invertible parent transforms, and captures original transforms before preview. Missing targets or invalid selected parents reject before publication. Selected descendants of a selected ancestor do not receive a second transform.

Preview movement runs through the scene's group traversal and applies the same world displacement to selected roots. Release converts that displacement into each root's parent coordinates and emits one logical commit containing the transform mutations. Cancellation retains the previous document. The retained gesture budget is now 128 KiB; the existing query admission ceilings (256 selected IDs, 8192 ID bytes, depth 32) still apply. These architectural limits and retirement costs require the remaining large-work audit; this is not a claim of unrestricted document complexity.

Shared language-neutral ancestry fixtures exercise siblings, nested groups, descendant deduplication and empty selection. The TypeScript twin passes 23 tests/938 assertions; Three.js validates parent-space displacement and Immer independently validates movement of selected subtrees, including unchanged unselected leaves. Evidence: `ts-selection-move-final.txt`. The intentional preimplementation run (`ts-selection-move-red.txt`) failed on the missing selectionRoots export.

Native production check `check-selection-move.txt` passed in 18.7 seconds before the final budget/cleanup edits. New native tests cover actual selected-group-plus-leaf canvas preview, cancellation, persistent selection and exact undo/redo, as well as mixed-parent displacement and atomic refusal of locked/hidden/missing/singular targets. **These new tests are not yet verified**: `tests-selection-move.txt` stopped while compiling the shared pixels dependency because `🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🦀️.rs` was missing. No assertion executed. The previously verified 327-test suite predates multi-selection movement.

Changed files: canvas-pointer-down implementation and unit tests; editor retained payload/dispatch and integration tests; schema scene flattening; canvas renderer; translation Rust/TypeScript twins, ancestry fixture and TypeScript oracle. `git diff --check` passes. No browser claim is made; hub bootstrap remains unverified.

## Multi-Selection Native Verification Follow-Up

The shared pixels compositing module is now present. Older direct-drag unit fixtures were updated to run incremental movement preparation and expect the vector of selected roots. New mutation assertions use the framework's current `artifact_mutations` field. The native suite compiled and started 331 tests in `tests-selection-move-current.txt`. The group/leaf integration reached release, where its expected completion count was wrong: two transform mutations correctly produced one retained publication. The test now requires one publication. The subsequent undo/redo assertions remain required; the rerun is active as session 99625 (`tests-selection-move-history.txt`). The initial run also hit the 15-second suite execution budget after the failed assertion caused a destructor abort; no full-suite pass is claimed from it.

### Multi-Selection Native Suite Passed

`tests-selection-move-history.txt` completed successfully: **331 tests executed, 331 passed, zero skipped**. Assertions took 3.611 seconds; the complete Nx task took 2m19s. This verifies selected-group plus selected-child deduplication, unchanged preview snapshots, group/leaf world-space preview, cancellation, preserved framework selection, one publication for multiple transform mutations, and exact undo/redo. The native mixed-parent and invalid-target tests passed as well. This supersedes the previous pending status.

Browser delivery is still pending. A fresh Draw-only `describe materialize-dev` run is rebuilding the matching component and descriptor (`draw-describe-selection-move.txt`). The hub endpoint compile mismatch was fixed by declaring `Json<HubObservabilityV1>` as `admin_observability`'s response type, matching the model it returns; no response data or authentication behavior changed. The hub build is live as session 57930 (`hub-build-observability.txt`); its result has not been claimed. Changed dependency file: `🌎️hub/🏗️bootstrap/🦀️.rs`.

### Transform Handle Integration Audit — 2026-09-27

The shared Canvas2dGumballOverlay exists but is not connected to Draw. It only becomes visible for activeUtility === "transform"; Draw declares "transformMove" and supplies no gumball metadata. Its move handler immediately dispatches incremental artifact actions and its pointer-up handler only clears local refs. It does not supply a start/preview/commit/cancel transaction to Draw's retained DrawingSession. Simply emitting a gumball meta row would therefore fail the single-history-edit and cancellation requirements, and would not display under the current utility identifier.

The existing retained Draw gesture owner lives at artifacts/drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down, despite neighboring commands living under standards/1/subsets/any/editor. Its preview projection supports translation only. A complete handle implementation needs declared handle geometry plus an exact gesture owner and absolute transform preview, one atomic release commit, Escape/pointer-cancel rollback, keyboard-accessible handle actions, and native/React twins. It must reuse the existing selected-root/parent-inverse preparation without applying repeated deltas to already-updated state. The framework overlay's current FEM coordinate default must not enter Draw world-coordinate transforms.


### Exact Transform Representation

The later [affine transform checkpoint](↗️affine-transforms.md) removes the five-scalar representation blocker by retaining shear, signed scales and collapsed axes across schema, mutation, clone/digest and scene projection. The registered TS suite now passes 66 tests / 1,307 assertions. Native/build and handle interaction remain unverified. This is the foundation for the transactional resize/rotate work described above, not completion of that interaction.


### Shared Canvas Handles

[Transform handles](🎛️transform-handles.md) corrects the earlier selection-hook assumption: the current request-context hook supplies InteractionView. Nine painted handles now connect to retained pointer preparation, absolute matrix preview and release-only commits. The TS suite passes 75 tests / 1,373 assertions. Native registered-editor and browser checks are still unverified; large render work, Boolean/trace bounds and remaining handle accessibility/snapping are open.


## Stable Browser Movement

On port 6065, after selecting Orange Wedge, a drag from (420,360) to (440,375) committed Position X=7.532956685499059 and Y=5.649717514124291, consistent with the canvas zoom 2.655. The document-layer name stayed Orange Wedge. A single Command-Z restored X=0 and Y=0 in the inspector. AX updates arrived asynchronously after the pointer command rather than in the first immediate snapshot. Browser error/warning logs were empty before this journey. This is a running-app movement/undo confirmation, separate from the new node-drag tool still awaiting activation.
