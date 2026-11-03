import sys
import av
import numpy as np
p, f = sys.argv[1], float(sys.argv[2])
x0, x1 = int(sys.argv[3]), int(sys.argv[4])
with av.open(p) as c:
    fr = [x.to_ndarray(format="rgb24") for x in c.decode(video=0)]
im = fr[int(f * (len(fr) - 1))]
for y in range(300, 380):
    row = im[y, x0:x1]
    bright = int((row.sum(axis=1) > 300).sum())
    print(y, bright)
