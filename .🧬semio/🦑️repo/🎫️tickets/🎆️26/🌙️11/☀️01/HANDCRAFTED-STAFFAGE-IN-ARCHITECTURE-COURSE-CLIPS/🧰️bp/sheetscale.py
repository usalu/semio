"""🔬 bp: frames at true clip scale: sheetscale.py <out.png> <exportdir> <figure_px> <cast> ... ; mid frame, figure height scaled to figure_px, magnified x3 nearest so pixels stay visible."""
import json
import sys
from pathlib import Path

import pymupdf
from PIL import Image

out, root, px, casts = Path(sys.argv[1]), Path(sys.argv[2]), float(sys.argv[3]), sys.argv[4:]
tiles = []
for cast in casts:
    folder = root / cast
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    name = index["frames"][len(index["frames"]) // 2]
    page = pymupdf.open(stream=(folder / name).read_bytes(), filetype="svg")[0]
    zoom = px / (index["heightM"] * index["mmPerMetre"]) * (96 / 25.4) * 0 or px / (index["heightM"] * index["mmPerMetre"])
    zoom = px / (index["heightM"] * index["mmPerMetre"] * 72 / 25.4)
    pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)
    image = Image.frombytes("RGBA", (pix.width, pix.height), pix.samples)
    bg = Image.new("RGBA", image.size, "#101418")
    bg.alpha_composite(image)
    tiles.append(bg.convert("RGB").resize((pix.width * 3, pix.height * 3), Image.NEAREST))
W = sum(t.width for t in tiles) + 8 * (len(tiles) - 1)
H = max(t.height for t in tiles)
sheet = Image.new("RGB", (W, H), "#202830")
x = 0
for t in tiles:
    sheet.paste(t, (x, 0)); x += t.width + 8
sheet.save(out)
print(sheet.size)
