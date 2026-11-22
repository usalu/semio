"""🔎 worker-carrying-box: lists the solids of one list and scale with their bone, kind and world centre in the rest pose."""
import importlib.util, json, os, sys
T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rl", os.path.join(T, "rig-lateral-standing-adult.py")); rl = importlib.util.module_from_spec(spec); spec.loader.exec_module(rl)
CAT = os.path.join(rl.REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "worker-carrying-box", "🔣️.json")
path = sys.argv[3] if len(sys.argv) > 3 else CAT
a = json.load(open(path))["asset"]
fr = rl.frames(a["rig"]["bones"])
hip = 0.94
def world(bone, p):
    o, b = fr[bone]; q = rl.apply(b, p); return [round(o[i] + q[i] + (hip if i == 1 else 0), 3) for i in range(3)]
def centre(s):
    k = s.get("kind")
    if k == "capsule": return [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)], "r%.3f/%.3f L%.3f" % (s["r0"], s["r1"], sum((s["p0"][i]-s["p1"][i])**2 for i in range(3))**.5)
    if k == "loft": 
        n = len(s["sections"]); return [sum(q["at"][i] for q in s["sections"]) / n for i in range(3)], "n%d rx%.3f..%.3f ry%.3f..%.3f" % (n, min(q["rx"] for q in s["sections"]), max(q["rx"] for q in s["sections"]), min(q["ry"] for q in s["sections"]), max(q["ry"] for q in s["sections"]))
    if k in ("ellipsoid", "box"): return s.get("transform", {}).get("translation", [0, 0, 0]), str(s.get("radii", s.get("size")))
    if k == "revolve": return s["origin"], "rev"
    return [0, 0, 0], k or "ref:" + str(s.get("of"))
for s in a["volumes"][sys.argv[1]][sys.argv[2]]:
    c, d = centre(s)
    b = s.get("bone")
    print(s["id"], b, s.get("kind", "-"), s.get("part", ""), s.get("member", s.get("region", "")), world(b, c) if b in fr else "", d, "slender" if s.get("slender") else "")
