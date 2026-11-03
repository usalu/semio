import sys, os, glob, re
sys.stdout.reconfigure(encoding="utf8")
from PIL import Image, ImageDraw
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = T + "/🗑️generated/C2/qa14-r/"
O = T + "/🗑️generated/C2/qa14-m/"
os.makedirs(O, exist_ok=True)
asset, scale, cols = sys.argv[1], sys.argv[2], int(sys.argv[3])
sub = sys.argv[4:]
fs = sorted(f for f in glob.glob(R + asset + "/*.png") if f.endswith("-1-%s.png" % scale))
if sub:
    fs = [f for f in fs if any(s in os.path.basename(f) for s in sub)]
ims = []
for f in fs:
    im = Image.open(f).convert("RGB")
    d = ImageDraw.Draw(im)
    d.text((3, 2), os.path.basename(f).replace(asset + "--", "").split("--views")[0], fill=(255, 80, 80))
    ims.append(im)
w = max(i.width for i in ims); h = max(i.height for i in ims)
rows = (len(ims) + cols - 1) // cols
M = Image.new("RGB", (w * cols, h * rows), "white")
for k, im in enumerate(ims):
    M.paste(im, ((k % cols) * w, (k // cols) * h))
p = O + "m-%s-%s%s.png" % (asset, scale, ("-" + sub[0]) if sub else "")
M.save(p)
print(p, M.size, len(ims))
