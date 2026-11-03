import sys
import av
from PIL import Image, ImageDraw
path, out, box = sys.argv[1], sys.argv[2], tuple(int(v) for v in sys.argv[3].split(","))
times = [float(t) for t in sys.argv[4].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
tiles = []
for t in times:
    im = frames[min(len(frames) - 1, int(t * 15))].crop(box)
    im = im.resize((im.width * 2, im.height * 2))
    ImageDraw.Draw(im).text((4, 4), f"{t}s", fill=(255, 255, 0))
    tiles.append(im)
sheet = Image.new("RGB", (tiles[0].width * 3, tiles[0].height * ((len(tiles) + 2) // 3)))
for i, im in enumerate(tiles):
    sheet.paste(im, ((i % 3) * im.width, (i // 3) * im.height))
sheet.save(out)
