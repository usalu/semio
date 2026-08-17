# Mode Dock Corner Chips Report

## Summary

Implemented four-corner mode-dock tab chips with per-tab inline actions (focus / new window / close), corner-aware drag/drop, `ui.window.*` hotkeys/labels, and matching keybinding + i18n wiring.

## Canvas changes

### Types / helpers added
- `WINDOW_STACK_CORNERS`, `resolveWindowCorner`, `modeStackTabsByCorner`
- `flatIndexForCornerInsert`, `insertWindowAsTabAtCorner`, `setWindowCornerInLayout`
- `insertWindowAsTab(..., corner?)`
- `ModeDropZone` tab: `{ corner }`
- `ModeStackDropTargets`: `{ corners, body }`
- `ModeTabInsertPreview`: `{ corner }`
- `ModeCanvasDropTarget` tab: optional `corner`
- `ModeProps.onWindowOpenInNewWindow`
- `ModeDockContextValue.openWindowInNewWindow` + cornered `registerStackDropTargets`

### UI
- `ModeDockTabBar`: wrapper `data-slot="mode-dock-tab"` with `role="tab"` activate button, inline focus/new-window/close (`ChromeControlHint`), `DragHandle`; mobile only close; empty-corner drop pad while dragging
- `ModeDockStack`: four corner bars → `titleChips` / `capRightChips` / `footerLeftChips` / `footerRightChips`; stack-level `enlarge`/`close` removed

### Drag / drop
- `computeModeDropZone` hit-tests all four corner bars, then body, then root
- `applyModeDrop` tab path: always remove + `insertWindowAsTabAtCorner`
- `mergeStackTabsIntoStack` inserts into the drop corner
- Insert preview ghosts only when `insertPreview.corner` matches the bar

### Hotkeys in `Mode`
- `ui.window.close` → close `activeWindowId`
- `ui.window.focus` → `toggleMaximize` for that window's stack
- `ui.window.newWindow` → `onWindowOpenInNewWindow?.(activeWindowId)`

### Exports
`modeStackTabsByCorner`, `insertWindowAsTabAtCorner`, `setWindowCornerInLayout`, `WINDOW_STACK_CORNERS`, `resolveWindowCorner`

## Keybinding / UiDriver / I18n
- `SHELL_KEYBINDINGS`: `ui.window.close` / `focus` / `newWindow` (present)
- `useControlHotkey` + `useControlKeybinding`: `SHELL_KEYBINDINGS[resolveControlLabelId(...)]` fallback
- `resolveControlLabelId`: `framework.modeDock.*.close|focus|newWindow` → `ui.window.*` (present)
- Added `UiTranslationSchema.ui.window` + de/en bundle strings

## Not in this turn
- ShellHost `onWindowOpenInNewWindow` wiring
- wgpu / TUI renderer parity
- Automated test suite run
