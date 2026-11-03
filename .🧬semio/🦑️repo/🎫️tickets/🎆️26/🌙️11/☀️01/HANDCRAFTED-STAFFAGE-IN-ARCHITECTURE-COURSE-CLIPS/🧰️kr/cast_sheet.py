import json
import sys
from pathlib import Path

import pymupdf as fitz
from PIL import Image, ImageDraw

folder = Path(json.load(open(Path(__file__).parents[1] / "🧰️tp" / "folders.json", encoding="utf8"))["physics-fundamentals-kettle"])
index = json.load(open(folder / "🔣️.json", encoding="utf8"))
fps = index["fps"]
times = [float(v) for v in sys.argv[2].split(",")]
tiles = []
for t in times:
    n = min(len(index["frames"]) - 1, round(t * fps))
    doc = fitz.open(str(folder / f"{n:04d}.svg"))
    pix = doc[0].get_pixmap(matrix=fitz.Matrix(12, 12), alpha=False)
    im = Image.frombytes("RGB", (pix.width, pix.height), pix.samples)
    ImageDraw.Draw(im).text((4, 4), f"{t:.0f}s #{n}", fill="white")
    tiles.append(im)
w, h = tiles[0].size
cols = 4
rows = (len(tiles) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for k, im in enumerate(tiles):
    sheet.paste(im, ((k % cols) * w, (k // cols) * h))
sheet.save(sys.argv[1])
print(w, h, len(tiles))
