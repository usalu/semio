import sys, glob, os
from PIL import Image, ImageDraw
d = sys.argv[1]
fs = sorted(glob.glob(os.path.join(d, "f", "c_*.png")))
per = 6
for s in range(0, len(fs), per):
    W, H = 800, 450
    im = Image.new("RGB", (W * 2, H * 3), "black")
    dr = ImageDraw.Draw(im)
    for k, f in enumerate(fs[s:s + per]):
        i = int(os.path.basename(f)[2:6])
        fr = Image.open(f).convert("RGB").resize((W, H))
        x, y = (k % 2) * W, (k // 2) * H
        im.paste(fr, (x, y))
        dr.rectangle([x, y, x + 70, y + 16], fill="red")
        dr.text((x + 3, y + 2), f"{i*4}s", fill="white")
    im.save(os.path.join(d, f"sheet_{s//per:02d}.png"))
