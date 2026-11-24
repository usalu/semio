"""📋 Counts, from the asset documents alone, the authored bands whose front or plan shape list is byte-identical to the same view of another authored band (a lower bound of `derived-view-band-copy`). usage: python3 static-copies.py <catalog-dir>"""
import collections
import glob
import json
import sys

rows = []
per_category = collections.Counter()
assets_per_category = collections.Counter()
for path in sorted(glob.glob(sys.argv[1] + "/*/*/🔣️.json")):
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    assets_per_category[asset["category"]] += 1
    authored = {key: rep for key, rep in asset["representations"].items() if rep["status"] == "authored"}
    hits = []
    for key, rep in authored.items():
        for view in ("front", "plan"):
            mine = json.dumps(rep["geometry"][view], sort_keys=True)
            same = [other for other, target in authored.items() if other != key and json.dumps(target["geometry"][view], sort_keys=True) == mine]
            if same:
                hits.append((key, view, same))
    if hits:
        per_category[asset["category"]] += 1
        rows.append((asset["category"], asset["id"], hits))
print("assets with an identical front or plan in two bands:", len(rows), "of", sum(assets_per_category.values()))
for category in sorted(assets_per_category):
    print(f"  {category}: {per_category[category]} of {assets_per_category[category]}")
print("band-view entries:", sum(len(hits) for _, _, hits in rows))
for category, asset, hits in rows:
    print(category, asset, " ".join(f"{key}:{view}={'/'.join(same)}" for key, view, same in hits))
