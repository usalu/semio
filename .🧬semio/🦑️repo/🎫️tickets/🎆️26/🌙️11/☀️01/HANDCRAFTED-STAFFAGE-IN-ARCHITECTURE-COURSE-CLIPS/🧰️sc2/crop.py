import sys
import av

path, out, index, box = sys.argv[1], sys.argv[2], int(sys.argv[3]), [int(v) for v in sys.argv[4].split(",")]
with av.open(path) as c:
    for i, f in enumerate(c.decode(video=0)):
        if i == index:
            img = f.to_image().crop(box)
            img.resize((img.width * 4, img.height * 4)).save(out)
            break
