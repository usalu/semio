import sys, av
from PIL import Image
src, dst = sys.argv[1], sys.argv[2]
x0, y0, x1, y1 = map(int, sys.argv[3:7])
ts = [float(t) for t in sys.argv[7:]]
want = {int(t * 15): t for t in ts}
tiles = []
with av.open(src) as c:
    for i, f in enumerate(c.decode(video=0)):
        if i in want:
            tiles.append(f.to_image().crop((x0, y0, x1, y1)).resize(((x1 - x0) * 3, (y1 - y0) * 3), Image.LANCZOS))
w, h = tiles[0].size
out = Image.new("RGB", (w * len(tiles), h))
for k, t in enumerate(tiles):
    out.paste(t, (k * w, 0))
out.save(dst)
