import sys, os, glob
sys.stdout.reconfigure(encoding="utf8")
from PIL import Image, ImageDraw
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = T + "/🗑️generated/C2/qa14-r/"
O = T + "/🗑️generated/C2/qa14-m/"
asset, name = sys.argv[1], sys.argv[2]
scales = sys.argv[3].split(",")
ims = []
for sc in scales:
    f = R + asset + "/" + asset + "--" + name + "--views-dark-line-1-" + sc + ".png"
    if not os.path.exists(f):
        continue
    im = Image.open(f).convert("RGB")
    ImageDraw.Draw(im).text((3, 2), name + " 1:" + sc, fill=(255, 80, 80))
    ims.append(im)
w = max(i.width for i in ims)
M = Image.new("RGB", (w, sum(i.height for i in ims)), "white")
y = 0
for im in ims:
    M.paste(im, (0, y)); y += im.height
p = O + "s-%s-%s.png" % (asset, name)
M.save(p)
print(p, M.size)
