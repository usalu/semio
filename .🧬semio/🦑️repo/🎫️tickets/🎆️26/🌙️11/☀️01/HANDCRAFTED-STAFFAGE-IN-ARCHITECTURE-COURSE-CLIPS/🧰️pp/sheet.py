"""🖼️ pp: contact sheet of cast frames: sheet.py <out.png> <cast-folder>:<frame> ..."""
import sys

import pymupdf as fitz

out = sys.argv[1]
cells = []
for arg in sys.argv[2:]:
    folder, frame = arg.rsplit(":", 1)
    doc = fitz.open(stream=open(f"{folder}/{int(frame):04d}.svg","rb").read(), filetype="svg")
    pix = doc[0].get_pixmap(matrix=fitz.Matrix(8, 8), alpha=False)
    cells.append(pix)
w = sum(c.width for c in cells)
h = max(c.height for c in cells)
sheet = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, w, h), False)
sheet.set_rect(sheet.irect, (24, 24, 28))
x = 0
for c in cells:
    c.set_origin(x, 0)
    sheet.copy(c, c.irect)
    x += c.width
sheet.save(out)
print(out, w, h)
