"""Elderly-with-cane 1:500: merged-mass lofts built from the 1:500 drawings (author-33-hull.ts), one solid per bone."""
from author_33_lib import *
slug = "elderly-with-cane"
S = 500
b = []
def H(bone, id, mat, n=9, **o):
    o.setdefault("tol", 0.004)
    b.append(hull(slug, S, bone, id, mat, n, **o))
H("head", "head-cap", "skin", 6, exp=2.3, round=0.8, tone=0.6, range="0,0.1674", ebone="head")
H("torso", "torso", "tweed", 14, exp=2.8, range="0,0.43245", tone=0.4)
H("pelvis", "pelvis", "wool-trouser", 4, exp=2.6, range="0,0.093", tone=0.3)
b.append(loft("neck", "neck", "skin", "body", [{"at": [0, 0, 0], "rx": 0.05, "ry": 0.05, "exponent": 2, "rotation": [0, -0.70711, 0, 0.70711]}, {"at": [0.093, 0, 0], "rx": 0.05, "ry": 0.05, "exponent": 2, "rotation": [0, -0.70711, 0, 0.70711]}], 0.7))
for sd in "lr":
    H(f"arm-{sd}", f"arm-{sd}", "tweed", 6, exp=2.4, tone=0.4, range="0,0.3069")
    H(f"forearm-{sd}", f"forearm-{sd}", "tweed", 6, exp=2.4, tone=0.4, range="0,0.2418")
    H(f"hand-{sd}", f"hand-{sd}", "skin", 3, exp=2.2, round=0.8, tone=0.7)
    H(f"thigh-{sd}", f"thigh-{sd}", "wool-trouser", 12, exp=2.6, tone=0.3)
    H(f"shank-{sd}", f"shank-{sd}", "wool-trouser", 12, exp=2.4, tone=0.3)
    H(f"foot-{sd}", f"shoe-{sd}", "leather-shoe", 4, exp=2.6, tone=0.2)
    H(f"toe-{sd}", f"shoe-toe-{sd}", "leather-shoe", 3, exp=2.6, round=0.8, tone=0.2)
b.append({"id": "cane", "kind": "capsule", "bone": "cane", "material": "ash-wood", "role": "body", "tone": 0.45, "p0": [0.0, 0, -0.005], "p1": [0.86, 0, -0.005], "r0": 0.0375, "r1": 0.0375})
d = load(slug); d["volumes"]["500"]["body"] = b; save(slug, d)
print(len(b), check(slug, 500, "volumes-silhouette,volumes-feature,volumes-bounds,volumes-bones"))
print(thermal(slug, 500, "cap=head-cap")[:1500])
d = load(slug)
for t in d["volumes"]["500"]["thermal"]:
    if t["id"] == "th-plume": t["transform"]["translation"][0] -= 0.015
save(slug, d)
person_structure(slug, 500)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "500", "--write")[-200:])
print(check(slug, 500))
