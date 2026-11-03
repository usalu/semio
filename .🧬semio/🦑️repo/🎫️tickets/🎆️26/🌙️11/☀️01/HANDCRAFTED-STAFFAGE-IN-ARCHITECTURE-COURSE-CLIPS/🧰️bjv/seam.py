"""📏 bjv: mean absolute pixel difference of the seam step (last frame to frame 0) against ordinary adjacent-frame steps of the same cast: seam.py <outdir> <cast> ..."""
import json
import sys
from pathlib import Path

import pymupdf
from PIL import Image, ImageChops, ImageStat

out = Path(sys.argv[1])
folders = json.loads((out / "folders.json").read_text(encoding="utf-8"))


def raster(path):
    page = pymupdf.open(stream=path.read_bytes(), filetype="svg")[0]
    zoom = 260 / page.rect.height
    pix = page.get_pixmap(matrix=pymupdf.Matrix(zoom, zoom), alpha=False)
    return Image.frombytes("RGB", (pix.width, pix.height), pix.samples)


def mad(a, b):
    return sum(ImageStat.Stat(ImageChops.difference(a, b)).mean) / 3


result = {}
for cast in sys.argv[2:]:
    folder = Path(folders[cast])
    frames = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))["frames"]
    n = len(frames)
    images = [raster(folder / f) for f in frames]
    steps = [mad(images[i], images[i + 1]) for i in range(n - 1)]
    seam = mad(images[-1], images[0])
    result[cast] = {
        "frames": n,
        "mean_step": round(sum(steps) / len(steps), 3),
        "max_step": round(max(steps), 3),
        "seam_step": round(seam, 3),
        "frame0_vs_frame1": round(mad(images[0], images[1]), 3),
        "frame_last_vs_frame_last_minus_1": round(mad(images[-1], images[-2]), 3),
    }
    print(cast, result[cast])
(out / "seam-metric.json").write_text(json.dumps(result, ensure_ascii=False, indent=1), encoding="utf-8")
