## Root cause

Engagement suggestion rows relied on cmdk `onSelect`, which often never fired when the popover used `pointerDown` `preventDefault` to keep command-input focus (especially for text-node targets inside rows). Clicks on labels such as ConstructExternalWall, Fill, or Brush did nothing.

## Fix

- `ui/react/index.tsx`: pick possibles on row `pointerDown` (with deduped `selectPossible`); sync hover index on `mouseEnter`; remove popover-root `preventDefault` that closed the list during pointer handling.
