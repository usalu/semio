"""📏 Lists the horizontal and vertical rules of a PDF page (from its vector drawings) and renders the page.

usage: python measure.py <pdf> <page (1-based)> [png out] [dpi]
"""
import sys
import pymupdf

pdf, number = sys.argv[1], int(sys.argv[2])
page = pymupdf.open(pdf)[number - 1]
rows = []
for drawing in page.get_drawings():
    box = drawing["rect"]
    color = drawing.get("fill") or drawing.get("color")
    if box.width < 3 and box.height < 3:
        continue
    kind = "h" if box.height < 1.5 else "v" if box.width < 1.5 else "area"
    rows.append((round(box.y0, 2), round(box.x0, 2), kind, round(box.width, 2), round(box.height, 2), tuple(round(c, 2) for c in color) if color else None, drawing.get("fill_opacity")))
for row in sorted(rows):
    print(*row)
if len(sys.argv) > 3:
    page.get_pixmap(dpi=int(sys.argv[4]) if len(sys.argv) > 4 else 110).save(sys.argv[3])
