import sys, glob, os
from PIL import Image, ImageDraw
src, dst, prefix = sys.argv[1], sys.argv[2], sys.argv[3]
os.makedirs(dst, exist_ok=True)
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
W, H = 640, 360
PER = 6
STEP = 5
for s in range(0, len(files), PER):
    sheet = Image.new("RGB", (W * 3, H * 2))
    for k, f in enumerate(files[s:s + PER]):
        im = Image.open(f).convert("RGB").resize((W, H))
        d = ImageDraw.Draw(im)
        t = (s + k) * STEP
        d.rectangle((W - 74, 2, W - 2, 20), fill=(0, 0, 0))
        d.text((W - 70, 4), f"{t // 60}:{t % 60:02d}", fill=(255, 255, 0))
        sheet.paste(im, ((k % 3) * W, (k // 3) * H))
    sheet.save(os.path.join(dst, f"{prefix}_{s // PER:02d}.png"))
