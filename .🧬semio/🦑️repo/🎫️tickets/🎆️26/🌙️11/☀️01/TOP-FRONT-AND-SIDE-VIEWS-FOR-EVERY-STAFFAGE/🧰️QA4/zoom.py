import sys, glob, os, fitz
from PIL import Image, ImageDraw
import numpy as np
src, dst, scale, per = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
H = int(sys.argv[5]) if len(sys.argv) > 5 else 320
os.makedirs(dst, exist_ok=True)
for d in sorted(glob.glob(f"{src}/*/{scale}/")):
    asset = d.replace("\\", "/").rstrip("/").split("/")[-2]
    strips = []
    for svg in sorted(glob.glob(d + "*.svg")):
        name = os.path.basename(svg).split("--views")[0].split("--", 1)[1]
        pix = fitz.open(svg)[0].get_pixmap(dpi=500)
        im = Image.frombytes("RGB", (pix.width, pix.height), pix.samples)
        a = np.asarray(im.convert("L"))
        w = im.width
        rgb = np.asarray(im)
        mask = (rgb[:, :, 0] < 12) & (rgb[:, :, 1] > 8) & (rgb[:, :, 1] < 30) & (rgb[:, :, 2] > 14) & (rgb[:, :, 2] < 40)
        cols = mask.sum(axis=0) > im.height * 0.02
        runs, start = [], None
        for x, v in enumerate(cols):
            if v and start is None: start = x
            if not v and start is not None:
                if x - start > 80: runs.append((start, x))
                start = None
        if start is not None: runs.append((start, len(cols)))
        cells = []
        for (x0, x1) in runs[:3]:
            rows = np.where(mask[:, x0:x1].sum(axis=1) > (x1 - x0) * 0.5)[0]
            if len(rows) == 0: continue
            y0, y1 = rows.min(), rows.max()
            hh = int((y1 - y0) * 0.46)
            xe = x0 + int((x1 - x0) * 0.6)
            panel = (rgb[y0:y0 + hh, x0:xe].sum(axis=2) > 450)
            ys, xs = np.where(panel)
            if len(xs) == 0: continue
            m = 6
            cells.append(im.crop((x0 + max(0, xs.min() - m), y0 + max(0, ys.min() - m), x0 + xs.max() + m, y0 + ys.max() + m)))
        if not cells: continue
        s = min(H / max(c.height for c in cells), 4.0)
        cells = [c.resize((max(1, int(c.width * s)), max(1, int(c.height * s))), Image.LANCZOS) for c in cells]
        row = Image.new("RGB", (sum(c.width for c in cells) + 40, H + 18), "white")
        x = 0
        for c in cells:
            row.paste(c, (x, 18 + H - c.height)); x += c.width + 20
        ImageDraw.Draw(row).text((2, 2), name, fill="black")
        strips.append(row)
    for i in range(0, len(strips), per):
        grp = strips[i:i + per]
        W = max(r.width for r in grp)
        out = Image.new("RGB", (W, sum(r.height for r in grp)), "white")
        y = 0
        for r in grp:
            out.paste(r, (0, y)); y += r.height
        out.save(f"{dst}/{asset}-{scale}-{i // per:02d}.png")
