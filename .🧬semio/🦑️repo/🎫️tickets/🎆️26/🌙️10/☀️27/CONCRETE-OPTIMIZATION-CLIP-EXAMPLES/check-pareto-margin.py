"""📏 Report keyframes with ink closer than 24 px to the left, right or top frame edge."""

import sys
from pathlib import Path

import numpy as np
from PIL import Image

folder = Path(sys.argv[1])
for path in sorted(folder.glob("Beat5_ParetoFront-*-*.png")):
    pixels = np.asarray(Image.open(path).convert("RGB")).astype(int)
    ink = np.abs(pixels - pixels[2, 2]).sum(axis=2) > 60
    for name, band in (("left", ink[:, :24]), ("right", ink[:, -24:]), ("top", ink[:24, :])):
        if band.any():
            where = np.argwhere(band)[0]
            print(f"[DEBUG] {path.name}: ink within 24 px of {name} at {tuple(where[::-1])}")
print("[DEBUG] margins checked")
