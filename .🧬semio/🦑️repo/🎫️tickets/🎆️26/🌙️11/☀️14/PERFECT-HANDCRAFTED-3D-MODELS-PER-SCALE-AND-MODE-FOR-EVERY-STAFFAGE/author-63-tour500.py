"""Tour-guide 1:500: one hull loft per bone from the 1:500 drawings (author-33-hull.ts), slender pointer and pole at true size."""
import re, json
import author_63_lib as L
from author_33_lib import hull, by
slug = "tour-guide"; S = 500
d = L.load(slug); old = d["volumes"]["500"]["body"]
mat = lambda i: by(old, i)["material"]
tone = lambda i: by(old, i).get("tone", 0.5)
new = []
def H(bone, id, n, **o):
    o.setdefault("tol", 0.004); o.setdefault("exp", 2.4)
    new.append(hull(slug, S, bone, id, mat(id), n, tone=tone(id), **o))
H("head", "head", 6, ebone="torso", range="0,0.285", round=0.8, exp=2.2)
H("torso", "torso", 14, ebone="torso", range="-0.115,0.455", exp=3.0)
H("pelvis", "pelvis", 4, ebone="torso", range="-0.12,0.107", exp=2.6)
for sd in "lr":
    H(f"upper-arm-{sd}", f"upper-arm-{sd}", 8)
    H(f"forearm-{sd}", f"forearm-{sd}", 8)
    H(f"hand-{sd}", f"hand-{sd}", 3, ebone=f"forearm-{sd}", range="0,0.18", round=0.8, exp=2.2)
    H(f"thigh-{sd}", f"thigh-{sd}", 10, exp=2.6)
    H(f"shank-{sd}", f"shank-{sd}", 10)
    H(f"foot-{sd}", f"shoe-{sd}", 4, exp=2.6)
    H(f"toe-{sd}", f"toe-{sd}", 3, round=0.8, exp=2.6)
for s0 in new:
    if s0["id"] == "head": s0["sections"][0]["rx"] = 0.05; s0["sections"][0]["ry"] = 0.05
byid = {s["id"]: s for s in new}
out = []
for s in old:
    if s["id"] in byid:
        n = dict(s); n["sections"] = byid[s["id"]]["sections"]; out.append(n)
    elif s["id"] == "pointer":
        n = dict(s); n["r0"] = n["r1"] = 0.005; n["slender"] = True; out.append(n)
    elif s["id"] == "flag-pole":
        n = dict(s); n["r0"] = n["r1"] = 0.012; n["slender"] = True; out.append(n)
    else: out.append(s)
L.save(slug, S, body=out)
for s in out:
    if s["kind"] == "loft": print(s["id"], len(s["sections"]), [(round(q["at"][0], 3), round(q["rx"], 3), round(q["ry"], 3)) for q in s["sections"]])
print(L.check(slug, S, full=False))
