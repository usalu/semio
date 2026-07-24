# Projection pane bottom-middle + fold

## Changes
- Default anchor: `bottom-middle` (framework + CAD)
- Wired `folded` / `onFoldToggle` so chrome collapse/uncollapse works
- Pane middle anchors: dual-edge resize with `deltaFactor={2}` (panel parity); bottom anchors keep `flex-col-reverse` grow-up

## Verified
Pane vitest: bottom-middle placement/grow, fold toggle — 3 passed
