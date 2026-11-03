import sys

import av
from PIL import Image, ImageDraw

path, out, start, stop, step = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4]), float(sys.argv[5])
cols = int(sys.argv[6]) if len(sys.argv) > 6 else 3
ts = []
t = start
while t <= stop + 1e-6:
    ts.append(t)
    t += step
want = {int(round(t * 15)): t for t in ts}
shots = {}
with av.open(path) as c:
    for i, f in enumerate(c.decode(video=0)):
        if i in want:
            shots[want[i]] = f.to_image()
w, h = next(iter(shots.values())).size
rows = (len(shots) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for k, (t, im) in enumerate(sorted(shots.items())):
    ImageDraw.Draw(im).text((4, 4), f"{t:.2f}s", fill=(255, 255, 0))
    sheet.paste(im, ((k % cols) * w, (k // cols) * h))
sheet.save(out)
