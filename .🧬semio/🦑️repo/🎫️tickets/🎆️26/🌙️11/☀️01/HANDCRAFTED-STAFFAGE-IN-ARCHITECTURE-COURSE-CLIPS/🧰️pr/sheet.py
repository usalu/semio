import sys, av
from PIL import Image
src, dst, step = sys.argv[1], sys.argv[2], float(sys.argv[3])
fr = []
with av.open(src) as c:
    for i, f in enumerate(c.decode(video=0)):
        if i % int(step * 15) == 0:
            fr.append(f.to_image().resize((427, 240)))
cols = 4
rows = (len(fr) + cols - 1) // cols
out = Image.new("RGB", (427 * cols, 240 * rows))
for k, im in enumerate(fr):
    out.paste(im, ((k % cols) * 427, (k // cols) * 240))
out.save(dst)
print(len(fr))
