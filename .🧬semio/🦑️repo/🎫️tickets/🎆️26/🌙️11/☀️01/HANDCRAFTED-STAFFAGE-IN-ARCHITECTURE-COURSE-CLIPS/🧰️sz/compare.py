"""🧪 Compares two cast exports: byte totals, the largest frame, equal index files and pixel equality of rasterised frames (first, middle, last and the largest of every cast, PyMuPDF at 300 dpi with alpha).

@see ./anatomy.py — attribute shares
"""

import json
import sys
from pathlib import Path

import numpy as np
import pymupdf

before, after = Path(sys.argv[1]), Path(sys.argv[2])


def pixels(path: Path) -> np.ndarray:
    document = pymupdf.open(stream=path.read_bytes(), filetype="svg")
    pixmap = document[0].get_pixmap(dpi=300, alpha=True)
    return np.frombuffer(pixmap.samples, dtype=np.uint8).reshape(pixmap.height, pixmap.width, pixmap.n)


totals = {"before": 0, "after": 0}
largest = {"before": 0, "after": 0}
files = 0
unique = {"before": 0, "after": 0}
checked = different = index_different = 0
worst = []
for cast in sorted(p.name for p in before.iterdir() if p.is_dir()):
    a, b = before / cast, after / cast
    index_a, index_b = json.loads((a / "🔣️.json").read_text(encoding="utf-8")), json.loads((b / "🔣️.json").read_text(encoding="utf-8"))
    if index_a != index_b:
        index_different += 1
    names = sorted(p.name for p in a.glob("*.svg"))
    assert names == sorted(p.name for p in b.glob("*.svg")), cast
    for name in names:
        sa, sb = (a / name).stat().st_size, (b / name).stat().st_size
        totals["before"] += sa
        totals["after"] += sb
        largest["before"] = max(largest["before"], sa)
        largest["after"] = max(largest["after"], sb)
        files += 1
    biggest = max(names, key=lambda n: (a / n).stat().st_size)
    sample = sorted({names[0], names[len(names) // 2], names[-1], biggest})
    for name in sample:
        checked += 1
        pa, pb = pixels(a / name), pixels(b / name)
        if pa.shape != pb.shape or not np.array_equal(pa, pb):
            different += 1
            worst.append((cast, name, pa.shape, pb.shape, int(np.abs(pa.astype(int) - pb.astype(int)).max()) if pa.shape == pb.shape else -1))
    worst.sort(key=lambda w: -w[4])
print(json.dumps({"files": files, "bytes": totals, "max": largest, "casts": len(list(p for p in before.iterdir() if p.is_dir())),
                  "raster samples": checked, "raster different": different, "index different": index_different, "differences": worst[:10]}, indent=1, ensure_ascii=False))
