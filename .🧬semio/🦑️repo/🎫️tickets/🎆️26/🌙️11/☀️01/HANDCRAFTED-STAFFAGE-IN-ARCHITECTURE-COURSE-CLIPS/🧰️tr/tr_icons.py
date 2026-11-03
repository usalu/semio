"""🔎 tr_icons.py <mp4> <out.png> <x0,y0,x1,y1> <fraction,...>: crops one region at several times onto a 4x sheet."""
import sys

import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
box = tuple(int(v) for v in sys.argv[3].split(","))
fractions = [float(v) for v in sys.argv[4].split(",")]
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
crops = [frames[int(f * (len(frames) - 1))].crop(box).resize(((box[2] - box[0]) * 4, (box[3] - box[1]) * 4), Image.LANCZOS) for f in fractions]
sheet = Image.new("RGB", (crops[0].width, crops[0].height * len(crops)))
for i, crop in enumerate(crops):
    sheet.paste(crop, (0, i * crop.height))
sheet.save(out)
