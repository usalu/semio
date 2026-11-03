import json, sys, os
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
aid = sys.argv[1]
what = sys.argv[2]
d = json.load(open(os.path.join(root, aid, "🔣️.json"), encoding="utf-8"))["asset"]
def j(x): return json.dumps(x, ensure_ascii=False)
if what == "rig":
    print(j(d["rig"]["root"]))
    for b in d["rig"]["bones"]:
        print(j(b))
    print("ANCHORS", j(d["anchors"]))
elif what == "motion":
    for p in d["poses"]:
        print("POSE", j(p))
    for c in d["clips"]:
        print("CLIP", j(c)[:3000])
elif what == "meta":
    for k in ("dimensions","materials","presentation","completion","thermal","structure"):
        print(k.upper(), j(d[k])[:2500])
else:
    reps = d["representations"]
    r = reps[what] if isinstance(reps, dict) else [x for x in reps if str(x.get("scale")) == what][0]
    view = sys.argv[3] if len(sys.argv) > 3 else "elevation"
    print({k: v for k, v in r.items() if k != "geometry"} if view == "meta" else "")
    if view == "meta":
        pass
    else:
        for s in r["geometry"][view]:
            print(j(s))
