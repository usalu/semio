"""🔍️ The shapes of chosen ids or bones of one asset document at one scale, per view, with their construction and ranges, and the volume solids of those bones: `python3 part.py <file.json> <scale> <id or bone>…`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
scale, names = sys.argv[2], set(sys.argv[3:])
rep = a["representations"][scale]
for view in ("elevation", "front", "plan"):
    for s in rep["geometry"][view]:
        if s["id"] in names or s.get("bone") in names:
            t = {k: v for k, v in s.items() if k not in ("points", "title", "d")}
            p = s.get("points")
            r = f' {len(p)}pts x[{min(q[0] for q in p):.3f},{max(q[0] for q in p):.3f}] y[{min(q[1] for q in p):.3f},{max(q[1] for q in p):.3f}]' if p else ""
            print(view[:2], json.dumps(t, ensure_ascii=False)[:420] + r)
for s in (a.get("volumes", {}).get(scale) or {}).get("body", []):
    if s["id"] in names or s.get("bone") in names:
        print("vol", json.dumps(s)[:500])
for b in a["rig"]["bones"]:
    if b["id"] in names: print("bone", json.dumps({k: v for k, v in b.items() if k not in ("title", "mass")}))
for an in a.get("anchors", []):
    if an["id"] in names or an.get("bone") in names: print("anchor", json.dumps({k: v for k, v in an.items() if k != "title"}))
