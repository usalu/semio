import json, sys, fitz
for svg, png in json.load(open(sys.argv[1], encoding="utf-8")):
    fitz.open(svg)[0].get_pixmap(dpi=int(sys.argv[2])).save(png)
