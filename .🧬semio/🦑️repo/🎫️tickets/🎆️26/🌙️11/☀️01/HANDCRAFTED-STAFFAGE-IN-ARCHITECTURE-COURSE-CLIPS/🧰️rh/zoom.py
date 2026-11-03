import sys

import av
from PIL import Image

path, out, t = sys.argv[1], sys.argv[2], sys.argv[3]
x0, y0, x1, y1 = (int(v) for v in sys.argv[4:8])
ts = [float(v) for v in t.split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
crops = []
for s in ts:
    im = frames[min(len(frames) - 1, int(s * 15))].crop((x0, y0, x1, y1))
    crops.append(im.resize(((x1 - x0) * 3, (y1 - y0) * 3), Image.LANCZOS))
w, h = crops[0].size
sheet = Image.new("RGB", (w * min(2, len(crops)), h * ((len(crops) + 1) // 2)))
for i, im in enumerate(crops):
    sheet.paste(im, ((i % 2) * w, (i // 2) * h))
sheet.save(out)
