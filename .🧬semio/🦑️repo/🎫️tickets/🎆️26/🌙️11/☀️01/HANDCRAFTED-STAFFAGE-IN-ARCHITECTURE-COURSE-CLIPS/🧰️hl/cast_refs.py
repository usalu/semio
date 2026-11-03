import json, os

ROOT = r"C:\git\semio"
CAT = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
VIS = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "⚡️energy", "🏗️authoring", "🎨️visuals", "🧍️staffage", "🔣️.json")

assets = {}
for root, _, files in os.walk(CAT):
    if "🔣️.json" in files and os.path.basename(root) != "🗂️catalog":
        with open(os.path.join(root, "🔣️.json"), encoding="utf-8") as fh:
            d = json.load(fh)
        if "asset" in d:
            assets[d["asset"]["id"]] = d["asset"]

with open(VIS, encoding="utf-8") as fh:
    casts = json.load(fh)["casts"]

problems = []
for name, c in casts.items():
    a = assets.get(c["asset"])
    if a is None:
        problems.append(f"{name}: unknown asset {c['asset']}")
        continue
    if "clip" in c and c["clip"] not in [x["id"] for x in a.get("clips", [])]:
        problems.append(f"{name}: clip {c['clip']} not in {c['asset']}")
    if "pose" in c and c["pose"] not in [x["id"] for x in a.get("poses", [])]:
        problems.append(f"{name}: pose {c['pose']} not in {c['asset']}")
print("casts", len(casts), "problems", len(problems))
for p in problems:
    print(p)
