import sys, os
import pymupdf as fitz

out, x0, y0, x1, y1, zoom = sys.argv[1], *map(float, sys.argv[2:7])
files = sys.argv[7:]
k = 72 / 25.4
pix = []
for f in files:
    page = fitz.open(f)[0]
    clip = fitz.Rect(x0 * k, y0 * k, x1 * k, y1 * k)
    pix.append(page.get_pixmap(matrix=fitz.Matrix(zoom, zoom), clip=clip, alpha=False))
w = max(p.width for p in pix)
h = max(p.height for p in pix)
cols = 2
rows = (len(pix) + 1) // 2
sheet = fitz.open()
page = sheet.new_page(width=cols * (w + 6), height=rows * (h + 6))
for i, p in enumerate(pix):
    x, y = (i % cols) * (w + 6), (i // cols) * (h + 6)
    page.insert_image(fitz.Rect(x, y, x + p.width, y + p.height), pixmap=p)
page.get_pixmap(alpha=False).save(out)
print(out)
