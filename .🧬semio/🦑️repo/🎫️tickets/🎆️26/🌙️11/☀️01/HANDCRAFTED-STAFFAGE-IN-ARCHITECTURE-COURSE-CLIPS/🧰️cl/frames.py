import sys
import av

path, out, *times = sys.argv[1:]
want = sorted(float(t) for t in times)
with av.open(path) as c:
    stream = c.streams.video[0]
    print("duration", float(stream.duration * stream.time_base) if stream.duration else None)
    i = 0
    for frame in c.decode(stream):
        if i < len(want) and frame.time >= want[i]:
            frame.to_image().save(f"{out}_{want[i]:06.2f}.png")
            i += 1
