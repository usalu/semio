import sys
import av
from PIL import Image

path, out, lo, hi, n = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4]), int(sys.argv[5])
with av.open(path) as c:
    frames = [f.to_image() for f in c.decode(video=0)]
idx = [int((lo + (hi - lo) * i / (n - 1)) * (len(frames) - 1)) for i in range(n)]
w, h = frames[0].size
cols = 3
rows = (n + cols - 1) // cols
sheet = Image.new("RGB", (cols * w // 2, rows * h // 2))
for k, i in enumerate(idx):
    sheet.paste(frames[i].resize((w // 2, h // 2)), ((k % cols) * w // 2, (k // cols) * h // 2))
sheet.save(out)
print(idx)
