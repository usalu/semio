import sys, av
from PIL import Image
path, out = sys.argv[1], sys.argv[2]
fr = [float(v) for v in sys.argv[3].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
n = len(frames)
cells = [frames[int(f * (n - 1))] for f in fr]
w, h = cells[0].size
cols = 3
rows = (len(cells) + cols - 1) // cols
sheet = Image.new("RGB", (w * cols, h * rows))
for i, c in enumerate(cells):
    sheet.paste(c, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
print(n, sheet.size)
