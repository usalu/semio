import sys, fitz
for p in sys.argv[1:]:
    doc = fitz.open(p)
    pix = doc[0].get_pixmap(dpi=96)
    pix.save(p[:-4] + ".png")
    print(p[:-4] + ".png", pix.width, pix.height)
