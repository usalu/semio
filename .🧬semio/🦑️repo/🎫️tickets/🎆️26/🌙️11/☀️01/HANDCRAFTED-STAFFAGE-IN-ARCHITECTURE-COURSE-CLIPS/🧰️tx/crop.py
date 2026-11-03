"""crop.py <in> <out> x0 y0 x1 y1 scale: crop and upscale."""
import sys

from PIL import Image

im = Image.open(sys.argv[1])
x0, y0, x1, y1, s = (int(v) for v in sys.argv[3:8])
c = im.crop((x0, y0, x1, y1))
c.resize((c.width * s, c.height * s), Image.LANCZOS).save(sys.argv[2])
