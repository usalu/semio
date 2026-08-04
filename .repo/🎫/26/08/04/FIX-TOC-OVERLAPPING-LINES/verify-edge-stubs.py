"""Verify table row rules no longer overshoot side borders."""
import fitz
from collections import defaultdict

PDF = r"e:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht-dark.pdf"
BORDER = (0.482, 0.51, 0.49)
OUT = r"e:\semio\.repo\🎫\26\08\04\FIX-TOC-OVERLAPPING-LINES"

d = fitz.open(PDF)


def analyze(page_index, label):
    p = d[page_index]
    longs = []
    verts = []
    for dr in p.get_drawings():
        r = dr.get("rect")
        if not r:
            continue
        c = dr.get("color")
        if not c:
            continue
        if tuple(round(x, 3) for x in c) != BORDER:
            continue
        if 80 < r.y0 < 700 and r.height < 2.5 and r.width > 200:
            longs.append(r)
        if 80 < r.y0 < 700 and r.width < 2 and r.height > 10:
            verts.append(r)
    if not longs:
        print(f"{label}: no long rules")
        return
    x1_rule = max(r.x1 for r in longs)
    x0_rule = min(r.x0 for r in longs)
    right_verts = [v for v in verts if v.x0 > (x0_rule + x1_rule) / 2]
    left_verts = [v for v in verts if v.x0 < (x0_rule + x1_rule) / 2]
    overshoot_right = 0
    overshoot_left = 0
    if right_verts:
        right_x = max(v.x0 for v in right_verts)
        # stubs: rule ends more than 0.2pt past the rightmost vertical
        for r in longs:
            if r.x1 - right_x > 0.2:
                overshoot_right += 1
    if left_verts:
        left_x = min(v.x0 for v in left_verts)
        for r in longs:
            if left_x - r.x0 > 0.2:
                overshoot_left += 1
    # short mid-page border horizontals (exclude chip-sized intentional tops)
    shorts = []
    for dr in p.get_drawings():
        r = dr.get("rect")
        if not r or r.height >= 2.5 or r.width < 2 or r.width > 100:
            continue
        c = dr.get("color")
        if not c:
            continue
        if tuple(round(x, 3) for x in c) != BORDER:
            continue
        if 100 < r.y0 < 700:
            shorts.append((round(r.y0, 1), round(r.x0, 1), round(r.x1, 1), round(r.width, 1)))
    print(
        f"{label} p{page_index+1}: longs={len(longs)} "
        f"rule_x=[{x0_rule:.2f},{x1_rule:.2f}] "
        f"overshoot_R={overshoot_right} overshoot_L={overshoot_left} "
        f"mid_shorts<100={len(shorts)}"
    )
    # crop right join
    y0 = min(r.y0 for r in longs[:8])
    y1 = min(max(r.y1 for r in longs[:8]) + 80, y0 + 120)
    clip = fitz.Rect(x1_rule - 25, y0 - 2, x1_rule + 10, y1)
    pix = p.get_pixmap(matrix=fitz.Matrix(6, 6), clip=clip)
    pix.save(f"{OUT}\\verify-{label}-right.png")


for idx, label in [
    (2, "toc"),
    (48, "huerden"),
    (20, "project"),
    (51, "bb"),
    (89, "req"),
    (95, "gloss"),
]:
    analyze(idx, label)
