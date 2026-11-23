import sys, os
from PIL import Image, ImageDraw
src, dst, start, end = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
os.makedirs(dst, exist_ok=True)
W, H = 480, 270
times = list(range(start, end, 5))
for s in range(0, len(times), 9):
    sheet = Image.new("RGB", (W * 3, H * 3))
    for k, t in enumerate(times[s:s + 9]):
        im = Image.open(os.path.join(src, f"f_{t:05d}.png")).convert("RGB").resize((W, H))
        d = ImageDraw.Draw(im)
        d.rectangle((W - 70, 2, W - 2, 16), fill=(0, 0, 0))
        d.text((W - 66, 3), f"{t // 60}:{t % 60:02d}", fill=(255, 255, 0))
        sheet.paste(im, ((k % 3) * W, (k // 3) * H))
    sheet.save(os.path.join(dst, f"n_{start}_{s // 9:02d}.png"))
