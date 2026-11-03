import sys, os, glob, warnings
warnings.filterwarnings("ignore")
import pymupdf
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "QA12", "prev")
dpi = int(sys.argv[1]) if len(sys.argv) > 1 else 150
pat = sys.argv[2] if len(sys.argv) > 2 else "*"
for svg in sorted(glob.glob(os.path.join(D, pat + ".svg"))):
    png = svg[:-4] + ".png"
    if os.path.exists(png) and os.path.getmtime(png) >= os.path.getmtime(svg):
        continue
    pymupdf.open(svg)[0].get_pixmap(dpi=dpi).save(png)
print("done")
