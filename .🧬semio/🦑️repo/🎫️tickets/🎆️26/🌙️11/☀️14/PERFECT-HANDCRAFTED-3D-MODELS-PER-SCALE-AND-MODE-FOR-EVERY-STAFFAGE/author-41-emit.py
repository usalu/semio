"""Author 41: emitter fixes after kit-thermal (the plume becomes a thin horizontal disc lying on the back, the nose jet is shortened to 45 %).
Usage: python3 author-41-emit.py <asset> <scale> <plume-tag> <plumeRadiusM> <plumeThicknessM> [<jetShorten=0.45>]"""
import json, sys
asset, scale, tag, rad, thick = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5])
k = float(sys.argv[6]) if len(sys.argv) > 6 else 0.45
minr = float(sys.argv[7]) if len(sys.argv) > 7 else 0.0
f = f"volumes/{asset}.json"
d = json.load(open(f))
for t in d["volumes"][scale]["thermal"]:
    tags = t.get("tags", [])
    if tag in tags and t["kind"] == "ellipsoid":
        t["radii"] = [rad, max(thick, minr), rad]
        tr = list(t["transform"]["translation"])
        import os
        if "PLY" in os.environ: tr[1] = float(os.environ["PLY"])
        t["transform"] = {"translation": tr}
        print("plume disc", t["id"], t["radii"])
    if any(x in tags for x in ("breath-nose", "breath-nares", "breath-nostril")) and t["kind"] == "capsule":
        p0, p1 = t["p0"], t["p1"]
        t["p1"] = [round(p0[i] + (p1[i] - p0[i]) * k, 5) for i in range(3)]
        import os
        if "JET" in os.environ:
            a, b, c2, e = [float(x) for x in os.environ["JET"].split(",")]
            t["p0"], t["p1"] = [a, b, 0], [c2, e, 0]
        print("jet shortened", t["id"])
for t in d["volumes"][scale]["thermal"]:
    if "emitter" in t.get("tags", []) and t["kind"] == "ellipsoid" and "plume-back" not in t["tags"]:
        t["radii"] = [round(max(r, minr), 5) for r in t["radii"]]
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
