import json
import sys
from collections import defaultdict
from pathlib import Path

MANIFEST = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧍️staffage\🔣️.json")
casts = json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"]
groups = defaultdict(list)
for cid, e in casts.items():
    groups[json.dumps(e, sort_keys=True, ensure_ascii=False)].append(cid)
print("EXACT DUPLICATES")
for k, ids in groups.items():
    if len(ids) > 1:
        print(len(ids), ids, k)
print()
print("BY ASSET (asset | pose/clip | view | vp | env | mirror | extras) -> ids")
by_asset = defaultdict(list)
for cid, e in casts.items():
    by_asset[e["asset"]].append((e.get("pose") or e.get("clip"), e["view"], e.get("viewpoint", "elevation"), e.get("environment") or ("timeline:" + json.dumps(e.get("timeline"))) if e.get("timeline") else e.get("environment"), e.get("mirror"), json.dumps({k: v for k, v in e.items() if k not in {"asset", "pose", "clip", "view", "viewpoint", "environment", "mirror", "theme", "timeline"}}), cid))
for a in sorted(by_asset):
    print(a)
    for row in sorted(by_asset[a], key=lambda r: tuple(str(x) for x in r)):
        print("   ", row)
