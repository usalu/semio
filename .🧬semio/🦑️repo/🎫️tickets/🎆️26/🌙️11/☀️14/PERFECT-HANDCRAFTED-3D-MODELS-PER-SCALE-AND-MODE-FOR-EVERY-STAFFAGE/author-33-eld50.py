"""Elderly-with-cane 1:50: repair of the 07 seed (feature flags, glasses), then thermal and structure."""
from author_33_lib import *
slug = "elderly-with-cane"
orig = json.load(open(f"{T}/🗑️generated/author-33/elderly.orig07.json"))
cur = load(slug)
v = copy.deepcopy(orig["volumes"]["50"])
b = v["body"]
for s in b:
    if s["id"] == "glasses-temple-l": s["p0"], s["p1"], s["r0"], s["r1"] = [0.066, -0.0526, 0.052], [0.046, 0.026, 0.080], 0.002, 0.002
    if s["id"] == "glasses-temple-r": s["p0"], s["p1"], s["r0"], s["r1"] = [0.066, -0.0526, -0.052], [0.046, 0.026, -0.080], 0.002, 0.002
    if s["id"] in ("jacket-front-1", "jacket-front-2", "glasses-temple-l", "glasses-temple-r"): s["slender"] = True
    if s["id"] in ("glasses-lens-l", "glasses-lens-r"): s["texture"] = True
b[:] = [s for s in b if s["id"] != "glasses-bridge"]
b.append({"id": "glasses-frame", "kind": "capsule", "bone": "head", "material": "skin", "role": "detail", "tone": 0.15, "p0": [0.066, -0.0794, -0.052], "p1": [0.066, -0.0794, 0.052], "r0": 0.0024, "r1": 0.0024, "slender": True})
cur["volumes"]["50"]["body"] = b
cur["volumes"]["50"]["thermal"] = []
save(slug, cur)
print(thermal(slug, 50)[:200])
DET = {"jacket-front-1","jacket-front-2","jacket-button-0","jacket-button-1","jacket-button-2","jacket-pocket-l","jacket-pocket-r","cap-brim","thumb-l","thumb-r","fingers-l-0","fingers-l-1","fingers-l-2","fingers-r-0","fingers-r-1","fingers-r-2","ear","eye-l","eye-r","glasses-lens-l","glasses-lens-r","glasses-temple-l","glasses-temple-r","glasses-frame","shoulder-l","shoulder-r","cane-crook-1","cane-crook-2","cane-crook-3","cane-crook-4","cane-crook-5","cane-shaft","cane-ferrule"}
print(prune_and_rebuild_shells(slug, 50, DET))
print(check(slug, 50, "volumes-feature,volumes-silhouette"))

d = load(slug)
for t in d["volumes"]["50"]["thermal"]:
    if t["id"] == "th-plume": t["transform"]["translation"] = [0.1691, 0.0451, 0]
save(slug, d)
print(check(slug, 50, "volumes-thermal-coverage,volumes-slender"))
