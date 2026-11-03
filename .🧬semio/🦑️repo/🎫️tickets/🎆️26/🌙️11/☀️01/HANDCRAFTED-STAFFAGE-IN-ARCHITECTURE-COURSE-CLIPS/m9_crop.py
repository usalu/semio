"""M9: magnified crops of one video: m9_crop.py <mp4> <png> <x0,y0,x1,y1 in 2562x960 pixels> <t1,t2,...>"""
import sys

import av

path, out, box, times = sys.argv[1], sys.argv[2], [int(v) for v in sys.argv[3].split(",")], [float(t) for t in sys.argv[4].split(",")]
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
crops = [frames[min(len(frames) - 1, int(t * 15))].crop(box) for t in times]
width, height = crops[0].size
scale = 3
sheet = crops[0].resize((width * scale, height * scale)).copy()
from PIL import Image

sheet = Image.new("RGB", (width * scale * len(crops), height * scale))
for index, crop in enumerate(crops):
    sheet.paste(crop.resize((width * scale, height * scale)), (index * width * scale, 0))
sheet.save(out)
print(sheet.size)
