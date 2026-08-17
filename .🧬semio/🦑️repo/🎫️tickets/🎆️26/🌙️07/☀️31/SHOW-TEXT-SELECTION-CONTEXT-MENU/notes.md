# Show Text Selection Context Menu

## Goal
`🎯️r2602/RUNNING-SKETCHPAD` — when DOM text is selected in the UI, right-click shows Cut/Copy/Paste/Select all with platform shortcuts (`⌘️C` / `Ctrl+C`).

## Approach
- Native browser context menus remain suppressed (`installElementsSurfaceBrowserDefaultSuppression`).
- Added `TextSelectionContextMenuHost` in `@semio-tech/ui-react` ContextMenu region: capture-phase `contextmenu` when the pointer intersects a non-empty DOM selection opens the shared `ContextMenuController` with clipboard actions.
- Mounted in Framework OS shell and Storybook `StorySurfaceHost` (appearance decorator).
- Fixed `@semio-tech/ui-styling` / `@semio-tech/ui-react` package `exports` to local `./…` paths (broken round-trip emoji paths broke vitest resolution).

## Verification
```
bunx vitest run -c 🧪️vitest.config.ts -t "ContextMenu"
```
→ 16 passed | 469 skipped (see `vitest-context-menu-2.log`).

Host test covers: menu open over selected text, Copy row, no Cut on non-editable, clipboard `writeText`.
