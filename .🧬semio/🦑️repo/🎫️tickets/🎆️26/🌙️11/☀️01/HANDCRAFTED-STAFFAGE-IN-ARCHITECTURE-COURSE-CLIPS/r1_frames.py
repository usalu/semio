"""R1: frames of a rendered scene: r1_frames.py <mp4> <outdir> <name> [fractions comma list]; writes <name>-sheet.png (2 columns) and <name>-NN.png."""
import sys
from pathlib import Path

import av
from PIL import Image, ImageDraw

path, out, name = sys.argv[1], Path(sys.argv[2]), sys.argv[3]
fractions = [float(f) for f in (sys.argv[4] if len(sys.argv) > 4 else "0.12,0.4,0.65,0.92").split(",")]
out.mkdir(parents=True, exist_ok=True)
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
fps = 15
print(len(frames), "frames", round(len(frames) / fps, 1), "s")
picks = []
for i, f in enumerate(fractions):
    index = min(len(frames) - 1, int(f * (len(frames) - 1)))
    image = frames[index].copy()
    ImageDraw.Draw(image).text((4, 4), f"{index / fps:.1f}s", fill=(255, 255, 0))
    image.save(out / f"{name}-{i:02d}.png")
    picks.append(image)
w, h = picks[0].size
rows = (len(picks) + 1) // 2
sheet = Image.new("RGB", (2 * w, rows * h))
for i, im in enumerate(picks):
    sheet.paste(im, ((i % 2) * w, (i // 2) * h))
sheet.save(out / f"{name}-sheet.png")
