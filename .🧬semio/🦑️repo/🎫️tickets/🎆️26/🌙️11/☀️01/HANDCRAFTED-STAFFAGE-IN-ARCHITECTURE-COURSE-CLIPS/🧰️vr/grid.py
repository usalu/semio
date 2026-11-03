import sys, glob, av
from PIL import Image
root, lang, scene, step = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
c = av.open(glob.glob(f"{root}/media-{lang}/videos/**/{scene}.mp4", recursive=True)[0])
fr = [f.to_image().resize((427, 240)) for i, f in enumerate(c.decode(video=0))]
sel = fr[step::step]
cols = 3
rows = (len(sel) + cols - 1) // cols
s = Image.new("RGB", (427 * cols, 240 * rows))
for k, im in enumerate(sel):
    s.paste(im, (427 * (k % cols), 240 * (k // cols)))
s.save(f"{root}/{lang}-{scene}-grid.png")
print(len(fr), len(sel))
