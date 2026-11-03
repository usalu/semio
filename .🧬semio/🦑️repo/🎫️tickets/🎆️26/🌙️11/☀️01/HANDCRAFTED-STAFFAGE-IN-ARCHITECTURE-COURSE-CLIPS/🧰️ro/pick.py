import sys
import av
from PIL import Image, ImageDraw
path, outdir = sys.argv[1], sys.argv[2]
n = int(sys.argv[3])
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
print(len(frames), "frames", len(frames) / 15, "s", frames[0].size)
for k in range(n):
    i = int((k + .5) * len(frames) / n)
    im = frames[i]
    ImageDraw.Draw(im).text((4, 4), f"{i/15:.1f}s", fill=(255, 255, 0))
    im.save(f"{outdir}/{k:02d}.png")
