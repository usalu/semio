#!/usr/bin/env python3
"""usage: look2.py <category> <asset> <scale> <poses|all|clip:id@t,...> [views=light-line,light-solid] [pxwidth=2000] [name]
Composes a tight grid (rows = poses, cols = views) from individual preview renders."""
import re, subprocess, sys, json
cat, asset, scale, poses = sys.argv[1:5]
views = (sys.argv[5] if len(sys.argv) > 5 else "light-line,light-solid").split(",")
px = sys.argv[6] if len(sys.argv) > 6 else "2000"
name = sys.argv[7] if len(sys.argv) > 7 else f"{asset}--{scale}--grid"
R = "/home/deck/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
out = f"{T}/🗑️generated/refine-ov/tmp"
folder = {"objects": "🪑️objects", "components": "🧱️components", "vegetation": "🌳️vegetation", "vehicles": "🚜️vehicles"}[cat]
file = f"{S}/🗂️catalog/{folder}/🔣️.json"
doc = json.load(open(file))
a = next(x for x in doc["assets"] if x["id"] == asset)
items = []
if poses == "all":
    items = [("pose", p["id"], None) for p in a["poses"] if not p.get("key")]
else:
    for p in poses.split(","):
        if p.startswith("clip:"):
            cid, t = p[5:].split("@"); items.append(("clip", cid, t))
        else: items.append(("pose", p, None))
cells = []
for kind, ident, t in items:
    row = []
    for v in views:
        args = ["repo", "tools", "exec", "bun", "--", f"{S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts", "--file", file, "--asset", asset, "--view", v, "--scale", scale, "--out", out, "--no-png"]
        args += ["--pose", ident] if kind == "pose" else ["--clip", ident, "--time", t]
        r = subprocess.run(args, cwd=R, capture_output=True, text=True)
        if r.returncode: print(r.stdout, r.stderr); sys.exit(1)
        path = [l for l in r.stdout.splitlines() if l.endswith(".svg")][-1]
        if not path.startswith("/"): path = f"{R}/{path}"
        s = open(path).read()
        m = re.search(r'width="([\d.]+)mm" height="([\d.]+)mm" viewBox="([^"]+)"', s)
        row.append((float(m.group(1)), float(m.group(2)), s, m))
    cells.append(row)
colw = [max(r[c][0] for r in cells) for c in range(len(views))]
rowh = [max(c[1] for c in r) for r in cells]
W, H = sum(colw) + 2 * len(views), sum(rowh) + 2 * len(cells)
body = ""; y = 0
for r, row in enumerate(cells):
    x = 0
    for c, (w, h, s, m) in enumerate(row):
        inner = s[s.index(">", s.index("<svg")) + 1: s.rindex("</svg>")]
        body += f'<svg x="{x}" y="{y}" width="{w}" height="{h}" viewBox="{m.group(3)}">{inner}</svg>'
        x += colw[c] + 2
    y += rowh[r] + 2
svg = f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns:staffage="https://semio.tech/staffage/v1" width="{W}mm" height="{H}mm" viewBox="0 0 {W} {H}"><rect width="{W}" height="{H}" fill="#888"/>{body}</svg>'
p = f"{T}/🗑️generated/refine-ov/{name}"
import os
if os.environ.get("CROP"):
    fx, fy, fw, fh = [float(v) for v in os.environ["CROP"].split(",")]
    svg = svg.replace(f'width="{W}mm" height="{H}mm" viewBox="0 0 {W} {H}"', f'width="{W*fw}mm" height="{H*fh}mm" viewBox="{W*fx} {H*fy} {W*fw} {H*fh}"')
open(p + ".svg", "w").write(svg)
subprocess.run(["rsvg-convert", "-w", px, "-f", "png", "-o", p + ".png", p + ".svg"], check=True)
print(p + ".png", round(W), "x", round(H), "mm")
