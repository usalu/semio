import sys
from pathlib import Path
import numpy as np
from PIL import Image
d = Path(sys.argv[1]); t = sys.argv[2]
a = Image.open(d / f"one{t}-{t}.png").convert("RGB"); b = Image.open(d / f"fwd-{t}.png").convert("RGB")
diff = np.abs(np.asarray(a).astype(int) - np.asarray(b).astype(int)).sum(axis=2)
mask = Image.fromarray(((diff > 0) * 255).astype("uint8")).convert("RGB")
w, h = a.size
out = Image.new("RGB", (w * 3, h)); out.paste(a, (0, 0)); out.paste(b, (w, 0)); out.paste(mask, (2 * w, 0))
out.save(sys.argv[3])
