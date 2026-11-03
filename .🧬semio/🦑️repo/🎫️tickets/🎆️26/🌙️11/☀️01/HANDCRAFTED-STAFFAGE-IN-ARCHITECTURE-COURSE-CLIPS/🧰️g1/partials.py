"""G1: last frame of chosen partial movies: partials.py <media dir> <png> <cols> <idx,idx,...> [crop x0,y0,x1,y1]"""
import glob
import sys

import av
from PIL import Image, ImageDraw

media, out, cols, idx = sys.argv[1], sys.argv[2], int(sys.argv[3]), [int(i) for i in sys.argv[4].split(",")]
crop = [int(v) for v in sys.argv[5].split(",")] if len(sys.argv) > 5 else None
files = {int(p.replace("\\", "/").split("uncached_")[1][:5]): p for p in glob.glob(media + "/**/uncached_*.mp4", recursive=True)}
shots = []
for i in idx:
    with av.open(files[i]) as container:
        frames = [f.to_image() for f in container.decode(video=0)]
    image = frames[-1]
    if crop:
        image = image.crop(crop)
    ImageDraw.Draw(image).text((4, 4), f"anim {i} ({len(frames)} frames)", fill=(255, 255, 0))
    shots.append(image)
w, h = shots[0].size
rows = (len(shots) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for n, image in enumerate(shots):
    sheet.paste(image, ((n % cols) * w, (n // cols) * h))
sheet.save(out)
print(sheet.size)
