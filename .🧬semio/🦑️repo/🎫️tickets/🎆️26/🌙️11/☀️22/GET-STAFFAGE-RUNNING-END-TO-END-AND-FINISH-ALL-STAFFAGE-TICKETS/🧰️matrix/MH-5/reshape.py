"""🪚️ Prints `set` ops that give one shape, in the three views, the geometry it has in another band with its profile scaled: `python3 reshape.py <id> <asset file> <scale> <shape id> <from scale> <su> <sb> [pu] [pb]` scales the first profile coordinate by `su` about `pu` (default: its maximum) and the second by `sb` about `pb` (default: its middle); the solid is kept. The ops state the whole geometry, so applying them twice changes nothing."""
import json, sys
KEYS = ["kind", "points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"]
_, _id, path, scale, shape, source, su, sb = sys.argv[:8]
su, sb = float(su), float(sb)
doc = json.load(open(path))["asset"]
ops = []
for view in ("elevation", "front", "plan"):
    src = next(s for s in doc["representations"][source]["geometry"][view] if s["id"] == shape)
    pts = src["points"]
    pu = float(sys.argv[8]) if len(sys.argv) > 8 else max(p[0] for p in pts)
    pb = float(sys.argv[9]) if len(sys.argv) > 9 else (min(p[1] for p in pts) + max(p[1] for p in pts)) / 2
    geometry = {key: src.get(key) for key in KEYS}
    geometry["points"] = [[round(pu + (p[0] - pu) * su, 5), round(pb + (p[1] - pb) * sb, 5)] for p in pts]
    ops.append({"op": "set", "scale": scale, "view": view, "id": shape, "bone": src["bone"], "set": geometry})
print(json.dumps(ops + [{"op": "measure", "scale": scale, "id": shape, "bone": ops[0]["bone"]}]))
