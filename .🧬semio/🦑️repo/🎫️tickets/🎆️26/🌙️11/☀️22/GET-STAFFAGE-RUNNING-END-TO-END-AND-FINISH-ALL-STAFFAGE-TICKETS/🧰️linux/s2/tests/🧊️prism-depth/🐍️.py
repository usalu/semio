"""🧊️ Oracle of the prism depth (🧪️tests/🧊️prism-depth): numpy places every profile point of an extruded solid at both ends of its sweep in the frame of its bone and reads the nearest coordinate of each view.

Input: the path of a JSON document `{length, restOriginZ, prisms: [{id, solid, points}], frames: {pose: {o, u, v, w}}}` (vectors as `[x, y, z]`). Output: JSON `{pose: {prism id: {"elevation": d, "front": d, "plan": d}}}`.
The sweeps by their definition in the schema: `w` has the profile in (u, v) and the span along w measured from the rest origin of the bone in world z; `v` has the profile in (u, w) and the span along v; `u` has the profile in (v, w) and the span along u. A span tapers linearly along the first profile coordinate to `zTip` at the bone length, clamped to the bone. The nearest point is the highest in the plan (+y), the foremost in the front (+x) and the one of the smallest lateral position in the side elevation (-z).
"""
import json
import sys

import numpy as np

data = json.load(open(sys.argv[1], encoding="utf8"))
length = data["length"]


def corners(prism, frame):
    solid = prism["solid"]
    o, u, v, w = (np.array(frame[key], dtype=float) for key in ("o", "u", "v", "w"))
    out = []
    for a, b in prism["points"]:
        t = min(max(a / length, 0.0), 1.0) if "zTip" in solid and length > 0 else 0.0
        for end in (0, 1):
            start = solid["z0"] if end == 0 else solid["z1"]
            tip = solid["zTip"][end] if "zTip" in solid else start
            span = start + (tip - start) * t
            if solid["sweep"] == "w":
                local = (a, b, span - data["restOriginZ"])
            elif solid["sweep"] == "v":
                local = (a, span, b)
            else:
                local = (span, a, b)
            out.append(o + local[0] * u + local[1] * v + local[2] * w)
    return np.array(out)


result = {}
for pose, frame in data["frames"].items():
    result[pose] = {}
    for prism in data["prisms"]:
        points = corners(prism, frame)
        result[pose][prism["id"]] = {"elevation": float(np.max(-points[:, 2])), "front": float(np.max(points[:, 0])), "plan": float(np.max(points[:, 1]))}
print(json.dumps(result))
