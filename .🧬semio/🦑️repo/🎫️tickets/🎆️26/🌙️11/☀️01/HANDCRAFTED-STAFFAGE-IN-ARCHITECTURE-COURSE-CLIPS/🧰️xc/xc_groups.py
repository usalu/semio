import json
import sys
from collections import defaultdict
from pathlib import Path

MANIFEST = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧍️staffage\🔣️.json")
casts = json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"]


def norm(e):
    e = dict(e)
    if e.get("fps") == 15 or "pose" in e:
        e.pop("fps", None)
    return json.dumps(e, sort_keys=True, ensure_ascii=False)


groups = defaultdict(list)
for cid, e in casts.items():
    groups[norm(e)].append(cid)
n = 0
for k, ids in groups.items():
    if len(ids) > 1:
        n += 1
        print(len(ids), k)
        print("    ", ids)
print(n, "groups,", sum(len(v) - 1 for v in groups.values()), "removable ids; distinct", len(groups), "of", len(casts))
