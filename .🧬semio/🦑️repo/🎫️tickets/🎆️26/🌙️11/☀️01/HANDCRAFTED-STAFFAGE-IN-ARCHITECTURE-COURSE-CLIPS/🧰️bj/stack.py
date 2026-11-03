import sys
from PIL import Image
out, *srcs = sys.argv[1:]
ims = [Image.open(s) for s in srcs]
w = max(i.width for i in ims)
sheet = Image.new("RGB", (w, sum(i.height for i in ims)), "#101418")
y = 0
for i in ims:
    sheet.paste(i, (0, y)); y += i.height
sheet.save(out)
