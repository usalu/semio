"""rv_mosaic.py <out> <zoom> <cols> <png@x0,y0,x1,y1>...: grid of zoomed crops."""
import sys

from PIL import Image

out, zoom, cols = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
crops = []
for spec in sys.argv[4:]:
    path, box = spec.split("@")
    c = Image.open(path).crop(tuple(int(v) for v in box.split(",")))
    crops.append(c.resize((c.width * zoom, c.height * zoom), Image.LANCZOS))
w = max(c.width for c in crops)
h = max(c.height for c in crops)
rows = (len(crops) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, c in enumerate(crops):
    sheet.paste(c, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(sheet.size)
