"""Leaning-person 1:50 step b: wrist skin below the sleeve cuff, hoodie hem over the pelvis, real wall thickness; thermal; structure."""
from author_33_lib import *
slug = "leaning-person"
exec(open(f"{T}/author-45-lean50a.py").read().split("save(slug, d)")[0])
for s in b:
    if s["id"] == "wall": s["size"] = [0.38, 1.9, 1.2]; s["transform"] = {"translation": [-0.19, 0.95, 0]}; s["sectionFaces"] = ["+v"]
CUT = 0.24
for sd in "lr":
    f = by(b, f"forearm-{sd}")
    secs = [q for q in f["sections"] if q["at"][0] < CUT]
    last = copy.deepcopy(secs[-1]); last["at"][0] = CUT
    for k in ("rx", "ry"): last[k] = interp(f, CUT, k)
    f["sections"] = secs + [last]
    rot = f["sections"][0]["rotation"]; z = f["sections"][0]["at"][2]
    w = [{"at": [CUT, 0.0015, z], "rx": last["rx"] * 0.9, "ry": last["ry"] * 0.9, "exponent": 2.2, "rotation": rot},
         {"at": [0.2639, 0.001, z], "rx": 0.0255, "ry": 0.02, "exponent": 2.2, "rotation": rot}]
    b.append(loft(f"wrist-{sd}", f"forearm-{sd}", "skin", "body", w, 0.7))
for q in by(b, "torso")["sections"]: q["exponent"] = 3.6; q["rx"] *= 1.04; q["ry"] *= 1.04
for q in by(b, "hoodie-hood")["sections"]: q["rx"] *= 1.12; q["ry"] *= 1.12
for sd in "lr":
    h_ = by(b, f"hand-{sd}"); h_["sections"] = [q for q in h_["sections"] if q["at"][0] > 0.0]
    first = copy.deepcopy(h_["sections"][0]); first["at"][0] = -0.003; h_["sections"].insert(0, first)
for s_ in b:
    if s_["id"] == "hoodie-pocket": s_["size"] = [0.12, 0.016, 0.27]; s_["transform"] = {"translation": [0.03, -0.108, 0]}
p = by(b, "pelvis")
hem = [sec(u, interp(p, u, "rx") + 0.008, interp(p, u, "ry") + 0.008, 3.2) for u in (-0.108, -0.085, -0.06, -0.03, 0.0, 0.03, 0.06, 0.09)]
hem.append(sec(0.098, hem[-1]["rx"] * 0.97, hem[-1]["ry"] * 0.97, 2.4))
for q in hem: q["at"][1] = p["sections"][0]["at"][1] + 0.0; q["rotation"] = p["sections"][0]["rotation"]
band = [sec(u, interp(p, u, "rx") + 0.009, interp(p, u, "ry") + 0.009, 2.4, at=None) if False else sec(u, interp(p, u, "rx") + 0.013, interp(p, u, "ry") + 0.013, 2.4) for u in (-0.1, -0.085, -0.07)]
for q in band: q["rotation"] = p["sections"][0]["rotation"]
b.append(loft("hoodie-band", "pelvis", "cotton-clothing", "detail", band, 0.45))
for sd in "lr":
    f = by(b, f"forearm-{sd}")
    cs = [{"at": [u, 0, f["sections"][0]["at"][2]], "rx": interp(f, u, "rx") + 0.008, "ry": interp(f, u, "ry") + 0.008, "exponent": 2.2, "rotation": f["sections"][0]["rotation"]} for u in (0.17, 0.2, CUT)]
    b.append(loft(f"cuff-{sd}", f"forearm-{sd}", "cotton-clothing", "detail", cs, 0.45))
b.append(loft("hoodie-hem", "pelvis", "cotton-clothing", "detail", hem, 0.5))
for sd in "lr":
    for n in (f"upper-arm-{sd}", f"forearm-{sd}"):
        for q in by(b, n)["sections"]: q["rx"] *= 1.03; q["ry"] *= 1.03
d["volumes"]["50"].update(body=b, thermal=[], structure=[], decisions=[])
save(slug, d)
print(check(slug, 50, "volumes-bones,volumes-feature,volumes-bounds"))
print(thermal(slug, 50, "torso-clothing=torso+hoodie-hood+hoodie-hem+hoodie-band+hoodie-pocket+hood-string-l+hood-string-r,arm-skin=wrist-l+wrist-r,arm-clothing=upper-arm-l+upper-arm-r+forearm-l+forearm-r+shoulder-l+shoulder-r+cuff-l+cuff-r")[:2500])
