"""🖼️ bp: contact sheet from an export folder: sheetdir.py <out.png> <exportdir> <cast> ... ; columns first, middle, last, plus the bounds of every cast."""
import json
import sys
from pathlib import Path

import pymupdf
from PIL import Image

out, root, casts = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3:]
CELL = 420
sheet = Image.new("RGB", (CELL * 3, CELL * len(casts)), "#101418")
for row, cast in enumerate(casts):
    folder = root / cast
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    frames = index["frames"]
    print(cast, "bounds", index["boundsM"], "height", index["heightM"], "frames", len(frames), "loop", index["loop"], "dur", index["durationS"])
    for col, name in enumerate([frames[0], frames[len(frames) // 2], frames[-1]]):
        page = pymupdf.open(stream=(folder / name).read_bytes(), filetype="svg")[0]
        zoom = min((CELL - 8) / page.rect.width, (CELL - 8) / page.rect.height)
        pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)
        image = Image.frombytes("RGBA", (pix.width, pix.height), pix.samples)
        sheet.paste(image, (col * CELL + (CELL - pix.width) // 2, row * CELL + (CELL - pix.height) // 2), image)
sheet.save(out)
