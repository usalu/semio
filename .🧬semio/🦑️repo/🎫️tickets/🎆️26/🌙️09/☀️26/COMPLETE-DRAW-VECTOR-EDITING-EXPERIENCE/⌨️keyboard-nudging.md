# Canvas Keyboard Nudging

## Implemented Contract

Arrow keys move selected layers or selected path points by one document unit; Shift plus an arrow moves by ten. Edit Nodes uses the framework-owned point domain. Direct, rectangle and lasso selection plus Move use the layer domain. Empty point selection and drawing tools do not move entire layers accidentally.

Eight argument-free typed input actions share one planner. They are declared in the manifest, action bridge, retained command catalog, publication authority fixture and binary/text command roundtrip list. Labels are authored in English and German. No new executable task or runtime dependency was introduced.

The planner traverses group ancestry and converts document displacement through the complete affine basis. Selected ancestors own descendant layer movement once. Node movement uses the existing atomic point-translation operation, preserving adjacent handles and rebinding selected references to the resulting geometry. All mutations are prepared before one commit. Locked, hidden, missing, singular and stale targets fail without mutating the source. A local limit bounds the new synchronous planner to 4,096 layers plus edited segments and 4,096 selected references; large-document resumable movement remains outstanding.

## Evidence

- Added neutral world-displacement fixtures before implementation. They cover identity, rotated/scaled, reflected/sheared, large translated origins and singular matrices. Rust and TypeScript share these cases; TypeScript checks the output using independent Three.js inverse matrices and vector arithmetic.
- Added an argument-free JSON schema and eight neutral shortcut rows. Ajv checks payload shape. Source contract checks confirm keybindings and input audience. Native tests cover typed decoding, text/binary roundtrip, retained publication and one-step undo/redo for all eight actions.
- The initial world-nudge test failed for the missing export. An initial shortcut test also exposed a fixture reader pointing at the subset module instead of the editor module; the reader was corrected before the final run.
- `bun nx run @semio-tech/draw-js:test --excludeTaskDependencies` passes **120 tests, 18,279 assertions**, plus the 44-case field-patch oracle and publication authority checks for 36 Draw commands. Final log: `🗑️generated/tests-keyboard-nudge-current.txt`. Session **57481** completed successfully.
- Native tests for ancestor ownership, atomic rejection, tool isolation, every shortcut/history and repeated selected-point rebinding are authored. They are **not yet claimed passing**.
- Existing native job **71498** and component job **93519** are still live. Both started before this work; inspect their coverage after completion and run fresh native/build validation if needed. Do not duplicate or restart them while live.
- No new browser behavior is claimed. Preview **76268**, browser tab **5** at `http://127.0.0.1:6065/`, still uses the older component. It was retained for continuation.

## Remaining Verification

1. Resolve native compilation/test failures and establish coverage of point selection and these nudge commands.
2. Finish component generation, explicitly activate without Nx cache, then reload the stable preview.
3. In the browser, click a path point, verify its filled selection marker, exercise arrows and Shift-arrows repeatedly, then undo each logical edit. Capture console diagnostics and a screenshot.
4. Verify layer/group movement, widget keyboard ownership, locked/hidden handling and cancellation/undo under concurrent document changes.
5. Add node multi-selection, combined dragging, deletion, and resumable large-document behavior. These shortcuts do not complete the full editor acceptance scope.

## Changed Sources

- Path editing Rust/TypeScript world-displacement helper, shared fixtures, native and third-party TypeScript tests.
- Shared nudge command planner and generated typed direction modules, input schema, shortcut fixture, native unit tests.
- Drawing command module registry, editor routing/manifest/publication contracts and native command/history tests.
- Publication authority fixture and shortcut contract test.
- This note and the acceptance ledger.

## Continued Native Verification

The earlier native run 71498 is terminal with an unresolved `blake3` test dependency. It started before the manifest addition; a stale dependency graph is the current explanation to verify. Fresh run 78443 now owns current native validation. Combined node gestures and pick membership have been added since this note; see [Multiple Point Selection and Combined Dragging](🖱️multiple-point-selection.md). Latest TypeScript total: 121 tests / 18,318 assertions.
