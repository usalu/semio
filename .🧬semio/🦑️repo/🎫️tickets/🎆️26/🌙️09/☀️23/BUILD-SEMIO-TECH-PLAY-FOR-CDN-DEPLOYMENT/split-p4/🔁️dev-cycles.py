#!/usr/bin/env python3
"""🔁️ Reports every dev-dependency of a cut family that reaches back to the crate itself through normal dependencies.

A test that names a crate above its own crate links a second copy of the crate under test, so its types never unify;
such tests belong to the upper crate. Reads the family manifests a cut config's crate roots imply.

usage: 🔁️dev-cycles.py <cut.json>
"""
import json, os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
config = json.load(open(sys.argv[1], encoding="utf-8"))
deps, dev = {}, {}
for row in config["crates"]:
    manifest = os.path.join(os.path.dirname(os.path.join(REPO, row["root"])), "📦️packages", "🦀️rust", "Cargo.toml")
    sections, current = {}, None
    for line in open(manifest, encoding="utf-8"):
        header = re.match(r"^\[(.+)\]\s*$", line)
        if header:
            current = sections.setdefault(header[1], [])
        elif current is not None and (row := re.match(r"^([\w-]+)\s*=", line)):
            current.append(row[1])
    name = re.search(r'(?m)^name = "([^"]+)"', open(manifest, encoding="utf-8").read())[1]
    deps[name] = {d for key, rows in sections.items() if key == "dependencies" or key.endswith(".dependencies") and key.startswith("target.") for d in rows}
    dev[name] = set(sections.get("dev-dependencies", []))


def reaches(start, goal, seen=None):
    seen = seen or set()
    for d in deps.get(start, ()):
        if d == goal:
            return True
        if d in deps and d not in seen:
            seen.add(d)
            if reaches(d, goal, seen):
                return True
    return False


for name in deps:
    upward = sorted(d for d in dev[name] if d in deps and reaches(d, name))
    if upward:
        print(f"{name}: {', '.join(upward)}")
