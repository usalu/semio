"""🖼 cs: lays the cast SVGs of one view and scale onto contact sheets (PyMuPDF), reference assets first, new assets after."""
import json
import re
import sys
from pathlib import Path

import fitz

HERE = Path(__file__).resolve().parent
G = HERE.parent / "🗑️generated" / "cs"
meta = json.loads((G / "meta.json").read_text(encoding="utf-8"))
BG = {"dark-line": (0.06, 0.08, 0.1), "light-line": (1, 1, 1), "thermal": (0.06, 0.08, 0.1)}
FG = {"dark-line": (0.8, 0.8, 0.8), "light-line": (0.2, 0.2, 0.2), "thermal": (0.8, 0.8, 0.8)}
DPI_ZOOM = 2.2
WIDTH = 1700
GAP = 14


def size_mm(svg):
    m = re.search(r'width="([\d.]+)mm" height="([\d.]+)mm"', svg)
    return float(m.group(1)), float(m.group(2))


def sheet(view, scale, ids, name, zoom=DPI_ZOOM):
    cells = []
    for a in ids:
        f = G / "cast" / f"{a}-{view}-{scale}"[:64] / "0000.svg"
        if not f.exists():
            continue
        svg = f.read_text(encoding="utf-8")
        w, h = size_mm(svg)
        k = 72 / 25.4 * zoom
        cells.append((a, f, w * k, h * k))
    rows, row, x = [], [], 0
    for c in cells:
        cw = max(c[2], 60 * zoom)
        if row and x + cw > WIDTH:
            rows.append(row)
            row, x = [], 0
        row.append(c)
        x += cw + GAP
    if row:
        rows.append(row)
    heights = [max(c[3] for c in r) + 26 for r in rows]
    page_h = sum(heights) + GAP * (len(rows) + 1)
    doc = fitz.open()
    page = doc.new_page(width=WIDTH + 2 * GAP, height=max(page_h, 100))
    page.draw_rect(page.rect, color=None, fill=BG[view])
    y = GAP
    for r, hh in zip(rows, heights):
        x = GAP
        for a, f, w, h in r:
            cw = max(w, 60 * zoom)
            src = fitz.open(f)
            pdf = fitz.open("pdf", src.convert_to_pdf())
            page.show_pdf_page(fitz.Rect(x, y, x + w, y + h), pdf, 0)
            tag = "REF " if meta[a]["kind"] == "ref" else ""
            page.insert_text((x, y + h + 14), f"{tag}{a}", fontsize=9, color=FG[view])
            x += cw + GAP
        y += hh + GAP
    out = G / f"{name}.png"
    page.get_pixmap(dpi=72).save(out)
    print(out, len(cells), "cells", int(page.rect.width), "x", int(page.rect.height))


def main():
    refs = [a for a, m in meta.items() if m["kind"] == "ref"]
    new = [a for a, m in meta.items() if m["kind"] == "new"]
    groups = {"humans": [a for a in new if meta[a]["category"] == "humans"], "appliances": [a for a in new if meta[a]["category"] == "appliances"],
              "rest": [a for a in new if meta[a]["category"] not in ("humans", "appliances")]}
    for view in BG:
        for scale in (50, 100):
            for g, ids in groups.items():
                zoom = {"appliances": 5.0 if scale == 50 else 8.0, "humans": 2.2 if scale == 50 else 4.0}.get(g, 2.2 if scale == 50 else 4.0)
                sheet(view, scale, ([r for r in refs if r in ("seated-reader", "heat-pump-outdoor-unit", "stepladder", "bookshelf")] if g == "appliances" else refs) + ids, f"sheet-{view}-{scale}-{g}", zoom)


main()
