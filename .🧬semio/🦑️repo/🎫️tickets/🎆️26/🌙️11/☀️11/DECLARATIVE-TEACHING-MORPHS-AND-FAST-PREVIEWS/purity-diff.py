import sys
from pathlib import Path
import numpy as np
from PIL import Image
d = Path(sys.argv[1])
for t in ("0.0", "0.07", "0.25", "0.5", "0.999", "1.0"):
    one = np.asarray(Image.open(d / f"one{t}-{t}.png").convert("RGB")).astype(int)
    for tag in ("fwd", "rev", "shuf"):
        other = np.asarray(Image.open(d / f"{tag}-{t}.png").convert("RGB")).astype(int)
        diff = np.abs(one - other)
        ys, xs = np.nonzero(diff.sum(axis=2))
        print(t, tag, int((diff.sum(axis=2) > 0).sum()), (xs.min(), xs.max(), ys.min(), ys.max()) if len(xs) else None)
