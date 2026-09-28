# Direct Path Manipulation

## Current Implementation

The path editor has a schema-first `position` operation with index, anchor/control selector and a two-dimensional target. Rust and TypeScript apply both coordinates in one source-preserving edit, moving incoming and outgoing Bézier controls with an anchor. Coordinate inspector edits keep the same behavior. The source is unchanged on invalid handles or nonfinite output. This provides one geometry mutation for a complete drag instead of separate X/Y history entries.

The neutral position fixtures cover an anchor between cubic and quadratic segments, independent control positioning and a missing-control rejection. The independent oracle uses Immer with Three.js vector translation. Native bridge text/binary roundtrip and registered one-edit undo/redo coverage are authored, but not executed successfully yet.

The pure `drag_path_point` / `dragPathPoint` twins map a world-space pointer delta through the inverse ancestor linear transform and add it to the original local point. They preserve the offset between pointer-down and the selected anchor/handle, avoiding a jump to the pointer center. They reject singular transforms and missing handles; Shift constrains the world-space axis. Five neutral fixtures cover shear, reflection, a rotated parent with an axis constraint, singular parents and unavailable handles. Three.js full-matrix inversion independently checks the expected output. Inverting only the linear part avoids unnecessary translation overflow for delta mapping.

## Completed Verification

- `tests-path-position-red.txt`, process30576: failed before implementation.
- `tests-path-position.txt`, process39566: 80 tests /1,421 assertions passed.
- `tests-node-drag-red.txt`, process72575: failed on the missing drag helper before implementation.
- `tests-node-drag.txt`, process94521: **81 tests /1,438 assertions passed**, plus44 independent Ajv field-patch cases.

This is the geometry/command foundation. No canvas node overlay, node target selection, retained node preview or release gesture has been implemented. It does not establish an end-user node-drag workflow.

## Browser Finding: Painted-Shape Picking

The current preview at127.0.0.1:6064 loads the Demo and paints the canvas. Clicking the orange region at viewport(420,360) selected **Red Frame**, as shown by the inspector's Name value and red fill. Captured browser error/warning logs were empty. Screenshot: `🗑️generated/path-picking-reproduction.png`.

The current `TracePointerWork::PathBounds` only accumulates geometric bounds and optional control-point proximity. `consider_trace_candidate` accepts any pointer inside that bounding rectangle. A concave path can therefore win in empty space inside its rectangle and hide the intended underlying path. The existing TypeScript selection oracle explicitly compares bounding boxes too, so it cannot detect this bug.

Precise painted-shape hit-testing is now a prerequisite to direct node dragging. Keep bounds as broad-phase rejection. Add fill winding/even-odd and stroke-distance tests for actual line/quadratic/cubic/arc geometry, implicit fill closure and multiple contours, preserving nested transforms, frontmost ordering and control-point selection. The retained picker must advance curve work in bounded slices; do not call a whole-path flattening loop synchronously from a pointer request. Use a real Canvas Path2D or independent geometry test oracle. Existing bbox-only fixture expectations need to be reassessed against painted geometry rather than preserved as compatibility behavior.

For node gestures, reuse the retained pointer query to identify a node on selected visible/unlocked paths. Retain the original point, ancestor transform and pointer-down position; previews should carry a small point-edit descriptor. Build the final replacement segments incrementally before one UpdatePathGeometry publication. Escape/cancel must retire retained work and emit no artifact mutation. Avoid cloning or transforming an entire large path on every pointer sample merely to paint preview.

## Native Failures And Current Runs

Processes29811(Draw) and23844(native renderer) both terminated with exit1. The compiler failed in shared dependencies before running the requested tests: stdio-contract used unresolved `dsl::ToValue`; infinite-canvas lacked its viewport dependency in the compile graph. Current source already uses `kernel::ToValue`, and current infinite Cargo.toml includes the first-party viewport dependency. No speculative dependency edits were made. Compiler fingerprint diagnostics were inspected to identify the exact stale location.

Fresh registered native runs, after those source corrections and the node math changes:

- **69843**: Draw full native library suite; `tests-native-node-position.txt`.
- **45203**: native renderer `scenes::canvas2d_tests`; `tests-native-pointer-modifiers-current.txt`.
- **43904**: existing Draw describe/materialize build remains live; do not start a duplicate.

The browser still runs the older component without the new shear/text/handle controls. Tab4 is marked for handoff. The ticket and full Draw goal remain active.


## Bounded Position Preview

The Rust and TypeScript position edit now share a constant-size patch operation. It copies the edited segment and only an affected outgoing quadratic/cubic tangent, rejects missing handles and nonfinite results, and leaves the input owned by the document. Numeric path edits use the same patch logic. Existing language-neutral position fixtures and an Immer splice oracle validate equivalent preview/commit geometry. The red test run 66263 failed on the missing export; green run 40562 passed 109 tests, 18,041 assertions and 44 Ajv cases. Direct canvas gesture integration remains in progress.


## Canvas Integration

`editNodes` is registered as an English/German canvas utility. The retained point query identifies anchors/controls only on selected unlocked paths; node movement keeps a constant-size local geometry preview, preserves press offset through full affine ancestors, and uses Shift for world-axis constraint. The scene projects square anchors, diamond controls, and tangent stems at screen-stable sizes. Pointer release produces one path geometry mutation; cancellation clears the preview. Integration test covers a scaled cubic control, preview isolation, cancelled release, selection retention, and undo/redo.

The shared point-hit fixture has six cases and uses Three.js distances after matrix transforms as oracle. TypeScript run 3337 passed 110 tests, 18,063 assertions and 44 Ajv cases. Native run 24892 passed all 375 tests, including the two new pure geometry tests, before the canvas gesture integration was compiled. Native run 22911 now validates the added gesture integration. Runtime browser proof is still pending. Full scene projection and final full-path mutation assembly remain proportional to document/path size and need a later bounded-work pass.


Native integration run 22911 finished successfully: 376/376 tests passed, including the new retained node drag preview/cancel/undo/redo test. The subsequent small refinements also let Ctrl/Meta node-tool releases use existing point-selection semantics and reuse the new point-hit helper for normal control-hover picking. Build 79374 is producing the updated component/description for browser verification.


Next continuation: poll live native final-refinement run 21749 (`tests-native-node-picking-current.txt`), browser build 79374 (`build-draw-node-gestures.txt`), and renderer run 44523 (`tests-native-canvas-tree-current.txt`). Do not restart live runs. Build 79374 has completed component-dev and is still building the describe host. After it succeeds, run the existing Nx `@semio-tech/framework-os-dev:activate-draw-react-dev` target before reloading browser tab 4 on port 6065. Preview session 76268 is live with HMR disabled. The previous component has now demonstrated precise picking and path move/resize/rotation/undo in that browser; new Edit Nodes tool remains the next runtime test.


Run 21749 terminated with a shared plugin compile error before Draw tests: `EditableTreeNode` derived Clone although its owned UiValue arguments intentionally do not implement Clone. The editable tree builder consumes those arguments. Removed only the invalid Clone derive from that descriptor. Earlier 376/376 native success still stands; latest-source validation must be rerun.


Latest handoff: 21749 is terminal failed (shared owned-argument derive error corrected). Replacement Draw validation is live as 59565, log `tests-native-node-owned-arguments.txt`. Renderer validation is live as 44523, log `tests-native-canvas-tree-current.txt`. Component/description/materialization run 79374 remains live; do not restart it. Preview 76268 and browser tab 4 on 6065 remain available. The Demo has been restored by one undo after each verified move, resize and rotation. Screenshots retain the successful selection and rotated state.


Continuation audit: the previous goal turn made concrete progress (bounded point editing, retained node gestures, passing native integration, and verified browser picking/move/resize/rotate/undo). Current build 79374 is confirmed terminal successful; explicit activation is now running. Native validation 59565 and renderer validation 44523 are confirmed live. No blocked condition applies.


Activation 70989 completed successfully in 4.9s. Reloaded stable browser tab 4 now exposes Edit Nodes, and selecting that utility checks its state. The runtime anchor interaction is being exercised next.

## Browser verification, 2026-09-27

Activated the successful node-tool component explicitly after materialization. In the stable React preview on port 6065, selected Orange Wedge and enabled Edit Nodes. Dragged its first anchor from screen (377, 632) to (397, 612). The contour visibly changed, Anchor 1 changed from (1.25, 196.933) to (8.782956685499059, 189.40004331450095), and layer position remained (0, 0). One Command-Z restored the original contour and anchor coordinates. Captured 🗑️generated/node-drag-verified.png before undo. No error logs appeared; one startup warning reports 59 unrelated staged plugin modules absent from this Draw-only activation receipt. The inspector projection settles asynchronously after gestures; the first immediate accessibility observation can contain the previous values.

This is progress toward the active goal, not feature-complete acceptance. Multiple-node selection, node keyboard editing, handle constraints, native vector rendering, and the remaining acceptance matrix still require implementation or verification.

## Bézier browser journey

Expanded Orange Wedge's Anchor 2 and invoked Convert to Curve. Two control handles appeared on the formerly straight segment. Dragged the first handle from screen (408,600) to (438,600). Its X coordinate changed from 12.916666666666666 to 24.21610169491526; its Y, the second handle and anchor coordinates stayed unchanged. The contour visibly curved. One Command-Z restored the first handle to its original coordinates. Captured 🗑️generated/bezier-control-drag-verified.png. The browser console error list was empty.

Usability gap found: node_row currently attaches all six cubic coordinates directly to one inline tree row. At the normal inspector width this produces six unreadable narrow fields. Move each handle pair into its own labeled child row, retaining the anchor pair on the parent and preserving field ids/actions. The disclosure button also lacks an accessible label in the current shared tree renderer. These are pending fixes, not accepted behavior.

A second Command-Z removed the conversion itself: the first and second handle fields disappeared and Convert to Curve returned. This confirms the conversion and the handle drag are separate undoable edits. Browser tab 4 is preserved with Edit Nodes active, Orange Wedge selected, its original straight contour restored, and Anchor 2 expanded in the scrolled inspector.

## Readable coordinate row implementation

Previous goal turn classification: progress. It implemented typed stroke values and produced new browser evidence for anchor/control dragging and separate undo steps. Both unfinished validation handles were confirmed live again at this turn's start.

The node inspector now keeps the anchor's X/Y inputs on the parent row and places each handle's X/Y inputs in a labeled child row. Existing input ids, coordinate actions and locked-state behavior are preserved. A neutral move/quadratic/cubic/close fixture defines the expected row-to-input mapping; the actual projected tree is compared to that fixture for English/German and editable/locked states. The browser's six unreadable inline fields is the observed failing behavior that prompted this change. Native validation and rebuilt browser layout verification remain pending.

The same layout repair is applied to fill coordinates and gradient stops: each gradient coordinate is a labeled child row, and stop opacity/position each get a separate row below the color control. These branches default open so the values remain discoverable. A second neutral fixture tests linear and radial gradient row mappings and stop mappings in both locales and lock states. Both features use the existing tree schema and preserve command ids/arguments.
