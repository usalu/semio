import json, sys

path = sys.argv[1]
with open(path, encoding="utf-8") as f:
    d = json.load(f)
a = d["asset"]
print("KEYS", list(a.keys()))
print("DIMS", a.get("dimensions"))
print("PRESENTATION", a.get("presentation"))
print("COMPLETION", a.get("completion"))
mats = a.get("materials", [])
print("MATERIALS", [(m["id"], m.get("emissivity"), m.get("solarAbsorptivity")) for m in mats])
rig = a.get("rig", {})
print("RIG bones", [(b["id"], b.get("parent"), b.get("x"), b.get("y"), b.get("angleDeg"), b.get("lengthM")) for b in rig.get("bones", [])])
masses = [(b["id"], b.get("mass")) for b in rig.get("bones", []) if b.get("mass")]
tot = sum(m["kg"] for _, m in masses if m.get("kg") is not None)
print("MASS SUM", round(tot, 3), "vs massKg", a.get("dimensions", {}).get("massKg"))
print("ANCHORS", [(x["id"], x.get("bone"), x.get("kind"), x.get("x"), x.get("y")) for x in a.get("anchors", [])])
print("POSES", [(p.get("id"), list(p.keys())) for p in a.get("poses", [])])
print("CLIPS", [(c.get("id"), c.get("durationS"), c.get("loop")) for c in a.get("clips", [])])
th = a.get("thermal", {})
print("THERMAL keys", list(th.keys()))
body = th.get("body") or {}
print("BODY", json.dumps(body, ensure_ascii=False)[:1500])
resp = th.get("respiration")
print("RESP", json.dumps(resp, ensure_ascii=False)[:2500] if resp else None)
print("OPERATING", json.dumps(th.get("operating"), ensure_ascii=False)[:800])
print("EMITTERS", [(e.get("id"), e.get("kind")) for e in th.get("emitters", [])])
regs = th.get("regions", [])
print("REGIONS", [(r.get("id"), r.get("model"), r.get("areaM2"), r.get("heatCapacityJM2K"), r.get("skyViewFactor"), r.get("body")) for r in regs])
