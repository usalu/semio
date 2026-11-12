import sys, re, json; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("miscanthus-grass")
raw, doc, idx = load(p)
a = doc["asset"]
G = a["representations"]["50"]["geometry"]
bone_of = {s["id"]: s["bone"] for s in a["volume"]["solids"] if s["id"].startswith("blade-")}
FAMILY = {"elevation": re.compile(r"^blade-"), "front": re.compile(r"^[lr]\d-\d-[ab]$"), "plan": re.compile(r"^([lr]\d-\d-[ab]|crown-fill-\d+)$")}
ops = []
for view in ("elevation", "front", "plan"):
    baked = json.load(open(f"🗑️generated/silhouette/misc-baked-{view}.json"))
    arr = G[view]
    gone = [k for k, s in enumerate(arr) if FAMILY[view].match(s["id"])]
    tpl = {}
    for k in gone:
        s = arr[k]
        if s["bone"] not in tpl and s["role"] == "body" and s["id"][0] != "c": tpl[s["bone"]] = s
    new = []
    for sid, rings in baked.items():
        t = tpl.get(bone_of[sid])
        if t is None: t = next(iter(tpl.values()))
        for n, ring in enumerate(rings):
            if len(ring) < 3: continue
            sh = {"id": sid if len(rings) == 1 else f"{sid}-{n + 1}", "kind": "poly", "bone": bone_of[sid], "role": "body", "z": t["z"], "filled": True, "tone": t["tone"], "closed": True, "points": ring}
            for k2 in ("region", "regions", "material"):
                if k2 in t: sh[k2] = t[k2]
            new.append(sh)
    keep = next(k for k, s in enumerate(arr) if k not in gone)
    base = ("asset", "representations", "50", "geometry", view)
    ops.append(("delete_many", base, gone))
    ops.append(("insert_after", base + (keep,), new))
    print(view, "delete", len(gone), "insert", len(new))
print(apply(p, ops, expect=raw))
