import sys, glob, os, re
from PIL import Image, ImageDraw
src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
files = sorted(glob.glob(os.path.join(src, "f_*.png")))
W, H = 640, 360
for s in range(0, len(files), 6):
    sheet = Image.new("RGB", (W * 3, H * 2))
    for k, f in enumerate(files[s:s + 6]):
        im = Image.open(f).convert("RGB").resize((W, H))
        d = ImageDraw.Draw(im)
        t = int(re.search(r"f_(\d+)\.png", f).group(1))
        d.rectangle((W - 90, 2, W - 2, 20), fill=(0, 0, 0))
        d.text((W - 86, 4), f"{t // 60}:{t % 60:02d}", fill=(255, 255, 0))
        sheet.paste(im, ((k % 3) * W, (k // 3) * H))
    sheet.save(os.path.join(dst, f"s_{s // 6:03d}.png"))
