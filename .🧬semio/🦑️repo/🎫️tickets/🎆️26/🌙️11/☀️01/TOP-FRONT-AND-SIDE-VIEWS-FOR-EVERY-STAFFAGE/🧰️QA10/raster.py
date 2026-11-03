import sys, glob, os, warnings
warnings.filterwarnings("ignore")
import pymupdf
rnd = sys.argv[1]
base = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🗑️generated", "QA10", rnd)
n = 0
for svg in glob.glob(os.path.join(base, "*", "*.svg")):
    png = svg[:-4] + ".png"
    pymupdf.open(svg)[0].get_pixmap(dpi=150).save(png)
    n += 1
print(n)
