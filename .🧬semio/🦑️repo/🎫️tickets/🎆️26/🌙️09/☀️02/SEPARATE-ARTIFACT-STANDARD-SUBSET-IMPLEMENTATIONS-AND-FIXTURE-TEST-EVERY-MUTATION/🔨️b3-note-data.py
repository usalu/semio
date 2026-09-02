# Shared data for the note artifact split.

SUBSET_VECTORS = {
    "document": [("rename-note", "🏷️rename-note", "retitles-the-document")],
    "canvas": [
        ("change-grid-visible", "👁️change-grid-visible", "hides-the-grid"),
        ("change-grid-spacing", "📏️change-grid-spacing", "widens-grid-spacing"),
        ("change-grid-subdivisions", "🔢️change-grid-subdivisions", "doubles-grid-subdivisions"),
        ("change-grid-opacity", "🌫️change-grid-opacity", "raises-grid-opacity"),
        ("change-snap-enabled", "🧲️change-snap-enabled", "enables-snap"),
        ("change-snap-grid-spacing", "📐️change-snap-grid-spacing", "halves-snap-grid-spacing"),
    ],
    "ink": [
        ("change-pencil-width", "✏️change-pencil-width", "thickens-pencil"),
        ("change-eraser-radius", "🧽️change-eraser-radius", "enlarges-eraser"),
        ("change-block-ink-width", "🖊️change-block-ink-width", "thickens-the-sketch-stroke"),
        ("edit-block-ink-stroke", "🎨️edit-block-ink-stroke", "redraws-the-sketch-polyline"),
    ],
    "asset": [
        ("create-asset", "🆕️create-asset", "adds-a-second-image-asset"),
        ("replace-asset-payload", "🔁️replace-asset-payload", "swaps-logo-payload-for-svg"),
        ("delete-asset", "🗑️delete-asset", "removes-the-logo-asset"),
    ],
    "block": [
        ("create-block", "➕️create-block", "inserts-a-photo-block-at-root-index-2"),
        ("delete-block", "❌️delete-block", "removes-the-math-block"),
        ("delete-blocks", "🧺️delete-blocks", "removes-the-ink-and-image-blocks"),
        ("duplicate-block", "🎯️duplicate-block", "copies-the-math-block-right-after-its-source"),
        ("duplicate-blocks", "👥️duplicate-blocks", "copies-ink-and-table-with-shifting-indices"),
        ("move-block-to-container", "🚚️move-block-to-container", "reparents-ink-into-the-callout-group"),
        ("drag-blocks", "🤏️drag-blocks", "nudges-ink-and-the-whole-group-subtree"),
        ("rename-block", "🔖️rename-block", "renames-the-table-block"),
        ("change-block-visible", "👀️change-block-visible", "hides-the-image-block"),
        ("change-block-locked", "🔒️change-block-locked", "locks-the-callout-group"),
        ("move-block", "📍️move-block", "repositions-the-math-block"),
        ("resize-block", "↔️resize-block", "enlarges-the-image-block"),
        ("change-block-font-size", "🔤️change-block-font-size", "enlarges-the-intro-font"),
    ],
    "text": [("edit-block-text", "📝️edit-block-text", "replaces-the-intro-paragraphs")],
    "math": [("edit-block-math", "🧮️edit-block-math", "replaces-the-tex-with-pythagoras")],
    "table": [
        ("insert-table-row", "⬇️insert-table-row", "appends-a-blank-third-row"),
        ("remove-table-row", "⬆️remove-table-row", "drops-the-trailing-blank-row"),
        ("insert-table-column", "➡️insert-table-column", "appends-the-lettered-column-c"),
        ("remove-table-column", "⬅️remove-table-column", "drops-the-trailing-column-b"),
    ],
}

SUBSET_ORDER = ["document", "canvas", "ink", "asset", "block", "text", "math", "table"]
assert set(SUBSET_ORDER) == set(SUBSET_VECTORS)
assert sum(len(v) for v in SUBSET_VECTORS.values()) == 33

CAPABILITY = {s: f"note-1-{s}-mutate" for s in SUBSET_ORDER}
CATALOG = {s: f"note-1-{s}" for s in SUBSET_ORDER}
