# Ticket

## Todos
- [x] Define a reusable selection composition mechanism for `additive`, `subtractive`, and `intersect`.
- [x] Define how selection modes move into tool settings and stay app-agnostic.
- [x] Define integration/test/spec rollout across `Design`, `Kit`, and `Type`.
- [x] Implement shared selection composition helpers in `semio/js/sketchpad/shared.ts`.
- [x] Integrate shared selection composition into `Design` lasso/node/scene selection flows.
- [x] Integrate shared selection composition into `Kit` table row selection flows.
- [x] Integrate shared selection composition into `Type` connector selection flows and add intersect mode tool.
- [x] Update i18n entries and sketchpad specs docs for the canonical composition mechanism.
- [x] Extend existing `semio/js/sketchpad.test.ts` Type coverage for intersect mode presence checks.
- [x] Run targeted verification (`tsc`, unit tests, targeted Playwright Type test).
- [x] Add dedicated unit test coverage for selection composition functionality in existing semio test structure without creating new test files.

## Changes
- Added shared selection mechanism exports in `semio/js/sketchpad/shared.ts`:
  - `SelectionCompositionKind`
  - `resolveSelectionCompositionKind`
  - `toSelectionToolKind`
  - `isSelectionToolKind`
  - `applySelectionComposition`
- Refactored `semio/js/sketchpad/Design.tsx`:
  - Lasso composition now routes through shared selection composition helpers.
  - Node drag pending selection uses shared composition kinds.
  - Node drag stop applies composed selection via shared helper.
  - Scene model click selection uses shared composition resolver.
  - Selection modifier key transitions now support intersect and use centralized resolver.
- Refactored `semio/js/sketchpad/Kit.tsx`:
  - Table row selection composition uses shared helper for additive/subtractive/intersect.
  - Shift-only range behavior is preserved.
  - Selection modifier key transitions now support intersect and use centralized resolver.
- Refactored `semio/js/sketchpad/Type.tsx`:
  - Connector click selection uses shared composition helper.
  - Added `SelectionIntersectTool` and included it in `TypeAppTools`.
  - Type selection settings now include intersect mode toggle.
  - Selection modifier key transitions now support intersect and use centralized resolver.
- Updated i18n:
  - Added type intersect selection label in `semio/js/sketchpad/locales/en.json`.
  - Added full type selection mode labels (normal/additive/subtractive/intersect) in `semio/js/sketchpad/locales/de.json`.
- Updated specs/docs:
  - `semio/js/README.md` Sketchpad selection section with canonical composition semantics and modifier precedence.
  - `semio/js/sketchpad/README.md` Specs with shared composition contract and modifier resolution rules.
- Extended existing test file:
  - `semio/js/sketchpad.test.ts` Type test now checks intersect mode toggle presence when selection tool is visible.
- Extended existing unit test file:
  - `semio/js/semio.test.ts` now includes `Sketchpad Selection Composition` test coverage for replace/additive/subtractive/intersect behavior, dedupe/order semantics, and tool/modifier mode resolution.

## Log
- Reviewed existing open tickets and moved planning to `MOVE-SELECTION-MODES-TO-TOOL-SETTINGS` as requested.
- Replaced empty ticket with a concrete, cross-app implementation plan.
- Implemented shared selection composition in `shared.ts`.
- Migrated selection composition callsites in Design/Kit/Type.
- Added Type intersect mode tool + labels.
- Updated README specs for selection composition contract.
- Ran `npx tsc --noEmit` in `semio/js` (pass).
- Ran `npm run test:unit` in `semio/js` (11/11 pass).
- Ran `npm run test:e2e -- semio/js/sketchpad.test.ts --grep "Type" --workers=1` in `semio/js` (pass).
- Ran `npm run test:unit` in `semio/js` after unit test extension (12/12 pass).

## Plan (2026-02-12)

### Objective
Implement a selection mechanism that supports `additive`, `subtractive`, and `intersect` in a single reusable way and works in multiple apps (`Design`, `Kit`, `Type`) by putting mode state in tool settings instead of app-local logic.

### Core Mechanism
1. Canonical selection mode kind:
- `replace`
- `additive`
- `subtractive`
- `intersect`

2. Canonical shared operation:
- `applySelectionComposition(previousIds, incomingIds, modeKind) => nextIds`

3. Canonical semantics:
- `replace`: `incoming`
- `additive`: `previous ∪ incoming`
- `subtractive`: `previous - incoming`
- `intersect`: `previous ∩ incoming`

4. Deterministic ordering:
- Keep existing `previous` order first.
- For new additive entries, append by first-seen order from `incoming`.
- Remove duplicates at composition boundary.

### Tool Settings Refactor
1. Move selection mode source of truth into tool settings state:
- `tool.selection.modeKind`

2. Keep `select` tool active while mode changes:
- `select` is the tool.
- `additive/subtractive/intersect` are mode settings, not separate tools.

3. Keep temporary keyboard overrides centralized:
- `Shift => additive`
- `Alt/Option => subtractive`
- `Shift+Alt => intersect`
- Fallback to `tool.selection.modeKind`

### App Integration Contract
1. Shared layer responsibilities:
- Mode resolution.
- Selection composition math.
- ID dedupe + order normalization.

2. App responsibilities (`Design`, `Kit`, `Type`):
- Resolve selected IDs from click/pick/lasso/tree events.
- Provide current selected IDs.
- Apply composed IDs back to app state.

3. No app-specific composition math allowed after migration.

### Rollout Plan
1. Shared foundation
- Add `SelectionCompositionKind` and `applySelectionComposition` in shared sketchpad selection utilities.
- Add `resolveSelectionModeKind(toolSettingsModeKind, keyboardState)`.

2. Migrate `Design`
- Replace current mode handling with shared resolver + shared composition.
- Keep existing event handlers, swap only the composition step.

3. Migrate `Kit`
- Remove per-entity custom composition branches and route through shared helper.
- Preserve entity-specific ID extraction hooks.

4. Migrate `Type`
- Apply same adapter pattern and remove local composition logic.

5. Unify toolbar/tool-settings UI
- Surface mode toggle controls in tool settings panel for all three apps.
- Keep labels and i18n keys consistent across apps.

### Testing Plan (Existing Test File Only)
Extend `semio/js/sketchpad.test.ts` with a single selection-composition unit region containing multiple tests:
- Pure composition tests for `replace/additive/subtractive/intersect`.
- Empty incoming behavior tests.
- Duplicate ID normalization tests.
- Stable ordering tests.
- Mode resolution tests with keyboard overrides.
- Cross-app parity tests using the same `previous/incoming/mode` fixtures.

### Specs/Documentation Plan
Update existing specs sections only (no new files):
- Root `README.md` under `# Specs`: canonical selection composition contract.
- `semio/js/README.md` under Sketchpad specs: tool settings as source of truth for selection mode.
- `semio/js/sketchpad/README.md`: shared composition + app adapter responsibilities.
- Touched source files: file-level `Specs` headers/docstrings with mode semantics + ordering guarantees.

## Summary

Added selection-composition functionality tests inside existing semio test structure by extending semio/js/semio.test.ts (no new test files). Verified with npm run test:unit (12/12).
