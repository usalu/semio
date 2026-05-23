from pathlib import Path

p = Path("elements/client/lib/board/play/index.tsx")
t = p.read_text(encoding="utf-8")
if "boardElementInteractionChrome" not in t:
    t = t.replace(
        "  BOARD_SELECTION_TARGETS_DEFAULT,\n",
        "  BOARD_SELECTION_TARGETS_DEFAULT,\n  boardElementInteractionChrome,\n",
    )
t = t.replace(
    "function boardPlayChromeIds(selectionIds: Set<string>, preselection: BoardPreselectSnapshot): Set<string> {\n"
    "  return preselection.ids.length > 0 ? new Set(preselection.ids) : selectionIds;\n"
    "}\n\n"
    "function nakaginBoardMarkers(fixture: BoardFixtureV1, chromeIds: Set<string>): ReactElement {",
    "function nakaginBoardMarkers(\n"
    "  fixture: BoardFixtureV1,\n"
    "  chrome: { highlightedIds: Set<string>; selectedIds: Set<string> },\n"
    "): ReactElement {",
)
t = t.replace("selected={chromeIds.has(node.id)}", "highlighted={chrome.highlightedIds.has(node.id)} selected={chrome.selectedIds.has(node.id)}")
t = t.replace("selected={chromeIds.has(handle.id)}", "highlighted={chrome.highlightedIds.has(handle.id)} selected={chrome.selectedIds.has(handle.id)}")
t = t.replace("selected={chromeIds.has(edge.id)}", "highlighted={chrome.highlightedIds.has(edge.id)} selected={chrome.selectedIds.has(edge.id)}")
t = t.replace(
    "{nakaginBoardMarkers(fixture, boardPlayChromeIds(selectionIds, preselection))}",
    "{nakaginBoardMarkers(fixture, boardElementInteractionChrome(selectionIds, preselection))}",
)
p.write_text(t, encoding="utf-8")
print("patched play/index.tsx")
