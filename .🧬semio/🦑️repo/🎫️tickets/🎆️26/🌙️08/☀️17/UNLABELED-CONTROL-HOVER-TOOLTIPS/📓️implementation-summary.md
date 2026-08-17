# Unlabeled Control Hover Tooltips

## Goal

`R26-02/RUNNING-SKETCHPAD`

## Summary

Centralized native `title` tooltip composition for icon-only chrome controls. Tooltips respect `UiDriver.tooltips` and append hotkeys when `UiDriver.hotkeys` is not `none`. Drag handles accept an optional `subject` for contextual copy such as "Click and hold left click to drag Perspective Window".

## Key changes

- `useControlTooltipText` + `formatControlTooltipText` compose label + hotkey
- `ChromeControlHint` uses tooltip text for `title`, accessible label for `aria-label`
- `Action`, `ActionGroupItem`, `ButtonGroupItem`, `ToggleGroupItem`, `WindowPaneChromeToggle` wired
- `DragHandle.subject` + `ui.tree.drag.sortTarget` / `transferTarget` i18n keys
- Call sites: `ModeDockTabBar`, `PanelTabBar`, `WindowPaneChromeToggle`, panel unit rows

## Verification

- `bun x nx run @semio-tech/ui-react:test -- --run -t "formats control tooltip|icon-only controls expose|drag handles with a subject"` — 3 passed
