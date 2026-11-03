import sys
import av

src, out, count = sys.argv[1], sys.argv[2], int(sys.argv[3])
with av.open(src) as c:
    n = c.streams.video[0].frames
    want = {int(n * (i + 0.5) / count) for i in range(count)}
    for i, f in enumerate(c.decode(video=0)):
        if i in want:
            f.to_image().save(f"{out}/f{i:04d}.png")
    print(n)
