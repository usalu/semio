"""🎯️ The hard tier of an asset from its document: `python3 singles.py <file.json>…` prints per bone the scales at which the bone draws exactly one shape in two views with the same id (the pairs the matrix compares as `sync-bone-*`), with the construction of the shape in elevation, front and plan, grouped by equal construction."""
import json, sys, collections
def kind(s):
    if s is None: return "-"
    so = s.get("solid")
    base = ("tip" if s.get("followTip") else "flat")
    if so is None: return base
    if so["kind"] == "extruded": return "sw" + so.get("sweep", "?")
    return so["kind"][:3] + ("" if so.get("mirror") else "½")
for f in sys.argv[1:]:
    a = json.load(open(f))["asset"]
    rows = collections.OrderedDict()
    for key, rep in a["representations"].items():
        if rep.get("status") != "authored": continue
        per = collections.defaultdict(lambda: {"elevation": [], "front": [], "plan": []})
        for view in ("elevation", "front", "plan"):
            for s in rep["geometry"][view]:
                if s.get("role") != "ground": per[s["bone"]][view].append(s)
        for bone, v in per.items():
            pairs = []
            for x, y in (("elevation", "front"), ("elevation", "plan"), ("front", "plan")):
                if len(v[x]) == 1 and len(v[y]) == 1 and v[x][0]["id"] == v[y][0]["id"]: pairs.append(x[0] + y[0])
            if not pairs: continue
            sig = f'{bone}: {"/".join(kind(v[x][0]) if len(v[x]) == 1 else str(len(v[x])) for x in ("elevation", "front", "plan"))} {",".join(pairs)}'
            rows.setdefault(sig, []).append(key)
    print(a["id"])
    for sig, keys in rows.items(): print("   ", sig, "@", ",".join(keys))
