"""🔍️ Exploration: hand positions of candidate hands-in-pockets arm settings on the scaled standing-adult (writes a probe file in 🗑️generated)."""
import copy, itertools, json
from hw17lib import *
S_ = 1.78 / 1.69
doc = load("standing-adult"); a = doc["asset"]
scale_asset(a, S_, 1.0)
base = a["poses"][0]
cands = []
for i, (ang, ab, fa, h) in enumerate(itertools.product((-12, -6, 0), (10, 16), (30, 45, 60), (0, 15))):
    p = copy.deepcopy(base); p["id"] = f"c{i}"
    p["bones"].update({"arm-l": {"angleDeg": ang, "abductionDeg": ab}, "forearm-l": {"angleDeg": fa}, "hand-l": {"angleDeg": h},
                       "arm-r": {"angleDeg": ang, "abductionDeg": -ab}, "forearm-r": {"angleDeg": fa}, "hand-r": {"angleDeg": h}})
    a["poses"].append(p); cands.append((p["id"], ang, ab, fa, h))
path = f"{GEN}/probe-pockets.json"
json.dump(doc, open(path, "w"))
fr = frames(path, ["relaxed"] + [c[0] for c in cands])
r = fr["relaxed"]["bones"]
print("thigh-l o", r["thigh-l"]["o"], "pelvis", r["pelvis"]["o"], "hand-l relaxed", r["hand-l"]["o"], r["hand-l"]["tip"])
for c in cands:
    b = fr[c[0]]["bones"]
    print(c, "wrist", b["hand-l"]["o"], "tip", b["hand-l"]["tip"], "elbow", b["forearm-l"]["o"])
