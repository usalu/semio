"""rr_strip.py <mp4> <out.png> <x0,y0,x1,y1> <fraction,...>: cropped frames, 2x, side by side."""
import sys
import av
from PIL import Image

path, out = sys.argv[1], sys.argv[2]
box = tuple(int(v) for v in sys.argv[3].split(","))
fr = [float(v) for v in sys.argv[4].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
crops = [frames[int(f * (len(frames) - 1))].crop(box) for f in fr]
w, h = crops[0].size
sheet = Image.new("RGB", (w * len(crops) * 2, h * 2))
for i, im in enumerate(crops):
    sheet.paste(im.resize((w * 2, h * 2), Image.LANCZOS), (i * w * 2, 0))
sheet.save(out)
