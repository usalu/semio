"""🧩️ Joins the SVG drawings that svg.ts wrote for one scale (side, front, plan; one row per drawing mode) into one sheet SVG on a paper-coloured ground: sheet.py <folder> <scale> <out.svg> [modes]"""
import os, re, sys
folder, scale, out = sys.argv[1:4]
modes = (sys.argv[4] if len(sys.argv) > 4 else "light-line,light-solid").split(",")
cells, widths, heights = [], [0, 0, 0], []
for mode in modes:
    row, h = [], 0
    for k, view in enumerate(("elevation", "front", "plan")):
        svg = open(f"{folder}/{scale}-{mode}-{view}.svg").read()
        w0, h0 = (float(x) for x in re.search(r'viewBox="0 0 ([\d.]+) ([\d.]+)"', svg).groups())
        body = svg[svg.index("<svg"):]
        row.append((body, w0, h0))
        widths[k], h = max(widths[k], w0), max(h, h0)
    cells.append(row)
    heights.append(h)
gap = 4
W, H = sum(widths) + 4 * gap, sum(heights) + (len(modes) + 1) * gap
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}mm" height="{H}mm" viewBox="0 0 {W} {H}"><rect width="{W}" height="{H}" fill="#f4f0e2"/>']
y = gap
for row, h in zip(cells, heights):
    x = gap
    for k, (body, w0, h0) in enumerate(row):
        parts.append(re.sub(r"<svg ", f'<svg x="{x}" y="{y}" ', re.sub(r' width="[\d.]+mm" height="[\d.]+mm"', f' width="{w0}" height="{h0}"', body, 1), 1))
        x += widths[k] + gap
    y += h + gap
parts.append("</svg>")
open(out, "w").write("\n".join(parts))
print(out)
