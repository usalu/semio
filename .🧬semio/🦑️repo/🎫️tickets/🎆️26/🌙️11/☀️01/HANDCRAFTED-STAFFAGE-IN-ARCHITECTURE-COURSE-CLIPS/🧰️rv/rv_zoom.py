"""rv_zoom.py <out> <x0,y0,x1,y1> <zoom> <png>...: zoomed crops of PNG frames side by side."""
import sys

from PIL import Image

out, zoom = sys.argv[1], int(sys.argv[3])
box = tuple(int(v) for v in sys.argv[2].split(","))
crops = [Image.open(p).crop(box) for p in sys.argv[4:]]
crops = [c.resize((c.width * zoom, c.height * zoom), Image.LANCZOS) for c in crops]
w, h = crops[0].size
sheet = Image.new("RGB", (w * len(crops), h))
for i, c in enumerate(crops):
    sheet.paste(c, (i * w, 0))
sheet.save(out)
print(sheet.size)
