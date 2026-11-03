import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf8")
S = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
aid = sys.argv[1]
d = json.load(open(os.path.join(S, aid, "🔣️.json"), encoding="utf8"))
a = d["asset"]
print("KEYS", list(a.keys()))
print("DIM", json.dumps(a.get("dimensions")))
print("PRES", json.dumps(a.get("presentation"), ensure_ascii=False))
rig = a.get("rig")
print("RIG keys", list(rig.keys()) if isinstance(rig, dict) else type(rig))
bones = rig["bones"] if isinstance(rig, dict) and "bones" in rig else rig
for b in bones:
    print("BONE", json.dumps(b, ensure_ascii=False))
for k in ("poses", "clips"):
    v = a.get(k)
    if isinstance(v, list):
        for p in v:
            print(k.upper(), p.get("id"), "dur", p.get("durationS"), "loop", p.get("loop"), "keys", list(p.keys()))
    elif isinstance(v, dict):
        print(k.upper(), list(v.keys()))
reps = a["representations"]
for sc, r in reps.items():
    print("REP", sc, r.get("status"), "viewpoints" in r, list(r.get("geometry", {}).keys()) if r.get("geometry") else "")
    g = r.get("geometry")
    if g:
        for vw, sl in g.items():
            print("   ", vw, len(sl))
