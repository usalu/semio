import sys
from PIL import Image

src, dst, x0, y0, x1, y1, scale = sys.argv[1], sys.argv[2], *map(int, sys.argv[3:8])
image = Image.open(src).convert("RGB").crop((x0, y0, x1, y1))
image = image.resize((image.width * scale, image.height * scale), Image.LANCZOS)
image.save(dst)
print(dst, image.size)
