import sys
import av

path, prefix = sys.argv[1], sys.argv[2]
ts = [float(v) for v in sys.argv[3].split(",")]
want = {int(t * 15): t for t in ts}
with av.open(path) as c:
    for i, f in enumerate(c.decode(video=0)):
        if i in want:
            f.to_image().save(f"{prefix}-{want[i]:.2f}.png")
