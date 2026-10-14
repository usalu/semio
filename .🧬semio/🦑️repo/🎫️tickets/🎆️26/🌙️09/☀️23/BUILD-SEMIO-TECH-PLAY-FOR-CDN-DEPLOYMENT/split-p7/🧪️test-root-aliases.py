#!/usr/bin/env python3
"""🪪️ Gives every `[[test]]` root of a cut family the `extern crate <target> as <alias>;` lines its own crate needs
(split lane P7): a test target is a separate crate, so the library root's aliases (`store`, `protocol`, `dsl`, ...) do
not reach it. Scans the test root and every module file it mounts through `#[path]`, then inserts the missing aliases
after the root's `//!` docs and inner attributes.

usage: 🧪️test-root-aliases.py <cut config> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config = json.load(open(sys.argv[1], encoding="utf-8"))
mode = sys.argv[2]
artifact = os.path.join(REPO, config["artifact"])
aliases = config.get("aliases", {})
MOUNT = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*([;{])')


def closure(path, seen):
    if path in seen or not os.path.exists(path):
        return
    seen.add(path)
    text = open(path, encoding="utf-8").read()
    for m in MOUNT.finditer(text):
        target = m.group(1)
        if target.endswith(".rs") and "🧰️framework" not in target:
            closure(os.path.normpath(os.path.join(os.path.dirname(path), target)), seen)


changed = 0
for test in config.get("tests", []) + config.get("existing_tests", []):
    root = os.path.join(artifact, test["file"])
    files = set()
    closure(root, files)
    used = "\n".join(open(f, encoding="utf-8").read() for f in files)
    text = open(root, encoding="utf-8").read()
    need = [f"extern crate {target} as {alias};" for alias, target in aliases.items()
            if re.search(r"(?<![\w.$])(?:::)?" + re.escape(alias) + r"\s*(?:::|\))", used) and not re.search(r"extern crate \w+ as " + re.escape(alias) + r"\s*;", text)]
    if not need:
        continue
    lines = text.split("\n")
    k = 0
    while k < len(lines) and (lines[k].startswith("//!") or lines[k].startswith("#![") or not lines[k].strip()):
        k += 1
    lines[k:k] = need
    print(f"{test['name']}: +{', '.join(a.split(' as ')[1][:-1] for a in need)}")
    changed += 1
    if mode == "apply":
        open(root, "w", encoding="utf-8").write("\n".join(lines))
print(f"roots={changed}")
