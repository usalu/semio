# Ticket

## Todos
- [x] Register five placeholder sub-tools in Kit Selection toolbar dropdown.
- [x] Keep deterministic toolbar ordering and cleanup removals for all new sub-tool sections.
- [x] Update developer documentation in `README.md` and `AGENTS.md`.

## Changes
- Added `kitToolbarSelectionSubTools` in `js/semio/sketchpad/Kit.tsx` with five Selection sub-tool entries: `select`, `hand`, `additive`, `subtractive`, `intersect`.
- Replaced single Selection toolbar registration with iterative registrations across the five sub-tool section ids.
- Updated toolbar cleanup to remove all five Selection sub-tool section ids.
- Documented Kit Selection dropdown placeholder sub-tools in `README.md` (`# 📦 Bundles`, Sketchpad toolbar tooltree).
- Documented requirement and implementation references in `AGENTS.md` (`# Software Requirements Specification` and `# Codebase`).

## Log
- Opened ticket `KIT-SELECTION-PLACEHOLDER-SUBTOOLS`.
- Created `plan.md` with implementation steps.
- Implemented Kit Selection dropdown placeholder sub-tool registration.
- Updated root developer docs.
- Verified patch diff for `Kit.tsx`, `README.md`, and `AGENTS.md`.

## Summary

Planning ticket updated with cross-app selection composition implementation plan.
## Plan (2026-02-12)

### Goal
Implement a reusable selection composition mechanism that supports `additive`, `subtractive`, and `intersect` modes and works consistently across Sketchpad apps (`Design`, `Kit`, `Type`) while keeping app-specific hit resolution separate from selection math.

### Scope
- Selection behavior + state contracts in shared app/plugin layer.
- Toolbar mode wiring for all apps.
- Event pipeline integration (click, marquee/lasso, keyboard modifiers).
- Shared tests in existing `semio/js/sketchpad.test.ts` only.
- Documentation/spec updates in existing READMEs and relevant file-level specs.

### Architecture Decisions
- Canonical selection mode enum-like union: `replace | additive | subtractive | intersect`.
- Canonical operation signature:
  - `applySelectionComposition(previousIds: string[], incomingIds: string[], mode: SelectionCompositionKind): string[]`
- Canonical mode resolution:
  - Explicit toolbar mode is base mode.
  - Keyboard modifiers can temporarily override base mode (`Shift => additive`, `Alt/Option => subtractive`, `Shift+Alt => intersect`) behind a single shared resolver.
- App adapters provide only:
  - `resolveSelectionIdsFromEvent(eventContext): string[]`
  - `resolveSelectionIdsFromRegion(regionContext): string[]`
  - `getCurrentSelectedIds(): string[]`
  - `setCurrentSelectedIds(ids: string[]): void`

### Implementation Phases
1. Shared Selection Composition Core
- Add shared pure helpers in Sketchpad shared logic (same existing file/module where selection helpers live):
  - `toUniqueIds(ids)`
  - `composeReplace(previous, incoming)`
  - `composeAdditive(previous, incoming)`
  - `composeSubtractive(previous, incoming)`
  - `composeIntersect(previous, incoming)`
  - `applySelectionComposition(previous, incoming, mode)`
- Keep stable deterministic ordering using current selection order first, then first-seen incoming order for additive paths.

2. Shared Mode Resolution
- Implement `resolveSelectionCompositionMode(baseMode, keyboardState)` in shared selection pipeline.
- Ensure identical mode behavior for all selection entry points (single click, node click, scene pick, tree select, marquee/lasso).

3. App Integration (Design, Kit, Type)
- Replace app-local ad-hoc selection merge/remove logic with the shared composition core.
- Keep app-specific ID resolution local; only pass ID arrays into shared composition.
- Wire toolbar sub-tools (`select`, `additive`, `subtractive`, `intersect`) to update base composition mode in each app state.
- Preserve `hand` behavior as non-selection mode and ensure it bypasses composition logic.

4. Interaction Rules
- `replace`: result = incoming.
- `additive`: result = union(previous, incoming).
- `subtractive`: result = previous - incoming.
- `intersect`: result = intersection(previous, incoming).
- Empty incoming set behavior:
  - `replace` => clears selection.
  - `additive` => no change.
  - `subtractive` => no change.
  - `intersect` => clears selection.

5. Tests (Existing File Only)
- Extend `semio/js/sketchpad.test.ts` with a dedicated selection composition region that covers one unit (selection composition) using multiple tests:
  - Pure composition function tests for all four modes.
  - Mode override tests (toolbar mode + modifier combinations).
  - Cross-app parity tests asserting Design/Kit/Type apply identical composition outputs for equivalent previous/incoming ID sets.
  - Regression tests for duplicate IDs, ordering stability, and empty incoming behavior.

6. Specs/Docs Update
- Update existing relevant `README.md` specs sections:
  - Root and `semio/js/README.md` selection mechanism spec.
  - `semio/js/sketchpad/README.md` app/plugin integration spec.
- Update file-level specs/docstrings in touched selection-related files to define:
  - Canonical mode semantics.
  - Order guarantees.
  - Modifier precedence.

### Delivery Sequence
1. Refactor shared composition core + tests.
2. Migrate Kit app to core and verify tests.
3. Migrate Design app to core and verify tests.
4. Migrate Type app to core and verify tests.
5. Final docs/spec synchronization and full targeted test run.

### Risks and Mitigations
- Risk: Existing app-specific edge cases diverge silently.
  - Mitigation: Add parity tests against shared expected outputs before each app migration.
- Risk: Selection ordering regressions affect UI detail panes.
  - Mitigation: Explicit ordering tests and deterministic ordering helper.
- Risk: Keyboard modifiers behave differently across platforms.
  - Mitigation: Centralized resolver + tests for normalized modifier state.
