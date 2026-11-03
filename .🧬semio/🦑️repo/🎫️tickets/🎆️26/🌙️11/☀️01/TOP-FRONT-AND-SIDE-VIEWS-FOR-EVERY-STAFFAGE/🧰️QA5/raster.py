import glob, os, sys, fitz
root = sys.argv[1]
for svg in sorted(glob.glob(os.path.join(root, "**", "*.svg"), recursive=True)):
    png = svg[:-4] + ".png"
    if not os.path.exists(png) or os.path.getmtime(png) < os.path.getmtime(svg):
        fitz.open(svg)[0].get_pixmap(dpi=int(sys.argv[2])).save(png)
