"""🖼️ Compares the before and after sheets of an asset: `python3 sheetdiff.py <sheets-dir> <id>` prints per scale and pose the number of differing pixels and their bounding box, and writes `<id>.s<scale>.<pose>.diff.png` (the after sheet cropped to the three line panels, differing pixels red) for the sheets that differ."""
import sys, glob, os
from PIL import Image, ImageChops
d, a = sys.argv[1], sys.argv[2]
for before in sorted(glob.glob(os.path.join(d, a + ".s*.before.png"))):
    after = before.replace(".before.png", ".after.png")
    if not os.path.exists(after): continue
    x, y = Image.open(before).convert("RGB"), Image.open(after).convert("RGB")
    if x.size != y.size: print(os.path.basename(after), "sizes differ", x.size, y.size); continue
    diff = ImageChops.difference(x, y).convert("L").point(lambda v: 255 if v > 24 else 0)
    n = sum(1 for v in diff.getdata() if v)
    print(os.path.basename(after), "differing pixels", n, "bbox", diff.getbbox())
    if n:
        red = Image.new("RGB", y.size, (255, 0, 0))
        out = Image.composite(red, y, diff).crop((0, 0, y.size[0], y.size[1] // 2))
        out.save(after.replace(".after.png", ".diff.png"))
