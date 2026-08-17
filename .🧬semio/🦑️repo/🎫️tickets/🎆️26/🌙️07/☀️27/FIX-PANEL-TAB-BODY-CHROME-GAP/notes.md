# Fix Panel Tab Body Chrome Gap

## Root causes
1. `ui-glass-chrome` was removed from CSS while WindowChrome still stamped that dead class on chip caps — caps had no fill.
2. Panel tab buttons used opaque `ui-surface` / mode-dock inactive fills, painting a hard bottom edge across the tab strip while the fold control (transparent on glass) met the body cleanly.
3. **Vertical gap (chrome-hosted):** when a corner/middle panel was open, root tabs stayed in navbar/footer via `PanelChromeTabBar` while the floating `Panel` only rendered nested rows (`startDepth={1}`) + body + fold — tabs and fold were on different horizontal rows with empty canvas between them.
4. **Jump into canvas:** after moving open tabs onto `WindowChrome`, the panel still used `anchorPositionStyle` (`top/bottom: spacing-single` inside the middle region), so chips left the navbar/footer band and reappeared inset in the canvas.

## Fix
- `glassChromeClass` aliases `glassClass` (`ui-glass`)
- Remove inset box-shadow separators on `[data-window-silhouette-chip]`
- Zero piecewise borders on window-chrome-stack children
- Panel window tabs use transparent inactive chrome (`panelWindowInactiveTabClass`) so chip-cap glass is the boundary and meets the body
- WindowChrome body stacks at `z-[1]` under the cap chips (`z-[2]`)
- **Open chrome-hosted panels:** full tab strip on `WindowChrome` titleChips (same cap row as fold, U-gap between); `PanelChromeTabBar` returns null while `visible` and remains the folded representation only
- **Unfold in place:** `chromeHostedOpenPanelPositionStyle` pulls the open cap into the `h-large` navbar/footer band (centered `h-medium` row); open chrome-hosted panels use `z-navbar` so the overlapping cap paints above shell chrome
