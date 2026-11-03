"""Extract frames from an mp4 with pyav: python frames.py in.mp4 out_prefix t1 t2 ..."""
import sys

import av

src, prefix = sys.argv[1], sys.argv[2]
times = [float(t) for t in sys.argv[3:]]
with av.open(src) as c:
    s = c.streams.video[0]
    rate = float(s.average_rate)
    want = {round(t * rate): t for t in times}
    for i, f in enumerate(c.decode(s)):
        if i in want:
            f.to_image().save(f"{prefix}_{want[i]:g}.png")
print("done")
