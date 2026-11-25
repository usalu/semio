"""🪞️ Oracle of the slender in-band measure (🧪️tests/🪞️slender-in-band): shapely scales two point sets to their box and measures their Hausdorff distance under the eight candidate transforms.

Input: the path of the fixture `{cases: [{elevation: [ring...], other: [ring...]}]}` (closed rings as `[x, y]` lists). Output: JSON `[{"relative": d, "plain": d}]`, the smallest distance over the transforms, relative to the box of the two sets and plain.
A view is sampled at the vertices and edge midpoints of its rings, centred on its bounding box and scaled to a unit longest side.
"""
import json
import sys

from shapely import affinity
from shapely.geometry import MultiPoint

TRANSFORMS = [(1, 0, 0, 1), (-1, 0, 0, 1), (1, 0, 0, -1), (-1, 0, 0, -1), (0, -1, 1, 0), (0, 1, -1, 0), (0, 1, 1, 0), (0, -1, -1, 0)]


def sample(rings):
    points = []
    for ring in rings:
        for i, (x, y) in enumerate(ring):
            nx, ny = ring[(i + 1) % len(ring)]
            points += [(x, y), ((x + nx) / 2, (y + ny) / 2)]
    cloud = MultiPoint(points)
    minx, miny, maxx, maxy = cloud.bounds
    side = max(maxx - minx, maxy - miny)
    moved = affinity.translate(cloud, -(minx + maxx) / 2, -(miny + maxy) / 2)
    return affinity.scale(moved, 1 / side, 1 / side, origin=(0, 0))


def extents(a, b):
    ax0, ay0, ax1, ay1 = a.bounds
    bx0, by0, bx1, by1 = b.bounds
    return max(ax1 - ax0, bx1 - bx0), max(ay1 - ay0, by1 - by0)


out = []
for case in json.load(open(sys.argv[1], encoding="utf8"))["cases"]:
    a, b = sample(case["elevation"]), sample(case["other"])
    relative, plain = [], []
    for m in TRANSFORMS:
        moved = affinity.affine_transform(b, [m[0], m[1], m[2], m[3], 0, 0])
        ex, ey = extents(a, moved)
        sx, sy = (1 / ex if ex > 0 else 0), (1 / ey if ey > 0 else 0)
        relative.append(affinity.scale(a, sx, sy, origin=(0, 0)).hausdorff_distance(affinity.scale(moved, sx, sy, origin=(0, 0))))
        plain.append(a.hausdorff_distance(moved))
    out.append({"relative": min(relative), "plain": min(plain)})
print(json.dumps(out))
