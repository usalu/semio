import sys, av
from pathlib import Path
src, out, times = sys.argv[1], Path(sys.argv[2]), [float(t) for t in sys.argv[3:]]
out.mkdir(parents=True, exist_ok=True)
c = av.open(src)
s = c.streams.video[0]
print("duration", float(c.duration) / 1e6)
for t in times:
    c.seek(int(t * 1e6))
    for f in c.decode(video=0):
        if f.time >= t - 0.05:
            f.to_image().save(out / f"{Path(src).stem}_{t:07.1f}.png")
            break
