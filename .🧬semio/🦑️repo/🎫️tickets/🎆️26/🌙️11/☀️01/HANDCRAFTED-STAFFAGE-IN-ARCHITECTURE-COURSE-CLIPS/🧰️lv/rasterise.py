import glob, os, sys
import fitz
base = sys.argv[1]
for svg in sorted(glob.glob(os.path.join(base, "*", "*.svg"))):
    doc = fitz.open(svg)
    page = doc[0]
    pix = page.get_pixmap(dpi=400)
    out = os.path.join(base, os.path.basename(os.path.dirname(svg)) + ".png")
    pix.save(out)
    print(os.path.basename(out).encode("ascii","replace").decode(), pix.width, pix.height)
