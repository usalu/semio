"""sm2: crop.py <mp4> <out.png> <x0,y0,x1,y1 in 854x480 frame> <scale> <t1,t2,...> — zoomed crops at the given seconds, streamed (only wanted frames kept)."""
import sys

import av
from PIL import Image, ImageDraw

path, out, box, scale = sys.argv[1], sys.argv[2], [int(v) for v in sys.argv[3].split(",")], int(sys.argv[4])
times = [float(t) for t in sys.argv[5].split(",")]
wanted = {round(t * 15): t for t in times}
crops = {}
with av.open(path) as container:
    for index, frame in enumerate(container.decode(video=0)):
        if index in wanted:
            im = frame.to_image()
            sx = im.width / 854
            c = im.crop(tuple(int(v * sx) for v in box))
            c = c.resize((c.width * scale, c.height * scale), Image.LANCZOS)
            ImageDraw.Draw(c).text((4, 4), f"{wanted[index]:.1f}s", fill=(255, 255, 0))
            crops[wanted[index]] = c
        if index >= max(wanted):
            break
ordered = [crops[t] for t in times if t in crops]
cols = min(4, len(ordered))
rows = (len(ordered) + cols - 1) // cols
sheet = Image.new("RGB", (cols * ordered[0].width, rows * ordered[0].height), (30, 30, 30))
for i, c in enumerate(ordered):
    sheet.paste(c, ((i % cols) * c.width, (i // cols) * c.height))
sheet.save(out)
print(sheet.size)
