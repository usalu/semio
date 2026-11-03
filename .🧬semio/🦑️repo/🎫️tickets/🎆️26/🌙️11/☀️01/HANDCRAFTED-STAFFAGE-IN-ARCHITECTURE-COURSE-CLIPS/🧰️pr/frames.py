import sys, pathlib, av
from PIL import Image

src, out = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
frames = []
with av.open(str(src)) as c:
    for f in c.decode(video=0):
        frames.append(f.to_image())
n = len(frames)
picks = {"a": 0, "b": n // 4, "c": n // 2, "d": 3 * n // 4, "e": n - 1}
if len(sys.argv) > 3:
    picks = {f"t{int(s)}": min(n - 1, int(float(s) * 15)) for s in sys.argv[3:]}
name = src.stem
for k, i in picks.items():
    frames[i].save(out / f"{name}-{k}-{i}.png")
print(name, n, "frames")
