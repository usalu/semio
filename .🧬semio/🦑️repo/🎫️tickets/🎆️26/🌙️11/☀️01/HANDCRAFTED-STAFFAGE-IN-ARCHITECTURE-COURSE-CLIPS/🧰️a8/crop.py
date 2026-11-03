import sys
from PIL import Image
src, out, x0, y0, x1, y1, z = sys.argv[1], sys.argv[2], *map(int, sys.argv[3:8])
im = Image.open(src).crop((x0, y0, x1, y1))
im.resize((im.width * z, im.height * z), Image.LANCZOS).save(out)
