"""Author 29: rebuilds the plume-back emitter solid of a source as a thin horizontal disc lying on the back (thin axis up) instead of the drafter's upright wall.
Usage: python3 author-29-emit.py <asset> <scale> <bone> <boneAngleDeg> <lift>"""
import json, math, sys
asset, scale, bone, ang, lift = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5])
f = f"volumes/{asset}.json"
d = json.load(open(f))
for t in d["volumes"][scale]["thermal"]:
    if "plume-back" in t.get("tags", []) and t.get("kind") == "ellipsoid":
        a = math.radians(90 - ang) / 2
        t["transform"]["rotation"] = [0, 0, round(math.sin(a), 5), round(math.cos(a), 5)]
        t["transform"]["translation"][1] = round(t["transform"]["translation"][1] + lift, 5)
        print("plume disc fixed", t["id"])
for t in d["volumes"][scale]["thermal"]:
    if ("breath-nostril" in t.get("tags", []) or "breath-nares" in t.get("tags", []) or "breath-nose" in t.get("tags", [])) and t.get("kind") == "capsule":
        p0, p1 = t["p0"], t["p1"]
        k = 0.45
        t["p1"] = [round(p0[i] + (p1[i] - p0[i]) * k, 5) for i in range(3)]
        print("nostril jet shortened", t["id"])
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
