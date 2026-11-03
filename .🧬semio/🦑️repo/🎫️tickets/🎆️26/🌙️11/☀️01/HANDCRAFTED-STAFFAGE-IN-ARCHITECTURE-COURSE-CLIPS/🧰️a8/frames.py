"""frames.py <mp4> <png> <box x0,y0,x1,y1> <fraction,fraction,...>: crop sheet of chosen relative positions."""
import sys

import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
box = tuple(int(v) for v in sys.argv[3].split(","))
fractions = [float(v) for v in sys.argv[4].split(",")]
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
crops = [frames[int(f * (len(frames) - 1))].crop(box) for f in fractions]
w, h = crops[0].size
cols = min(6, len(crops))
rows = (len(crops) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, im in enumerate(crops):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(len(frames), frames[0].size, sheet.size)
