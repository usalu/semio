# 🪣️ Spill planner: for an over-budget WP5 crate, the caller-closed sets of methods that can move one rank up without creating an upward call.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stg
from blank import blank
ms = stg.load()
crate = sys.argv[1]; need = int(sys.argv[2])
by = collections.defaultdict(list)
for m in ms: by[m["name"]].append(m)
inside = [m for m in ms if m["crate"] == crate]
key = lambda m: (m["name"], m["slug"], m["start"])
calls = {}
for m in inside:
    b = blank(m["body"])
    names = set(re.findall(r"\bself\s*\.\s*([A-Za-z_0-9]+)\s*(?:::<[^>]*>)?\(", b)) | set(re.findall(r"\b(?:Self|ShellState)\s*::\s*([A-Za-z_0-9]+)\s*(?:::<[^>]*>)?\(", b))
    calls[key(m)] = {key(t) for n in names for t in by.get(n, []) if t["crate"] == crate}
callers = collections.defaultdict(set)
for k, v in calls.items():
    for t in v: callers[t].add(k)
size = {key(m): m["end"] - m["start"] + 1 for m in inside}
def closure(k):
    seen = {k}; todo = [k]
    while todo:
        x = todo.pop()
        for c in callers[x]:
            if c not in seen: seen.add(c); todo.append(c)
    return seen
rows = []
for m in inside:
    k = key(m); cl = closure(k); rows.append((sum(size[x] for x in cl), len(cl), m["name"], m["slug"], size[k]))
rows.sort()
total = sum(size.values())
print(crate, "methods", len(inside), "method lines", total)
for r in rows: print("%5d lines in closure of %2d  %s (%s) own %d" % r)
