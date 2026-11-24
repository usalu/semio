"""🧾️ Collects the declared dimensions the repair of SV-2 changed (the `dims` ops of `repair/<slug>.json`, compared with the state before the repair) into `manifest-changes.json` for the coordinator; prints one line per plant.
python3 manifest-changes.py"""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
BACKUP = os.path.join(HERE, "..", "..", "🗑️generated", "silhouette", "SV-2", "backup")
out = {}
for path in sorted(glob.glob(os.path.join(HERE, "repair", "*.json"))):
    slug = os.path.basename(path)[:-5]
    changed = {}
    for op in json.load(open(path)):
        if op["op"] == "dims": changed.update(op["set"])
    before_file = os.path.join(BACKUP, slug + ".json")
    if os.path.exists(before_file):
        before = json.load(open(before_file))["asset"]["dimensions"]
        changed = {k: v for k, v in changed.items() if before.get(k) != v}
        if changed: print(slug, {k: (before.get(k), v) for k, v in changed.items()})
    if changed: out[slug] = changed
json.dump(out, open(os.path.join(HERE, "manifest-changes.json"), "w"), indent=2, ensure_ascii=False)
open(os.path.join(HERE, "manifest-changes.json"), "a").write("\n")
