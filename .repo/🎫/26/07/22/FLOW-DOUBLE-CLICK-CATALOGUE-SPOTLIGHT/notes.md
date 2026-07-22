# Notes

## Behavior
- Double-click empty flow canvas → open `FlowSpotlight` at cursor.
- Typing filters catalogue via `flowRankCatalogueSuggestions`.
- Top match (while typing) and hovered/arrow-selected rows call `setGhostWidget` so the canvas paints a highlighted ghost node.
- Enter / row click → `addWidget` at world point + commit fixture; Escape / outside click clears ghost and closes.

## Placement
Implemented in `framework/renderer/react/index.tsx` (`FlowGraphCanvasHost`), reusing existing ghost paint path from catalogue drag.
