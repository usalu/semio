import sys, json
from pathlib import Path
import numpy as np
from PIL import Image
a, b = Path(sys.argv[1]), Path(sys.argv[2])
for p in sorted(a.glob("*.png")):
    x = np.asarray(Image.open(p).convert("RGB")).astype(int); y = np.asarray(Image.open(b / p.name).convert("RGB")).astype(int)
    d = np.abs(x - y)
    if d.max(): print(p.name, "maxdiff", d.max(), "pixels", int((d.sum(2) > 0).sum()), "of", d.shape[0] * d.shape[1])
ma = json.loads((a / "out.json").read_text(encoding="utf-8"))["metadata"]; mb = json.loads((b / "out.json").read_text(encoding="utf-8"))["metadata"]
for i, (p, q) in enumerate(zip(ma, mb)):
    if p != q:
        keys = [k for k in p if p[k] != q.get(k)]
        print("meta", i, keys)
