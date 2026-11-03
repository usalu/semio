"""🎞️ sheet.py <mp4> <outdir> <n> [t1,t2,...]: n evenly spaced frames (or given seconds) as sheets of 4 frames (2x2) plus single PNGs."""
import os
import sys

import av
from PIL import Image, ImageDraw

path, out, n = sys.argv[1], sys.argv[2], int(sys.argv[3])
os.makedirs(out, exist_ok=True)
with av.open(path) as container:
    frames = [f.to_image() for f in container.decode(video=0)]
name = os.path.splitext(os.path.basename(path))[0]
if len(sys.argv) > 4:
    idx = [min(len(frames) - 1, int(float(t) * 15)) for t in sys.argv[4].split(",")]
else:
    idx = [int(i * (len(frames) - 1) / (n - 1)) for i in range(n)]
w, h = frames[0].size
for s in range(0, len(idx), 4):
    chunk = idx[s:s + 4]
    sheet = Image.new("RGB", (2 * w, 2 * h))
    for i, k in enumerate(chunk):
        im = frames[k].copy()
        ImageDraw.Draw(im).text((4, 4), f"{k / 15:.1f}s", fill=(255, 255, 0))
        sheet.paste(im, ((i % 2) * w, (i // 2) * h))
    sheet.save(os.path.join(out, f"{name}_{s // 4}.png"))
print(name, len(frames), "frames", f"{len(frames) / 15:.1f}s", [round(k / 15, 1) for k in idx])
