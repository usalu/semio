"""🩹️ Moves the points of a traced shape of an ops file that lie on one side of a line across the gap to the neighbouring part, so that the drawn part overlaps its neighbour where the body leaves a joint narrower than a raster cell (the guard seals such a joint in the body, a drawing keeps it open): close-gap.py <id> <scale> <view> <shape id> <x|y> <gt|lt> <limit> <shift>"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
slug, scale, view, sid, axis, side, limit, shift = sys.argv[1:9]
k, limit, shift = "xy".index(axis), float(limit), float(shift)
path = f"{HERE}/ops/{slug}.json"
ops = json.load(open(path))
o = next(o for o in ops if o["op"] == "shape-add" and o["scale"] == scale and o["view"] == view and o["shape"]["id"] == sid)
moved = 0
for p in o["shape"]["points"]:
    if (p[k] > limit) if side == "gt" else (p[k] < limit):
        p[k] = round(p[k] + shift, 5)
        moved += 1
open(path, "w").write("[\n" + ",\n".join(json.dumps(x, ensure_ascii=False) for x in ops) + "\n]\n")
print(f"{slug} {scale} {view} {sid}: {moved} points moved by {shift}")
