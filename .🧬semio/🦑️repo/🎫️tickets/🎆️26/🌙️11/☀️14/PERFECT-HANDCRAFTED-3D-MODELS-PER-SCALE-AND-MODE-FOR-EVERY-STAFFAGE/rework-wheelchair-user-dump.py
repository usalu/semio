"""🔎 wheelchair-user: world boxes of the solids of one list of a candidate document (rest pose), to measure gaps and proportions by hand.

Usage: python3 rework-wheelchair-user-dump.py <document> <scale> <list> [id-regex]
"""
import copy, importlib.util, json, os, re, sys
T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_kit", os.path.join(T, "rig-lateral-standing-adult.py"))
K = importlib.util.module_from_spec(spec); spec.loader.exec_module(K)
doc = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
frames = K.frames(copy.deepcopy(doc["rig"]["bones"]))
pattern = re.compile(sys.argv[4] if len(sys.argv) > 4 else ".")
def world(bone, p):
    o, m = frames[bone]
    q = K.apply(m, p)
    return [o[i] + q[i] for i in range(3)]
by = {s["id"]: s for s in doc["volumes"][sys.argv[2]][sys.argv[3]]}
for s in doc["volumes"][sys.argv[2]][sys.argv[3]]:
    if not pattern.search(s["id"]):
        continue
    g = s if "kind" in s else None
    ref = ""
    if g is None:
        src = s.get("solid") or s.get("ref") or s.get("of")
        ref = " ref " + json.dumps({k: v for k, v in s.items() if k not in ("id",)})[:150]
        print(s["id"], ref)
        continue
    k = g["kind"]
    pts, r = [], 0
    if k == "capsule":
        pts, r = [g["p0"], g["p1"]], g.get("radius", g.get("r", 0))
    elif k == "loft":
        pts = [x["at"] for x in g["sections"]]; r = max(max(x["rx"], x["ry"]) for x in g["sections"])
    elif k == "ellipsoid":
        pts = [g["transform"]["translation"]]; r = max(g["radii"])
    else:
        t = (g.get("transform") or {}).get("translation", [0, 0, 0])
        pts = [t]; r = -1
    w = [world(g["bone"], p) for p in pts]
    lo = [round(min(p[i] for p in w) - max(r, 0), 3) for i in range(3)]
    hi = [round(max(p[i] for p in w) + max(r, 0), 3) for i in range(3)]
    extra = {a: g[a] for a in ("part", "member", "members", "slender", "sheet", "role", "material") if a in g}
    print(s["id"], g["bone"], k, "r", round(r, 4), "box", lo, hi, json.dumps(extra)[:140], "" if k in ("capsule", "loft", "ellipsoid") else json.dumps({a: b for a, b in g.items() if a not in ("id", "bone", "kind", "material", "tone", "role")})[:260])
