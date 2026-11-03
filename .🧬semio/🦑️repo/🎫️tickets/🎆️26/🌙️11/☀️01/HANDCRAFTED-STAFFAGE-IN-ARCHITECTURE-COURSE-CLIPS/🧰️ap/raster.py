import sys, glob
import fitz
for name in sys.argv[2:]:
    svg = glob.glob(f"{sys.argv[1]}/{name}/*.svg")[0]
    doc = fitz.open(svg)
    doc[0].get_pixmap(matrix=fitz.Matrix(3, 3)).save(f"{sys.argv[1]}/{name}.png")
    print(name, "ok")
