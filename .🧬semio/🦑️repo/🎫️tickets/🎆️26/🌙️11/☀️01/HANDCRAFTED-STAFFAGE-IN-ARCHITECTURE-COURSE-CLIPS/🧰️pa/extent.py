"""📏 pa: drawn extent of an asset at 1:50 in a pose by forward kinematics - usage: extent.py <asset-id> <pose>."""
import math
import sys
from collections import Counter

from jio import load

asset_id, pose_id = sys.argv[1], sys.argv[2]
_, data = load(asset_id)
a = data["asset"]
bones = {b["id"]: b for b in a["rig"]["bones"]}
pose = next(p for p in a["poses"] if p["id"] == pose_id)
override = {k: v.get("angleDeg") for k, v in pose.get("bones", {}).items()}


def chain(bone_id):
    out = []
    while bone_id is not None:
        out.append(bones[bone_id])
        bone_id = bones[bone_id]["parent"]
    return list(reversed(out))


def world(bone_id, x, y):
    for b in reversed(chain(bone_id)):
        ang = math.radians(override.get(b["id"], b["angleDeg"]) or 0)
        x, y = x * math.cos(ang) - y * math.sin(ang) + b["x"], x * math.sin(ang) + y * math.cos(ang) + b["y"]
    return x, y


def points(shape):
    k = shape["kind"]
    if k == "rect":
        x, y, w, h = shape["x"], shape["y"], shape["w"], shape["h"]
        return [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
    if k == "poly":
        return [tuple(p) for p in shape["points"]]
    if k == "circle":
        cx, cy, r = shape.get("cx", shape.get("x", 0)), shape.get("cy", shape.get("y", 0)), shape["r"]
        return [(cx - r, cy - r), (cx + r, cy + r)]
    return []


scale = sys.argv[3] if len(sys.argv) > 3 else "50"
shapes = a["representations"][scale]["geometry"]["elevation"]
print(Counter(s["kind"] for s in shapes))
xs, ys = [], []
for s in shapes:
    for px, py in points(s):
        wx, wy = world(s["bone"], px, py)
        xs.append(wx)
        ys.append(wy)
print("x", round(min(xs), 3), round(max(xs), 3), "w", round(max(xs) - min(xs), 3))
print("y", round(min(ys), 3), round(max(ys), 3), "h", round(max(ys) - min(ys), 3))
