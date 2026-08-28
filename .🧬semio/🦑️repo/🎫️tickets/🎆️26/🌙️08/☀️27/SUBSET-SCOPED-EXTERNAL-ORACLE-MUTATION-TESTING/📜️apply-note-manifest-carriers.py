import json

path = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"

ENTRIES = {
    "rename-note":               (["pdf"],             ["note.document.title"]),
    "change-grid-visible":       ([],                  ["note.editor-only-setting"]),
    "change-grid-spacing":       ([],                  ["note.editor-only-setting"]),
    "change-grid-subdivisions":  ([],                  ["note.editor-only-setting"]),
    "change-grid-opacity":       ([],                  ["note.editor-only-setting"]),
    "change-snap-enabled":       ([],                  ["note.editor-only-setting"]),
    "change-snap-grid-spacing":  ([],                  ["note.editor-only-setting"]),
    "change-pencil-width":       ([],                  ["note.editor-only-setting"]),
    "change-eraser-radius":      ([],                  ["note.editor-only-setting"]),
    "create-asset":              (["svg"],             ["note.block.image.payload"]),
    "replace-asset-payload":     (["svg"],             ["note.block.image.payload"]),
    "delete-asset":              (["svg"],             ["note.block.image.payload"]),
    "create-block":              (["dxf","svg","pdf"], ["note.block.ink.geometry","note.block.text.content"]),
    "delete-block":              (["dxf","svg","pdf"], ["note.block.ink.geometry","note.block.text.content"]),
    "delete-blocks":             (["dxf","svg","pdf"], ["note.block.ink.geometry","note.block.text.content"]),
    "duplicate-block":           (["dxf","svg","pdf"], ["note.block.ink.geometry","note.block.text.content"]),
    "duplicate-blocks":          (["dxf","svg","pdf"], ["note.block.ink.geometry","note.block.text.content"]),
    "move-block-to-container":   ([],                  ["note.block.reparent"]),
    "drag-blocks":               (["svg"],             ["note.block.transform"]),
    "rename-block":              ([],                  ["note.block.name"]),
    "change-block-visible":      (["svg"],             ["note.block.visibility"]),
    "change-block-locked":       ([],                  ["note.block.lock-state"]),
    "move-block":                (["svg"],             ["note.block.transform"]),
    "resize-block":              (["svg"],             ["note.block.transform"]),
    "change-block-font-size":    ([],                  ["note.block.font-size"]),
    "edit-block-text":           (["pdf","svg"],       ["note.block.text.content"]),
    "edit-block-math":           ([],                  ["note.block.math.content"]),
    "change-block-ink-width":    (["svg"],             ["note.block.ink.stroke-width"]),
    "edit-block-ink-stroke":     (["dxf","svg"],       ["note.block.ink.geometry"]),
    "insert-table-row":          ([],                  ["note.block.table.cell-content"]),
    "remove-table-row":          ([],                  ["note.block.table.cell-content"]),
    "insert-table-column":       ([],                  ["note.block.table.cell-content"]),
    "remove-table-column":       ([],                  ["note.block.table.cell-content"]),
}
assert len(ENTRIES) == 33

RATIONALE_BY_CAP = {
    "note.editor-only-setting": "editor/tool state (grid, snap, pencil, eraser) is never written to any exported carrier — invisible by construction, not merely by an implementation gap.",
    "note.block.reparent": "MoveBlockToContainer's own diff (🔺️diff/🦀️component.rs) removes and re-adds the SAME block clone under a new parent id/index without touching x/y — no carrier's output changes.",
    "note.block.name": "no carrier renders a block's `name` field.",
    "note.block.lock-state": "no carrier's rendering path reads `locked`.",
    "note.block.font-size": "the SVG drawing bridge wires `font_size` to the <text> element's Y COORDINATE (draw_node_from_note_block), never to a font-size style — the property is not encoded as font size by any carrier, so no oracle may claim to check it.",
    "note.block.math.content": "Math blocks always render as a generic outline rectangle (note_document_to_drawing_snapshot) — TeX content never reaches any carrier.",
    "note.block.table.cell-content": "Table blocks always render as a generic outline rectangle keyed only to width/height; row/column mutations only touch `rows`/`columns`, never `width`/`height` (confirmed in insert-table-row's own diff) — invisible in every carrier.",
}

d = json.load(open(path, encoding="utf-8"))
mm = d["mutationManifests"][0]
assert len(mm["mutations"]) == 33, len(mm["mutations"])
seen = set()
for mutation in mm["mutations"]:
    carriers, capabilities = ENTRIES[mutation["id"]]
    seen.add(mutation["id"])
    mutation["carriers"] = carriers
    if carriers:
        mutation["oracleRequirements"] = [{"capability": cap, "qualifyingKind": "third-party-library"} for cap in capabilities]
    else:
        req = {"capability": capabilities[0], "qualifyingKind": "third-party-library"}
        note = RATIONALE_BY_CAP.get(capabilities[0])
        if note:
            req["note"] = note
        mutation["oracleRequirements"] = [req]
assert seen == set(ENTRIES.keys())

json.dump(d, open(path, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
open(path, "a", encoding="utf-8").write("\n")
print("applied carriers to", len(mm["mutations"]), "mutations")
