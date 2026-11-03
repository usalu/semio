"""Contact sheet of exported casts on the dark clip background: python sheet.py <cast-dir> <out.png> [height-px] [frame-index]"""
import json
import os
import sys

import pymupdf
from PIL import Image, ImageDraw

root, out = sys.argv[1], sys.argv[2]
cell_h = int(sys.argv[3]) if len(sys.argv) > 3 else 420
pick = int(sys.argv[4]) if len(sys.argv) > 4 else 0
tiles = []
for name in sorted(os.listdir(root)):
    index_path = os.path.join(root, name, "\U0001f523️.json")
    if not os.path.isfile(index_path):
        continue
    index = json.load(open(index_path, encoding="utf-8"))
    frame = index["frames"][min(pick, len(index["frames"]) - 1)]
    doc = pymupdf.open(os.path.join(root, name, frame))
    page = doc[0]
    zoom = (cell_h - 30) / page.rect.height
    zoom = min(zoom, 900 / page.rect.width)
    pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)
    image = Image.frombytes("RGBA", (pix.width, pix.height), pix.samples)
    tiles.append((name, image))
width = sum(max(i.width, 120) + 16 for _, i in tiles) + 16
sheet = Image.new("RGB", (width, cell_h + 20), (18, 20, 28))
draw = ImageDraw.Draw(sheet)
x = 16
for name, image in tiles:
    sheet.paste(image, (x, 24), image)
    draw.text((x, 4), name[:34], fill=(200, 200, 200))
    x += max(image.width, 120) + 16
sheet.save(out)
print(sheet.size)
