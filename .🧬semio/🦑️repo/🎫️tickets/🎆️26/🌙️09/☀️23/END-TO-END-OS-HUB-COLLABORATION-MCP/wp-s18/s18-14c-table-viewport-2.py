# -*- coding: utf-8 -*-
"""S18 §14c (C12 P1), step 2 on top of s18-14c-table-viewport.py: the viewport cap keeps one row of slack (a scrolled
viewport shows two partial edge rows), the tree-window viewport excludes a sticky header declared as `scroll-padding-top`
(the table's header row sits over the first materialised row), and a keyboard move below the viewport lands the row
fully above the viewport's bottom (it landed one row short: the header row was not counted). Idempotent."""
import json
import pathlib

INTERPRETER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx")
TABLE_LAWS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/📊️table/🟦️.tsx")
FIXTURE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪟️tree-window-served.json")

EDITS = [
    (INTERPRETER,
     """export function treeWindowViewportMetrics(viewport: HTMLElement): { readonly originTop: number; readonly height: number } {
  const owner = viewport.ownerDocument ?? null;
  if (viewport === owner?.scrollingElement || viewport === owner?.documentElement) return { originTop: 0, height: Math.max(0, owner?.defaultView?.innerHeight ?? viewport.clientHeight) };
  const rect = viewport.getBoundingClientRect();
  return { originTop: rect.top + viewport.clientTop, height: Math.max(0, viewport.clientHeight || rect.height) };
}""",
     """export function treeWindowViewportMetrics(viewport: HTMLElement): { readonly originTop: number; readonly height: number } {
  const owner = viewport.ownerDocument ?? null;
  if (viewport === owner?.scrollingElement || viewport === owner?.documentElement) return { originTop: 0, height: Math.max(0, owner?.defaultView?.innerHeight ?? viewport.clientHeight) };
  const rect = viewport.getBoundingClientRect();
  const inset = treeWindowStickyInsetPx(viewport);
  return { originTop: rect.top + viewport.clientTop + inset, height: Math.max(0, (viewport.clientHeight || rect.height) - inset) };
}

/** 📌️ The band a scroll viewport's own sticky header covers, as the viewport declares it (`scroll-padding-top`, which
 * also keeps focus and `scrollIntoView` from parking a row under that header): rows beneath it are not visible, so the
 * window rule must not spend the served window on them (the table's header row hid the first materialised row). */
function treeWindowStickyInsetPx(viewport: HTMLElement): number {
  const inset = Number.parseFloat(viewport.ownerDocument?.defaultView?.getComputedStyle(viewport).scrollPaddingTop ?? "");
  return Number.isFinite(inset) && inset > 0 ? inset : 0;
}"""),
    (INTERPRETER,
     """export function tableWindowViewportCapRowsV1(learned: number | null, previous: number | null, total: number): number | null {
  const servable = learned ?? previous;
  return servable !== null && servable > 0 && servable < total ? servable : null;
}""",
     """export function tableWindowViewportCapRowsV1(learned: number | null, previous: number | null, total: number): number | null {
  const servable = learned ?? previous;
  return servable !== null && servable > 0 && servable < total ? Math.max(1, servable - 1) : null;
}"""),
    (INTERPRETER,
     " * than that — `null` = uncapped. A viewport taller than the rows the guest can materialise at once shows blank rows no\n",
     " * than that, one row short of it (a viewport scrolled between rows shows a partial row at each edge, and the served\n * window must cover both) — `null` = uncapped. A viewport taller than the rows the guest can materialise at once shows blank rows no\n"),
    (INTERPRETER,
     "    if (scroller) scroller.scrollTop = tableWindowScrollTopForRowV1(index, rowPx, scroller.scrollTop, scroller.clientHeight);\n",
     "    if (scroller) scroller.scrollTop = tableWindowScrollTopForRowV1(index, rowPx, scroller.scrollTop, scroller.clientHeight - rowPx);\n"),
    (INTERPRETER,
     "style={viewportCapRows === null ? undefined : { maxHeight: (viewportCapRows + 1) * rowPx }}>",
     "style={{ scrollPaddingTop: rowPx, maxHeight: viewportCapRows === null ? undefined : (viewportCapRows + 1) * rowPx }}>"),
    (INTERPRETER,
     "    const page = Math.max(1, Math.floor((scrollRef.current?.clientHeight ?? rowPx) / rowPx));\n",
     "    const page = Math.max(1, Math.floor(((scrollRef.current?.clientHeight ?? 2 * rowPx) - rowPx) / rowPx));\n"),
    (TABLE_LAWS,
     "      expect(scrollTop).toBe(40 * treeRowHeightPx - viewportHeight);\n",
     "      expect(scrollTop).toBe(41 * treeRowHeightPx - viewportHeight);\n"),
    (TABLE_LAWS,
     "      expect(last.viewportRows).toBe(10);\n",
     "      expect(last.viewportRows).toBe(9);\n"),
]

CAPS = [
    {"name": "no short answer yet: uncapped", "learned": None, "previous": None, "total": 33, "capRows": None},
    {"name": "Home on 7800: the guest serves 11 of 33 → 10 rows show, a partial row at each edge still served", "learned": 11, "previous": None, "total": 33, "capRows": 10},
    {"name": "a new total forgot the capacity: the last one holds until re-taught", "learned": None, "previous": 11, "total": 34, "capRows": 10},
    {"name": "a re-taught capacity replaces the previous one", "learned": 22, "previous": 11, "total": 34, "capRows": 21},
    {"name": "a list no longer than the capacity is uncapped", "learned": 11, "previous": None, "total": 11, "capRows": None},
    {"name": "a list shorter than the capacity is uncapped", "learned": None, "previous": 11, "total": 7, "capRows": None},
    {"name": "a zero capacity never caps (an empty answer teaches nothing)", "learned": 0, "previous": None, "total": 5, "capRows": None},
    {"name": "a one-row capacity still shows one row", "learned": 1, "previous": None, "total": 5, "capRows": 1},
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
    data = json.loads(FIXTURE.read_text(encoding="utf-8"))
    if data.get("tableViewportCaps") != CAPS:
        data["tableViewportCaps"] = CAPS
        FIXTURE.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("ok")


main()
