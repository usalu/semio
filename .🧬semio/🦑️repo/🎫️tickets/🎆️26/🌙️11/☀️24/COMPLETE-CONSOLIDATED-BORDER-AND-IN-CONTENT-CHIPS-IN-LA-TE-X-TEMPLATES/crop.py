"""🔍 Renders a horizontal band of a PDF page for close inspection.

usage: python crop.py <pdf> <page (1-based)> <y0> <y1> <png out> [dpi]
"""
import sys
import pymupdf

page = pymupdf.open(sys.argv[1])[int(sys.argv[2]) - 1]
band = pymupdf.Rect(0, float(sys.argv[3]), page.rect.width, float(sys.argv[4]))
page.get_pixmap(dpi=int(sys.argv[6]) if len(sys.argv) > 6 else 140, clip=band).save(sys.argv[5])
