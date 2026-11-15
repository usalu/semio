"""Leaning-person 1:50 step a: repair the 07 seed (feature flags), add the wall as a ground-role box."""
from author_33_lib import *
slug = "leaning-person"
seed = json.load(open(f"{T}/🗑️generated/author-45/leaning.seed.json"))
d = load(slug)
b = copy.deepcopy(seed["volumes"]["50"]["body"])
for s in b:
    if s["id"] == "mouth": s["r0"] = s["r1"] = 0.004
    if s["id"] in ("hood-string-l", "hood-string-r"): s["r0"] = s["r1"] = 0.004
b.append({"id": "wall", "kind": "box", "bone": "wall", "material": "masonry", "role": "ground", "tone": 0.6, "size": [0.12, 1.9, 1.2], "transform": {"translation": [-0.06, 0.95, 0]}, "sectionFaces": ["-u"], "texture": True})
d["volumes"]["50"].update(body=b, thermal=[], structure=[], decisions=[])
save(slug, d)
print(check(slug, 50, "volumes-bones,volumes-silhouette,volumes-feature,volumes-bounds,volumes-views-disagree"))
