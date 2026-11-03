import json, shutil
from lib import path_of
from abd import solve
from lib import save as _save
_save("agave", json.load(open("agave.before.json", encoding="utf8")))
doc, out = solve()
a = doc["asset"]
bones = {b["id"]: b for b in a["rig"]["bones"]}
stat = {}
for n, v in out.items():
    for bid, (ang, phi) in v.items():
        bones[bid]["angleDeg"] = round(ang, 4)
        stat[bid] = round(phi, 4)
def put(d):
    for bid, phi in stat.items():
        e = d.setdefault(bid, {})
        e["abductionDeg"] = phi
for p in a["poses"]:
    put(p["bones"])
for c in a["clips"]:
    for k in c["keys"]:
        put(k.setdefault("bones", {}))
LIM = {"a": 25, "b": 30}
for bid, b in bones.items():
    if bid.startswith("leaf-") and bid != "leaf-S":
        k = "b" if bid.endswith("-b") else "a"
        b["joint"]["minDeg"] = -LIM[k]
        b["joint"]["maxDeg"] = LIM[k]
for p in a["poses"]:
    if p["id"] in ("drought-infolded", "frost-sag"):
        for bid, e in p["bones"].items():
            if bid == "leaf-S" or "angleDeg" not in e:
                continue
            ref = bid[:-2] + "-a" if bid.endswith("-b") else bid
            sgn = 1 if bones[ref]["angleDeg"] < 0 else -1
            isb = bid.endswith("-b")
            if p["id"] == "drought-infolded":
                e["angleDeg"] = sgn * (22.0 if isb else 18.0)
            else:
                e["angleDeg"] = sgn * (-14.0 if isb else -9.0)
from tipy import chain
FLOOR = 0.14
for p in a["poses"]:
    if p["id"] != "frost-sag":
        continue
    for n in out:
        ids = [b for b in p["bones"] if b.startswith(f"leaf-{n}")]
        full = {b: p["bones"][b]["angleDeg"] for b in ids}
        for g in [i / 20 for i in range(20, -1, -1)]:
            for b in ids:
                p["bones"][b]["angleDeg"] = round(full[b] * g, 3)
            res = chain(bones, p["bones"])
            if min(min(res[b][0][1], res[b][1][1]) for b in ids) >= FLOOR - 1e-9:
                break
from lib import save
save("agave", doc)
print("applied", len(stat))
