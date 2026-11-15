"""Leaning-person 1:200: kit coarsen of 1:100, thermal, structure."""
import sys; sys.path.insert(0, ".")
from author_33_lib import *
exec(open(f"{T}/author-45-lean100.py").read().split("PADS = ")[1].join(["PADS = ", ""]).split("person_structure(slug, 100)")[0]) if False else None
slug = "leaning-person"
PADS = {"wall": ([0.045, 1.33, 0], [0.07, 0.16, 0.26]), "wall-x": ([0.11, 0.157, 0], [0.08, 0.12, 0.22]), "wall-frame": ([-0.073, 0.007, 0], [0.07, 0.1, 0.22])}
def wall_pads(slug, scale):
    d = load(slug); st = d["volumes"][str(scale)]["structure"]
    st[:] = [t for t in st if t.get("bone") not in PADS]
    for bone, (c_, sz) in PADS.items():
        st.append({"id": f"member-{bone}", "kind": "box", "bone": bone, "material": "masonry", "role": "body", "tone": 0.5, "size": [max(x, 0.0) for x in sz], "transform": {"translation": c_}, "member": bone, "part": "member"})
    save(slug, d)
print(coarsen(slug, 100, 200, 0.99))
d = load(slug); v = d["volumes"]["200"]
print([(s["id"], s["kind"]) for s in v["body"]])
d = load(slug); v = d["volumes"]["200"]; b = v["body"]
b[:] = [s for s in b if s["id"] not in ("hair", "shoulder-l", "shoulder-r", "hoodie-pocket")]
d100 = d["volumes"]["100"]["body"]
for sd in "lr":
    w = copy.deepcopy(by(d100, f"wrist-{sd}")); w["sections"][0]["at"][0] = 0.222
    for q in w["sections"]: q["rx"] *= 1.05; q["ry"] *= 1.05
    b.append(w)
    f = by(b, f"forearm-{sd}")
    last = f["sections"][-1]; print(sd, round(last["at"][0], 3), round(last["rx"], 3))
save(slug, d)
print(thermal(slug, 200, "torso-clothing=torso+hoodie-hood+hoodie-hem,hair=beanie,arm-skin=wrist-l+wrist-r,arm-clothing=upper-arm-l+upper-arm-r+forearm-l+forearm-r,phone-surface=phone,hand-skin=hand-l+hand-r")[:2600])
d = load(slug)
for t in d["volumes"]["200"]["thermal"]:
    if t.get("of") == "wall": t["sectionFaces"] = ["+v"]
save(slug, d)
person_structure(slug, 200)
wall_pads(slug, 200)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "200", "--write")[-300:])
