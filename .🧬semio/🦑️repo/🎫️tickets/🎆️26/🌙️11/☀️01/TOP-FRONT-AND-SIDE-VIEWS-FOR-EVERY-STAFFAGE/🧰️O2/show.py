import json, os, sys, math
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
a = sys.argv[1]
scales = sys.argv[2:] or ["50"]
d = json.load(open(os.path.join(root, a, "🔣️.json"), encoding="utf-8"))["asset"]
print("dims", d["dimensions"], "pose", d["presentation"]["defaultPose"])
for b in d["rig"]["bones"]:
    j = b["joint"]
    print("BONE %-14s par=%-12s o=(%s,%s) a=%s L=%s %s" % (b["id"], b["parent"], b["x"], b["y"], b["angleDeg"], b["lengthM"], {k: v for k, v in j.items()}))
print("regions", [(r["id"]) for r in d["thermal"]["regions"]])
print("poses", [(p["id"], p.get("bones")) for p in d["poses"]])
for c in d["clips"]:
    print("clip", c["id"], c["durationS"], [(k["t"], k["pose"]) for k in c["keys"]])
for s in scales:
    r = d["representations"][s]
    print("#### scale", s, r["status"], "pens", r.get("pens"), "hatches", [(h["id"], h["pattern"]) for h in r.get("hatches", [])])
    g = r.get("geometry", {})
    for v in ("elevation", "front", "plan"):
        for sh in g.get(v, []):
            geo = {k: v2 for k, v2 in sh.items() if k not in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "kind", "hatch", "penMm")}
            gs = json.dumps(geo, ensure_ascii=False)
            if len(gs) > 170:
                gs = gs[:170] + ".."
            print("%s %-16s %-10s %-4s z%-4s t%-4s %-5s %s %s%s %s" % (v[0].upper(), sh["id"], sh["bone"], sh["role"][:4], sh["z"], sh["tone"], "F" if sh["filled"] else "-", sh["kind"], sh.get("region") or sh.get("regions"), (" pen" + str(sh["penMm"])) if "penMm" in sh else "", gs))
