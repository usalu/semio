import json, os, sys
root = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
ids = ["retractable-bollard","shopping-trolley","stepladder","storage-crate-stack","street-lamp","tealight","waste-bin"]
if len(sys.argv) > 1:
    ids = sys.argv[1:]
for i in ids:
    d = json.load(open(os.path.join(root, i, "🔣️.json"), encoding="utf-8"))["asset"]
    print("==", i, list(d.keys()))
    print("  dims", d.get("dimensions"), "pres", d.get("presentation"))
    rig = d.get("rig", {})
    print("  rig", json.dumps(rig, ensure_ascii=False)[:1500])
    print("  poses", [p.get("id") for p in d.get("poses", [])] if isinstance(d.get("poses"), list) else list(d.get("poses", {}).keys()))
    print("  clips", [c.get("id") for c in d.get("clips", [])] if isinstance(d.get("clips"), list) else list(d.get("clips", {}).keys()))
    reps = d.get("representations")
    for s, r in (reps.items() if isinstance(reps, dict) else [(r.get("scale"), r) for r in reps]):
        g = r.get("geometry", {})
        print("  ", s, r.get("status"), {k: (len(v) if hasattr(v, "__len__") else v) for k, v in g.items()})
