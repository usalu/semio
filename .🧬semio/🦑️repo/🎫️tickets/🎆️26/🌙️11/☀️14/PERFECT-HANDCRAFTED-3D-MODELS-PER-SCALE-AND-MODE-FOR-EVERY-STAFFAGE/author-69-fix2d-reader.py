"""Root repair seated-reader 1:500 elevation: near/far arm (the left forearm and hand that 1:200 draws were missing: the 3D left hand stood outside the drawn silhouette) and the cup notch drawn as wide as the plan's cup (80 mm)."""
import sys, copy
sys.path.insert(0, ".")
from author_69_lib import *
def fn(r):
    e200 = r["200"]["geometry"]["elevation"]; e500 = r["500"]["geometry"]["elevation"]
    ids = {s["id"] for s in e500}
    for src in ("forearm-l", "hand-l"):
        s = copy.deepcopy(next(x for x in e200 if x["id"] == src))
        s["id"] = "sil-" + src; s["z"] = 5.9 if src == "forearm-l" else 6.1
        s.pop("solid", None)
        if "sil-" + src not in ids: e500.append(s)
    b = next(x for x in e500 if x["id"] == "sil-bench")
    b["points"] = [[-0.255 if p[0] == -0.245 else -0.175 if p[0] == -0.185 else p[0], p[1]] for p in b["points"]]
edit_member("seated-reader", "representations", fn)
