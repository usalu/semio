"""Running child thermal, structure and decisions: python3 author-24-child-finish.py"""
import sys
sys.path.insert(0, ".")
from author_24_lib import *

slug = "running-child"
A = asset(slug)
V = seed(slug)
spec = {}
for t in V["50"]["thermal"]:
    if "of" in t and t["of"] != "tshirt-print":
        reg = "tshirt" if t["of"].startswith("shoulder-") else t["region"]
        spec.setdefault(reg, []).append(t["of"])


def thermal_extra(a, body, scale):
    ids = {s["id"] for s in body}
    out = []
    for side in "lr":
        if f"sock-{side}" in ids:
            continue
        sh = next(s for s in body if s["id"] == f"shank-{side}")
        L = next(b["lengthM"] for b in a["rig"]["bones"] if b["id"] == f"shank-{side}")
        u0 = (0.7 if scale < 500 else 0.45) * L
        us = [u0, (u0 + L) / 2, L]
        rot = sh["sections"][0].get("rotation", ROT_U)
        secs = []
        for u in us:
            v, w, rx, ry, e = sec_at_u(sh["sections"], u)
            secs.append(sec(u, v, w, rx, ry, e, rot))
        c = loft(f"th-socks-sock-{side}", f"shank-{side}", "cotton-sock", 0.9, secs)
        c["region"] = "socks"
        out.append(c)
    return out


CFG = dict(
    thermal={50: spec},
    thermal_extra=thermal_extra,
    emitter_region={"*": "face-skin"},
    masses=True,
    joints={200: [b for b in member_bones(A) if b.split("-")[0] in ("arm", "thigh", "shank", "head", "neck", "torso")], 500: []},
    decisions=__import__("author_24_child_decisions").DECISIONS,
)
vols = load(slug)
finish(slug, vols, CFG)
vols = load(slug)
clip_members(slug, vols)
fix_joints(slug, vols)
shorten_emitters(vols)
save(slug, vols)
print("finished", summary(slug))
