import av, sys
from PIL import Image
c = av.open(sys.argv[1]); s = c.streams.video[0]
print(float(c.duration / 1e6), s.frames, s.average_rate)
ts = [float(x) for x in sys.argv[3:]]
for f in c.decode(video=0):
    t = float(f.pts * s.time_base)
    for x in list(ts):
        if t >= x:
            ts.remove(x); f.to_image().save(f"{sys.argv[2]}/f{int(x)}.png")
