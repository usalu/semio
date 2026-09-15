# Window Pane Toggles Behind Panels

## Symptom

Opening a right-hand inspector (anchored chrome `Panel`) shifts the host window's **Window Options** measures pane and top-right window controls left, as if they were laid out beside the panel.

## Intended model

- **Panels** (`z-panel`, app-root stacking) float over the layout middle band.
- **Windows** (`z-window`) and their **pane toggles** (measures, engagement, search, utilities) live on the canvas behind that layer.
- Pane chrome must keep its authored `anchorPositionStyle` inset; overlap is resolved by paint order, not by pushing window chrome away.

`chromePanelSafeArea` / `useChromePanelSafeArea` remain for **in-window content overlays** (e.g. generation3d world rails, folded engagement quick actions) that must stay hit-testable.

## Cause

`🪟️Window/🟦️.tsx` (wave B47) subscribed to `useShellChromePanelBoxes`, computed `rightChromeReservePx` via `chromePanelSafeArea(..., "inline")`, and applied it to the measures `Pane` (`inlineEdgeReservePx`) and the absolute window-controls row.

## Fix

Remove the window right-edge chrome reserve path from `Window`; pane toggles no longer react to open panels.
