"""M6: one frame at a fraction, cropped and magnified: m6_crop.py <mp4> <png> <fraction> <x0> <y0> <x1> <y1> (fractions of the frame)."""
import sys

import av

path, out, at = sys.argv[1], sys.argv[2], float(sys.argv[3])
x0, y0, x1, y1 = map(float, sys.argv[4:8])
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
image = frames[int(at * (len(frames) - 1))]
w, h = image.size
crop = image.crop((int(x0 * w), int(y0 * h), int(x1 * w), int(y1 * h)))
crop = crop.resize((crop.width * 3, crop.height * 3))
crop.save(out)
print(len(frames), crop.size)
