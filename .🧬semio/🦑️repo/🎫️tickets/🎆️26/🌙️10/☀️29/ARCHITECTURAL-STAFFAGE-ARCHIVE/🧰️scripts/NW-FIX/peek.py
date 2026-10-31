"""Peek generated vegetation documents and missing defaultScale."""
import json
import os

gen = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-V2"
for name in os.listdir(gen):
    if not name.endswith(".json"):
        continue
    path = os.path.join(gen, name)
    data = json.load(open(path))
    print(name, "keys", list(data)[:8], "bytes", os.path.getsize(path))
    asset = data.get("asset", data)
    print("  id", asset.get("id"), "cat", asset.get("category"), "pres", asset.get("presentation", {}).keys() if isinstance(asset.get("presentation"), dict) else None)
    reps = asset.get("representations", {})
    print("  scales", {k: (v.get("status") if isinstance(v, dict) else type(v)) for k, v in reps.items()})

root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
needles = ("vegetation", "furniture", "objects", "vehicles", "machinery", "components")
missing = []
for cat in os.listdir(root):
    if not any(n in cat for n in needles):
        continue
    catp = os.path.join(root, cat)
    if not os.path.isdir(catp):
        continue
    for folder in os.listdir(catp):
        fp = os.path.join(catp, folder)
        if not os.path.isdir(fp):
            continue
        docs = [f for f in os.listdir(fp) if f.endswith(".json")]
        if not docs:
            continue
        data = json.load(open(os.path.join(fp, docs[0])))
        asset = data["asset"]
        pres = asset.get("presentation", {})
        if "defaultScale" not in pres:
            scales = [k for k, v in asset["representations"].items() if isinstance(v, dict) and v.get("status") == "authored"]
            missing.append((asset["id"], scales, pres.get("defaultPose")))
print("MISSING defaultScale", len(missing))
for row in missing:
    print(" ", row)
