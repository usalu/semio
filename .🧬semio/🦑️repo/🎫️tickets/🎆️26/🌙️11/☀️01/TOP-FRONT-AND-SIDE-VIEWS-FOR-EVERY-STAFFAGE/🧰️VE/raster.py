"""Rasterizes SVG previews of a folder to PNG with PyMuPDF. usage: python raster.py <dir> [dpi]"""
import glob
import os
import sys

import fitz

folder = sys.argv[1]
dpi = float(sys.argv[2]) if len(sys.argv) > 2 else 200
for svg in glob.glob(os.path.join(folder, "*.svg")):
    png = svg[:-4] + ".png"
    if os.path.exists(png) and os.path.getmtime(png) >= os.path.getmtime(svg):
        continue
    doc = fitz.open(svg)
    pix = doc[0].get_pixmap(matrix=fitz.Matrix(dpi / 72, dpi / 72), alpha=False)
    pix.save(png)
    print("[DEBUG]", png)
