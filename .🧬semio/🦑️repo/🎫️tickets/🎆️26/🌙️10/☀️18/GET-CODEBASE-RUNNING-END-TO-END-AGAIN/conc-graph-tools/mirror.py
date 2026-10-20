#!/usr/bin/env python3
"""Builds a scratch overlay of the repository for the conc-graph set: every touched file is a byte copy, every directory on its path is real, every other entry is a symlink to the live tree (never a hard link, nothing is written to the live tree).

usage: mirror.py OVERLAY_DIR [--root REPO_ROOT]     then:  apply.py apply --write --root OVERLAY_DIR --ignore-running"""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
overlay = os.path.abspath(sys.argv[1])
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
spec = json.load(open(os.path.join(HERE, "staged", "ops.json"), encoding="utf8"))
paths = sorted(set(row["path"] for row in spec["ops"]) | set(row["path"] for row in spec.get("replace", [])) | set(spec["new"].values()))
files = set(row["path"] for row in spec["ops"]) | set(row["path"] for row in spec.get("replace", []))


def build(rel_dir, children):
    real, target = os.path.join(ROOT, rel_dir), os.path.join(overlay, rel_dir)
    os.makedirs(target, exist_ok=True)
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in children:
                continue
            link = os.path.join(target, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(real, name), link)
    for name, sub in children.items():
        path = os.path.join(rel_dir, name)
        if sub is None:
            if path in files:
                shutil.copyfile(os.path.join(ROOT, path), os.path.join(overlay, path))
            continue
        build(path, sub)


tree = {}
for path in paths:
    node = tree
    parts = path.split(os.sep)
    for part in parts[:-1]:
        node = node.setdefault(part, {})
    node.setdefault(parts[-1], None)
shutil.rmtree(overlay, ignore_errors=True)
build("", tree)
E = lambda code: chr(code) + "\ufe0f"
LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
for real in (LIB, os.path.join(E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process")):
    target = os.path.join(overlay, real)
    shutil.rmtree(target)
    shutil.copytree(os.path.join(ROOT, real), target, symlinks=True, ignore=shutil.ignore_patterns("node_modules", "target", ".DS_Store"))
for base, dirs, names in os.walk(overlay):
    dirs[:] = [d for d in dirs if not os.path.islink(os.path.join(base, d))]
    for name in names:
        link = os.path.join(base, name)
        if os.path.islink(link) and os.path.isfile(link):
            target = os.path.realpath(link)
            os.remove(link)
            shutil.copyfile(target, link)
print("overlay ready:", overlay)
