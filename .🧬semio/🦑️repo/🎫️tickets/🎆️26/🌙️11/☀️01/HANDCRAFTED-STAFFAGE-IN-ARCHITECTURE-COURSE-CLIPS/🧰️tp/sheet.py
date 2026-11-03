"""🖼️ tp: contact sheet of first, middle and last frame per cast: sheet.py <out.png> <folders.json> <cast> ... (system python with pymupdf and PIL)"""
import json
import sys
from pathlib import Path

import pymupdf
from PIL import Image

out = Path(sys.argv[1])
folders = json.load(open(sys.argv[2], encoding="utf8"))
casts = sys.argv[3:]
CELL = 260
sheet = Image.new("RGB", (CELL * 3, CELL * len(casts)), "#101418")
for row, cast in enumerate(casts):
    folder = Path(folders[cast])
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    frames = index["frames"]
    picks = [frames[0], frames[len(frames) // 2], frames[-1]]
    for col, name in enumerate(picks):
        document = pymupdf.open(stream=(folder / name).read_bytes(), filetype="svg")
        page = document[0]
        zoom = min((CELL - 8) / page.rect.width, (CELL - 8) / page.rect.height)
        pixmap = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)
        image = Image.frombytes("RGBA", (pixmap.width, pixmap.height), pixmap.samples)
        sheet.paste(image, (col * CELL + (CELL - pixmap.width) // 2, row * CELL + (CELL - pixmap.height) // 2), image)
sheet.save(out)
print(out, sheet.size)
