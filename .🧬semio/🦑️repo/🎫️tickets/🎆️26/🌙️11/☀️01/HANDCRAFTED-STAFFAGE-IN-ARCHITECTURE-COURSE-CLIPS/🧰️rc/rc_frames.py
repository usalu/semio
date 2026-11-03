"""rc_frames.py <mp4> <png> <cols> <fraction,...> [crop x0,y0,x1,y1 as fractions]: contact sheet of frames at relative positions."""
import sys

import av
from PIL import Image, ImageDraw

path, out, cols = sys.argv[1], sys.argv[2], int(sys.argv[3])
fractions = [float(v) for v in sys.argv[4].split(",")]
crop = [float(v) for v in sys.argv[5].split(",")] if len(sys.argv) > 5 else None
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
print(len(frames), "frames", len(frames) / 15, "s", frames[0].size)
picks = []
for f in fractions:
    im = frames[min(len(frames) - 1, int(f * (len(frames) - 1)))].copy()
    if crop:
        w, h = im.size
        im = im.crop((int(crop[0] * w), int(crop[1] * h), int(crop[2] * w), int(crop[3] * h))).resize((int((crop[2] - crop[0]) * w * 2), int((crop[3] - crop[1]) * h * 2)))
    ImageDraw.Draw(im).text((4, 4), f"{f * len(frames) / 15:.1f}s", fill=(255, 255, 0))
    picks.append(im)
w, h = picks[0].size
rows = (len(picks) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, im in enumerate(picks):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(sheet.size)
