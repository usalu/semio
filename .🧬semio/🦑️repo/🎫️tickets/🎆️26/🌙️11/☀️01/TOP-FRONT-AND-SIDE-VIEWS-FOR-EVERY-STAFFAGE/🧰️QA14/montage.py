import sys, os, glob, re
sys.stdout.reconfigure(encoding="utf8")
import pymupdf
from PIL import Image, ImageDraw
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
D = T + "/🗑️generated/C2/qa14/"
OUT = T + "/🗑️generated/C2/qa14-m/"
os.makedirs(OUT, exist_ok=True)
asset, scale = sys.argv[1], sys.argv[2]
dpi = int(sys.argv[3]) if len(sys.argv) > 3 else 150
cols = int(sys.argv[4]) if len(sys.argv) > 4 else 2
pat = re.compile(r"views-dark-line-1-%s\.svg$" % scale)
files = sorted(f for f in glob.glob(D + asset + "/*.svg") if pat.search(f))
ims = []
for f in files:
    pm = pymupdf.open(f)[0].get_pixmap(dpi=dpi, alpha=False)
    im = Image.frombytes("RGB", (pm.width, pm.height), pm.samples)
    lab = os.path.basename(f).replace(asset + "--", "").split("--views")[0]
    d = ImageDraw.Draw(im); d.text((4, 2), lab, fill=(200, 0, 0))
    ims.append(im)
if not ims: sys.exit("none")
w = max(i.width for i in ims); h = max(i.height for i in ims)
rows = (len(ims) + cols - 1) // cols
M = Image.new("RGB", (w * cols, h * rows), "white")
for k, im in enumerate(ims): M.paste(im, ((k % cols) * w, (k // cols) * h))
p = OUT + asset + "-" + scale + ".png"
M.save(p); print(p, M.size, len(ims))
