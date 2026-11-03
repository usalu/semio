"""rr2_sheet.py <mp4> <out.png> <fraction,...>: 2 columns contact sheet at 0.75 scale."""
import sys
import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
fr = [float(v) for v in sys.argv[3].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
ims = [frames[int(f * (len(frames) - 1))] for f in fr]
w, h = ims[0].size
w, h = int(w * 0.75), int(h * 0.75)
rows = (len(ims) + 1) // 2
sheet = Image.new("RGB", (w * 2, h * rows))
for i, im in enumerate(ims):
    sheet.paste(im.resize((w, h), Image.LANCZOS), ((i % 2) * w, (i // 2) * h))
sheet.save(out)
