import sys
from pathlib import Path
from PIL import Image
src, x0, y0, x1, y1, factor = sys.argv[1], *map(int, sys.argv[2:6]), int(sys.argv[6])
out = Path(src).with_name("crop.png")
image = Image.open(src).crop((x0, y0, x1, y1))
image.resize((image.width * factor, image.height * factor), Image.LANCZOS).save(out)

