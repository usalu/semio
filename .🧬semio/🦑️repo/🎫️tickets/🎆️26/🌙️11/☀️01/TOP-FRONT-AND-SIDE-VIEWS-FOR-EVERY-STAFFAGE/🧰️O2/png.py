import fitz, sys, glob, os
zoom = float(sys.argv[1])
for pat in sys.argv[2:]:
    for f in glob.glob(pat):
        doc = fitz.open(f)
        pix = doc[0].get_pixmap(matrix=fitz.Matrix(zoom, zoom), alpha=False)
        pix.save(f[:-4] + ".png")
        print(pix.width, pix.height)
