import json, sys, collections
p = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧍️staffage\🔣️.json"
d = json.load(open(p, encoding="utf-8"))
groups = collections.defaultdict(list)
for k, v in d["casts"].items():
    if v["view"] == "thermal":
        env = v.get("environment") or "TL:" + ",".join(x["environment"] for x in v["timeline"])
        parts = k.split("-")
        groups["-".join(parts[:3])].append((k, v["asset"], env))
print(len(d["casts"]), sum(len(g) for g in groups.values()))
for g, items in sorted(groups.items()):
    print(g)
    for k, a, e in items:
        print("   ", k, a, e)
