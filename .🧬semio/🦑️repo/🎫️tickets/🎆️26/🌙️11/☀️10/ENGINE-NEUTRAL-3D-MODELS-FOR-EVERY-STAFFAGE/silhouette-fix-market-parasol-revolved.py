import sys; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("market-parasol")
raw, doc, idx = load(p)
a = doc["asset"]
HINT = {"kind": "revolved", "axisBone": "mast", "axis": "u", "axisX": 0, "mirror": True}
ops = []
for sc, r in a["representations"].items():
    g = r.get("geometry")
    if g is None: continue
    el = g["elevation"]
    gone = [k for k, s in enumerate(el) if s["id"].endswith("-core") and s["id"].startswith("panel-")]
    if gone: ops.append(("delete_many", ("asset", "representations", sc, "geometry", "elevation"), gone))
    for k, s in enumerate(el):
        if s["id"] in ("panel-right", "panel-left"):
            n = dict(s); n["solid"] = HINT
            ops.append(("replace", ("asset", "representations", sc, "geometry", "elevation", k), n))
print(len(ops), apply(p, ops, expect=raw))
