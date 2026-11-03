"""ws_crop.py <png> <out> x0,y0,x1,y1 [scale]: crop and upscale."""
import sys

from PIL import Image

img = Image.open(sys.argv[1])
x0, y0, x1, y1 = (int(v) for v in sys.argv[3].split(","))
s = int(sys.argv[4]) if len(sys.argv) > 4 else 6
img.crop((x0, y0, x1, y1)).resize(((x1 - x0) * s, (y1 - y0) * s), Image.LANCZOS).save(sys.argv[2])
