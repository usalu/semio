# Keep Panel Toggles Visible During Canvas Ghost

## Symptom

Canvas interactions (e.g. rectangle selection in 3D) correctly ghost open panes/panels, but also hid navbar/footer `PanelChromeTabBar` toggles. Open panel/pane borders (`[data-slot="chrome-frame"]`) stayed visible because the frame layer was intentionally not `data-dim`.

## Cause

1. Every `GhostRegionShell` set `data-ghost` whenever the global ghost session was active — including chrome tab bars in navbar/footer.
2. Chrome tab rows carried `data-dim`, so they opacity-0'd under ghost CSS.
3. `chrome-frame` was excluded from `data-dim`, so borders remained after fill/content hid.

## Fix

1. `GhostRegionShell` gains `sessionGhost` (default `true`). `PanelChromeTabBar` uses `sessionGhost={false}`.
2. Tab rows only get `data-dim` when open (`variant !== "chrome" && showActiveColor`).
3. Open panels/panes mark fill, `chrome-frame`, chrome header, and body with `data-dim`; folded toggles stay undimmed.
4. Window measures/engagement/search/utility stacks only set `data-dim` when unfolded/expanded.
