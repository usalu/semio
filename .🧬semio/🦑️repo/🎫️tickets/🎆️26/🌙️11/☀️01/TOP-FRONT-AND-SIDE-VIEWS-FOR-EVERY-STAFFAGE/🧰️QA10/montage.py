import sys, glob, os
from PIL import Image, ImageDraw
rnd, asset, scale = sys.argv[1], sys.argv[2], sys.argv[3]
per = int(sys.argv[4]) if len(sys.argv) > 4 else 5
base = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🗑️generated", "QA10")
files = sorted(glob.glob(os.path.join(base, rnd, asset, "*-1-" + scale + ".png")))
outdir = os.path.join(base, rnd, "_m")
os.makedirs(outdir, exist_ok=True)
for n in range(0, len(files), per):
    imgs = [Image.open(f).convert("RGB") for f in files[n:n + per]]
    w = max(i.width for i in imgs)
    h = sum(i.height + 14 for i in imgs)
    sheet = Image.new("RGB", (w, h), "white")
    d = ImageDraw.Draw(sheet)
    y = 0
    for f, i in zip(files[n:n + per], imgs):
        d.text((4, y + 1), os.path.basename(f).split("--views")[0], fill="red")
        sheet.paste(i, (0, y + 14))
        y += i.height + 14
    sheet.save(os.path.join(outdir, "%s-%s-%d.png" % (asset, scale, n // per)))
    print(asset, scale, n // per, len(imgs))
