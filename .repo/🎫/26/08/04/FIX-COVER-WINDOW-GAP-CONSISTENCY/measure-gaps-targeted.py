#!/usr/bin/env python3
"""Targeted gap stats on known cover rows (3-up meta + 2-up title)."""
import fitz
import statistics

PDF = r"E:\semio\print\dist\zwischenbericht.pdf"
SCALE = 3
PT = 72 / (96 * SCALE)


def lum(r, g, b):
    return 0.299 * r + 0.587 * g + 0.114 * b


def is_border(rgb):
    return lum(*rgb) < 175


doc = fitz.open(PDF)
pix = doc[0].get_pixmap(matrix=fitz.Matrix(SCALE, SCALE))
w, h = pix.width, pix.height
data = pix.samples


def px(x, y):
    i = (y * w + x) * 3
    return data[i], data[i + 1], data[i + 2]


def clusters(points, tol=2):
    if not points:
        return []
    out = [[points[0]]]
    for p in points[1:]:
        if p - out[-1][-1] <= tol:
            out[-1].append(p)
        else:
            out.append([p])
    return [sum(c) / len(c) for c in out]


def inner_gaps_at_y(y):
    cols = clusters([x for x in range(w) if is_border(px(x, y))])
    gaps = [b - a for a, b in zip(cols, cols[1:])]
  # inter-window only: ignore page side margins
    return [g * PT for g in gaps if 2 < g < 20]


def inner_gaps_at_x(x):
    rows = clusters([y for y in range(h) if is_border(px(x, y))])
    gaps = [b - a for a, b in zip(rows, rows[1:])]
    return [g * PT for g in gaps if 2 < g < 20]


# Title row ~12%, meta row ~46%, logos ~86%
samples = {
    "title-row-h": inner_gaps_at_y(int(h * 0.12)),
    "meta-row-h": inner_gaps_at_y(int(h * 0.46)),
    "logos-row-h": inner_gaps_at_y(int(h * 0.86)),
    "column-v": inner_gaps_at_x(int(w * 0.35)),
}

for name, vals in samples.items():
    if vals:
        print(
            name,
            f"n={len(vals)}",
            f"min={min(vals):.2f}",
            f"max={max(vals):.2f}",
            f"median={statistics.median(vals):.2f}",
        )
