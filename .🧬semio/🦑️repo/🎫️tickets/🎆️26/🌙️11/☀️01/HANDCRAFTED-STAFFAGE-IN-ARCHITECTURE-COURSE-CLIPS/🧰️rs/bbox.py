import sys
from pathlib import Path
import numpy as np
from PIL import Image
a, b = Path(sys.argv[1]), Path(sys.argv[2])
for n in sys.argv[3:]:
    x = np.asarray(Image.open(a / n).convert("RGB")).astype(int); y = np.asarray(Image.open(b / n).convert("RGB")).astype(int)
    m = np.abs(x - y).sum(2) > 0
    ys, xs = np.nonzero(m)
    print(n, xs.min(), xs.max(), ys.min(), ys.max(), x[ys[0], xs[0]], y[ys[0], xs[0]])
