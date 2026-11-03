import json, sys
from pathlib import Path
import pymupdf

here = Path(__file__).resolve().parent
gen = here.parent / "🗑️generated" / "qa"
svg = gen / "svg"
index = [row for row in json.loads((svg / "index.json").read_text(encoding="utf-8")) if "file" in row]
new = set((here / "ids.txt").read_text().strip().split(","))
CW, CH, ROWS, LAB = 640, 340, 4, 22
rows = {}
for row in index:
    rows.setdefault((row["category"], row["id"], row["viewpoint"]), {})[row["view"]] = row["file"]
cats = {}
for (cat, ident, vp), cells in sorted(rows.items()):
    cats.setdefault(cat, []).append((ident, vp, cells))


def cell(file, canvas, x, y):
    doc = pymupdf.open(str(svg / file))
    page = doc[0]
    r = page.rect
    s = min((CW - 8) / r.width, (CH - LAB - 8) / r.height)
    pix = page.get_pixmap(matrix=pymupdf.Matrix(s, s), alpha=True)
    canvas.insert_image(pymupdf.Rect(x + (CW - pix.width) / 2, y + LAB + (CH - LAB - pix.height) / 2, x + (CW + pix.width) / 2, y + LAB + (CH + pix.height) / 2 - 0), pixmap=pix)
    return r.width, r.height


for cat, items in cats.items():
    for part in range(0, len(items), ROWS):
        chunk = items[part:part + ROWS]
        doc = pymupdf.open()
        page = doc.new_page(width=CW * 2, height=CH * len(chunk))
        for r, (ident, vp, cells) in enumerate(chunk):
            for c, view in enumerate(("dark-line", "thermal")):
                x, y = c * CW, r * CH
                back = (0.0, 0.07, 0.09) if view == "dark-line" else (0.04, 0.04, 0.05)
                page.draw_rect(pymupdf.Rect(x, y, x + CW, y + CH), color=(0.4, 0.4, 0.4), fill=back, width=0.5)
                if view in cells:
                    w, h = cell(cells[view], page, x, y)
                else:
                    w = h = 0
                tag = "NEW " if ident in new else "REF "
                page.insert_text((x + 6, y + 15), f"{tag}{ident} [{vp}] {view} {w:.0f}x{h:.0f}mm", fontsize=12, color=(1, 1, 1))
        pix = page.get_pixmap(dpi=72)
        pix.save(str(gen / f"{cat}-{part // ROWS + 1}.png"))
        print(cat, part // ROWS + 1, len(chunk))
