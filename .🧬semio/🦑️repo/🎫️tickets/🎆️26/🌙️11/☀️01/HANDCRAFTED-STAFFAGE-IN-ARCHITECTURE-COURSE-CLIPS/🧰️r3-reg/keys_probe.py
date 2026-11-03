import json, collections
d = json.load(open("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf-8"))
c = collections.Counter()
vals = collections.defaultdict(collections.Counter)
for k, v in d["casts"].items():
    for kk, vv in v.items():
        c[kk] += 1
        if kk not in ("asset",):
            vals[kk][str(vv)] += 1
print(d.keys(), c)
for k in ("environment", "timeline", "transient", "view", "viewpoint", "theme", "clip", "pose"):
    print(k, dict(vals.get(k, {})))
