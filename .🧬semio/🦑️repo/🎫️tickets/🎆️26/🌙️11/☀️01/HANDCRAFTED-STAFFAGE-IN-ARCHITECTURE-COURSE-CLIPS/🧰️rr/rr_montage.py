"""rr_montage.py <out.png> <png>...: 2-column montage."""
import sys
from PIL import Image

ims = [Image.open(p) for p in sys.argv[2:]]
w, h = ims[0].size
cols = 2
rows = (len(ims) + 1) // 2
sheet = Image.new("RGB", (w * cols, h * rows))
for i, im in enumerate(ims):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(sys.argv[1])
