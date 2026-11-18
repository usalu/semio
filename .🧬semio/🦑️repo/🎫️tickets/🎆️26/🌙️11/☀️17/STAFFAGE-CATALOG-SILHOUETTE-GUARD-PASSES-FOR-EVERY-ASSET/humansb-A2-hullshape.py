"""🔷 Ops entry replacing a plan/front shape by a flat polygon from world points; keeps the other fields of the shape. python3 humansb-A2-hullshape.py <asset> <scale> <view> <shapeId> '<json points>'"""
import json, sys
slug, sc, view, sid, pts = sys.argv[1:6]
d = json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json' % slug))['asset']
s = [x for x in d['representations'][sc]['geometry'][view] if x['id'] == sid][0]
n = {"kind": "poly", "points": json.loads(pts), "closed": True}
for k, v in s.items():
    if k not in ("kind", "points", "closed", "solid", "d", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"): n[k] = v
print(json.dumps([{"op": "shape", "scale": sc, "view": view, "id": sid, "shape": n}]))
