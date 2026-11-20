"""🔍 Round 3 cooking pot: enlarges the product's own 2D SVG panels of r3-2d.ts (written with --svg) around the drawn pot, so that a 5 mm drawing can be judged: rewrites the viewBox of a copy around the origin stated in the SVG metadata and lets Quick Look rasterise it.

Usage: python3 r3-cooking-pot-zoom2d.py <scale>   (after bun r3-2d.ts objects/cooking-pot --scale N --svg); writes 🗑️generated/r3-cooking-pot/zoom2d/*.png
"""
import glob, os, re, subprocess, sys
scale = float(sys.argv[1])
out = "🗑️generated/r3-cooking-pot/zoom2d"
os.makedirs(out, exist_ok=True)
for f in sorted(glob.glob("🗑️generated/r3-2d/objects/cooking-pot.s%d.*.svg" % scale)):
    text = open(f, encoding="utf-8").read()
    ox = float(re.search(r'origin-x-mm="([\d.-]+)"', text).group(1))
    oy = float(re.search(r'origin-y-mm="([\d.-]+)"', text).group(1))
    k = 1000 / scale
    w = h = 0.3 * k
    cy = oy if ".plan." in f else oy - 0.05 * k
    box = "%.3f %.3f %.3f %.3f" % (ox - w / 2, cy - h / 2, w, h)
    text = re.sub(r'width="[\d.]+mm" height="[\d.]+mm" viewBox="[^"]+"', 'width="1500" height="1500" viewBox="%s"' % box, text, count=1)
    name = os.path.join(out, os.path.basename(f))
    open(name, "w", encoding="utf-8").write(text)
    subprocess.run(["qlmanage", "-t", "-s", "1500", "-o", out, name], capture_output=True)
    print(name + ".png")
