import json,sys
from dump import *
def r(v): return round(v,4) if isinstance(v,float) else v
def short(s):
    o={k:r(v) for k,v in s.items() if k not in("role","filled")}
    return json.dumps(o,ensure_ascii=False,separators=(",",":"))
a=sys.argv[1]; d=load(a); W=world(d)
for b in d["rig"]["bones"]: print("bone",b["id"],"par",b["parent"],"world",[round(v,3) for v in W[b["id"]]],"len",b["lengthM"],b["joint"])
print("poses:",[(p["id"],p["bones"]) for p in d["poses"]])
print("clips:",[(c["id"],c["durationS"],[(k["t"],k["pose"]) for k in c["keys"]], [t["target"] for t in c.get("tracks",[]) if not t["target"].startswith(("operating","emitter"))]) for c in d["clips"]])
print("regions",[r_["id"] for r_ in d["thermal"]["regions"]],"mats",[m["id"] for m in d["materials"]])
for sc in sys.argv[2:]:
    rep=d["representations"][sc]
    print("=== scale",sc,rep["status"],rep.get("pens"),rep.get("hatches"))
    for s in rep["geometry"]["elevation"]: print("  ",short(s))
