# Ticket

## Todos
- [x] Open ticket `PORT-COLOR-STRATEGY-FOR-COMPATIBILITY` and create `plan.md`.
- [x] Audit current port rendering and compatibility behavior across Kit, Type, and Design apps.
- [x] Implement a shared port color strategy keyed by ports and compatibility.
- [x] Apply the strategy to Type and Design connector visuals and interactions.
- [x] Apply the strategy to Kit views where ports are represented.
- [x] Validate behavior with tests/checks.
- [x] Update `README.md` and `AGENTS.md` documentation.
- [x] Finalize summary and close ticket with touched files.
- [x] Reopen ticket for visibility regression: "i cant see these color strategy".
- [x] Restore missing runtime integrations of `portColor` in active Type/Design/Kit render paths.
- [x] Fix connector port edit persistence to maintain `PortId` object semantics.
- [x] Increase default visibility of per-port identity colors.
- [x] Revalidate tests/build and close ticket.

## Changes
- Added `js/compose/sketchpad/portColor.ts` with deterministic compatibility-family port tones, connector port guid helpers, and compatibility-state resolution.
- Updated `js/compose/sketchpad/Design.tsx` connector handles to consume shared port tones and show compatibility/incompatibility emphasis while a source connector is selected.
- Updated `js/compose/sketchpad/Type.tsx` connector scene visuals to use shared port tones and normalized connector port/compatible port editing paths.
- Updated `js/compose/sketchpad/Kit.tsx` to apply shared port tones to port avatars in both table and diagram nodes.
- Extended `js/compose/sketchpad/elements.tsx` `TableAvatar` with `fallbackStyle` so domain surfaces can supply deterministic avatar fills.
- Updated `README.md` and `AGENTS.md` to document the compatibility-family port color strategy in Bundles/SRS/Codebase docs.
- Reapplied `portColor` wiring in active Type/Design/Kit rendering paths where it was not present in the current working state.
- Updated Type connector editing paths so port and compatible port fields write/read `PortId` values consistently.
- Changed default visual treatment so ungrouped ports remain distinctly colorized by per-port identity.

## Log
- Prompt: "create a color strategy for ports to enhance UX including compatible ports and different port types, refactor as needed, and ensure ticket plan/log/summary workflow."
- Opened ticket via `repo ticket open` and initialized `plan.md`.
- Implemented shared port color strategy and integrated it into Kit, Type, and Design rendering paths.
- Validation run: `npx nx test @compose/js --skipNxCache`.
- Validation run: `npx nx build @compose/js --skipNxCache`.
- Reopened with prompt: "i cant see these color strategy".
- Identified active code paths missing `portColor` consumption and corrected integrations.
- Validation run: `npx nx test @compose/js --skipNxCache`.
- Validation run: `npx nx build @compose/js --skipNxCache`.

## Summary

Reapplied and strengthened visible port color strategy across Kit/Type/Design, fixed PortId editing paths, and ensured per-port identity colors remain visible without explicit compatibility links.
