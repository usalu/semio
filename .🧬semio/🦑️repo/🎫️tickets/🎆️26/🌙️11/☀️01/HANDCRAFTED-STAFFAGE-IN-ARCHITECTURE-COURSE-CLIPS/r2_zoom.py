"""r2_zoom.py <mp4> <png> <x0,y0,x1,y1> <fraction,...> [zoom]: zoomed crop contact sheet."""
import sys

import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
box = tuple(int(v) for v in sys.argv[3].split(","))
fractions = [float(v) for v in sys.argv[4].split(",")]
zoom = int(sys.argv[5]) if len(sys.argv) > 5 else 2
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
crops = [frames[int(f * (len(frames) - 1))].crop(box) for f in fractions]
crops = [c.resize((c.width * zoom, c.height * zoom), Image.LANCZOS) for c in crops]
w, h = crops[0].size
cols = min(3, len(crops))
rows = (len(crops) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, im in enumerate(crops):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
