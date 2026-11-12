"""Applies the plan of lateral-convert.ts to a catalog asset as exact-string replacements of the shape objects.
usage: lateral-apply.py <slug> [--dry] [--only view[,view]]   (plan: 🗑️generated/lateral/plan-<slug>.json; the file is re-read right before the write and must be unchanged)"""
import json, sys, os
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
T = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
dry = "--dry" in sys.argv
only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
FILE = f"{S}{slug}/🔣️.json"
raw = open(FILE, encoding="utf-8").read()
doc = json.loads(raw)
plan = json.load(open(f"{T}/🗑️generated/lateral/plan-{slug}.json", encoding="utf-8"))
def dump(obj, base):
    return json.dumps(obj, indent=2, ensure_ascii=False).replace("\n", "\n" + " " * base)
edits = []
for e in plan:
    if only and e["view"] not in only: continue
    old = doc["asset"]["representations"][e["scale"]]["geometry"][e["view"]][e["index"]]
    assert old["id"] == e["id"] and old["bone"] == e["bone"], e
    assert "solid" not in old, e
    texts = [dump(old, b) for b in (12,)]
    text = texts[0]
    n = raw.count(text)
    assert n >= 1, f"old text of {e['scale']} {e['view']} {e['index']} {e['id']} not found"
    edits.append((text, dump(e["new"], 12), e))
# resolve duplicates: every shape of the document with the same text, in document order, against the occurrences in file order
from collections import defaultdict
wanted = {(e["scale"], e["view"], e["index"]): (t, n_) for t, n_, e in edits}
where = defaultdict(list)
def walk(node, path):
    if isinstance(node, dict):
        yield path, node
        for k, v in node.items(): yield from walk(v, path + [k])
    elif isinstance(node, list):
        for i, v in enumerate(node): yield from walk(v, path + [i])
texts = {t for t, _, _ in edits}
for path, node in walk(doc, []):
    if isinstance(node, dict) and "id" in node and "bone" in node:
        t = dump(node, 12)
        if t in texts: where[t].append(tuple(path))
spans = []
for t in texts:
    pos, at = [], 0
    while True:
        i = raw.find(t, at)
        if i < 0: break
        pos.append(i); at = i + len(t)
    locs = where[t]
    assert len(pos) == len(locs), f"{len(pos)} occurrences but {len(locs)} document places"
    for i, path in zip(pos, locs):
        if len(path) >= 6 and path[0] == "asset" and path[1] == "representations" and path[3] == "geometry":
            key = (path[2], path[4], path[5])
            if key in wanted and wanted[key][0] == t: spans.append((i, i + len(t), wanted[key][1]))
assert len(spans) == len(edits), f"{len(spans)} spans for {len(edits)} edits"
spans.sort()
for a, b in zip(spans, spans[1:]): assert a[1] <= b[0], "overlap"
print(slug, "edits", len(spans), "bytes", sum(len(n) - (b - a) for a, b, n in spans))
if dry: sys.exit(0)
again = open(FILE, encoding="utf-8").read()
assert again == raw, "file changed meanwhile"
for a, b, n in sorted(spans, reverse=True): raw = raw[:a] + n + raw[b:]
json.loads(raw)
open(FILE, "w", encoding="utf-8").write(raw)
