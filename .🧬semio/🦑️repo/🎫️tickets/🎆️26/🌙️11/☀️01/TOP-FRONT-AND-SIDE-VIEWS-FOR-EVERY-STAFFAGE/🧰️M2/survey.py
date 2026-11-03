import json, sys, glob, os
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🏗️machinery"
ids = ["oil-pumpjack","plate-heat-exchanger","scissor-lift","scroll-compressor","sorption-wheel","tower-crane","wheel-loader","solar-thermal-collector"]
for i in ids:
    d = json.load(open(os.path.join(root, i, "🔣️.json"), encoding="utf-8"))["asset"]
    print("==", i, list(d.keys()))
    print("  dims", d.get("dimensions"), "defaultPose", d.get("presentation", {}).get("defaultPose"), "viewpoints", d.get("presentation", {}).get("viewpoints"))
    for k, v in d.items():
        if isinstance(v, dict) and k in ("rig",):
            print("  rig keys", list(v.keys()))
    reps = d.get("representations")
    if isinstance(reps, dict):
        reps = list(reps.items())
    elif isinstance(reps, list):
        reps = [(r.get("scale"), r) for r in reps]
    for s, r in reps or []:
        g = r.get("geometry", {})
        print("  ", s, r.get("status"), {k: (len(v) if hasattr(v, "__len__") else v) for k, v in g.items()})
