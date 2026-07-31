# Notes

## Bug
`FlowGraphCanvasHost` rendered `SpotlightOverlay` whenever `session.previewText()` was non-empty.

`previewText()` returns the **OutputPreview sink** content summary (e.g. `{3 keys}` for geometry dicts), not a pending spotlight commit. Commit was dispatched with empty `ops`, so the panel was both incorrect and inert.

## Fix
Removed `SpotlightOverlay` and the local `previewText` state wiring from `framework/renderer/react/index.tsx`. Preview sink content continues to render on the canvas Preview widget.
