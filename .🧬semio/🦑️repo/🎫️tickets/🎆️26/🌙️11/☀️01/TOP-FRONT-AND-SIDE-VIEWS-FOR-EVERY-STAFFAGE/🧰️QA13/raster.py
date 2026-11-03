import sys, glob, os, warnings
warnings.filterwarnings("ignore")
import fitz
root = sys.argv[1]
only = sys.argv[2:] 
n = 0
for svg in glob.glob(root + "/**/*.svg", recursive=True):
    png = svg[:-4] + ".png"
    if os.path.exists(png) and os.path.getmtime(png) >= os.path.getmtime(svg):
        continue
    if only and not any(o in svg for o in only):
        continue
    fitz.open(svg)[0].get_pixmap(dpi=150).save(png)
    n += 1
print("rasterized", n)
