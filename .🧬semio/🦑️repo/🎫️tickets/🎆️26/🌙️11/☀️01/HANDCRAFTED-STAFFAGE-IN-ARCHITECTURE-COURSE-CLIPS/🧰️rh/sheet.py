import sys

import av
from PIL import Image, ImageDraw

path, prefix = sys.argv[1], sys.argv[2]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 12
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
print(len(frames), "frames", len(frames) / 15, "s")
idx = sorted({min(len(frames) - 1, round(i * (len(frames) - 1) / (n - 1))) for i in range(n)})
w, h = frames[0].size
per = 4
for part in range(0, len(idx), per):
    chunk = idx[part:part + per]
    sheet = Image.new("RGB", (2 * w, 2 * h))
    for i, k in enumerate(chunk):
        im = frames[k].copy()
        ImageDraw.Draw(im).text((4, 4), f"{k / 15:.1f}s", fill=(255, 255, 0))
        sheet.paste(im, ((i % 2) * w, (i // 2) * h))
    sheet.save(f"{prefix}-{part // per}.png")
print(w, h, len(idx))
