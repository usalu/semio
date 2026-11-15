"""Leaning-person 1:500 step b (after kit-fit and author-45-descent.ts): thermal, structure, wall pads."""
import sys; sys.path.insert(0, ".")
from author_33_lib import *
slug = "leaning-person"
PADS = {"wall": ([0.045, 1.33, 0], [0.07, 0.16, 0.26]), "wall-x": ([0.11, 0.157, 0], [0.08, 0.12, 0.22]), "wall-frame": ([-0.073, 0.007, 0], [0.1, 0.1, 0.22])}
def wall_pads(slug, scale):
    d = load(slug); st = d["volumes"][str(scale)]["structure"]
    st[:] = [t for t in st if t.get("bone") not in PADS]
    for bone, (c_, sz) in PADS.items():
        st.append({"id": f"member-{bone}", "kind": "box", "bone": bone, "material": "masonry", "role": "body", "tone": 0.5, "size": [max(x, 0.08) for x in sz], "transform": {"translation": c_}, "member": bone, "part": "member"})
    save(slug, d)
print(thermal(slug, 500, "torso-clothing=torso,hair=head-cap,head-skin=head-cap,arm-skin=hand-l+hand-r,arm-clothing=upper-arm-l+upper-arm-r+forearm-l+forearm-r,phone-surface=phone,hand-skin=hand-l+hand-r,leg-clothing=pelvis+thigh-l+thigh-r+shank-l+shank-r,shoes=shoe-l+shoe-r+shoe-toe-l+shoe-toe-r")[:300])
d = load(slug)
for t in d["volumes"]["500"]["thermal"]:
    if t.get("of") == "wall": t["sectionFaces"] = ["+v"]
save(slug, d)
person_structure(slug, 500)
wall_pads(slug, 500)
