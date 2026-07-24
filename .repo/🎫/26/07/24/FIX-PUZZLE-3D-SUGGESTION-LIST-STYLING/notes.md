# Fix Puzzle 3D Suggestion List Styling

## Problem
Suggestion popup rows showed a leading color swatch (often default sky `#38bdf8`) next to the icon — a literal blue box that did not match CanvasPickMenu / floating-menu density. Checked-row hover also dropped the active fill back to gray.

## Fix
- `suggestionMenuItems`: omit `color` (icon + active highlight only; object-kind color stays on the 3D ghost).
- ContextMenu controller rows: drop extra `bg-transparent`; keep `bg-active-base` through hover (`hover:bg-active-base/90`).

## Verify
`bun x vitest run --config ui/js/react/vitest.config.ts --testNamePattern ContextMenu`
`bun x vitest run --config framework/renderer/react/vitest.config.ts --testNamePattern "maps suggestion-style|maps context menu specs"`
