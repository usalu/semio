# Fix Window Celebrate Conic Gradient

## Bug
Docked window silhouettes used `window-silhouette-border-celebrated-cycle`, which swapped the stroke among primary/secondary/tertiary as solid colors. Buttons use a spinning three-stop `conic-gradient` via `[data-celebrated="true"]::after`.

## Fix
- Celebrated silhouette paints a masked `foreignObject` fill (`.window-silhouette-border-celebrated-fill`) with the same conic + `celebrate-border-spin` as buttons.
- Mask path (`.window-silhouette-border-celebrated-mask`) keeps the thickness burst via the existing stroke-width pulse.
- Removed the solid color-cycle keyframes.
