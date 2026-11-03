import sys, glob
from PIL import Image
files = sorted(glob.glob(sys.argv[1] + "/*.png"))
cols = int(sys.argv[3])
ims = [Image.open(f) for f in files]
w, h = ims[0].size
s = Image.new("RGB", (cols * w, ((len(ims) + cols - 1) // cols) * h))
for i, im in enumerate(ims):
    s.paste(im, ((i % cols) * w, (i // cols) * h))
s.save(sys.argv[2])
