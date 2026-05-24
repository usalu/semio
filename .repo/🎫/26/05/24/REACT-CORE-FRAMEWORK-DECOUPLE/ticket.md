# React Core Framework Decouple

**Goal:** elements architecture — `@elements/lib/react/core` pure React, no `@elements/framework` dependency.

**Status:** open

## Plan

- Remove `@elements/framework` / `@elements/framework-react` imports and re-exports from `@elements/ui` (`elements/lib/react/core`).
- Local `Expertise` enum in react core (label/tooltip chrome).
- Workbench shell lives in `@elements/framework-react` (`workbench-view.tsx`, `workbench-mount.tsx`).
- Move framework integration tests to framework-react vitest.
- Update sketchpad to import workbench chrome from `@elements/framework-react`.
