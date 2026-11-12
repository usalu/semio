import json, sys
c = json.load(open(f"../🗑️generated/authoring-humans-b/{sys.argv[1]}.ctx.json"))
d = json.load(open(f"../🗑️generated/drafts/humans/{sys.argv[1]}.json"))["report"]
print(c["dimensions"], "poses", c["poses"], "clips", c["clips"])
for b in c["bones"]:
    f = b["frame"]; print(f"  {b['id']:16}{str(b['parent']):14} o({f['o'][0]:.3f},{f['o'][1]:.3f},{f['o'][2]:.3f}) u({f['u'][0]:.2f},{f['u'][1]:.2f},{f['u'][2]:.2f}) L{b['lengthM']}")
for v in ("elevation", "front", "plan"):
    print("==", v)
    for s in c[v]:
        if not s["closed"] or s["role"] == "texture" or not s["rings"]: continue
        xs = [p[0] for r in s["rings"] for p in r]; ys = [p[1] for r in s["rings"] for p in r]
        print(f"  {s['id']:22}{s['bone']:14}{s['kind'][:4]:5}{s['role'][:4]:5} {s['material'] or ''} x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}]")
print("flags needs-author:", [f["subject"] for f in d["flags"] if f["severity"] == "needs-author"])
print("unmatched:", [(u["view"][0], u["id"]) for u in d["unmatched"]])
