"""Pixel-check: does the horizontal rule extend past the vertical border?"""
import fitz
import numpy as np
from PIL import Image

PDF = r"e:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht-dark.pdf"
OUT = r"e:\semio\.repo\🎫\26\08\04\FIX-TOC-OVERLAPPING-LINES"
d = fitz.open(PDF)

for page_i, label, clip in [
    (2, "toc", fitz.Rect(540, 100, 560, 200)),
    (48, "huerden", fitz.Rect(490, 160, 510, 240)),
    (51, "bb", fitz.Rect(510, 120, 535, 200)),
]:
    p = d[page_i]
    pix = p.get_pixmap(matrix=fitz.Matrix(8, 8), clip=clip)
    path = f"{OUT}\\pixel-{label}.png"
    pix.save(path)
    a = np.array(Image.open(path).convert("RGB"))
    g = a.mean(axis=2)
    # Find vertical border column: max column mean of mid-gray
    col_score = []
    for x in range(a.shape[1]):
        col = g[:, x]
        col_score.append(((col > 90) & (col < 180)).sum())
    vx = int(np.argmax(col_score))
    # For each row that looks like a horizontal rule, check pixels right of vx
    overhang = 0
    shortfall = 0
    rule_rows = 0
    for y in range(a.shape[0]):
        row = g[y]
        if (row > 90).sum() < a.shape[1] * 0.35:
            continue
        rule_rows += 1
        # bright past vertical?
        right = row[vx + 2 : vx + 12]
        left_near = row[max(0, vx - 12) : max(0, vx - 2)]
        if len(right) and right.mean() > 90 and right.max() > 110:
            overhang += 1
        if len(left_near) and left_near.mean() < 50:
            shortfall += 1
    print(
        f"{label}: vert_x={vx}px rule_rows={rule_rows} "
        f"overhang_rows={overhang} shortfall_rows={shortfall} "
        f"size={a.shape[1]}x{a.shape[0]}"
    )
