"""🌾️ Collects the seconds of every finished asset record of the whole ticket folder since the engine of a given time: `python3 harvest.py <U> <since YYYY-MM-DD HH:MM> [until]`; prints one TSV line per record `<asset> <category> <document bytes> <seconds> <record bytes> <mtime> <folder>` (the newest record of an asset, current digest only)."""
import glob, hashlib, json, os, sys, time

root, since = sys.argv[1], time.mktime(time.strptime(sys.argv[2], "%Y-%m-%d %H:%M"))
catalog = os.path.join(root, *[".."] * 7, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
docs = {}
for path in glob.glob(os.path.join(catalog, "*", "*", "🔣️.json")):
    docs[os.path.basename(os.path.dirname(path))] = (os.path.basename(os.path.dirname(os.path.dirname(path))), path)
best = {}
for path in glob.glob(os.path.join(root, "🗑️generated", "**", "cells", "*.json"), recursive=True):
    mtime = os.path.getmtime(path)
    if mtime < since:
        continue
    try:
        record = json.load(open(path))
    except Exception:
        continue
    if record.get("outcome") != "done" or record["asset"] not in docs:
        continue
    category, doc = docs[record["asset"]]
    if record["asset"] in best and best[record["asset"]][0] > mtime:
        continue
    best[record["asset"]] = (mtime, category, doc, record, path)
for asset, (mtime, category, doc, record, path) in sorted(best.items()):
    current = hashlib.sha256(json.dumps(json.load(open(doc))["asset"], ensure_ascii=False, separators=(",", ":")).encode()).hexdigest()
    print(asset, category, os.path.getsize(doc), round(record["seconds"], 1), os.path.getsize(path), time.strftime("%H:%M", time.localtime(mtime)), "current" if current == record["digest"] else "stale", path.split("matrix/")[1].split("/")[0], sep="\t")
