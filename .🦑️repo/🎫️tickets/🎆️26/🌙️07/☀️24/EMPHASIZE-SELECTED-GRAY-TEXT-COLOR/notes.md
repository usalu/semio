# Emphasize Selected Gray Text Color

## Problem
`::selection` only set `background-color: var(--accent)` (primary `#ff344f`).
Muted/gray copy (`text-muted-foreground` / `#7b827d`) kept its color while selected,
so introduction step descriptions were unreadable on the active fill (~1.1:1 contrast).

## Fix
Set selection text `color: var(--border-emphasized-color)` (= foreground) for both
`::selection` and `::-moz-selection`. Emphasized dark on primary is ~5.4:1.

## Validation
- `bun test ui/styling/js/index.test.ts` — pass (selection CSS assertion + existing suite)
