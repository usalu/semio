#!/usr/bin/env python3
"""🧭️ Reports lib-mounted test code that names a family crate depending back on its own crate (split lane P7): such a
`#[cfg(test)]` module links a second copy of the crate under test and must become a `[[test]]` target instead.

usage: 🧭️unit-test-backedges.py <cut config>
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config_path = sys.argv[1]
config = json.load(open(config_path, encoding="utf-8"))
plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))["family"]
artifact = os.path.join(REPO, config["artifact"])
deps = {}
for name, info in plan.items():
    pkg = os.path.join(artifact, info["owner"], "📦️packages/🦀️rust/Cargo.toml") if info["owner"] else os.path.join(artifact, "📦️packages/🦀️rust/Cargo.toml")
    text = open(pkg, encoding="utf-8").read().split("[dev-dependencies]")[0]
    deps[name] = set(re.findall(r"(?m)^(" + re.escape(config["crate"].replace("_", "-")) + r"[\w-]*) =", text))
def reach(n):
    seen, stack = set(), [n]
    while stack:
        for d in deps.get(stack.pop(), ()):
            if d not in seen:
                seen.add(d); stack.append(d)
    return seen
targets = {os.path.join(artifact, t["file"]) for t in config.get("tests", []) + config.get("existing_tests", [])}
bad = 0
for name, info in plan.items():
    files = [os.path.join(artifact, f) for f in info["lib"] + info["test"]]
    for f in files:
        if not os.path.exists(f) or f in targets:
            continue
        text = open(f, encoding="utf-8").read()
        for other in plan:
            if other != name and name in reach(other) and re.search(r"(?<![\w])" + other.replace("-", "_") + r"\b", text):
                bad += 1
                print(f"{name}: {os.path.relpath(f, artifact)} names {other}")
print(f"backedges={bad}")
