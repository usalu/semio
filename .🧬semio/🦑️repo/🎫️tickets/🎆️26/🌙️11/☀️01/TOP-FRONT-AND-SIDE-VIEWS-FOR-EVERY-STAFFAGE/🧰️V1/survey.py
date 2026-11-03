import json, os
base = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🌳️vegetation"
for a in "agave apple-tree bamboo-clump date-palm english-oak hornbeam-hedge hosta-clump ivy-on-trellis".split():
    d = json.load(open(os.path.join(base, a, "🔣️.json"), encoding="utf-8"))["asset"]
    print("==", a, "size", os.path.getsize(os.path.join(base, a, "🔣️.json")))
    print(" dims", d["dimensions"], "pres", d["presentation"])
    print(" rig bones", [b.get("id") for b in d["rig"].get("bones", [])] if isinstance(d["rig"], dict) else type(d["rig"]))
    print(" poses", [p.get("id") for p in d["poses"]], "clips", [c.get("id") for c in d["clips"]])
    reps = d["representations"]
    print(" reps type", type(reps).__name__)
    it = reps.items() if isinstance(reps, dict) else enumerate(reps)
    for k, r in it:
        if not isinstance(r, dict):
            print("  ", k, str(r)[:80]); continue
        g = r.get("geometry", {})
        print("  ", k, r.get("scale"), r.get("status"), list(r.keys())[:8], {kk: len(v) if isinstance(v, list) else type(v).__name__ for kk, v in g.items()})
