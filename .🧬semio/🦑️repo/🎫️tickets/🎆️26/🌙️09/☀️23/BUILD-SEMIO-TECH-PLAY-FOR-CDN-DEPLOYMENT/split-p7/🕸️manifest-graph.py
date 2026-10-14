#!/usr/bin/env python3
"""🕸️ Reads a cut family's member manifests and reports family edges, lib cycles and dev-dependency back edges
(a dev-dependency on a crate that depends on the tester links two copies of the tester).

usage: 🕸️manifest-graph.py <artifact dir (repo-relative)> <crate prefix>
"""
import glob, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
artifact, prefix = sys.argv[1:3]
A = os.path.join(REPO, artifact)
deps, dev = {}, {}
for path in glob.glob(os.path.join(A, "**/📦️packages/🦀️rust/Cargo.toml"), recursive=True):
    if any(x in path for x in ("🏭️bridge", "👽️guest", "🔮️oracles")):
        continue
    text = open(path, encoding="utf-8").read()
    name = re.search(r'(?m)^name = "([^"]+)"', text).group(1)
    section, d, v = None, set(), set()
    for line in text.splitlines():
        h = re.match(r"^\[(.+)\]$", line.strip())
        if h:
            section = h.group(1)
            continue
        m = re.match(r'^(%s[\w-]*)\s*=' % re.escape(prefix), line)
        if m and section == "dependencies":
            d.add(m.group(1))
        elif m and section == "dev-dependencies":
            v.add(m.group(1))
    deps[name], dev[name] = d, v
def reach(start):
    seen, stack = set(), [start]
    while stack:
        for n in deps.get(stack.pop(), ()):
            if n not in seen:
                seen.add(n); stack.append(n)
    return seen
for name in sorted(deps):
    print(f"{name}: {' '.join(sorted(x.replace(prefix, '') or '<root>' for x in deps[name]))}")
bad = 0
for name in sorted(deps):
    r = reach(name)
    if name in r:
        bad += 1; print(f"CYCLE {name}")
    for t in sorted(dev[name]):
        if name in reach(t) or t == name:
            bad += 1; print(f"DEVBACK {name} -> {t}")
    for t in sorted(deps[name] | dev[name]):
        if t not in deps:
            bad += 1; print(f"MISSING {name} -> {t}")
print(f"crates={len(deps)} problems={bad}")
