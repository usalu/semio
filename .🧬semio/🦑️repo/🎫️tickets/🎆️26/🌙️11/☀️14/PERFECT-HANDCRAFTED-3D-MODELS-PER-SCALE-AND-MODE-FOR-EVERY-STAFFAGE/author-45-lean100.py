"""Leaning-person 1:100: kit coarsen of the finished 1:50, hand repairs, thermal, structure."""
from author_33_lib import *
slug = "leaning-person"
DROP = ("eye-l","eye-r","brow-l","brow-r","nose","mouth","thumb-l","thumb-r","fingers-l-0","fingers-l-1","fingers-l-2","fingers-r-0","fingers-r-1","fingers-r-2","hood-string-l","hood-string-r","hoodie-pocket","cuff-l","cuff-r","beanie-rim","hoodie-band","phone-screen")
print(coarsen(slug, 50, 100, 0.995, drop=DROP))
d = load(slug); v = d["volumes"]["100"]
print([(s["id"], s["kind"]) for s in v["body"]])
d50 = load(slug)["volumes"]["50"]["body"]
b = v["body"]
b.append(copy.deepcopy(by(d50, "hoodie-pocket")))
for sd in "lr":
    w = copy.deepcopy(by(d50, f"wrist-{sd}")); b.append(w)
    w["sections"][0]["at"][0] = 0.24
    for q in w["sections"]: q["rx"] *= 1.1; q["ry"] *= 1.1
    f = by(b, f"forearm-{sd}")
    print(sd, [(round(q["at"][0], 3), round(q["rx"], 3)) for q in f["sections"]][-3:])
for sd in "lr":
    for n in (f"upper-arm-{sd}", f"forearm-{sd}"):
        for q in by(b, n)["sections"]: q["rx"] *= 1.03; q["ry"] *= 1.03
for s_ in b:
    if s_["id"] == "phone": s_["size"] = [0.132, 0.034, 0.064]
for sd in "lr":
    h_ = by(b, f"hand-{sd}"); h_["sections"] = [q for q in h_["sections"] if q["at"][0] > 0.0]
    first = copy.deepcopy(h_["sections"][0]); first["at"][0] = -0.003; h_["sections"].insert(0, first)
save(slug, d)
print(thermal(slug, 100, "torso-clothing=torso+hoodie-hood+hoodie-hem+hoodie-pocket,arm-skin=wrist-l+wrist-r,arm-clothing=upper-arm-l+upper-arm-r+forearm-l+forearm-r+shoulder-l+shoulder-r,phone-surface=phone,hand-skin=hand-l+hand-r")[:3000])

d = load(slug)
for t in d["volumes"]["100"]["thermal"]:
    if t.get("of") == "wall": t["sectionFaces"] = ["+v"]
save(slug, d)
PADS = {"wall": ([0.045, 1.33, 0], [0.07, 0.16, 0.26]), "wall-x": ([0.11, 0.157, 0], [0.08, 0.12, 0.22]), "wall-frame": ([-0.073, 0.007, 0], [0.07, 0.1, 0.22])}
def wall_pads(slug, scale):
    d = load(slug); st = d["volumes"][str(scale)]["structure"]
    st[:] = [t for t in st if t.get("bone") not in PADS]
    for bone, (c_, sz) in PADS.items():
        st.append({"id": f"member-{bone}", "kind": "box", "bone": bone, "material": "masonry", "role": "body", "tone": 0.5, "size": sz, "transform": {"translation": c_}, "member": bone, "part": "member"})
    save(slug, d)
person_structure(slug, 100)
wall_pads(slug, 100)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "100", "--write")[-300:])
