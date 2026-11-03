"""Renders states-<scale>.json (written by probe.ts) as sheets: one row per pose or clip instant, columns side | front | plan.
usage: python render.py <asset> <scale> [filter] [ppm]"""
import json
import sys

from PIL import Image, ImageDraw

OUT = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H4"
aid, sc = sys.argv[1], sys.argv[2]
flt = sys.argv[3] if len(sys.argv) > 3 and sys.argv[3] != "-" else ""
ppm = float(sys.argv[4]) if len(sys.argv) > 4 else 220
states = [s for s in json.load(open(f"{OUT}\\{aid}\\states-{sc}.json", encoding="utf-8")) if flt in s["name"]]


def bounds(shapes):
    xs = [p[0] for s in shapes for r in s["rings"] for p in r]
    ys = [p[1] for s in shapes for r in s["rings"] for p in r]
    return (min(xs), max(xs), min(ys), max(ys)) if xs else (0, 1, 0, 1)


allb = {v: [bounds(s["views"][v]) for s in states] for v in ("elevation", "front", "plan")}
ext = {}
for v in allb:
    ext[v] = (min(b[0] for b in allb[v]), max(b[1] for b in allb[v]), min(b[2] for b in allb[v]), max(b[3] for b in allb[v]))
pad = 0.08
cw = {v: (ext[v][1] - ext[v][0] + 2 * pad) * ppm for v in ext}
hh = max((ext[v][3] - ext[v][2] + 2 * pad) * ppm for v in ext)
cols = 3 if len(states) > 6 else 2
rows_per = 3
per = cols * rows_per if False else 3


def cell(draw, ox, oy, shapes, e, v):
    w = (e[1] - e[0] + 2 * pad) * ppm
    h = (e[3] - e[2] + 2 * pad) * ppm
    def tp(p):
        return (ox + (p[0] - e[0] + pad) * ppm, oy + hh - (p[1] - e[2] + pad) * ppm)
    draw.rectangle([ox, oy, ox + w, oy + hh], outline=(210, 210, 210))
    gy = oy + hh - (0 - e[2] + pad) * ppm
    for s in sorted(shapes, key=lambda s: s["z"]):
        g = int(255 * (1 - min(max(s["tone"], 0), 1) * 0.85)) if s["filled"] else 0
        for r in s["rings"]:
            q = [tp(p) for p in r]
            if len(q) < 2:
                continue
            if s["filled"] and s["closed"] and len(q) > 2:
                draw.polygon(q, fill=(g, g, g), outline=(0, 0, 0))
            else:
                draw.line(q + ([q[0]] if s["closed"] else []), fill=(90, 90, 90), width=1)
    return w


chunks = [states[i:i + 4] for i in range(0, len(states), 4)]
for ci, ch in enumerate(chunks):
    W = int(sum(cw.values())) + 20
    img = Image.new("RGB", (W, int(hh) * len(ch) + 4), "white")
    d = ImageDraw.Draw(img)
    for ri, st in enumerate(ch):
        x = 0
        for v in ("elevation", "front", "plan"):
            cell(d, x, ri * hh, st["views"][v], ext[v], v)
            x += cw[v] + 10
        d.text((4, ri * hh + 3), st["name"], fill=(200, 0, 0))
    img.save(f"{OUT}\\{aid}\\sheet-{sc}-{ci}.png")
print(len(chunks), "sheets")
