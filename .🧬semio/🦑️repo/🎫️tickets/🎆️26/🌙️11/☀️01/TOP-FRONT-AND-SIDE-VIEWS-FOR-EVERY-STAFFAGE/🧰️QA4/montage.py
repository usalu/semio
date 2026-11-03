import sys, glob, os
from PIL import Image
src, dst, per = sys.argv[1], sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 4
os.makedirs(dst, exist_ok=True)
for d in sorted(glob.glob(src + "/*/*/")):
    parts = d.replace("\\", "/").rstrip("/").split("/")
    asset, scale = parts[-2], parts[-1]
    files = sorted(glob.glob(d + "*.png"))
    for i in range(0, len(files), per):
        ims = [Image.open(f).convert("RGB") for f in files[i:i + per]]
        w = max(im.width for im in ims)
        h = sum(im.height for im in ims)
        sheet = Image.new("RGB", (w, h), "white")
        y = 0
        for im in ims:
            sheet.paste(im, (0, y)); y += im.height
        sheet.save(f"{dst}/{asset}-{scale}-{i // per:02d}.png")
