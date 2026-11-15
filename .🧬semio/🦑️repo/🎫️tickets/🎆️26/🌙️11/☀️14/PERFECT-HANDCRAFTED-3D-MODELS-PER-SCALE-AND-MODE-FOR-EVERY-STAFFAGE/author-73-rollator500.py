"""Author 73: rollator 1:500 person re-hulled from the redrawn 1:500 elevation/front/plan."""
from author_73_lib import *
from author_33_lib import hull, by
slug = "person-with-rollator"; S = 500
d = L.load(slug); old = d["volumes"]["500"]["body"]
mat = lambda i: by(old, i)["material"]
new = {}
def H(bone, id, n, **o):
    o.setdefault("tol", 0.004); o.setdefault("exp", 2.4)
    new[id] = hull(slug, S, bone, id, mat(id), n, **o)
for sd in "lr":
    H(f"thigh-{sd}", f"thigh-{sd}-trousers", 6, pick=f"thigh-{sd}", ebone=f"thigh-{sd}")
    H(f"thigh-{sd}", f"thigh-{sd}-coat", 5, pick=f"skirt-{sd}", ebone=f"thigh-{sd}")
    H(f"shank-{sd}", f"shank-{sd}-trousers", 6)
    H(f"foot-{sd}", f"foot-{sd}", 4, exp=2.6)
H("torso", "torso-coat", 8, exp=2.8, pick="torso,sil-torso", ebone="torso")
out = []
for s in old:
    if s["id"] in new:
        n = dict(s); n["sections"] = new[s["id"]]["sections"]; out.append(n)
    else: out.append(s)
L.save(slug, S, body=out)
for s in out:
    if s["id"] in new: print(s["id"], [(round(q["at"][0], 3), round(q["at"][1], 3), round(q["at"][2], 3), round(q["rx"], 3), round(q["ry"], 3)) for q in s["sections"]])
print(L.check(slug, S))
