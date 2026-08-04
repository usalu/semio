#!/usr/bin/env python3
"""Measure border-to-border gaps between cover windows on page 1."""
import fitz
import statistics

PDF = r"E:\semio\print\dist\zwischenbericht.pdf"
SCALE = 3
PT_PER_PX = 72 / (96 * SCALE)  # 3x pixmap: px -> pt


def luminance(r, g, b):
    return 0.299 * r + 0.587 * g + 0.114 * b


def is_border(r, g, b):
    return luminance(r, g, b) < 175


doc = fitz.open(PDF)
page = doc[0]
pix = page.get_pixmap(matrix=fitz.Matrix(SCALE, SCALE))
w, h = pix.width, pix.height
data = pix.samples


def px(x, y):
    i = (y * w + x) * 3
    return data[i], data[i + 1], data[i + 2]


def border_cols(y):
    return [x for x in range(w) if is_border(*px(x, y))]


def border_rows(x):
    return [y for y in range(h) if is_border(*px(x, y))]


def cluster(points, max_gap=2):
    if not points:
        return []
    clusters = [[points[0]]]
    for p in points[1:]:
        if p - clusters[-1][-1] <= max_gap:
            clusters[-1].append(p)
        else:
            clusters.append([p])
    return [sum(c) / len(c) for c in clusters]


def gaps_between_clusters(clusters):
    out = []
    for a, b in zip(clusters, clusters[1:]):
        out.append(b - a)
    return out


# Horizontal gaps: scan through title/body bands
h_gaps = []
for y in range(int(h * 0.10), int(h * 0.92), 8):
    cols = cluster(border_cols(y))
    if len(cols) >= 4:
        for g in gaps_between_clusters(cols):
            if 2 < g < 40:
                h_gaps.append(g * PT_PER_PX)

# Vertical gaps: scan center-ish columns
v_gaps = []
for x in range(int(w * 0.20), int(w * 0.80), 40):
    rows = cluster(border_rows(x))
    if len(rows) >= 4:
        for g in gaps_between_clusters(rows):
            if 2 < g < 40:
                v_gaps.append(g * PT_PER_PX)

print("PDF:", PDF)
if h_gaps:
    print(
        "Horizontal border gaps (pt):",
        f"n={len(h_gaps)}",
        f"min={min(h_gaps):.2f}",
        f"max={max(h_gaps):.2f}",
        f"median={statistics.median(h_gaps):.2f}",
    )
if v_gaps:
    print(
        "Vertical border gaps (pt):",
        f"n={len(v_gaps)}",
        f"min={min(v_gaps):.2f}",
        f"max={max(v_gaps):.2f}",
        f"median={statistics.median(v_gaps):.2f}",
    )
