"""Jogger 1:500: handcrafted merged-mass lofts built from the 1:500 drawings (author-33-hull.ts), one solid per bone."""
from author_33_lib import *
slug = "jogger"
S = 500
b = []
def H(bone, id, mat, n=9, **o):
    o.setdefault("tol", 0.004)
    b.append(hull(slug, S, bone, id, mat, n, **o))
H("head", "head", "skin", 5, exp=2.2, round=0.8, tone=0.7, range="0,0.279", ebone="torso")
H("torso", "torso", "cotton-clothing", 20, exp=3.0, range="0,0.4757", ebone="torso", tone=0.8)
H("pelvis", "pelvis", "cotton-clothing", 4, exp=2.6, range="-0.12,0.1034", ebone="torso", tone=0.8)
for sd in "lr":
    H(f"upper-arm-{sd}", f"upper-arm-{sd}", "cotton-clothing", 18, exp=2.4, tone=0.8)
    H(f"forearm-{sd}", f"forearm-{sd}", "skin", 18, exp=2.4, tone=0.7)
    H(f"hand-{sd}", f"hand-{sd}", "skin", 3, exp=2.2, round=0.8, tone=0.7)
    H(f"thigh-{sd}", f"thigh-{sd}", "cotton-clothing", 20, exp=2.6, tone=0.8)
    H(f"shank-{sd}", f"shank-{sd}", "skin", 20, exp=2.4, tone=0.7)
    H(f"foot-{sd}", f"shoe-{sd}", "leather-shoe", 4, exp=2.6, tone=0.3)
    H(f"toe-{sd}", f"shoe-toe-{sd}", "leather-shoe", 3, exp=2.6, round=0.8, tone=0.3)
pel = by(b, "pelvis")
for q, (rx, ry) in zip(pel["sections"], [(0.13, 0.085), (0.17, 0.12), (0.172, 0.12), (0.164, 0.115)]):
    q["rx"], q["ry"] = rx, ry
d = load(slug); d["volumes"]["500"]["body"] = b; save(slug, d)
print(len(b), check(slug, 500, "volumes-silhouette,volumes-feature,volumes-bounds"))
