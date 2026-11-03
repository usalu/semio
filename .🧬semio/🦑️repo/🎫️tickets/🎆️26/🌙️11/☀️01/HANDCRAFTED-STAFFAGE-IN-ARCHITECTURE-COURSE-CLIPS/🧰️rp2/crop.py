import sys
import av
from PIL import Image
path, out, box, scale = sys.argv[1], sys.argv[2], [int(v) for v in sys.argv[3].split(",")], int(sys.argv[4])
times = [float(t) for t in sys.argv[5].split(",")]
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
crops = [frames[min(len(frames) - 1, int(t * 15))].crop(box) for t in times]
w, h = crops[0].size
sheet = Image.new("RGB", (w * scale * len(crops), h * scale))
for i, im in enumerate(crops):
    sheet.paste(im.resize((w * scale, h * scale), Image.LANCZOS), (i * w * scale, 0))
sheet.save(out)
