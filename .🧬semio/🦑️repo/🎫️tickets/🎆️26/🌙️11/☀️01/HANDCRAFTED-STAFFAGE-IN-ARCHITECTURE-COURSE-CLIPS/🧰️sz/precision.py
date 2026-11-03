"""🔬 Experiment: how many pixels of the oak frame change when the path coordinates are rounded to 3 decimals (1 micrometre) instead of 4, at 300 dpi.

@see ./compare.py — the exact comparison
"""

import re
import sys
from pathlib import Path

import numpy as np
import pymupdf

text = Path(sys.argv[1]).read_text(encoding="utf-8")


def coarse(match: re.Match) -> str:
    return 'd="' + re.sub(r"-?\d*\.\d+", lambda n: re.sub(r"\.?0+$", "", f"{float(n.group(0)):.3f}") or "0", match.group(1)) + '"'


def pixels(svg: str) -> np.ndarray:
    pixmap = pymupdf.open(stream=svg.encode(), filetype="svg")[0].get_pixmap(dpi=300, alpha=True)
    return np.frombuffer(pixmap.samples, dtype=np.uint8).reshape(pixmap.height, pixmap.width, pixmap.n).astype(int)


reduced = re.sub(r'd="([^"]*)"', coarse, text)
a, b = pixels(text), pixels(reduced)
print("bytes", len(text), len(reduced), "pixels differing", int((a != b).any(axis=2).sum()), "of", a.shape[0] * a.shape[1], "max", int(np.abs(a - b).max()))
