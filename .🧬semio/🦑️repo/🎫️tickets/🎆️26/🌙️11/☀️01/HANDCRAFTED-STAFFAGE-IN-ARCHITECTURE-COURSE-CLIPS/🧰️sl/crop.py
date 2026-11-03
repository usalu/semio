"""🔍 crop.py <mp4> <out.png> <scale> <t> <x0,y0,x1,y1> [<t> <box> ...]: crops (frame pixels) of frames at seconds, upscaled and stacked horizontally."""
import sys

import av
from PIL import Image

path, out, scale = sys.argv[1], sys.argv[2], int(sys.argv[3])
with av.open(path) as container:
    frames = [f.to_image() for f in container.decode(video=0)]
tiles = []
for i in range(4, len(sys.argv), 2):
    t = float(sys.argv[i])
    box = tuple(int(v) for v in sys.argv[i + 1].split(","))
    im = frames[min(len(frames) - 1, int(t * 15))].crop(box)
    tiles.append(im.resize((im.width * scale, im.height * scale), Image.LANCZOS))
sheet = Image.new("RGB", (sum(t.width for t in tiles), max(t.height for t in tiles)))
x = 0
for t in tiles:
    sheet.paste(t, (x, 0))
    x += t.width
sheet.save(out)
print(sheet.size)
