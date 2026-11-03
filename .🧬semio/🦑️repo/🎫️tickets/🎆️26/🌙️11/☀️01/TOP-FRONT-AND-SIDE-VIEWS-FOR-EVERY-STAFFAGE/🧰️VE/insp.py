"""usage: python inspect.py <asset> [scale ...]  -> rest-world elevation shapes (id, bone, kind, bbox) and rest bones, to stdout"""
import sys

from lib import *

aid = sys.argv[1]
asset = load(aid)["asset"]
w = rest_world(asset)
print("bones (rest world x,y,angle,len,tip):")
for b in asset["rig"]["bones"]:
    x, y, a = w[b["id"]]
    tx, ty = tip(asset, w, b["id"])
    print(f"  {b['id']:12s} parent={str(b['parent']):10s} origin=({x:.3f},{y:.3f}) ang={math.degrees(a):.1f} len={b['lengthM']} tip=({tx:.3f},{ty:.3f}) {b['joint']['type']}")
for scale in sys.argv[2:]:
    rep = asset["representations"][scale]
    print("== scale", scale, rep["status"])
    if rep["status"] != "authored":
        continue
    print(report(asset, scale))
    for s in rep["geometry"]["elevation"]:
        pts = [to_world(w, s["bone"], x, y) for x, y in shape_points(s)]
        b = bbox(pts)
        extra = ""
        if s["kind"] == "path" and len(s["d"]) < 160:
            extra = s["d"]
        if s["kind"] == "poly" and len(s["points"]) <= 6:
            extra = str(s["points"])
        print(f"  {s['id']:22s} {s['bone']:10s} {s['role']:7s} z{s['z']:<4} {'F' if s['filled'] else '-'} t{s['tone']:<5} {str(s.get('region') or s.get('regions')):45s} {s.get('material')} {s['kind']:8s} x[{b[0]:.3f},{b[2]:.3f}] y[{b[1]:.3f},{b[3]:.3f}] {extra}")
