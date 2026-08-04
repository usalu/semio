import fitz
from PIL import Image
import numpy as np

d = fitz.open(
    r"e:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht-dark.pdf"
)
out = r"e:\semio\.repo\🎫\26\08\04\FIX-TOC-OVERLAPPING-LINES"

for i, label in [(51, "bb-m"), (20, "pk"), (95, "gloss"), (89, "req")]:
    p = d[i]
    longs = []
    for dr in p.get_drawings():
        r = dr.get("rect")
        if not r or r.height >= 2 or r.width < 300:
            continue
        c = dr.get("color")
        if not c:
            continue
        if tuple(round(x, 3) for x in c) != (0.482, 0.51, 0.49):
            continue
        if 100 < r.y0 < 700:
            longs.append(r)
    if not longs:
        print(label, "no longs")
        continue
    x1 = max(r.x1 for r in longs)
    y0 = min(r.y0 for r in longs)
    y1 = min(max(r.y1 for r in longs), y0 + 220)
    clip = fitz.Rect(x1 - 40, y0 - 2, x1 + 8, y1 + 2)
    pix = p.get_pixmap(matrix=fitz.Matrix(5, 5), clip=clip)
    path = f"{out}\\{label}-p{i+1}-right.png"
    pix.save(path)
    a = np.array(Image.open(path))
    g = a.mean(axis=2)
    # annotate gaps
    from PIL import ImageDraw

    im = Image.open(path).convert("RGB")
    draw = ImageDraw.Draw(im)
    gap_rows = 0
    for y in range(g.shape[0]):
        row = g[y]
        b = np.where(row > 95)[0]
        if len(b) < 20:
            continue
        gaps = np.diff(b)
        if gaps.max() > 4:
            gap_rows += 1
            gi = int(np.argmax(gaps))
            draw.line([(int(b[gi]), y), (int(b[gi + 1]), y)], fill=(255, 0, 0), width=2)
    im.save(f"{out}\\{label}-p{i+1}-right-ann.png")
    print(label, f"p{i+1}", "clip", tuple(clip), "gap_rows", gap_rows)

    # vector shorts near right edge
    shorts = []
    for dr in p.get_drawings():
        r = dr.get("rect")
        if not r or r.height >= 2.5 or r.width < 1 or r.width > 80:
            continue
        c = dr.get("color")
        if not c:
            continue
        if tuple(round(x, 3) for x in c) != (0.482, 0.51, 0.49):
            continue
        if abs(r.x1 - x1) < 15 and 100 < r.y0 < 700:
            shorts.append(
                (round(r.y0, 1), round(r.x0, 1), round(r.x1, 1), round(r.width, 2))
            )
    print("  vector shorts near right:", len(shorts), shorts[:8])
