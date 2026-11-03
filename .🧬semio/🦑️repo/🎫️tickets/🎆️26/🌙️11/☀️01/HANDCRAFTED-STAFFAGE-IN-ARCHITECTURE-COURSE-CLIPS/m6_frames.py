"""M6: contact sheet of evenly spaced frames of one mp4: m6_frames.py <mp4> <png> [cols] [count]."""
import sys

import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
cols = int(sys.argv[3]) if len(sys.argv) > 3 else 3
count = int(sys.argv[4]) if len(sys.argv) > 4 else 9
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
picks = [frames[int(i * (len(frames) - 1) / max(count - 1, 1))] for i in range(count)]
w, h = picks[0].size
w2, h2 = w * 3 // 4, h * 3 // 4
rows = (count + cols - 1) // cols
sheet = Image.new("RGB", (cols * w2, rows * h2))
for i, im in enumerate(picks):
    sheet.paste(im.resize((w2, h2)), ((i % cols) * w2, (i // cols) * h2))
sheet.save(out)
print(len(frames), sheet.size)
