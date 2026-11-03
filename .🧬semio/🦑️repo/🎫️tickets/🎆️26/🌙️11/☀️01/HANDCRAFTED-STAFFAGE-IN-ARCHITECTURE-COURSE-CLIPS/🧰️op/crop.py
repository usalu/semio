import sys
from PIL import Image
im = Image.open(sys.argv[1]); x0, y0, x1, y1 = map(int, sys.argv[3:7])
im.crop((x0, y0, x1, y1)).resize(((x1 - x0) * 4, (y1 - y0) * 4), Image.LANCZOS).save(sys.argv[2])
