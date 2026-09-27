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
