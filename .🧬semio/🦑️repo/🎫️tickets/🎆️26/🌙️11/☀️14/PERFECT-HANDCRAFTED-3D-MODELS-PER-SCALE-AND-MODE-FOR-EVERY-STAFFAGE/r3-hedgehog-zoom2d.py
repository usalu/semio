"""🔍 Round 3 hedgehog: enlarges the product's own 2D SVG panels of r3-2d.ts (written with --svg) around the drawn animal so that a 6 mm drawing can be judged: rewrites the viewBox of a copy and lets Quick Look rasterise it.

Usage: python3 r3-hedgehog-zoom2d.py <scale> [suffix]   (after bun r3-2d.ts animals/hedgehog --scale N --svg [--pose P]); writes 🗑️generated/r3-hedgehog/zoom2d/*.png
"""
import glob, os, re, subprocess, sys
scale = sys.argv[1]
suffix = sys.argv[2] if len(sys.argv) > 2 else ""
out = "🗑️generated/r3-hedgehog/zoom2d"
os.makedirs(out, exist_ok=True)
for f in sorted(glob.glob("🗑️generated/r3-2d/animals/hedgehog.s%s%s.*.svg" % (scale, suffix))):
    text = open(f, encoding="utf-8").read()
    line = re.search(r'id="ground-line" d="M([\d.]+) ([\d.]+)', text)
    gx, gy = (float(line.group(1)), float(line.group(2))) if line else (5.5, 12.0)
    span = 10.0 * 50 / float(scale)
    box = "%.3f %.3f %.3f %.3f" % (gx + 4.0 * 50 / float(scale), gy - span * 0.7, span, span)
    text = re.sub(r'width="[\d.]+mm" height="[\d.]+mm" viewBox="[^"]+"', 'width="1600" height="1600" viewBox="%s"' % box, text, count=1)
    text = text.replace("<title>", '<rect x="-100" y="-100" width="400" height="400" fill="#f5f1e1"/><title>', 1)
    name = os.path.join(out, os.path.basename(f))
    open(name, "w", encoding="utf-8").write(text)
    subprocess.run(["qlmanage", "-t", "-s", "1600", "-o", out, name], capture_output=True)
    print(name + ".png")
