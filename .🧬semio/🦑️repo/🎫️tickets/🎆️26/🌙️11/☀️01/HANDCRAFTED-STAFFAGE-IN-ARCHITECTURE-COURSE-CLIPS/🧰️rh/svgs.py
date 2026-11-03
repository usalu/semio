import sys
from pathlib import Path

import fitz
from PIL import Image

folder, out = Path(sys.argv[1]), sys.argv[2]
picks = sys.argv[3].split(",")
zoom = float(sys.argv[4]) if len(sys.argv) > 4 else 1.5
ims = []
for p in picks:
    doc = fitz.open(str(folder / f"{int(p):04d}.svg"))
    pix = doc[0].get_pixmap(matrix=fitz.Matrix(zoom, zoom), alpha=False)
    ims.append(Image.frombytes("RGB", (pix.width, pix.height), pix.samples))
w, h = ims[0].size
sheet = Image.new("RGB", (w * len(ims), h), (40, 40, 40))
for i, im in enumerate(ims):
    sheet.paste(im, (i * w, 0))
sheet.save(out)
print(w, h)
