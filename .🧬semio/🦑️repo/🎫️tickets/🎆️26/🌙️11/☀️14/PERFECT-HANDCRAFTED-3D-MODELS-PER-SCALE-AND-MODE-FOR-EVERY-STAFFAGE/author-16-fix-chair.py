import sys, copy
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")


def fix(a):
    n = 0
    g = a["representations"]["100"]["geometry"]["elevation"]
    g50 = {s["id"]: s for s in a["representations"]["50"]["geometry"]["elevation"]}
    for i, s in enumerate(g):
        if s["id"] in ("armrest-post-left", "armrest-post-right") and s["kind"] == "poly":
            t = copy.deepcopy(g50[s["id"]])
            t.pop("penMm", None)
            g[i] = t
            n += 1
    for r in a["thermal"]["regions"]:
        ls = r["layers"]
        if r["id"] == "chair-fabric" and ls[0]["id"] == "fabric":
            r["layers"] = [ls[1], ls[0]]
            n += 1
    return n


print(L.edit_asset("🛋️furniture", "office-swivel-chair", fix))
