"""c1_crop.py <mp4> <out.png> <frame:x0,y0,x1,y1> ... : magnified crops of chosen frame numbers side by side."""
import sys

import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
specs = [(int(a.split(":")[0]), tuple(int(v) for v in a.split(":")[1].split(","))) for a in sys.argv[3:]]
wanted = {n for n, _ in specs}
got = {}
with av.open(path) as container:
    for i, frame in enumerate(container.decode(video=0)):
        if i in wanted:
            got[i] = frame.to_image()
        if len(got) == len(wanted):
            break
crops = []
for n, box in specs:
    c = got[n].crop(box)
    k = 700 / c.width
    crops.append(c.resize((700, int(c.height * k))))
sheet = Image.new("RGB", (700 * len(crops), max(c.height for c in crops)))
for i, c in enumerate(crops):
    sheet.paste(c, (700 * i, 0))
sheet.save(out)
print(sheet.size)
