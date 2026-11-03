"""V1: magnified crops: v1_crop.py <mp4> <png> <x0,y0,x1,y1> <scale> <t1,t2,...>"""
import sys

import av
from PIL import Image, ImageDraw

path, out = sys.argv[1], sys.argv[2]
box = tuple(int(v) for v in sys.argv[3].split(","))
scale = int(sys.argv[4])
times = [float(t) for t in sys.argv[5].split(",")]
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
tiles = []
for t in times:
    im = frames[min(len(frames) - 1, int(t * 15))].crop(box)
    im = im.resize((im.width * scale, im.height * scale), Image.LANCZOS)
    ImageDraw.Draw(im).text((4, 4), f"{t:.1f}s", fill=(255, 255, 0))
    tiles.append(im)
cols = min(3, len(tiles))
rows = (len(tiles) + cols - 1) // cols
w, h = tiles[0].size
sheet = Image.new("RGB", (cols * w, rows * h))
for i, im in enumerate(tiles):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(sheet.size)
