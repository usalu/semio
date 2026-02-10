# Ticket

## Todos

- [x] Generalize `Diagram` Component in `elements.tsx`
- [x] Refactor `Kit.tsx` to use shared `Diagram`
- [x] Fix compilation errors in `Kit.tsx`
- [x] Update documentation (`README.md`, `AGENTS.md`)
- [x] Verify functionality

## Changes

### [elements.tsx](js/semio/sketchpad/elements.tsx)

- Integrated D3 `forceSimulation` into `DiagramInner`.
- Exposed `forceConfig` prop for physics customization.
- Implemented node drag reheating logic.
- Exposed React Flow primitives.

### [Kit.tsx](js/semio/sketchpad/Kit.tsx)

- Switched to the new shared `Diagram` component.
- Removed ~500 lines of manual physics code.
- Fixed `artifactKinds` regression.
- Corrected `generateUniqueName` type usage.
- Allowed `React.JSX.Element` in diagram node icons.

### [README.md](README.md)

- Added # Shared Infrastructure section under Bundles.
- Documented D3 simulation theory and coordinate alignment (`DIAGRAM_UNIT`).

### [AGENTS.md](AGENTS.md)

- Added Diagrams section under Software Requirements Specification.
- Updated codebase documentation for `elements.tsx` and `Kit.tsx`.

## Log

- **2026-01-28**: Started refactor.
- **2026-01-28**: Generalized `Diagram` in `elements.tsx`.
- **2026-01-28**: Integrated `Diagram` into `Kit.tsx` and removed legacy simulation logic.
- **2026-01-28**: Resolved all compilation errors in `Kit.tsx`.
- **2026-01-28**: Updated `README.md` and `AGENTS.md` to document the new shared components and requirements.

## Summary

Successfully generalized the D3 force-directed simulation logic from Kit.tsx into a reusable Diagram component in elements.tsx. This refactor centralized the physics engine, state management for node positions, and interaction handling (dragging, selection, and "reheating" simulations). The Kit app diagram now consumes this shared infrastructure, resulting in cleaner code and easier maintenance of complex graph layouts across different apps. Standardized coordinate systems and force configurations ensure consistent behavior throughout the Sketchpad. Documentation has been updated to reflect the new architecture and requirements.
