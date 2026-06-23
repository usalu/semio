# Ticket

## Todos
- [x] Open ticket `FIX-DESIGN-APP-SELECTION` and write `plan.md`.
- [x] Diagnose scene model selection regression (reopened again).
- [x] Implement and validate scene model selection fix (reopened again).
- [x] Update `README.md` and `AGENTS.md` docs for finalized reopened-again behavior.
- [x] Finalize summary and close ticket with touched files.

## Changes
- Updated `js/compose/sketchpad/Design.tsx` so panning pointer-guard activation is limited to non-primary mouse buttons, keeping primary click/Shift/lasso selection flow available to React Flow selection sync.
- Updated `js/compose/sketchpad/Design.tsx` `onNodesChangeReactFlow` to apply React Flow node-change deltas (including selection changes), not only position changes.
- Updated `js/compose/sketchpad/Design.tsx` diagram selection extraction to use node payload piece guids instead of truncating node ids.
- Updated `js/compose/sketchpad/Design.tsx` scene multi-select guid resolution to traverse selected object ancestry and read `userData.pieceId`/`userData.id`.
- Updated `js/compose/sketchpad/Design.tsx` scene piece render wrappers to stamp `userData.id` and `userData.pieceId` on both transform and mesh wrapper groups so loaded model meshes always propagate piece identity.
- Updated `README.md` under Bundles/Sketchpad selection docs with diagram + scene selection synchronization details.
- Updated `AGENTS.md` SRS/UI + Codebase docs with Design scene identity resolution and wrapper metadata constraints.

## Log
- Reopened with prompt: "selection is stil not working on the model in the scene window".
- Reopened with prompt: "i still cant select either in the diagram nor in the scene in the design app".
- Prompt: "fix the selection in the design app".
- Opened ticket via `repo ticket open` and wrote `plan.md`.
- Traced Design diagram selection path (`onSelectionChange`) and identified primary-button pointer-down panning guard as a selection blocker.
- Implemented fix in `Design.tsx` by scoping pointer-down panning guard to middle/right mouse buttons.
- Validation run (failed args): `npx nx test @compose/js --skipNxCache --runInBand --testPathPattern=sketchpad.test.ts --testNamePattern="Design"`.
- Validation run (failed filter): `npx nx test @compose/js --skipNxCache -- sketchpad.test.ts -t "Design"`.
- Validation run (pass): `npx nx test @compose/js --skipNxCache`.
- Reproduced reopened selection breakpoints in both Design diagram and scene selection pathways.
- Identified diagram piece guid extraction bug (`split("-").pop()`) and missing React Flow selection delta application in node changes.
- Identified scene multi-select guid resolution mismatch (`pieceId` lookup only) causing selection clear churn.
- Implemented Design diagram + scene selection fixes and reran tests.
- Validation run (pass): `npx nx test @compose/js --skipNxCache`.
- Reproduced model-selection miss in scene window and traced object ancestry ids from loaded meshes.
- Identified missing `userData.pieceId` metadata on scene transform/wrapper groups for non-design model meshes.
- Added explicit piece identity metadata on scene wrapper ancestors and reran tests.
- Validation run (pass): `npx nx test @compose/js --skipNxCache`.

## Summary

Fixed scene model selection by stamping piece identity metadata on transform and mesh wrapper ancestors so loaded model picks resolve to piece guids; selection sync now works across diagram and scene.
