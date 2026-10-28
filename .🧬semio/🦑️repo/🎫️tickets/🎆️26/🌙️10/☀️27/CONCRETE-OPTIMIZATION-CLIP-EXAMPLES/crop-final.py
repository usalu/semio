import sys
from PIL import Image

src, out = sys.argv[1], sys.argv[2]
x0, y0, x1, y1 = (int(v) for v in sys.argv[3:7])
scale = float(sys.argv[7]) if len(sys.argv) > 7 else 3.0
image = Image.open(src).convert("RGB").crop((x0, y0, x1, y1))
image = image.resize((int((x1 - x0) * scale), int((y1 - y0) * scale)), Image.LANCZOS)
image.save(out)
print(out, image.size)
