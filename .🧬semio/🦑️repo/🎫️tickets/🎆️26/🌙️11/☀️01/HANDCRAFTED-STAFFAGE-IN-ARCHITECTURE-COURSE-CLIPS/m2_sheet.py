import sys

import av
from PIL import Image

source, target, step, columns = sys.argv[1], sys.argv[2], float(sys.argv[3]), int(sys.argv[4])
container = av.open(source)
stream = container.streams.video[0]
frames, last = [], -1e9
for frame in container.decode(stream):
    t = float(frame.pts * stream.time_base)
    if t - last >= step:
        frames.append(frame.to_image().resize((640, 360)))
        last = t
rows = (len(frames) + columns - 1) // columns
sheet = Image.new("RGB", (640 * columns, 360 * rows))
for i, image in enumerate(frames):
    sheet.paste(image, (640 * (i % columns), 360 * (i // columns)))
sheet.save(target)
print(len(frames), "frames")
