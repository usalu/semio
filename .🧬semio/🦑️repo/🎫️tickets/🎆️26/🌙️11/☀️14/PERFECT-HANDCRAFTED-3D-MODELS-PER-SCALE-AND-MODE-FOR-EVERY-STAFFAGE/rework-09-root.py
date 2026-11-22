"""🌱 Author 09 root edit: replaces exact text spans in a catalog asset file (re-read immediately before the write; each span must occur exactly once). Usage: python3 rework-09-root.py <category/slug> <edits.json>  with edits = [[old, new], ...]"""
import glob, json, os, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
category, slug = sys.argv[1].split("/")
path = next(p for p in glob.glob(f"{S}/*{category}/{slug}/*.json"))
edits = json.load(open(sys.argv[2], encoding="utf-8"))
text = open(path, encoding="utf-8").read()
for old, new in edits:
    if text.count(old) != 1:
        raise SystemExit(f"span occurs {text.count(old)} times: {old[:80]}")
    text = text.replace(old, new)
json.loads(text)
with open(path + ".rework09.tmp", "w", encoding="utf-8") as handle:
    handle.write(text)
os.replace(path + ".rework09.tmp", path)
print("root written", path, len(edits), "edit(s)")
