"""F2: crop regions of mp4 frames and scale them up: f2_crop.py <mp4> <out.png> <x0,y0,x1,y1 in 1708-wide sheet-free frame fractions> <t1,t2,...> [zoom]"""
import sys

import av
from PIL import Image

path, out, box, times = sys.argv[1], sys.argv[2], [float(v) for v in sys.argv[3].split(",")], [float(t) for t in sys.argv[4].split(",")]
zoom = float(sys.argv[5]) if len(sys.argv) > 5 else 2.0
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
tiles = []
for t in times:
    im = frames[min(len(frames) - 1, int(t * 15))]
    w, h = im.size
    tile = im.crop((int(box[0] * w), int(box[1] * h), int(box[2] * w), int(box[3] * h)))
    tiles.append(tile.resize((int(tile.width * zoom), int(tile.height * zoom)), Image.LANCZOS))
sheet = Image.new("RGB", (sum(t.width for t in tiles), max(t.height for t in tiles)))
x = 0
for tile in tiles:
    sheet.paste(tile, (x, 0))
    x += tile.width
sheet.save(out)
print(sheet.size)
