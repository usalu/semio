import sys
from lib import *

asset = sys.argv[1]
scales = sys.argv[2].split(",")
d = load(asset)
r, bones = rest(d)
for sc in scales:
    rep = d["asset"]["representations"][sc]
    print("== scale", sc, "pens", rep["pens"], "hatches", [h.get("id") for h in rep.get("hatches", [])])
    allp = []
    for s in rep["geometry"]["elevation"]:
        pts = world_poly(r, s)
        if s["role"] != "ground":
            allp += pts
        b = bbox(pts) if pts else None
        print(f"  {s['id']:22s} {s['bone']:14s} {s['kind']:7s} {s['role']:7s} z={s['z']:<6} tone={s['tone']:<5} reg={s.get('region') or s.get('regions')} hatch={s.get('hatch')} bb=" + (str([round(v, 2) for v in b]) if b else "-") + " extra=" + str({k: v for k, v in s.items() if k in ("penMm", "filled")}))
    print("  TOTAL bbox", [round(v, 3) for v in bbox(allp)])
