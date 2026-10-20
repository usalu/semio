#!/usr/bin/env python3
"""Copy-on-write overlay of the repository for compile checks outside the repo.
usage: mkovl.py <overlay-dir> [--through GROUP] [--prefix REPO_RELATIVE_PREFIX]...
Builds <overlay-dir> mirroring the repository: every directory on the way to a staged file is real, all its other children are symlinks
to the live tree, and each staged file is written with its staged bytes. Nothing is written to the repository."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
args = sys.argv[1:]
dest = args[0]
manifest = json.load(open(os.path.join(HERE, "staged", "manifest.json"), encoding="utf8"))
groups = manifest["groups"]
through = args[args.index("--through") + 1] if "--through" in args else groups[-1]
allowed = groups[: groups.index(through) + 1]
prefixes = [args[i + 1] for i, a in enumerate(args) if a == "--prefix"]
reals = [args[i + 1] for i, a in enumerate(args) if a == "--real"]
shutil.rmtree(dest, ignore_errors=True)
os.makedirs(dest)
tree = {}
for real in reals:
    node = tree
    for part in real.split("/"):
        node = node.setdefault(part, {})
    target = os.path.join(dest, real)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copytree(os.path.join(ROOT, real), target, symlinks=True, ignore=shutil.ignore_patterns("node_modules", "target", "🗑️generated"))
for row in manifest["files"]:
    steps = [s for s in row["steps"] if s["group"] in allowed]
    if not steps or (prefixes and not any(row["path"].startswith(p) for p in prefixes)):
        continue
    node = tree
    parts = row["path"].split("/")
    for part in parts[:-1]:
        node = node.setdefault(part, {})
    node[parts[-1]] = os.path.join(HERE, "staged", steps[-1]["after"])

def build(real, over, node):
    os.makedirs(over, exist_ok=True)
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in node or os.path.lexists(os.path.join(over, name)):
                continue
            os.symlink(os.path.join(real, name), os.path.join(over, name))
    for name, child in node.items():
        if isinstance(child, dict):
            build(os.path.join(real, name), os.path.join(over, name), child)
        else:
            shutil.copyfile(child, os.path.join(over, name))
build(ROOT, dest, tree)
print(dest)
