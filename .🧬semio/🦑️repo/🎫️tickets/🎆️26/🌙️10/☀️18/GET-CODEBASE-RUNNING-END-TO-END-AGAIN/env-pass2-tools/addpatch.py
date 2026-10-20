#!/usr/bin/env python3
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
name, rel = sys.argv[1], sys.argv[2]
pairs = json.load(open(sys.argv[3], encoding="utf-8"))
path = os.path.join(HERE, "staged", "patches", name + ".json")
specs = json.load(open(path, encoding="utf-8")) if os.path.exists(path) else []
text = open(os.path.join(ROOT, rel), encoding="utf-8").read()
edits = []
for old, new in pairs:
    n = text.count(old)
    if n == 0:
        raise SystemExit("anchor missing: %r" % old[:80])
    edits.append({"old": old, "new": new, "count": n})
specs = [s for s in specs if s["file"] != rel] + [{"file": rel, "edits": edits}]
json.dump(specs, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("staged", rel[-80:], len(edits), "edits ->", name)
