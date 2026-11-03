import json, os, sys
base = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🔌️appliances"
ids = sys.argv[1:] or "ceiling-diffuser circulation-pump decentralised-hrv-fan desktop-monitor duct-fan electric-fan-heater electric-kettle electricity-meter gas-boiler gas-hob gas-meter split-ac-indoor-unit".split()
verbose = os.environ.get("V")
for a in ids:
    d = json.load(open(os.path.join(base, a, "🔣️.json"), encoding="utf-8"))["asset"]
    print("==", a)
    print("  dims", json.dumps(d["dimensions"], ensure_ascii=False))
    print("  pres", json.dumps(d["presentation"], ensure_ascii=False)[:300])
    rig = d["rig"]
    print("  rig", json.dumps(rig, ensure_ascii=False)[:1200] if verbose else list(rig.keys()) if isinstance(rig, dict) else len(rig))
    print("  poses", json.dumps(d["poses"], ensure_ascii=False)[:500] if verbose else (list(d["poses"].keys()) if isinstance(d["poses"], dict) else [p.get("id") for p in d["poses"]]))
    print("  clips", json.dumps(d["clips"], ensure_ascii=False)[:800] if verbose else (list(d["clips"].keys()) if isinstance(d["clips"], dict) else [p.get("id") for p in d["clips"]]))
    reps = d["representations"]
    print("  reps type", type(reps).__name__, (list(reps.keys()) if isinstance(reps, dict) else len(reps)))
    items = reps.items() if isinstance(reps, dict) else enumerate(reps)
    for k, r in items:
        g = r.get("geometry", {}) if isinstance(r, dict) else {}
        print("   ", k, r.get("status") if isinstance(r, dict) else r, list(r.keys()) if verbose and isinstance(r, dict) else "", {kk: len(v) if isinstance(v, list) else type(v).__name__ for kk, v in g.items()})
