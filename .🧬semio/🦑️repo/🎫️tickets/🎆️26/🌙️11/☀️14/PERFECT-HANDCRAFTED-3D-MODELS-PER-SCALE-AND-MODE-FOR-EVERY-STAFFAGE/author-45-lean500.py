"""Leaning-person 1:500: merged-mass lofts from the 1:500 drawings (author-33-hull.ts), one solid per bone, the wall as a texture box."""
import sys; sys.path.insert(0, ".")
from author_33_lib import *
slug = "leaning-person"
S = 500
b = []
def H(bone, id, mat, n=9, **o):
    o.setdefault("tol", 0.0015)
    b.append(hull(slug, S, bone, id, mat, n, **o))
H("torso", "torso", "cotton-clothing", 20, exp=3.0, range="0,0.4495", tone=0.5)
ROT0 = [0, -0.70711, 0, 0.70711]
b.append(loft("head-cap", "head", "skin", "body", [{"at": [u, y, 0], "rx": rx, "ry": ry, "exponent": 2.2, "rotation": ROT0} for u, y, rx, ry in ((0.0, -0.003, 0.05, 0.046), (0.06, 0.01, 0.062, 0.062), (0.12, 0.012, 0.074, 0.105), (0.19, 0.016, 0.078, 0.112), (0.245, 0.012, 0.06, 0.085), (0.268, 0.005, 0.0377, 0.0377))], 0.6))
for sd in "lr":
    H(f"upper-arm-{sd}", f"upper-arm-{sd}", "cotton-clothing", 5, exp=2.4, tone=0.5)
    H(f"forearm-{sd}", f"forearm-{sd}", "cotton-clothing", 5, exp=2.4, tone=0.5)
    H(f"hand-{sd}", f"hand-{sd}", "skin", 3, exp=2.2, round=0.8, tone=0.7)
    H(f"thigh-{sd}", f"thigh-{sd}", "cotton-clothing", 16, exp=2.6, tone=0.3)
    H(f"shank-{sd}", f"shank-{sd}", "cotton-clothing", 14, exp=2.4, tone=0.3)
    H(f"foot-{sd}", f"shoe-{sd}", "leather-shoe", 4, exp=2.6, tone=0.2)
    H(f"toe-{sd}", f"shoe-toe-{sd}", "leather-shoe", 3, exp=2.6, round=0.8, tone=0.2)
ROT = [0, -0.70711, 0, 0.70711]
b.append(loft("pelvis", "pelvis", "cotton-clothing", "body", [{"at": [-0.108, -0.002, 0], "rx": 0.115, "ry": 0.078, "exponent": 2.6, "rotation": ROT}, {"at": [0.0, 0.004, 0], "rx": 0.175, "ry": 0.117, "exponent": 2.6, "rotation": ROT}, {"at": [0.098, -0.003, 0], "rx": 0.155, "ry": 0.103, "exponent": 2.6, "rotation": ROT}], 0.4))
b.append({"id": "phone", "bone": "hand-r", "material": "phone-glass", "role": "body", "tone": 0.12, "kind": "box", "size": [0.132, 0.075, 0.075], "transform": {"translation": [0.0875, -0.029, -0.01]}})
b.append({"id": "wall", "kind": "box", "bone": "wall", "material": "masonry", "role": "ground", "tone": 0.6, "size": [0.38, 1.9, 1.2], "transform": {"translation": [-0.19, 0.95, 0]}, "sectionFaces": ["+v"], "texture": True})
d = load(slug); d["volumes"]["500"].update(body=b, thermal=[], structure=[], decisions=[]); save(slug, d)
print(len(b))
exec(open(f"{T}/author-45-lean200.py").read().split("def wall_pads")[0].split("PADS = ")[0].split("slug = ")[0]) if False else None
PADS = {"wall": ([0.045, 1.33, 0], [0.07, 0.16, 0.26]), "wall-x": ([0.11, 0.157, 0], [0.08, 0.12, 0.22]), "wall-frame": ([-0.073, 0.007, 0], [0.1, 0.1, 0.22])}
def wall_pads(slug, scale):
    d = load(slug); st = d["volumes"][str(scale)]["structure"]
    st[:] = [t for t in st if t.get("bone") not in PADS]
    for bone, (c_, sz) in PADS.items():
        st.append({"id": f"member-{bone}", "kind": "box", "bone": bone, "material": "masonry", "role": "body", "tone": 0.5, "size": [max(x, 0.08) for x in sz], "transform": {"translation": c_}, "member": bone, "part": "member"})
    save(slug, d)
print(thermal(slug, 500, "torso-clothing=torso,hair=head-cap,head-skin=head-cap,arm-skin=hand-l+hand-r,arm-clothing=upper-arm-l+upper-arm-r+forearm-l+forearm-r,phone-surface=phone,hand-skin=hand-l+hand-r,leg-clothing=pelvis+thigh-l+thigh-r+shank-l+shank-r,shoes=shoe-l+shoe-r+shoe-toe-l+shoe-toe-r")[:2800])
d = load(slug)
for t in d["volumes"]["500"]["thermal"]:
    if t.get("of") == "wall": t["sectionFaces"] = ["+v"]
save(slug, d)
person_structure(slug, 500)
wall_pads(slug, 500)
print(check(slug, 500))
