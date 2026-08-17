from pathlib import Path

p = Path("elements/client/lib/board/index.ts")
t = p.read_text(encoding="utf-8")
t = t.replace(
    'node.selected ? "node.selected" : "node"',
    'boardObjectChromeStyleKey("node", node)',
)
t = t.replace(
    'handle.selected ? "handle.selected" : "handle"',
    'boardObjectChromeStyleKey("handle", handle)',
)
old = (
    "\t/** @emoji 💠️ Preselect preview ids when non-empty, otherwise committed selection (mirrors WASM `selection_chrome_ids`). */\n"
    "\tprivate selectionChromeIds(): Set<string> {\n"
    "\t\treturn this.preselectIds.size > 0 ? this.preselectIds : this.selectionIds;\n"
    "\t}\n\n"
    "\tprivate applySelectionChromeToSceneObjects(): void {\n"
    "\t\tconst chrome = this.selectionChromeIds();\n"
    "\t\tfor (const object of this.scene.getAllObjects()) {\n"
    "\t\t\tobject.selected = chrome.has(object.id);\n"
    "\t\t}\n"
    "\t}"
)
new = (
    "\tprivate applySelectionChromeToSceneObjects(): void {\n"
    "\t\tconst { highlightedIds, selectedIds } = boardElementInteractionChrome(\n"
    "\t\t\tthis.selectionIds,\n"
    "\t\t\tthis.preselectStore.getSnapshot(),\n"
    "\t\t);\n"
    "\t\tfor (const object of this.scene.getAllObjects()) {\n"
    "\t\t\tobject.selected = selectedIds.has(object.id);\n"
    "\t\t\tobject.highlighted = highlightedIds.has(object.id);\n"
    "\t\t}\n"
    "\t}"
)
if old not in t:
    raise SystemExit("OLD BLOCK NOT FOUND")
t = t.replace(old, new)
old_cancel = (
    '\t\t\t\t\tcase "preselectCancel": {\n'
    "\t\t\t\t\t\tthis.updatePreselection([], [], true);\n"
    "\t\t\t\t\t\tbreak;\n"
    "\t\t\t\t\t}"
)
new_cancel = (
    '\t\t\t\t\tcase "preselectCancel": {\n'
    "\t\t\t\t\t\tthis.updatePreselection([], [], false);\n"
    "\t\t\t\t\t\tthis.applySelectionChromeToSceneObjects();\n"
    '\t\t\t\t\t\tthis.emit("preselectCancel", BOARD_PRESELECT_EMPTY);\n'
    "\t\t\t\t\t\tbreak;\n"
    "\t\t\t\t\t}"
)
if old_cancel not in t:
    raise SystemExit("CANCEL BLOCK NOT FOUND")
t = t.replace(old_cancel, new_cancel)
p.write_text(t, encoding="utf-8")
print("patched index.ts")
