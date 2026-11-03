import sys, glob, os, fitz
from PIL import Image
PV = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3\pv"
out = []
for pat in sys.argv[2:]:
    for f in sorted(glob.glob(os.path.join(PV, sys.argv[1], f"*{pat}*.svg")))[:1]:
        pix = fitz.open(f)[0].get_pixmap(dpi=150)
        im = Image.frombytes("RGB", (pix.width, pix.height), pix.samples)
        out.append(im)
w = max(i.width for i in out); h = sum(i.height for i in out)
sheet = Image.new("RGB", (w, h), "white"); y = 0
for i in out: sheet.paste(i, (0, y)); y += i.height
sheet.save(os.path.join(PV, f"zoom-{sys.argv[1]}.png"))
