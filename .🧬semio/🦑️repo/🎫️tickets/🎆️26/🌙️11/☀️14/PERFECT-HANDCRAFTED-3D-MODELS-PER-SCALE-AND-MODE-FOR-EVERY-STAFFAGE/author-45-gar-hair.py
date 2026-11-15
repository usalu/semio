"""Kneeling-gardener: hair mass over the back and top of the skull, under the hat (buries the skull skin the face region must not count)."""
from author_33_lib import *
slug = "kneeling-gardener"
d = load(slug)
for sc, s_, d_, step in (("50", 0.0045, 0.0035, 1), ("100", 0.0045, 0.0035, 1)):
    v = d["volumes"][sc]
    v["body"] = [x for x in v["body"] if x["id"] != "hair"]
    v["thermal"] = [x for x in v["thermal"] if x["id"] != "th-hair"]
    h = by(v["body"], "head")
    secs = []
    for u in (0.125, 0.158, 0.19, 0.2166):
        pa = path_at(h, u)
        secs.append({"at": [u, pa[1] + s_, pa[2]], "rx": interp(h, u, "rx") + 0.0075, "ry": interp(h, u, "ry") + d_, "exponent": 2, "rotation": h["sections"][0]["rotation"]})
    hair = {"id": "hair", "kind": "loft", "bone": "head", "material": "hair", "role": "body", "tone": 0.25, "sections": secs, "capStart": True, "capEnd": True}
    v["body"].append(hair)
    v["thermal"].append({"id": "th-hair", "of": "hair", "region": "hat"})
save(slug, d)
print(check(slug, 50, "volumes-silhouette,volumes-feature,volumes-bounds,volumes-bones"))
