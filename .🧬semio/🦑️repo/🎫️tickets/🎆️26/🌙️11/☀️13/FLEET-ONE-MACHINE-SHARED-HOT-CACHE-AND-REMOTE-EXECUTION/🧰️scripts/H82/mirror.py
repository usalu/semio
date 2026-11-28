#!/usr/bin/env python3
"""Builds a scratch mirror of the live checkout in which only the named files are real copies (editable),
everything else is a symlink to the live tree, so a staged patch can be tested without touching live files.
usage: mirror.py <scratch-root> <repo-root> <relative file>...   (re-run to refresh untouched copies; edited files are kept)"""
import os, shutil, sys
scratch, repo, files = sys.argv[1], sys.argv[2], sys.argv[3:]
tree = os.path.join(scratch, "tree")
os.makedirs(tree, exist_ok=True)
for name in ("node_modules", "package.json", "bunfig.toml", "tsconfig.json"):
    link = os.path.join(tree, name)
    if not os.path.lexists(link) and os.path.exists(os.path.join(repo, name)):
        os.symlink(os.path.join(repo, name), link)
def ensure_dir(rel):
    """make rel (a directory) real in the mirror, with every sibling symlinked at each level"""
    parts = rel.split(os.sep)
    for depth in range(len(parts)):
        live = os.path.join(repo, *parts[:depth]) if depth else repo
        mirror = os.path.join(tree, *parts[:depth]) if depth else tree
        os.makedirs(mirror, exist_ok=True)
        keep = parts[depth]
        for entry in os.listdir(live):
            if entry == keep or entry in ("node_modules",) and depth == 0:
                continue
            target = os.path.join(mirror, entry)
            if not os.path.lexists(target):
                os.symlink(os.path.join(live, entry), target)
        child = os.path.join(mirror, keep)
        if os.path.islink(child):
            os.remove(child)
    os.makedirs(os.path.join(tree, rel), exist_ok=True)
    for entry in os.listdir(os.path.join(repo, rel)):
        target = os.path.join(tree, rel, entry)
        if not os.path.lexists(target):
            os.symlink(os.path.join(repo, rel, entry), target)
for rel in files:
    directory = os.path.dirname(rel)
    ensure_dir(directory)
    dest = os.path.join(tree, rel)
    live = os.path.join(repo, rel)
    if os.path.islink(dest):
        os.remove(dest)
    if not os.path.exists(dest) and os.path.exists(live):
        shutil.copy2(live, dest)
    print(dest)
