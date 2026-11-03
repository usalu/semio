import sys, av
from PIL import Image
src, out = sys.argv[1], sys.argv[2]
times = [float(t) for t in sys.argv[3].split(",")]
c = av.open(src)
fr = []
for f in c.decode(video=0):
    fr.append(f)
tiles = []
for t in times:
    f = min(fr, key=lambda x: abs(float(x.pts * x.time_base) - t))
    tiles.append(f.to_image())
w, h = tiles[0].size
sheet = Image.new("RGB", (w * 2, h * ((len(tiles) + 1) // 2)))
for i, im in enumerate(tiles):
    sheet.paste(im, ((i % 2) * w, (i // 2) * h))
sheet.save(out)
print(len(fr), fr[-1].time, w, h)
