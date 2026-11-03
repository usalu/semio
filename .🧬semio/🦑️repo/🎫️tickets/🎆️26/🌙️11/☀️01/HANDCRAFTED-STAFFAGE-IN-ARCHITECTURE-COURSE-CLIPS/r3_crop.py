import sys
from pathlib import Path
from PIL import Image
src, dst, x0, y0, x1, y1, k = sys.argv[1], sys.argv[2], *map(int, sys.argv[3:8])
im = Image.open(src).crop((x0, y0, x1, y1))
im.resize((im.width * k, im.height * k), Image.LANCZOS).save(dst)
