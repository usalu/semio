#!/usr/bin/env python3
"""usage: look.py <category> <asset> <scale> [cols=2,3] [rows=all] [pxwidth=2200]
Renders sheet via preview CLI and writes cropped PNG <asset>--<scale>--crop.png into refine-ov."""
import re, subprocess, sys, os
cat, asset, scale = sys.argv[1:4]
cols = [int(c) for c in (sys.argv[4] if len(sys.argv) > 4 else "2,3").split(",")]
rows = sys.argv[5] if len(sys.argv) > 5 else "all"
px = sys.argv[6] if len(sys.argv) > 6 else "2200"
R = "/home/deck/semio"
S = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage"
T = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
out = f"{T}/🗑️generated/refine-ov"
folder = {"objects": "🪑️objects", "components": "🧱️components", "vegetation": "🌳️vegetation", "vehicles": "🚜️vehicles"}[cat]
subprocess.run(["repo", "tools", "exec", "bun", "--", f"{S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts", "--file", f"{S}/🗂️catalog/{folder}/🔣️.json", "--asset", asset, "--sheet", "--scale", scale, "--out", out, "--no-png"], cwd=R, check=True, capture_output=True)
svg = open(f"{out}/{asset}--sheet-1-{scale}.svg").read()
m = re.search(r'width="([\d.]+)mm" height="([\d.]+)mm" viewBox="0 0 ([\d.]+) ([\d.]+)"', svg)
W, H = float(m.group(3)), float(m.group(4))
labelW, head = 30, 8
cellW = (W - labelW) / 7
x0 = labelW + min(cols) * cellW
x1 = labelW + (max(cols) + 1) * cellW
# content-trim: keep full height
vb = f"{x0:.2f} 0 {x1 - x0:.2f} {H:.2f}"
crop = svg.replace(m.group(0), f'width="{x1 - x0:.2f}mm" height="{H:.2f}mm" viewBox="{vb}"')
cp = f"{out}/{asset}--{scale}--crop.svg"
open(cp, "w").write(crop)
subprocess.run(["rsvg-convert", "-w", px, "-f", "png", "-o", cp[:-4] + ".png", cp], check=True)
print(cp[:-4] + ".png", round(x1 - x0), "x", round(H), "mm")
