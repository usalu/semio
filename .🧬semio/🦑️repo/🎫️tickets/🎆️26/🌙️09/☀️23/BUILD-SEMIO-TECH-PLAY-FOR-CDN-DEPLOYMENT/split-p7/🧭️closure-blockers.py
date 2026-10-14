#!/usr/bin/env python3
"""🧭️ Splits a cut family's crates into those whose normal closure is Source-compliant and those whose closure contains a
refused upstream crate (split lane P7), from `cargo metadata` JSON on stdin-file and a list of refused crate names.

usage: 🧭️closure-blockers.py <metadata.json> <family prefix> <refused crate>...
"""
import json, sys
text = open(sys.argv[1], encoding="utf-8").read()
d = json.loads(text[text.index("{"):])
prefix, refused = sys.argv[2], set(sys.argv[3:])
name = {p["id"]: p["name"] for p in d["packages"]}
deps = {n["id"]: [x["pkg"] for x in n["deps"] if any(k["kind"] is None for k in x["dep_kinds"])] for n in d["resolve"]["nodes"]}
dev = {n["id"]: [x["pkg"] for x in n["deps"]] for n in d["resolve"]["nodes"]}
def closure(roots, graph):
    seen, stack = set(), list(roots)
    while stack:
        for x in graph.get(stack.pop(), []):
            if x not in seen:
                seen.add(x); stack.append(x)
    return seen
for i in sorted(d["workspace_members"], key=lambda i: name[i]):
    if not name[i].startswith(prefix):
        continue
    lib = {name[x] for x in closure([i], deps)} & refused
    tests = {name[x] for x in closure(dev.get(i, []), deps) | set(dev.get(i, []))} & refused
    print(f"{name[i]}\tlib={'BLOCKED by ' + ','.join(sorted(lib)) if lib else 'free'}\ttests={'BLOCKED by ' + ','.join(sorted(tests)) if tests else 'free'}")
