"""🚲️ Writes the repair ops of `used-bicycle-shelter`: at 1:500 the front draws the three purlins of the volume as bands over the full length like 1:200 does (the lowest one fills the 68 mm strip between gutter and roof panels that stood open, IoU 0.884); at 1:50 the outer edge of the rear tyre is drawn 0.5 mm further in in the side elevation, so the opening beside the tyre is more than 256 instead of 248 raster cells and stays open in the drawing as it does in the body (263 cells; the guard closes openings below 256 cells, see requests.md); the paper depot tag on the handlebar, an outline with no solid of its own, is drawn in the `texture` role (it counted as body 56 mm from the bike)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, write
a = asset("used-bicycle-shelter")
ops = []
previous = "gutter"
for k, (y0, y1) in enumerate(((2.3291, 2.4125), (2.4105, 2.4939), (2.4919, 2.5753)), 1):
    like = shape(a, 200, "front", f"purlin-{k}")
    ops.append({"op": "shape-add", "scale": "500", "view": "front", "after": previous, "shape": {**like, "points": [[-2.5, y0], [2.5, y0], [2.5, y1], [-2.5, y1]]}})
    previous = f"purlin-{k}"
R = 0.3505
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "tyre-rear", "set": {"d": f"M {R} 0.0 A {R} {R} 0 1 0 -{R} 0.0 A {R} {R} 0 1 0 {R} 0.0 Z M 0.311 0.0 A 0.311 0.311 0 1 0 -0.311 0.0 A 0.311 0.311 0 1 0 0.311 0.0 Z"}})
ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": "depot-tag", "set": {"role": "texture"}})
write("used-bicycle-shelter", ops)
