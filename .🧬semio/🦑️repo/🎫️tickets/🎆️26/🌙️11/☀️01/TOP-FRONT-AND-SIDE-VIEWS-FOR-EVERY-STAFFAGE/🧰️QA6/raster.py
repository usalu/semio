import glob, os, sys, re
import fitz
from PIL import Image, ImageDraw
Image.MAX_IMAGE_PIXELS = None
g = sys.argv[1]
only = sys.argv[2] if len(sys.argv) > 2 else ""
W = {"50": 2000, "100": 2000, "200": 1800, "500": 1800}
PER = {"50": 3, "100": 3, "200": 5, "500": 5}
for adir in sorted(glob.glob(g + "/*/")):
    asset = os.path.basename(adir.rstrip("/\\"))
    if asset in ("",) or (only and asset != only): continue
    for sdir in sorted(glob.glob(adir + "*/")):
        sc = os.path.basename(sdir.rstrip("/\\"))
        if sc not in W: continue
        ims = []
        for kind in ("dark-line", "thermal"):
            for f in sorted(glob.glob(f"{sdir}*--views-{kind}-1-{sc}.svg")):
                doc = fitz.open(f); page = doc[0]
                width = W[sc]; zoom = width / page.rect.width
                pix = page.get_pixmap(matrix=fitz.Matrix(zoom, zoom))
                im = Image.frombytes("RGB", (pix.width, pix.height), pix.samples)
                ImageDraw.Draw(im).text((4, 2), os.path.basename(f).split("--")[1] + " " + kind, fill=(200, 0, 0))
                ims.append(im)
        per = PER[sc]
        for i in range(0, len(ims), per):
            grp = ims[i:i + per]
            m = Image.new("RGB", (W[sc], sum(x.height for x in grp)), "white"); y = 0
            for x in grp: m.paste(x, (0, y)); y += x.height
            m.save(f"{adir}m{sc}-{i // per}.png")
        print(asset, sc, len(ims))
