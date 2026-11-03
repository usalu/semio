"""rp1: crop.py <mp4> <out.png> <x0,y0,x1,y1 in 854x480 frame> <t1,t2,...> — zoomed crops of frames side by side."""
import sys

import av
from PIL import Image, ImageDraw

path, out, box = sys.argv[1], sys.argv[2], [int(v) for v in sys.argv[3].split(",")]
times = [float(t) for t in sys.argv[4].split(",")]
with av.open(path) as container:
    frames = [f.to_image() for f in container.decode(video=0)]
w, h = frames[0].size
sx = w / 854
crops = []
for t in times:
    im = frames[min(len(frames) - 1, int(t * 15))]
    c = im.crop(tuple(int(v * sx) for v in box))
    c = c.resize((c.width * 2, c.height * 2), Image.LANCZOS)
    ImageDraw.Draw(c).text((4, 4), f"{t:.1f}s", fill=(255, 255, 0))
    crops.append(c)
cols = min(3, len(crops))
rows = (len(crops) + cols - 1) // cols
sheet = Image.new("RGB", (cols * crops[0].width, rows * crops[0].height))
for i, c in enumerate(crops):
    sheet.paste(c, ((i % cols) * c.width, (i // cols) * c.height))
sheet.save(out)
print(sheet.size, w, h)
