# React Core Framework Decouple

**Goal:** elements architecture — `@elements/lib/react/core` pure React, no `@elements/framework` dependency.

**Status:** closed

## Summary

- Removed all `@elements/framework` and `@elements/framework-react` imports/re-exports from `@elements/ui` (`elements/lib/react/core/index.tsx`).
- Added local `Expertise` enum in react core for tooltip/label chrome.
- Workbench shell (`WorkbenchView`, `ReactUI`, mount helpers, `useUIHistory`) lives in `@elements/framework-react` (`workbench-view.tsx`, `workbench-mount.tsx`, `workbench-history.tsx`).
- `workbench-bridge.tsx` exports workbench chrome from framework-react (imports UI primitives from `@elements/ui` only).
- Framework integration tests moved from `@elements/ui` to `workbench-view.tsx` / `ui-declarative-renderer.tsx`.
- Added `UiPanelHostSurfaceNode` + `registerUiPanelSurfaceHost` for sketchpad panel surfaces.
- Sketchpad + Storybook UI stories import workbench from `@elements/framework` / `@elements/framework-react`, not `@elements/ui`.
- Deleted `elements/lib/react/core/ui-declarative-renderer.tsx` re-export shim.

## Tests

- `@elements/ui`: 41 passed
- `@elements/framework-react`: 5 passed
- `@elements/framework` core: 3 passed
- `@elements/playground`: 3 passed

## Files

- `elements/lib/react/core/index.tsx`, `package.json`, `vitest.config.ts` (removed `ui-declarative-renderer.tsx`)
- `elements/lib/framework/core/index.ts`
- `elements/lib/framework/renderer/react/workbench-view.tsx`, `workbench-mount.tsx`, `workbench-bridge.tsx`, `ui-declarative-renderer.tsx`, `index.tsx`, `package.json`
- `compose/client/lib/sketchpad/js/index.ts`
- `.storybook/stories/elements/ui/UI.stories.tsx`
- `elements/lib/react/board/board-play-host.tsx`
