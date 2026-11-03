"""montage.py <out.png> <img>...: 2-column contact sheet."""
import sys

from PIL import Image

out, paths = sys.argv[1], sys.argv[2:]
images = [Image.open(p) for p in paths]
w, h = images[0].size
rows = (len(images) + 1) // 2
sheet = Image.new("RGB", (2 * w, rows * h))
for i, im in enumerate(images):
    sheet.paste(im, ((i % 2) * w, (i // 2) * h))
sheet.save(out)
