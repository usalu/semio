import sys
from PIL import Image
src, y0, y1, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
im = Image.open(src); w, h = im.size
im.crop((0, y0, w, min(h, y1))).save(out); print(w, h)
