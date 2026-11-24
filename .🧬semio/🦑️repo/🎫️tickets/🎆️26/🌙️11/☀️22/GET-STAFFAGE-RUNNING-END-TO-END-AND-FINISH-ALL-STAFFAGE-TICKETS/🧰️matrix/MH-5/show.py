"""🔬️ Prints the shapes of one id of an asset document at a scale in the three views (kind, solid, point count, ranges of both coordinates): `python3 show.py <file.json> <scale> <id> [--points]`."""
import json, sys
d = json.load(open(sys.argv[1]))["asset"]
g = d["representations"][sys.argv[2]]["geometry"]
for v in ("elevation", "front", "plan"):
    for s in g[v]:
        if s["id"] != sys.argv[3]: continue
        p = s.get("points")
        r = "" if not p else f' n={len(p)} a[{min(q[0] for q in p):.3f},{max(q[0] for q in p):.3f}] b[{min(q[1] for q in p):.3f},{max(q[1] for q in p):.3f}]'
        print(v, s["bone"], s["kind"], json.dumps(s.get("solid")), r, {k: s[k] for k in s if k not in ("points", "solid", "id", "bone", "kind", "d", "title")})
        if "--points" in sys.argv: print("   ", json.dumps(p if p else s.get("d"))[:1500])
