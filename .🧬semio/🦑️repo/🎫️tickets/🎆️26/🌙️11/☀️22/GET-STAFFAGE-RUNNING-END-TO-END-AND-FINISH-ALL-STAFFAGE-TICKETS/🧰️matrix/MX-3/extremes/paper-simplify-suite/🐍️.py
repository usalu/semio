"""✂️ Oracle of the paper simplification: shapely measures, for every pair of a ring and its simplification, the bounds of both and the Hausdorff distance between the two polygons.

Input: the path of a JSON document `{"pairs": [{"ring": [[x, y], ...], "simplified": [[x, y], ...]}]}`. Output: JSON `[{"original": [minx, miny, maxx, maxy], "simplified": [...], "hausdorff": number}]`.
"""
import json
import sys

from shapely.geometry import Polygon

pairs = json.load(open(sys.argv[1]))["pairs"]
out = []
for pair in pairs:
    original = Polygon(pair["ring"])
    simplified = Polygon(pair["simplified"])
    out.append({"original": list(original.bounds), "simplified": list(simplified.bounds), "hausdorff": original.hausdorff_distance(simplified)})
print(json.dumps(out))
