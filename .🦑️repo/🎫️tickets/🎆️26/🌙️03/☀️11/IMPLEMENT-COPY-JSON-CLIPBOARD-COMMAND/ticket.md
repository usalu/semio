---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Adjusted clipboard behavior to copy active kit JSON by default, removed design payload override from diagram Cmd/Ctrl+C, and updated existing clipboard e2e assertions to validate kit payload shape. TSC passes; focused Playwright remains blocked by existing @semio-tech/semio-assets named-export runtime mismatch.

## Changes

- Reopened ticket to align clipboard behavior with requirement: copy kit JSON.
- Updated `compose/js/sketchpad/Sketchpad.tsx`:
- Changed `compose.sketchpad.copyJsonToClipboard` default serialization from `dumpState()` to active kit snapshot resolution from current navigation (`/kits/:guid`), with first-kit fallback when no active kit path exists.
- Updated `compose/js/sketchpad/Design.tsx`:
- Removed design payload override for diagram `Cmd/Ctrl+C`; diagram shortcut now delegates to `copyJsonToClipboard` without payload so it copies kit JSON consistently.
- Updated `compose/js/sketchpad.test.ts`:
- Refactored existing `Copy Json To Clipboard Command` test to assert clipboard content is kit JSON (`guid`, `name`, `types`, `designs`) and not sketchpad root state.
- Updated `compose/js/sketchpad/Sketchpad.tsx`:
- Added `compose.sketchpad.copyJsonToClipboard` execution handling in `executeCommand`.
- Added clipboard fallback copy mechanism when `navigator.clipboard.writeText` is unavailable.
- Extended `copyJsonToClipboard(origin, payload?)` in `useSketchpadCommands` so callers can copy custom JSON payloads.
- Added a global hotkey binding using `useHotkeys("compose.sketchpad.navbar.copyJsonToClipboard", ...)`.
- Registered `compose.sketchpad.copyJsonToClipboard` in exported `commands`.
- Updated `compose/js/sketchpad/locales/en.json`:
- Added `compose.sketchpad.navbar.copyJsonToClipboard` label + hotkey (`Ctrl+Shift+J`).
- Updated `compose/js/sketchpad/locales/de.json`:
- Added German translation for `compose.sketchpad.navbar.copyJsonToClipboard` label + hotkey.
- Updated `compose/js/sketchpad.test.ts`:
- Extended the existing test file with `Copy Json To Clipboard Command` e2e test.
- Test stubs `navigator.clipboard.writeText`, triggers hotkey, and asserts copied JSON contains `sketchpad` and `kits`.
- Reopened ticket and updated `compose/js/sketchpad/Design.tsx`:
- Added diagram-local keyboard handling for `Cmd/Ctrl+C` that executes `copyJsonToClipboard` only when event target is inside diagram and not editable fields.
- Added diagram focus on pointer down and made diagram wrapper focusable (`tabIndex=0`) so keyboard shortcut is scoped to diagram interaction.
- Added diagram payload serialization so copy produces:
- full design JSON when no diagram element is selected.
- selected-only design JSON (`pieces`, `connections`) when diagram selection exists.
- Updated `compose/js/sketchpad.test.ts` clipboard test:
- Test now initializes Design app, focuses diagram, triggers `Meta+C`, and falls back to synthetic `meta+c` dispatch on the diagram element before asserting clipboard JSON payload.
- Test now asserts no-selection copy returns Design JSON shape (`pieces`, `connections`) and not full sketchpad state.
- Test now sets one selected piece, triggers copy again, and asserts only that selected piece is copied.

## Log

- Reopened existing ticket `2026/03/11/IMPLEMENT-COPY-JSON-CLIPBOARD-COMMAND` for follow-up behavior correction.
- Audited all clipboard-related tickets via `./repo/cli/cli tree "copy to clipboard"` and `./repo/cli/cli tree clipboard`; confirmed single relevant ticket.
- Reworked copy command default payload to active kit snapshot.
- Removed diagram-specific design payload serialization override.
- Updated clipboard e2e assertions from design payload shape to kit payload shape.
- Validation:
- `cd compose/js && npx tsc --noEmit` passed.
- `cd compose/js && npm run test:e2e -- --grep "Copy Json To Clipboard Command"` failed before test execution with existing `@semio-tech/semio-assets` named export interop error (`CodeIcon`).
- Gathered repo + ticket context via `./repo/cli/cli tree sketchpad`.
- Opened ticket under goal `SKETCHPAD-IMPROVEMENTS`.
- Implemented command + hotkey + locales + test extension.
- Reopened ticket for diagram-scoped shortcut behavior.
- Implemented Design diagram `Cmd+C` shortcut wiring.
- Updated clipboard e2e test to validate diagram-triggered copy behavior.
- Added payload-aware clipboard command plumbing to reuse clipboard fallback logic while enabling diagram-specific JSON copy.
- Added diagram selection-aware copy payload generation in Design app.
- Validation:
- Locale JSON parse check passed.
- `cd compose/js && npx tsc --noEmit` passed.
- Focused e2e run failed before test execution due to pre-existing import error:
- `SyntaxError: Named export 'CodeIcon' not found` from `@semio-tech/semio-assets` CommonJS interop in Playwright runtime.

## Todos

- Re-run `npm run test:e2e -- --grep "Copy Json To Clipboard Command"` after resolving the pre-existing `@semio-tech/semio-assets` named export runtime mismatch.
- Resolve the existing `@semio-tech/semio-assets` ESM/CJS named export mismatch in the Playwright test runtime.
- Re-run `npm run test:e2e -- --grep "Copy Json To Clipboard Command"` after the import issue is fixed.

## Plan

- Make `compose.sketchpad.copyJsonToClipboard` default to active kit JSON.
- Remove diagram payload override so `Cmd/Ctrl+C` copies kit JSON.
- Update existing clipboard test to assert kit JSON fields.
- Run `tsc` and focused e2e command; document results.
- Add payload support to `copyJsonToClipboard` command path.
- Serialize diagram copy payload from Design app based on current selection.
- Extend existing clipboard e2e in `sketchpad.test.ts` to cover no-selection and selected-only scenarios.
- Run `tsc` and focused e2e command; document blockers.
