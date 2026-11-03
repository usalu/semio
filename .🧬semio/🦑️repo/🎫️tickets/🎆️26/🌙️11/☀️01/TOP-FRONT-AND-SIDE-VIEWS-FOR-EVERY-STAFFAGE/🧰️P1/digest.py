import json, os, sys
base = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🔌️appliances"
out = r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\P1\digest"
os.makedirs(out, exist_ok=True)
ids = sys.argv[1:] or "ceiling-diffuser circulation-pump decentralised-hrv-fan desktop-monitor duct-fan electric-fan-heater electric-kettle electricity-meter gas-boiler gas-hob gas-meter".split()
def j(x):
    return json.dumps(x, ensure_ascii=False, separators=(",", ":"))
def strip(s):
    return {k: v for k, v in s.items() if k not in ("region", "material", "regions")}
for a in ids:
    d = json.load(open(os.path.join(base, a, "🔣️.json"), encoding="utf-8"))["asset"]
    L = []
    L.append(f"== {a} dims {j(d['dimensions'])} pres {j(d['presentation'])}")
    for b in d["rig"]["bones"]:
        L.append("bone " + j({k: b.get(k) for k in ("id", "parent", "x", "y", "angleDeg", "lengthM", "joint")}))
    for p in d["poses"]:
        L.append("pose " + p["id"] + " " + j(p.get("bones")))
    for c in d["clips"]:
        ch = c.get("channels") or c.get("tracks") or c.get("bones")
        L.append("clip " + c["id"] + " dur=" + str(c.get("durationS")) + " keys=" + str([k for k in c.keys()]) + " " + (j(ch)[:700] if ch else ""))
    for sc, r in d["representations"].items():
        L.append(f"--- scale {sc} {r['status']} pens={j(r.get('pens'))}")
        for v, shapes in r.get("geometry", {}).items():
            L.append(f"  {v}:")
            for s in shapes:
                L.append("    " + j(s))
        if r.get("decisions"):
            L.append("  decisions.silhouette: " + j(r["decisions"]["silhouette"]["en"]))
    L.append("regions: " + j([k for k in (d.get("thermal") or {}).keys()]))
    L.append("materials: " + j([m["id"] for m in d["materials"]]))
    open(os.path.join(out, a + ".txt"), "w", encoding="utf-8").write("\n".join(L) + "\n")
