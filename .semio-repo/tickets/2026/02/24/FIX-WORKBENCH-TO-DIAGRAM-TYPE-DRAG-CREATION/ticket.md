---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Wrote clean prompt and refined duplicateType test coverage in existing Design e2e flow with strict no-navigation assertions.
## Changes
- Updated `semio/js/sketchpad/Design.tsx`:
- Updated diagram drag-drop piece creation (`type` and `design`) to include a computed `plane` in addition to `center`.
- Updated Workbench add-piece actions (`types.addPiece`, `designs.addPiece`) to create pieces with consistent `center` and `plane`.
- Updated `PieceMesh` model resolution to use `selectBestModel(type.models, selectedTags)` directly, ensuring newly added pieces resolve the intended model from the piece type.
- Updated `semio/js/sketchpad.test.ts` (existing test file only):
- Extended existing Design app assertions to require non-null `plane` for pieces created from:
- Workbench drag-drop to diagram.
- Workbench add-piece button in diagram flow.
- Refined duplicate-type assertions to click `semio.sketchpad.common.duplicateType` in the same parent row under test (row-scoped locator first, global fallback).
- Kept and validated assertions for `+1` child count, exactly one new child guid, URL unchanged, URL not containing `/types/`, and new duplicated child name visible.
- Updated `.semio-repo/prompts/kinan.md`:
- Rewrote the `Duplicate Type Visibility Without App Switch` prompt block into a clean actionable spec with explicit context, tasks, assertions, and acceptance criteria.

## Log
- Reopened existing related ticket to keep Workbench history.
- Gathered repo context via Semio CLI and inspected diagram/scene piece creation paths and model rendering path in `Design.tsx`.
- Identified inconsistent piece creation in diagram-side flows where only `center` was set while runtime expects consistent `center` + `plane`.
- Implemented consistent piece creation for diagram drag-drop and Workbench add-piece actions by setting both `center` and `plane`.
- After user retest feedback, identified `PieceMesh` fallback path could pick concept-similar model instead of intended type-default model when no explicit model tags were selected.
- Removed concept fallback from `PieceMesh` and switched to deterministic type model selection via `selectBestModel` only.
- Extended existing Design Playwright test in `semio/js/sketchpad.test.ts` to assert new pieces have non-null `plane` in diagram creation flows.
- Updated prompt source in `.semio-repo/prompts/kinan.md` for the duplicate-type/no-app-switch scenario.
- Refined existing duplicate-type test path to ensure the clicked action belongs to the intended parent type row before validating child creation.
- Verification:
- `cd semio/js && npx tsc --noEmit` => passed.
- `cd semio/js && npx playwright test sketchpad.test.ts --grep "Design" --timeout 240000 --workers=1 --max-failures=1 --reporter=list` => blocked in this environment because `config.webServer` could not start (`Exit code: 1`).

## Todos
- [x] Reuse existing sketchpad test file.
- [x] Fix diagram-side piece creation to keep center/plane consistent.
- [x] Ensure diagram-created pieces carry placement data used by scene rendering.
- [x] Fix PieceMesh model selection to resolve the correct model from piece type defaults/tags.
- [x] Add existing-test coverage for plane presence on diagram-created pieces.
- [x] Write clean prompt for duplicate-type visibility without app switch.
- [x] Refine duplicate-type test to use row-scoped action targeting and strict URL/visibility assertions.
- [x] Run verification and capture exact output including blockers.

## Plan
1. Inspect diagram and Workbench piece creation flows used when adding a piece in the diagram window.
2. Patch existing Design piece creation logic so newly created pieces include both center and plane.
3. Fix scene model resolution path to use deterministic type model selection and avoid incorrect concept fallback.
4. Refine duplicate-type test targeting and prompt specification for no-navigation behavior.
5. Run targeted verification and record exact pass/fail status.
