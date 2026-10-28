"""📌 [backlog2] re-records bases.json of an rev3-style batch (edits.json + bases.json) to the sha256 of the CURRENT files of a root, only for files whose every edit still anchors exactly once. usage: python3 -I rebase_bases.py <batch-dir> [--root R]"""
import hashlib, json, os, shutil, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
d = sys.argv[1]
spec = json.load(open(d + "/edits.json", encoding="utf8")); bases = json.load(open(d + "/bases.json", encoding="utf8"))
if not os.path.exists(d + "/bases.json.pre-backlog2"): shutil.copyfile(d + "/bases.json", d + "/bases.json.pre-backlog2")
moved = 0
for rel in sorted({e["file"] for e in spec["edits"]}):
    data = open(f"{root}/{rel}", "rb").read(); text = data.decode("utf8")
    if not all(text.count(e["old"]) == 1 for e in spec["edits"] if e["file"] == rel): print("ANCHOR-BROKEN", rel); continue
    now = hashlib.sha256(data).hexdigest()
    if bases.get(rel) != now: bases[rel] = now; moved += 1
json.dump(bases, open(d + "/bases.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("re-recorded", moved, "bases")
