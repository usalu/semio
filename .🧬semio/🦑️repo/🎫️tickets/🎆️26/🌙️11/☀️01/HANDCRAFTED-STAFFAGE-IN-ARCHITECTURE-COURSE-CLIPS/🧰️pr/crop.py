import sys
from PIL import Image

src, dst, x0, y0, x1, y1 = sys.argv[1], sys.argv[2], *map(int, sys.argv[3:7])
im = Image.open(src).crop((x0, y0, x1, y1))
im = im.resize((im.width * 4, im.height * 4), Image.LANCZOS)
im.save(dst)
