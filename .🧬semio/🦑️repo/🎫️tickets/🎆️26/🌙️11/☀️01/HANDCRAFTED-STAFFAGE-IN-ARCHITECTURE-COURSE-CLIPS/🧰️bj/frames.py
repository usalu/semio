"""🖼️ bj: rasterise strips of cast frames after export.py: frames.py <outdir> <cast> ... (env BJ_FRAMES=0,8,16 picks indices, BJ_H=520 cell height, BJ_CROP=x0,y0,x1,y1 fractions)."""
import json
import os
import sys
from pathlib import Path

import pymupdf
from PIL import Image

out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
H = int(os.environ.get("BJ_H", "520"))
folders = json.loads((Path(__file__).parent / "folders.json").read_text(encoding="utf-8"))
for cast in sys.argv[2:]:
    folder = Path(folders[cast])
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    frames = index["frames"]
    n = len(frames)
    picks = [int(v) for v in os.environ["BJ_FRAMES"].split(",")] if "BJ_FRAMES" in os.environ else [0, n // 2, n - 1]
    picks = [min(p, n - 1) for p in picks]
    print(cast, "frames", n, "fps", index["fps"], "picks", picks)
    cells = []
    for p in picks:
        page = pymupdf.open(stream=(folder / frames[p]).read_bytes(), filetype="svg")[0]
        zoom = H / page.rect.height
        pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=True)
        image = Image.frombytes("RGBA", (pix.width, pix.height), pix.samples)
        if "BJ_CROP" in os.environ:
            a, b, c, d = (float(v) for v in os.environ["BJ_CROP"].split(","))
            image = image.crop((int(a * image.width), int(b * image.height), int(c * image.width), int(d * image.height)))
        cells.append(image)
    width = max(c.width for c in cells)
    height = max(c.height for c in cells)
    sheet = Image.new("RGB", (width * len(cells), height), "#101418")
    for i, c in enumerate(cells):
        sheet.paste(c, (i * width, 0), c)
    sheet.save(out / f"{cast}.png")
