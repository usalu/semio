import sys

import av

path, out, fraction = sys.argv[1], sys.argv[2], float(sys.argv[3])
box = tuple(int(v) for v in sys.argv[4].split(","))
with av.open(path) as container:
    frames = [frame.to_image() for frame in container.decode(video=0)]
image = frames[int(fraction * (len(frames) - 1))]
image.crop(box).resize(((box[2] - box[0]) * 3, (box[3] - box[1]) * 3)).save(out)
