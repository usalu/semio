import os
import sys

import av
from PIL import Image

path, out, count = sys.argv[1], sys.argv[2], int(sys.argv[3])
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
w, h = frames[0].size
cols = 3
rows = (count + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i in range(count):
    sheet.paste(frames[int((i + 0.5) / count * (len(frames) - 1))], ((i % cols) * w, (i // cols) * h))
os.makedirs(os.path.dirname(out), exist_ok=True)
sheet.save(out)
print(out, len(frames))
