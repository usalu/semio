import sys, os, glob
from PIL import Image
# usage: montage.py <sheetsdir> <asset-scale> <out-prefix> [per] [filter-substring]
d, key, outp = sys.argv[1], sys.argv[2], sys.argv[3]
per = int(sys.argv[4]) if len(sys.argv) > 4 else 4
flt = sys.argv[5] if len(sys.argv) > 5 else ""
files = sorted(f for f in glob.glob(os.path.join(d, key, "*.png")) if flt in os.path.basename(f))
for i in range(0, len(files), per):
    ims = [Image.open(f).convert("RGB") for f in files[i:i + per]]
    w = max(im.width for im in ims); h = sum(im.height for im in ims)
    out = Image.new("RGB", (w, h), (255, 255, 255)); y = 0
    for im in ims:
        out.paste(im, (0, y)); y += im.height
    p = "%s-%d.png" % (outp, i // per)
    out.save(p); print(p, out.size, [os.path.basename(f).split("--")[1] for f in files[i:i + per]])
