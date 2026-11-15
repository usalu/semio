"""Elderly-with-cane 1:100: kit coarsen of the finished 1:50, then hand repairs (cane chain, plume, details), thermal, structure."""
from author_33_lib import *
slug = "elderly-with-cane"
DROP = ("jacket-front-1", "jacket-front-2", "glasses-lens-l", "glasses-lens-r", "glasses-temple-l", "glasses-temple-r", "glasses-frame", "shoulder-l", "shoulder-r", "thumb-l", "thumb-r")
print(coarsen(slug, 50, 100, 0.995, drop=DROP))
d = load(slug); v = d["volumes"]["100"]; b = v["body"]
by_id = {s["id"]: s for s in b}
c = by_id["cane-shaft"]
c.update(p0=[0.05, 0, -0.04], p1=[0.84, 0, -0.04], r0=0.0105, r1=0.0095); c.pop("lateralScale", None)
pts = [[-0.03, 0, -0.04], [-0.062, 0.03, -0.04], [-0.045, 0.068, -0.04], [-0.03, 0.064, -0.04]]
for i in range(3):
    b.append({"id": f"cane-crook-{i + 1}", "kind": "capsule", "bone": "cane", "material": "ash-wood", "role": "body", "tone": 0.4, "p0": pts[i], "p1": pts[i + 1], "r0": 0.0125, "r1": 0.0125})
if "cane-ferrule" not in by_id:
    b.append({"id": "cane-ferrule", "kind": "capsule", "bone": "cane", "material": "rubber", "role": "body", "tone": 0.05, "p0": [0.82, 0, -0.04], "p1": [0.858, 0, -0.04], "r0": 0.013, "r1": 0.012})
for s_ in b:
    if s_["id"] == "cap-brim": s_["radii"] = [0.011, 0.045, 0.065]
save(slug, d)
print(thermal(slug, 100, "cap=cap+cap-brim")[:1500])
d = load(slug)
for t in d["volumes"]["100"]["thermal"]:
    if t["id"] == "th-plume": t["transform"]["translation"][0] -= 0.015
save(slug, d)
print(check(slug, 100))
person_structure(slug, 100)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "100", "--write")[-200:])
print(check(slug, 100))
