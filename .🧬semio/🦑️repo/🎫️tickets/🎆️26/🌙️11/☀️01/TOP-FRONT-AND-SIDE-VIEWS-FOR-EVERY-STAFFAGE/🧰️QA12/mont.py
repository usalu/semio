import glob, os, re
from collections import defaultdict
from PIL import Image, ImageDraw
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "QA12")
g = defaultdict(list)
for f in sorted(glob.glob(os.path.join(D, "prev", "*.png"))):
    n = os.path.basename(f)
    m = re.match(r"(.+?)--(.+?)--views-dark-line-1-(\d+)\.png", n)
    g[(m[1], m[3])].append((m[2], f))
for (a, sc), items in g.items():
    for k in range(0, len(items), 4):
        chunk = items[k:k + 4]
        ims = []
        for lab, f in chunk:
            im = Image.open(f).convert("RGB")
            w = 1700
            im = im.resize((w, int(im.height * w / im.width)))
            d = ImageDraw.Draw(im); d.text((6, 4), "%s 1:%s %s" % (a, sc, lab), fill=(200, 0, 0))
            ims.append(im)
        H = sum(i.height for i in ims)
        out = Image.new("RGB", (1700, H), "white")
        y = 0
        for i in ims:
            out.paste(i, (0, y)); y += i.height
        out.save(os.path.join(D, "mont", "%s-%s-%d.png" % (a, sc, k // 4)))
print(len(glob.glob(os.path.join(D, "mont", "*.png"))))
