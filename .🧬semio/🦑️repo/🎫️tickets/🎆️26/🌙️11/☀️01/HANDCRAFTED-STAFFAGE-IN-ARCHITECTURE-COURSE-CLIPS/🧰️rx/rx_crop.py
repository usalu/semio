import sys
from PIL import Image
src, out, x0, y0, x1, y1, scale = sys.argv[1], sys.argv[2], *map(float, sys.argv[3:])
im = Image.open(src)
w, h = im.size
box = tuple(int(v) for v in (x0 * w, y0 * h, x1 * w, y1 * h))
c = im.crop(box)
c = c.resize((int(c.width * scale), int(c.height * scale)), Image.LANCZOS)
c.save(out)
print(c.size)
