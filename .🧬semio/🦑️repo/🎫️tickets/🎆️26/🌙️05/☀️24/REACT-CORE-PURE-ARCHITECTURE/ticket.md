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

## Framework Product Plugins Follow-Up (agent)

- Board play: `buildBoardPlayRuntime`, `buildBoardPlayAppRuntime`, `boardPlayPlugin` (`PluginModule`), `registerWindowBody` / `registerSidePanelBody`; host uses `ProductView` + `runtime`.
- Storybook UI story: `ProductRuntime` / `AppRuntime` / `WindowKindRuntime` / `ProductView`; declarative `AppTools` as `ToolItem[]`.
- `elements/lib/framework/AGENTS.md`: vocabulary line updated (Product / Surface / Capability / Plugin / Contribution).
- **Repo MCP** (`ticket_open` / `ticket_close`, `repo://goals`) was unavailable in this session; run `ticket_close` manually when MCP is connected.
- **Tests run:** `bun ./📜️script.ts test` in `elements/lib/framework/core` — 7 passed. Board package Vitest fails on pre-existing duplicate exports in `index.tsx` (unrelated).
