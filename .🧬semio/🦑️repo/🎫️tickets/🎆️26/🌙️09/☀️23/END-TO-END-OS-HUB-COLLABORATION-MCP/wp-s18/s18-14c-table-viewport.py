# -*- coding: utf-8 -*-
"""S18 §14c (C12 P1): a windowed table's scroll viewport is capped at the rows its guest can serve, so a list the guest
answers in short windows scrolls (wheel, keyboard, programmatic) over every row instead of showing unreachable blanks.
Idempotent; run once, then tsc + laws + one boot (rule 20)."""
import json
import pathlib

INTERPRETER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx")
TABLE_LAWS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/📊️table/🟦️.tsx")
FIXTURE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json")

EDITS = [
    (INTERPRETER,
     "function useTreeWindowObserver(rootRef: RefObject<HTMLDivElement | null>, windows: TreeWindowContextValue | null, revision: unknown, store: unknown): void {\n  const windowsRef = useRef<TreeWindowContextValue | null>(windows);\n  windowsRef.current = windows;\n",
     "function useTreeWindowObserver(rootRef: RefObject<HTMLDivElement | null>, windows: TreeWindowContextValue | null, revision: unknown, store: unknown, onServed?: (memory: ReadonlyMap<string, TreeWindowServedMemoryV1>) => void): void {\n  const windowsRef = useRef<TreeWindowContextValue | null>(windows);\n  windowsRef.current = windows;\n  const onServedRef = useRef(onServed);\n  onServedRef.current = onServed;\n"),
    (INTERPRETER,
     "      servedRef.current = served.memory;\n",
     "      servedRef.current = served.memory;\n      onServedRef.current?.(served.memory);\n"),
    (INTERPRETER,
     "/** 📊️ The shared column track of a table's header and rows:",
     """/** 📊️ How many rows a windowed table's scroll viewport may show at most: the capacity the guest's short answers taught
 * (`learned`, {@link treeWindowServedRequestsV1}), else the one it taught before (a new `total` forgets the capacity
 * until the next short answer re-teaches it, and the viewport must not flash open meanwhile), while the list is longer
 * than that — `null` = uncapped. A viewport taller than the rows the guest can materialise at once shows blank rows no
 * scroll reaches: the whole list fits the viewport, so the scroll range is a row or two, the window never moves, and
 * every row past the capacity stays unreachable by wheel, keyboard and screen reader alike (Home on hub 7800, 11 of 29
 * spaces, ticket 26/09/23 S18 §14c / C12). Capped, the viewport scrolls over the list and the window follows it.
 * Rows: `🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json` `tableViewportCaps`. */
export function tableWindowViewportCapRowsV1(learned: number | null, previous: number | null, total: number): number | null {
  const servable = learned ?? previous;
  return servable !== null && servable > 0 && servable < total ? servable : null;
}

/** 📊️ The shared column track of a table's header and rows:"""),
    (INTERPRETER,
     "  const range = useLabel(\"ui.host.tableRowRange\", { from: rows.length > 0 ? leading + 1 : 0, to: leading + rows.length, total });\n",
     "  const range = useLabel(\"ui.host.tableRowRange\", { from: rows.length > 0 ? leading + 1 : 0, to: leading + rows.length, total });\n  const [servableRows, setServableRows] = useState<number | null>(null);\n  const viewportCapRows = tableWindowViewportCapRowsV1(null, servableRows, total);\n"),
    (INTERPRETER,
     "  useTreeWindowObserver(rootRef, windows, revision, store);\n  useEffect(() => {\n    const wanted = focusRowRef.current;",
     "  useTreeWindowObserver(rootRef, windows, revision, store, (memory) => {\n    const learned = memory.get(record.key)?.capacity ?? null;\n    if (learned !== null) setServableRows((current) => (current === learned ? current : learned));\n  });\n  useEffect(() => {\n    const wanted = focusRowRef.current;"),
    (INTERPRETER,
     "      <div ref={scrollRef} role=\"rowgroup\" data-slot=\"table-window-scroll\" className=\"min-h-0 min-w-0 flex-1 overflow-auto\">",
     "      <div ref={scrollRef} role=\"rowgroup\" data-slot=\"table-window-scroll\" data-table-viewport-cap={viewportCapRows ?? undefined} className=\"min-h-0 min-w-0 flex-1 overflow-auto\" style={viewportCapRows === null ? undefined : { maxHeight: (viewportCapRows + 1) * rowPx }}>"),
    (INTERPRETER,
     "    { TreeWindowContext, UiDocumentStore, UiNodeView, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, treeWindowRowHeightPx },",
     "    { TreeWindowContext, UiDocumentStore, UiNodeView, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, tableWindowViewportCapRowsV1, treeWindowRowHeightPx },"),
    (TABLE_LAWS,
     "  const { TreeWindowContext, UiDocumentStore, UiNodeView, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, treeWindowRowHeightPx } = dependencies;",
     "  const { TreeWindowContext, UiDocumentStore, UiNodeView, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, tableWindowViewportCapRowsV1, treeWindowRowHeightPx } = dependencies;"),
    (TABLE_LAWS,
     "    it(\"renders no DOM id twice: two editable rows' cells live under their own row\", () => {",
     """    it("caps the scroll viewport at the rows the guest serves, by the fixture's viewport law", () => {
      const served = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json"), "utf8"));
      expect(served.tableViewportCaps.length).toBeGreaterThan(0);
      for (const row of served.tableViewportCaps) expect(tableWindowViewportCapRowsV1(row.learned, row.previous, row.total), row.name).toBe(row.capRows);
    });

    it("renders no DOM id twice: two editable rows' cells live under their own row", () => {"""),
]

CAPS = [
    {"name": "no short answer yet: uncapped", "learned": None, "previous": None, "total": 33, "capRows": None},
    {"name": "Home on 7800: the guest serves 11 of 33 → the viewport shows 11", "learned": 11, "previous": None, "total": 33, "capRows": 11},
    {"name": "a new total forgot the capacity: the last one holds until re-taught", "learned": None, "previous": 11, "total": 34, "capRows": 11},
    {"name": "a re-taught capacity replaces the previous one", "learned": 22, "previous": 11, "total": 34, "capRows": 22},
    {"name": "a list no longer than the capacity is uncapped", "learned": 11, "previous": None, "total": 11, "capRows": None},
    {"name": "a list shorter than the capacity is uncapped", "learned": None, "previous": 11, "total": 7, "capRows": None},
    {"name": "a zero capacity never caps (an empty answer teaches nothing)", "learned": 0, "previous": None, "total": 5, "capRows": None},
]


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:80])
            text = text.replace(old, new)
        texts[path] = text
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    raw = FIXTURE.read_text(encoding="utf-8")
    data = json.loads(raw)
    if data.get("tableViewportCaps") != CAPS:
        data["tableViewportCaps"] = CAPS
        FIXTURE.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("ok")


main()
