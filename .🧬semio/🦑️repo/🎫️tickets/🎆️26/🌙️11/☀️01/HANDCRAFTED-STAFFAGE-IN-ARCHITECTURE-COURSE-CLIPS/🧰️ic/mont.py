import sys
from PIL import Image
d = sys.argv[1]; ts = sys.argv[2:]
ims = [Image.open(f"{d}/f{t}.png") for t in ts]
w, h = ims[0].size
cols = 3; rows = (len(ims) + 2) // 3
m = Image.new("RGB", (w * cols, h * rows))
for i, im in enumerate(ims): m.paste(im, ((i % cols) * w, (i // cols) * h))
m.save(f"{d}/mont.png")
