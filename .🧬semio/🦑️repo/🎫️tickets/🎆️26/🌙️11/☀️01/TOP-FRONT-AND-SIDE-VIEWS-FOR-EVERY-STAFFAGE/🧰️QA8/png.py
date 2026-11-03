import sys, os, glob, fitz
# usage: png.py <dir> [dpi]  -- rasterizes every svg (without a fresh png) in one process
d = sys.argv[1]; dpi = int(sys.argv[2]) if len(sys.argv) > 2 else 150
n = 0
for svg in glob.glob(os.path.join(d, "**", "*.svg"), recursive=True):
    png = svg[:-4] + ".png"
    if os.path.exists(png) and os.path.getmtime(png) >= os.path.getmtime(svg):
        continue
    fitz.open(svg)[0].get_pixmap(dpi=dpi).save(png); n += 1
print("png", n)
