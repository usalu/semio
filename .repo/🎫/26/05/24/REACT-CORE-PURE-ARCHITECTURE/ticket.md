# React Core Pure Architecture

**Goal:** `@elements/lib/react/core` pure React with no `@elements/framework` dependency; shell in `@elements/framework-react`.

## Summary

- Removed `@elements/framework-react` from `@elements/ui` `package.json`.
- Moved Golden Layout shell (`UICanvas`, `UISearch`, `UIFind`, `UIToolbar`, layout helpers) from `@elements/ui` into `@elements/framework-react` `shell-canvas` region.
- Replaced imperative classes in `@elements/ui` with `createDOMEventBinding`, `usePointerDrag`, `useNativeDragAndDrop`, `staticTreePanelDefinition`, `staticSidePanelTabDefinition`.
- Aligned `@elements/framework-react` and tests with `@elements/framework` `ProductRuntime` / `AppRuntime` / `ModeRuntime` / `WindowKindRuntime` API; exported `WorkbenchView` alias for `ProductView`.
- Updated `@elements/react/board` play host to use new UI hooks/factories.

## Tests

- `@elements/ui`: 36 passed
- `@elements/framework` core: 7 passed
- `@elements/framework-react`: 7 passed
- `@elements/playground`: 3 passed

## Files

- `elements/lib/react/core/index.tsx`, `package.json`
- `elements/lib/framework/renderer/react/index.tsx`, `package.json`
- `elements/lib/react/board/board-play-host.tsx`
