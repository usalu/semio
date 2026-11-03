import sys, glob, av
from PIL import Image
root, lang, scene, a, b, step = sys.argv[1:7]
a, b, step = int(a), int(b), int(step)
c = av.open(glob.glob(f"{root}/media-{lang}/videos/**/{scene}.mp4", recursive=True)[0])
sel = [f.to_image().resize((427, 240)) for i, f in enumerate(c.decode(video=0)) if a <= i < b and (i - a) % step == 0]
cols = 3
rows = (len(sel) + cols - 1) // cols
s = Image.new("RGB", (427 * cols, 240 * rows))
for k, im in enumerate(sel):
    s.paste(im, (427 * (k % cols), 240 * (k // cols)))
s.save(f"{root}/{lang}-{scene}-range.png")
