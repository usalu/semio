import sys, math; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("bamboo-clump")
raw, doc, idx = load(p)
a = doc["asset"]
SC = sys.argv[1] if len(sys.argv) > 1 else "50"
g = a["representations"][SC]["geometry"]
front = {s["id"]: s for s in g["front"]}
shapes = []
for s in a["volume"]["solids"]:
    if s["kind"] != "ellipsoid" or not s["id"].startswith("mass-"): continue
    q = s["transform"]["rotation"]; t = s["transform"]["translation"]
    ang = math.degrees(2 * math.atan2(q[2], q[3]))
    f = front[s["id"]]
    shapes.append({"id": s["id"], "kind": "ellipse", "bone": s["bone"], "role": "body", "z": 2.9, "filled": True, "tone": f["tone"], "region": f["region"],
                   "cx": t[0], "cy": t[1], "rx": s["radii"][0], "ry": s["radii"][1], "rotationDeg": round(ang, 3)})
k = max(i for i, s in enumerate(g["elevation"]) if s["id"].startswith(("culm", "node")))
print(len(shapes), apply(p, [("insert_after", ("asset", "representations", SC, "geometry", "elevation", k), shapes)], expect=raw))
