"""V1: sheet of frames at given seconds (or frame fraction with %): v1_pick.py <mp4> <png> <cols> <t1,t2,...>; prints duration."""
import sys

import av
from PIL import Image, ImageDraw

path, out, cols = sys.argv[1], sys.argv[2], int(sys.argv[3])
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
fps = 15
print(len(frames), "frames", len(frames) / fps, "s")
times = [float(t) for t in sys.argv[4].split(",")]
picks = [(t, frames[min(len(frames) - 1, int(t * fps))]) for t in times]
w, h = picks[0][1].size
rows = (len(picks) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, (t, im) in enumerate(picks):
    ImageDraw.Draw(im).text((4, 4), f"{t:.1f}s", fill=(255, 255, 0))
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(sheet.size)
