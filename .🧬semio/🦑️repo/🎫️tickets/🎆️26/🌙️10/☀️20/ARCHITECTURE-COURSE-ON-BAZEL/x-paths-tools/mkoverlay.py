#!/usr/bin/env python3
"""Builds a verification overlay: real COPIES of the leaf directories, symlinks to the live tree for everything else.

usage: mkoverlay.py <checkout-root> <overlay-root> <leaf-relative-path>...
Every ancestor of a leaf becomes a real directory whose other entries are symlinks into the live checkout, so
relative imports from the copied leaves resolve to the live framework while edits to the leaves never touch it.
"""
import os, shutil, sys

root, overlay, leaves = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3:]


def real_directory(live, copy):
    if os.path.islink(copy):
        os.unlink(copy)
    os.makedirs(copy, exist_ok=True)


def fill(live, copy, keep):
    for name in os.listdir(live):
        if name == keep or name == ".git":
            continue
        target = os.path.join(copy, name)
        if not os.path.lexists(target):
            os.symlink(os.path.join(live, name), target)


if os.path.lexists(overlay):
    shutil.rmtree(overlay)
os.makedirs(overlay)
for leaf in leaves:
    parts = leaf.split("/")
    live, copy = root, overlay
    for index, part in enumerate(parts):
        real_directory(live, copy)
        fill(live, copy, part)
        live, copy = os.path.join(live, part), os.path.join(copy, part)
    if os.path.lexists(copy):
        shutil.rmtree(copy) if not os.path.islink(copy) else os.unlink(copy)
    shutil.copytree(live, copy, symlinks=True)
print("overlay ready", overlay)
