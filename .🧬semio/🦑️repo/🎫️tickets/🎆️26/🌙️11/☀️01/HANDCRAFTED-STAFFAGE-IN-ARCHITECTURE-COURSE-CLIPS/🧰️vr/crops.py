import sys, glob, av
from PIL import Image
root, lang, scene, x0, y0, x1, y1 = sys.argv[1:8]
idx = [int(i) for i in sys.argv[8:]]
c = av.open(glob.glob(f"{root}/media-{lang}/videos/**/{scene}.mp4", recursive=True)[0])
box = tuple(map(int, (x0, y0, x1, y1)))
tiles = []
for i, f in enumerate(c.decode(video=0)):
    if i in idx:
        im = f.to_image().crop(box)
        tiles.append(im.resize((im.width * 3, im.height * 3)))
s = Image.new("RGB", (tiles[0].width * len(tiles), tiles[0].height))
for k, t in enumerate(tiles):
    s.paste(t, (t.width * k, 0))
s.save(f"{root}/{lang}-{scene}-crops.png")
