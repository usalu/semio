import sys
import av
from PIL import Image, ImageDraw

path, out, start, stop, step = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
box = (640, 40, 790, 170)
tiles = []
with av.open(path) as c:
    for i, f in enumerate(c.decode(video=0)):
        if start <= i < stop and (i - start) % step == 0:
            im = f.to_image().crop(box).resize((300, 260))
            ImageDraw.Draw(im).text((4, 4), str(i), fill="white")
            tiles.append(im)
cols = 6
rows = (len(tiles) + cols - 1) // cols
sheet = Image.new("RGB", (cols * 300, rows * 260))
for k, t in enumerate(tiles):
    sheet.paste(t, ((k % cols) * 300, (k // cols) * 260))
sheet.save(out)
print(len(tiles))
