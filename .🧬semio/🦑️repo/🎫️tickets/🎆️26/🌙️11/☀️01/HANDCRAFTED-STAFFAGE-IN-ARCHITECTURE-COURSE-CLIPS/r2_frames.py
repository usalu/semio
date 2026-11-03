"""r2_frames.py <mp4> <outdir> <fraction,...>: full frames at relative positions as PNG."""
import os
import sys

import av

path, out = sys.argv[1], sys.argv[2]
fractions = [float(v) for v in sys.argv[3].split(",")]
os.makedirs(out, exist_ok=True)
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
name = os.path.splitext(os.path.basename(path))[0]
for f in fractions:
    p = os.path.join(out, f"{name}_{int(f * 100):02d}.png")
    frames[int(f * (len(frames) - 1))].save(p)
    print(p)
print(len(frames), frames[0].size)
