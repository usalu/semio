#!/usr/bin/env python3
"""Builds a scratch overlay of the repository for the graph-mem set: the library and process module trees are real copies (so relative imports resolve inside the overlay), every other entry on the way and at the root is a symlink to the live tree (node_modules included), nothing is hard linked.

usage: mirror.py OVERLAY_DIR [--root REPO_ROOT]     then:  apply.py apply --write --root OVERLAY_DIR --ignore-running"""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
root = HERE
while not os.path.exists(os.path.join(root, ".git")) and root != os.path.dirname(root):
    root = os.path.dirname(root)
overlay = os.path.abspath(sys.argv[1])
if "--root" in sys.argv:
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
E = lambda code: chr(code) + "️"
LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
PROCESS = os.path.join(E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process")
real = [LIB, PROCESS]
shutil.rmtree(overlay, ignore_errors=True)


def link_siblings(rel_dir, keep):
    source, target = os.path.join(root, rel_dir), os.path.join(overlay, rel_dir)
    os.makedirs(target, exist_ok=True)
    for name in os.listdir(source):
        if name in keep:
            continue
        link = os.path.join(target, name)
        if not os.path.lexists(link):
            os.symlink(os.path.join(source, name), link)


tree = {}
for path in real:
    node = tree
    for part in path.split(os.sep):
        node = node.setdefault(part, {})


def build(rel_dir, children):
    link_siblings(rel_dir, set(children))
    for name, sub in children.items():
        path = os.path.join(rel_dir, name)
        if sub:
            build(path, sub)
        else:
            shutil.copytree(os.path.join(root, path), os.path.join(overlay, path), symlinks=True, ignore=shutil.ignore_patterns("node_modules", "target", ".DS_Store"))


build("", tree)
print("overlay ready:", overlay)
