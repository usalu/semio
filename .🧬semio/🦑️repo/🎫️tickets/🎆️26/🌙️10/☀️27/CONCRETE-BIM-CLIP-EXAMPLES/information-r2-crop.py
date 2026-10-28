import sys
from pathlib import Path
from PIL import Image

src, x0, y0, x1, y1, out = sys.argv[1], *map(int, sys.argv[2:6]), sys.argv[6]
image = Image.open(src).crop((x0, y0, x1, y1))
image = image.resize((image.width * 3, image.height * 3), Image.LANCZOS)
image.save(out)
