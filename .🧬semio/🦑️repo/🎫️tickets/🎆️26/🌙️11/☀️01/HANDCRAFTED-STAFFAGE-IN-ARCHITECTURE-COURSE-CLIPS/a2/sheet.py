import sys, glob, os
import fitz

root = sys.argv[1]
out = sys.argv[2]
names = sys.argv[3].split(",")
views = sys.argv[4].split(",")
cell = []
for n in names:
    row = []
    for v in views:
        files = sorted(glob.glob(os.path.join(root, f"{n}-{v}", "*.svg")))
        if files:
            idx = int(sys.argv[5]) if len(sys.argv) > 5 else 0
            row.append(files[min(idx, len(files) - 1)])
        else:
            row.append(None)
    cell.append(row)
zoom = float(sys.argv[6]) if len(sys.argv) > 6 else 1.0
pix = []
for row in cell:
    prow = []
    for f in row:
        if f is None:
            prow.append(None)
            continue
        d = fitz.open(f)
        p = d[0].get_pixmap(matrix=fitz.Matrix(zoom, zoom), alpha=False)
        prow.append(p)
    pix.append(prow)
w = max((p.width for r in pix for p in r if p), default=10)
h = max((p.height for r in pix for p in r if p), default=10)
W = w * len(views)
H = h * len(names)
sheet = fitz.Pixmap(fitz.csRGB, fitz.IRect(0, 0, W, H), False)
sheet.set_rect(sheet.irect, (128, 128, 128))
for i, r in enumerate(pix):
    for j, p in enumerate(r):
        if p:
            p.set_origin(j * w, i * h)
            sheet.copy(p, p.irect)
sheet.save(out)
print(out, W, H)
