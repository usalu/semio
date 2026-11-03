import sys, os, glob, fitz, numpy as np
from PIL import Image
# usage: zoom.py <sheetsdir> <asset-scale> <outprefix> <pxPerSheetCell> [filter] [per]
# rasterizes the real preview sheets large, crops the drawing part (top of each dark panel) of side/front/plan and stacks `per` instants per image
d, key, outp, cell, HM = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), float(sys.argv[7])
flt = sys.argv[5] if len(sys.argv) > 5 else ""
per = int(sys.argv[6])
WM = float(sys.argv[8]) if len(sys.argv) > 8 else 9.0
files = sorted(f for f in glob.glob(os.path.join(d, key, "*.svg")) if flt in os.path.basename(f))
rows = []
for f in files:
    page = fitz.open(f)[0]
    mmw = page.rect.width / 72 * 25.4
    scale = int(key.rsplit("-", 1)[1])
    dpi = max(60, int(cell / (HM * 1000 / scale) * 25.4))
    pm = page.get_pixmap(dpi=dpi)
    im = Image.frombytes("RGB", (pm.width, pm.height), pm.samples)
    a = np.asarray(im).astype(int)
    dark = (a.sum(axis=2) < 150)
    cols = dark.sum(axis=0) > im.height * 0.2
    spans, s = [], None
    for x, c in enumerate(cols):
        if c and s is None: s = x
        if not c and s is not None:
            if x - s > 40: spans.append((s, x))
            s = None
    if s is not None: spans.append((s, len(cols)))
    parts = []
    for (x0, x1) in spans[:3]:
        sub = dark[:, x0:x1]
        ys = np.where(sub.sum(axis=1) > (x1 - x0) * 0.5)[0]
        y0, y1 = (ys.min(), ys.max()) if len(ys) else (0, im.height)
        reg = a[y0:y0 + int((y1 - y0) * 0.5), x0:x1].sum(axis=2) > 400
        yy, xx = np.where(reg)
        mm = dpi / 25.4
        xmax = min(xx.max(), xx.min() + (WM * 1000 / scale + 9) * mm)
        ymax = min(yy.max(), yy.min() + (HM * 1000 / scale + 5) * mm)
        parts.append(im.crop((x0 + max(0, xx.min() - 8), y0 + max(0, yy.min() - 8), x0 + xmax + 8, y0 + ymax + 8)))
    label = os.path.basename(f).split("--")[1]
    H = max(p.height for p in parts); W = sum(p.width for p in parts) + 10 * len(parts)
    row = Image.new("RGB", (W, H + 22), (255, 255, 255)); x = 0
    from PIL import ImageDraw
    ImageDraw.Draw(row).text((4, 3), label, fill=(0, 0, 0))
    for p in parts:
        row.paste(p, (x, 22 + H - p.height)); x += p.width + 10
    rows.append(row)
for i in range(0, len(rows), per):
    g = rows[i:i + per]
    W = max(r.width for r in g); H = sum(r.height for r in g)
    out = Image.new("RGB", (W, H), (255, 255, 255)); y = 0
    for r in g:
        out.paste(r, (0, y)); y += r.height
    p = "%s-%d.png" % (outp, i // per); out.save(p); print(os.path.basename(p), out.size)
