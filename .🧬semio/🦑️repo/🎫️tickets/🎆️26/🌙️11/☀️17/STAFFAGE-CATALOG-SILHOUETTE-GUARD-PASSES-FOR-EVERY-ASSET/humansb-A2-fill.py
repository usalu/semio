"""🩹 Ops entry adding a fill polygon on the pelvis bone from WORLD points. python3 humansb-A2-fill.py <scale> <view> <after> <id> <tone> <region> '<json world points>' [material] [yoff=0.915]"""
import json, sys
sc, view, after, sid, tone, region, pts = sys.argv[1:8]
mat = sys.argv[8] if len(sys.argv) > 8 and sys.argv[8] != '-' else None
yoff = float(sys.argv[9]) if len(sys.argv) > 9 else 0.915
sh = {"kind": "poly", "points": [[round(x, 4), round(y - yoff, 4)] for x, y in json.loads(pts)], "closed": True, "id": sid, "bone": "pelvis", "role": "body", "z": 4.1, "filled": True, "tone": float(tone), "region": region}
if mat: sh["material"] = mat
print(json.dumps([{"op": "shape-add", "scale": sc, "view": view, "after": after, "shape": sh}]))
