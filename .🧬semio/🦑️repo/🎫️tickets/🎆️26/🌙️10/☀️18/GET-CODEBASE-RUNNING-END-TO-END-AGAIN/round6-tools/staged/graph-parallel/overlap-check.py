#!/usr/bin/env python3
"""🧭 Finds, in the CURRENT live files, every anchor of the farm set that overlaps an anchor of another staged set (character ranges), which would make the two sets apply in only one order.
usage: overlap-check.py [--root REPO_ROOT]   Other sets: graph-mem, conc-graph, graph-speed, loop-hunt (their ops.json under ../<name>-tools/staged/). Stdlib only."""
import json, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
root = HERE
while not (root / ".git").exists() and root != root.parent:
    root = root.parent
if "--root" in sys.argv:
    root = pathlib.Path(sys.argv[sys.argv.index("--root") + 1])
TICKET = HERE.parent
mine = []
for name in ("ops",):
    for op in json.loads((HERE / ("staged/%s.json" % name)).read_text(encoding="utf8"))["edits"]:
        mine.append((op["file"], op["anchor"], op["replacement"], name))


def others():
    for tool in ("graph-mem", "conc-graph", "graph-speed", "loop-hunt"):
        for path in sorted((TICKET / (tool + "-tools/staged")).glob("*.json")):
            try:
                data = json.loads(path.read_text(encoding="utf8"))
            except ValueError:
                continue
            rows = data.get("edits") if isinstance(data, dict) else None
            rows = rows or (data.get("ops") if isinstance(data, dict) else None) or []
            for row in rows:
                if isinstance(row, dict) and "anchor" in row:
                    yield tool + "/" + path.name, row.get("file") or row.get("path"), row["anchor"]


cache = {}
def text(file):
    if file not in cache:
        p = root / file
        cache[file] = p.read_text(encoding="utf8") if p.exists() else ""
    return cache[file]


def span(file, anchor):
    t = text(file)
    i = t.find(anchor)
    return (i, i + len(anchor)) if i >= 0 and t.count(anchor) == 1 else None


mine_spans = {}
for file, anchor, _, name in mine:
    s = span(file, anchor)
    if s:
        mine_spans.setdefault(file, []).append((s, anchor[:60].replace("\n", " "), name))
found = 0
for source, file, anchor in others():
    if file not in mine_spans:
        continue
    theirs = span(file, anchor)
    if theirs is None:
        continue
    for (a, b), label, name in mine_spans[file]:
        if a < theirs[1] and theirs[0] < b:
            found += 1
            print("OVERLAP %s [%s] with %s: %s | %s" % (file[-48:], name, source, label, anchor[:60].replace("\n", " ")))
print("overlaps:", found)
raise SystemExit(1 if found else 0)
