# Fix Puzzle 3D Context Menu Styling

## Problem
Suggestion / context-menu rows showed a leading `✓` (and reserved empty space when unchecked). That read as bloat and did not match the 3D preview highlight.

## Fix
In `ui/js/react/index.tsx` ContextMenu:
- Removed checkmark / tick leading glyph.
- `checked` now paints the same active row highlight as CanvasPickMenu / catalogue preview (`bg-active-base text-emphasized` + `data-selected`).
- Reused floating menu surface/item chrome so density matches other overlay menus.

## Verify
`bun x vitest run --config vitest.config.ts --testNamePattern ContextMenu` — 5 passed.
