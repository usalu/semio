# Fix Panel Tab Body Chrome Gap

## Root causes
1. `ui-glass-chrome` was removed from CSS while WindowChrome still stamped that dead class on chip caps — caps had no fill.
2. Panel tab buttons used opaque `ui-surface` / mode-dock inactive fills, painting a hard bottom edge across the tab strip while the fold control (transparent on glass) met the body cleanly.

## Fix
- `glassChromeClass` aliases `glassClass` (`ui-glass`)
- Remove inset box-shadow separators on `[data-window-silhouette-chip]`
- Zero piecewise borders on window-chrome-stack children
- Panel window tabs use transparent inactive chrome (`panelWindowInactiveTabClass`) so chip-cap glass is the boundary and meets the body
- WindowChrome body stacks at `z-[1]` under the cap chips (`z-[2]`)
