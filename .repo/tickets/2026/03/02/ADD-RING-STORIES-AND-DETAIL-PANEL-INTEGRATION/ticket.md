---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Ring replaces Slider for connector t property in the Type detail panel. ConnectorsListSection shows all connectors as orbs on a shared Ring. ConnectorSection shows a single-orb Ring for the selected connector. Storybook stories created for both Orb and Ring.

## Changes

- Created `semio/js/.storybook/stories/elements/input/Ring.stories.tsx` with Default, Base, Window, Panel, Overlay, Temporary stories.
- Created `semio/js/.storybook/stories/elements/input/Orb.stories.tsx` with Default, Selected, Hovered, Disabled, Base, Window, Panel, Overlay, Temporary stories.
- Added `Ring` import to `semio/js/sketchpad/Type.tsx`.
- `ConnectorsListSectionForm`: Added Ring at top showing all connector t values as draggable orbs with selection/hover. Removed per-connector Slider for t.
- `ConnectorSectionForm`: Replaced Slider for t with Ring showing single selected-orb for the connector.

## Log

- Phase 1: Created Orb/Ring stories and initial Ring integration in ConnectorsListSection.
- Phase 2: Replaced Slider t with Ring in both ConnectorsListSectionForm (removed per-connector Slider, Ring at top handles all) and ConnectorSectionForm (single-orb Ring).
- Build passes. All 14 vitest tests green.

## Todos

- [x] Gather context on stories and detail panel
- [x] Create Ring.stories.tsx
- [x] Create Orb.stories.tsx
- [x] Add Ring import to Type.tsx
- [x] Replace Slider t with Ring in ConnectorsListSection
- [x] Replace Slider t with Ring in ConnectorSection
- [x] Build and verify
- [x] Run tests
- [x] Close ticket

## Plan

1. Create Storybook stories for Orb and Ring.
2. Add Ring import to Type.tsx.
3. In ConnectorsListSectionForm: Add Ring showing all connectors as orbs. Remove per-connector Slider for t.
4. In ConnectorSectionForm: Replace Slider for t with single-orb Ring.
5. Build and run tests.
