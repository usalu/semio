import sys
from PIL import Image
d, names, out = sys.argv[1], sys.argv[2].split(","), sys.argv[3]
crop = tuple(int(x) for x in sys.argv[4].split(",")) if len(sys.argv) > 4 else None
ims = []
for n in names:
    im = Image.open(f"{d}/t_{n}.png").convert("RGB")
    if crop:
        im = im.crop(crop)
    ims.append(im)
w, h = ims[0].size
cols = 2
rows = (len(ims) + 1) // 2
sheet = Image.new("RGB", (w * cols, h * rows))
for i, im in enumerate(ims):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
