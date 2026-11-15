"""Elderly-with-cane 1:200: kit coarsen of 1:100 then hand repairs (cane, plume), thermal, structure."""
from author_33_lib import *
slug = "elderly-with-cane"
print(coarsen(slug, 100, 200, 0.99))
d = load(slug); v = d["volumes"]["200"]; b = v["body"]
b[:] = [s for s in b if not s["id"].startswith("cane-crook") and s["id"] not in ("cane-ferrule", "cane-shaft")]
b.append({"id": "cane-shaft", "kind": "capsule", "bone": "cane", "material": "ash-wood", "role": "body", "tone": 0.45, "p0": [0.05, 0, -0.04], "p1": [0.84, 0, -0.04], "r0": 0.0165, "r1": 0.0165})
pts = [[-0.03, 0, -0.04], [-0.064, 0.032, -0.04], [-0.04, 0.066, -0.04]]
for i in range(2):
    b.append({"id": f"cane-crook-{i + 1}", "kind": "capsule", "bone": "cane", "material": "ash-wood", "role": "body", "tone": 0.4, "p0": pts[i], "p1": pts[i + 1], "r0": 0.015, "r1": 0.015})
b.append({"id": "cane-ferrule", "kind": "capsule", "bone": "cane", "material": "rubber", "role": "body", "tone": 0.05, "p0": [0.81, 0, -0.04], "p1": [0.858, 0, -0.04], "r0": 0.0165, "r1": 0.0165})
save(slug, d)
print(thermal(slug, 200, "cap=cap+cap-brim")[:1700])
d = load(slug)
for t in d["volumes"]["200"]["thermal"]:
    if t["id"] == "th-plume": t["transform"]["translation"][0] -= 0.015
save(slug, d)
person_structure(slug, 200)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "200", "--write")[-200:])
print(check(slug, 200))
print([s["id"] for s in load(slug)["volumes"]["200"]["body"]])
