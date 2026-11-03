"""Write a contact sheet of evenly spaced frames: `p1_frames.py <mp4> <out.png> <count> [from_fraction]`."""

import sys

import av
import numpy as np
from PIL import Image

path, out, count = sys.argv[1], sys.argv[2], int(sys.argv[3])
start = float(sys.argv[4]) if len(sys.argv) > 4 else 0.0
frames = [frame.to_ndarray(format="rgb24") for frame in av.open(path).decode(video=0)]
picks = [frames[int((start + (1 - start) * i / (count - 1)) * (len(frames) - 1))] for i in range(count)]
columns = 3
rows = (count + columns - 1) // columns
height, width = picks[0].shape[:2]
sheet = np.zeros((rows * height, columns * width, 3), dtype=np.uint8)
for i, pick in enumerate(picks):
    sheet[(i // columns) * height:(i // columns + 1) * height, (i % columns) * width:(i % columns + 1) * width] = pick
Image.fromarray(sheet).save(out)
print(len(frames), "frames")
