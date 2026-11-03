"""rm_crop.py <in.png> <out.png> x0 y0 x1 y1 [scale]: crop and enlarge."""
import sys
from PIL import Image

src, dst = sys.argv[1], sys.argv[2]
x0, y0, x1, y1 = map(int, sys.argv[3:7])
scale = int(sys.argv[7]) if len(sys.argv) > 7 else 4
im = Image.open(src).crop((x0, y0, x1, y1))
im.resize((im.width * scale, im.height * scale), Image.LANCZOS).save(dst)
