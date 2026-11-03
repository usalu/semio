"""Rasterise SVG files to PNG with PyMuPDF (rsvg-convert is not installed here).

usage: python raster.py height_px out.png in1.svg in2.svg ...   (each svg is scaled to the same pixel height, tiled in one row on grey)
"""
import sys
import fitz

height = float(sys.argv[1])
out = sys.argv[2]
files = sys.argv[3:]
pix = []
for f in files:
    page = fitz.open(f)[0]
    z = height / page.rect.height
    pix.append(page.get_pixmap(matrix=fitz.Matrix(z, z), alpha=True))
gap = 8
w = sum(p.width for p in pix) + gap * (len(pix) + 1)
h = max(p.height for p in pix) + 2 * gap
sheet = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, w, h), False)
sheet.set_rect(sheet.irect, (180, 180, 180))
x = gap
for p in pix:
    bg = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, p.width, p.height), False)
    bg.set_rect(bg.irect, (16, 24, 30))
    flat = fitz.Pixmap(fitz.csRGB, p.irect, False)
    flat.set_rect(flat.irect, (16, 24, 30))
    flat.set_origin(0, 0)
    p2 = fitz.Pixmap(p, 0) if p.alpha else p
    sheet.copy(bg, fitz.IRect(0, 0, p.width, p.height)) if False else None
    bg.copy(p2, p2.irect) if not p.alpha else None
    final = bg if not p.alpha else None
    if p.alpha:
        # composite alpha over paper colour
        import numpy as np
        arr = np.frombuffer(p.samples, dtype=np.uint8).reshape(p.height, p.width, 4).astype(float)
        a = arr[:, :, 3:4] / 255.0
        paper = np.array([16, 24, 30], dtype=float)
        rgb = arr[:, :, :3] * a + paper * (1 - a)
        final = fitz.Pixmap(fitz.csRGB, p.width, p.height, rgb.astype(np.uint8).tobytes(), False)
    final.set_origin(x, gap)
    sheet.copy(final, fitz.IRect(x, gap, x + p.width, gap + p.height))
    x += p.width + gap
sheet.save(out)
print(w, h)
