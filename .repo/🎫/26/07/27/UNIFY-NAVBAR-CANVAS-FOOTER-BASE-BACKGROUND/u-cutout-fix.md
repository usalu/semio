# U-cutout filled with window background

## Symptom
Window chrome U-gap (between title chips and Focus/Close) painted solid window beige instead of showing the canvas/base floor.

## Root cause
`ModeDockTabBar` stamped `ui-glass` on the full `mode-dock-tabbar` row. The gap child is forced transparent by CSS (`[data-window-silhouette-gap]`), but still sits on the parent's window-level glass fill — so the notch looked like opaque window chrome.

Shell floor unification (Layout paints one base; ProductShell/navbar/footer/mode-body defer) was already in place; the remaining defect was ModeDock glass placement diverging from `WindowChrome` (chip-only glass).

## Fix
- Remove `ui-glass` from the tabbar root (single-tab and multi-tab paths).
- Stamp `ui-glass` on tab-cap / inactive tab cells and controls-cap only.
- Gap stays clear so Layout's base floor shows through the U-notch.
