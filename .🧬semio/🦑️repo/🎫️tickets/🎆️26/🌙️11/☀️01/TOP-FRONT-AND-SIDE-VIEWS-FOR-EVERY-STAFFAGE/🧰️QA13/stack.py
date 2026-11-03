import sys, glob, os, re
from PIL import Image
root = sys.argv[1]; per = int(sys.argv[2]) if len(sys.argv) > 2 else 3
out = root + "/../stack"; os.makedirs(out, exist_ok=True)
for d in sorted(glob.glob(root + "/*")):
    a = os.path.basename(d)
    groups = {}
    for p in sorted(glob.glob(d + "/*.png")):
        m = re.search(r"-1-(\d+)\.png$", p); sc = m.group(1)
        groups.setdefault(sc, []).append(p)
    for sc, ps in groups.items():
        for i in range(0, len(ps), per):
            ims = [Image.open(p).convert("RGB") for p in ps[i:i+per]]
            # crop the footer? keep
            w = max(im.width for im in ims); h = sum(im.height for im in ims)
            c = Image.new("RGB", (w, h), "white"); y = 0
            for im in ims: c.paste(im, (0, y)); y += im.height
            c.save(f"{out}/{a}-{sc}-{i//per:02d}.png")
