import json, math, pathlib
import numpy as np
from scipy.spatial import ConvexHull
out = pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🔀️morph/🧪️tests/🎞️full/🧫️cases")
rng = np.random.default_rng(11)
cases = []
for name, pts in (
    ("square-with-interior", [[0, 0], [4, 0], [4, 4], [0, 4], [1, 1], [2, 2], [3, 1], [2, 3]]),
    ("collinear-edge-points", [[0, 0], [1, 0], [2, 0], [2, 2], [0, 2], [1, 2], [1, 1]]),
    ("random-normal-60", np.round(rng.normal(size=(60, 2)), 2).tolist()),
    ("random-disc-400", np.round(rng.uniform(-1, 1, size=(400, 2)) * 3, 2).tolist()),
):
    arr = np.array(pts, dtype=float)
    hull = ConvexHull(arr)
    cases.append({"name": name, "points": pts, "hull": sorted([round(float(x), 2), round(float(y), 2)] for x, y in arr[hull.vertices])})
(out / "hull.json").write_text(json.dumps({"description": "Convex hull vertex sets computed with SciPy ConvexHull; any implementation must return exactly these vertices (rounded to 2 decimals).", "cases": cases}, indent=1) + "\n", encoding="utf-8")
remap = []
for duration, fps in ((1.0, 10), (0.5, 15), (0.2, 24), (0.1, 10), (2.0, 30)):
    n = math.ceil(duration * fps - 1e-9)
    remap.append({"duration": duration, "fps": fps, "frames": n, "progress": [1.0 if n == 1 else k / (n - 1) for k in range(n)]})
(out / "remap.json").write_text(json.dumps({"description": "Final-profile boundary remap: frame k of n inside Animation.interpolate shows progress k/(n-1) of the authored morph, the last frame (f1) exactly 1, a single frame is the target.", "cases": remap}, indent=1) + "\n", encoding="utf-8")
