"""Running child bodies: python3 author-24-child.py [--only N]  (rebuilds 1:50 from the first pass, then 100/200/500 by cascade; --only N refits just scale N from the saved coarser one)"""
import os
import sys
sys.path.insert(0, ".")
from author_24_lib import *

slug = "running-child"
only = int(sys.argv[sys.argv.index("--only") + 1]) if "--only" in sys.argv else None
DROP = {
    100: ["eye-l", "eye-r", "thumb-l", "thumb-r"],
    200: ["ear-l", "ear-r", "nose", "shoulder-l", "shoulder-r", "elbow-l", "elbow-r", "knee-l", "knee-r", "shoulders", "pack-pocket", "tshirt-print", "pack-strap-l", "pack-strap-l-2", "pack-strap-l-3", "pack-strap-r", "pack-strap-r-2", "pack-strap-r-3", "collar", "sock-l", "sock-r"],
    500: ["sleeve-l", "sleeve-r", "shoe-toe-l", "shoe-toe-r"],
}


def repair50(vols):
    by = {s["id"]: s for s in vols["50"]["body"]}
    for side in "lr":
        for q in by[f"thumb-{side}"]["sections"]:
            q["at"][1] *= 0.6
            q["rx"] *= 0.85
            q["ry"] *= 0.85
    by["tshirt-print"]["texture"] = True


def special(b, s):
    if s != 500:
        return
    for x in b:
        if x["id"].startswith("hand-"):
            x["sections"][0]["at"][0] = -0.0084
            x["sections"][-1]["at"][0] = 0.0674
        if x["id"] in ("head", "hair"):
            x["sections"][-1]["at"][0] = 0.159
        if x["id"] == "head":
            x["sections"][0]["at"][0] = -0.043
        if x["id"].startswith("shoe-"):
            for q in x["sections"]:
                q["ry"] = 0.022
            x["sections"][-1]["at"][0] = 0.125


def floor(b, s):
    for x in b:
        if x["kind"] == "loft" and not x["id"].startswith("shoe"):
            for q in x["sections"]:
                q["rx"] = max(q["rx"], 0.5008 * FEATURE[s])
                q["ry"] = max(q["ry"], 0.5008 * FEATURE[s])


def stage(s, prev):
    vols = load(slug)
    b = coarsen_all(prev, s, DROP[s])
    special(b, s)
    vols[str(s)]["body"] = b
    save(slug, vols)
    print(fit(slug, s))
    vols = load(slug)
    symmetrise(vols[str(s)]["body"])
    floor(vols[str(s)]["body"], s)
    save(slug, vols)
    print("\n".join(mess(slug, s, "volumes-silhouette,volumes-bounds")))
    return vols[str(s)]["body"]


if only:
    stage(only, load(slug)[str({100: 50, 200: 100, 500: 200}[only])]["body"])
    sys.exit()
vols = json.load(open(f"{T}/🗑️generated/author-24/running-child.first-pass.json"))["volumes"]
repair50(vols)
save(slug, vols)
print(fit(slug, 50))
vols = load(slug)
symmetrise(vols["50"]["body"])
for x in vols["50"]["body"]:
    if x["id"] == "pack-body":
        for q in x["sections"]:
            q["at"][1] += 0.003
            q["ry"] -= 0.003
    if x["id"] == "pack-pocket":
        x["size"] = [0.05, 0.014, 0.1]
        x["transform"]["translation"] = [0.01, 0.097, 0]
save(slug, vols)
print("\n".join(mess(slug, 50, "volumes-silhouette,volumes-bounds")))
prev = vols["50"]["body"]
for s in (100, 200, 500):
    prev = stage(s, prev)
