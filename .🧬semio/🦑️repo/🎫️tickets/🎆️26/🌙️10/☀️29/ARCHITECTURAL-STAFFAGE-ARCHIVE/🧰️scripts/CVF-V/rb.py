import sys, json, importlib, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib2 import *
G = f"{T}/🗑️generated/CVF-V"
MODS = {"english-oak": "dv_oak", "lombardy-poplar": "dv_poplar", "multi-stem-birch": "dv_birch", "scots-pine": "dv_pine", "date-palm": "dv_palm", "weeping-willow": "dv_willow", "hornbeam-hedge": "dv_hedge", "miscanthus-grass": "dv_grass"}

def rebuild(a, out=None):
    doc = json.load(open(f"{G}/orig/{a}.json")); A = doc["asset"]; R = Rig(A)
    mod = importlib.import_module(MODS[a])
    res = mod.design(R, A)
    for k in ("500", "200", "100", "50"):
        if k not in res: continue
        rep = A["representations"][k]
        r = res[k]
        if isinstance(r, list): r = {"shapes": r}
        rep["geometry"]["elevation"] = r["shapes"]
        if "hatches" in r: rep["hatches"] = r["hatches"]
        if "pens" in r: rep["pens"] = r["pens"]
        if "decisions" in r: rep["decisions"] = r["decisions"]
        if "validRange" in r: rep["validRange"] = r["validRange"]
    if hasattr(mod, "extra"): mod.extra(R, A, res)
    rg = {r["id"] for r in A["thermal"]["regions"]}
    hid = {r["id"] for r in A["thermal"]["regions"] if r.get("hidden")}
    h, w = A["dimensions"]["heightM"], A["dimensions"]["widthM"]
    for k in ("500", "200", "100", "50"):
        shp = A["representations"][k]["geometry"]["elevation"]
        got = set()
        for s in shp:
            if s.get("region"): got.add(s["region"])
            got |= set(s.get("regions") or [])
        ids = [s["id"] for s in shp]
        bb = bbox_shapes(R, [s for s in shp if s["role"] != "ground"])
        ok = lambda v, n: abs(v - n) <= max(0.02 * n, 0.015)
        print(a, k, "missing", sorted(rg - got - hid), "dups", len(ids) - len(set(ids)), "h", round(bb[3] - bb[2], 2), "w", round(bb[1] - bb[0], 2), "dims", h, w, "OK" if ok(bb[3] - bb[2], h) and ok(bb[1] - bb[0], w) else "DIMS-BAD", "shapes", len(shp))
    p = out or f"{G}/work/{a}.json"
    json.dump(doc, open(p, "w"), ensure_ascii=False, indent=1); open(p, "a").write("\n")

if __name__ == "__main__":
    for a in sys.argv[1:] or MODS: rebuild(a)
