"""🖼️ bjv: export five transport casts through the staffage cast exporter, rasterise frame 0 and the last frame, and compare them at the loop seam: render.py <outdir> <cast> ..."""
import importlib
import json
import sys
from pathlib import Path

import pymupdf
from PIL import Image, ImageChops

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
out = Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
casts = sys.argv[2:]
folders = {cast: str(S.ensure_casts([cast])[cast]) for cast in casts}
(out / "folders.json").write_text(json.dumps(folders, ensure_ascii=False, indent=1), encoding="utf-8")


def raster(path):
    page = pymupdf.open(stream=path.read_bytes(), filetype="svg")[0]
    zoom = 520 / page.rect.height
    pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=False)
    return Image.frombytes("RGB", (pix.width, pix.height), pix.samples)


report = {}
cells = []
for cast in casts:
    folder = Path(folders[cast])
    index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
    frames = index["frames"]
    n = len(frames)
    first = raster(folder / frames[0])
    last = raster(folder / frames[-1])
    diff = ImageChops.difference(first, last)
    bbox = diff.getbbox()
    nonzero = sum(1 for v in diff.convert("L").getdata() if v > 0)
    report[cast] = {"frames": n, "fps": index["fps"], "size": list(first.size), "seam_diff_pixels": nonzero, "seam_bbox": bbox}
    first.save(out / f"{cast}-frame0.png")
    cells.append(first)
    print(cast, "frames", n, "fps", index["fps"], "seam diff px", nonzero, "bbox", bbox)
width = max(c.width for c in cells)
height = max(c.height for c in cells)
sheet = Image.new("RGB", (width * len(cells), height), "#ffffff")
for i, c in enumerate(cells):
    sheet.paste(c, (i * width, 0))
sheet.save(out / "sheet-frame0.png")
(out / "seam.json").write_text(json.dumps(report, ensure_ascii=False, indent=1), encoding="utf-8")
