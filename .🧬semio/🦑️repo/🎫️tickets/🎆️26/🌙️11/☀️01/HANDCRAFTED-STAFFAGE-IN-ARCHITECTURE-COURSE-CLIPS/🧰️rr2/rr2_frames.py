"""rr2_frames.py <mp4> <outprefix> <fraction,...>: full frames as separate PNGs."""
import sys
import av

path, out = sys.argv[1], sys.argv[2]
fr = [float(v) for v in sys.argv[3].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
for i, f in enumerate(fr):
    frames[int(f * (len(frames) - 1))].save(f"{out}_{i}.png")
print(len(frames))
