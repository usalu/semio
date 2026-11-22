"""🖼️ Montage of up to four r3 sheets into one image for reading."""
import sys
from PIL import Image
out, *ins = sys.argv[1:]
ims = [Image.open(i).convert("RGB") for i in ins]
w, h = 1500, 1000
cols = 2 if len(ims) > 1 else 1
rows = (len(ims) + cols - 1) // cols
m = Image.new("RGB", (w * cols, h * rows), "white")
for k, im in enumerate(ims):
    m.paste(im.resize((w, h), Image.LANCZOS), ((k % cols) * w, (k // cols) * h))
m.save(out)
