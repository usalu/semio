import sys
from PIL import Image
d = sys.argv[1]; ts = sys.argv[2:]
tiles = []
for t in ts:
    im = Image.open(f"{d}/f{t}.png"); w, h = im.size
    tiles.append(im.crop((int(w*0.5), int(h*0.18), int(w*0.97), int(h*0.32))).resize((1100, 240)))
m = Image.new("RGB", (1100, 240*len(tiles)))
for i, t in enumerate(tiles): m.paste(t, (0, 240*i))
m.save(f"{d}/cards.png")
