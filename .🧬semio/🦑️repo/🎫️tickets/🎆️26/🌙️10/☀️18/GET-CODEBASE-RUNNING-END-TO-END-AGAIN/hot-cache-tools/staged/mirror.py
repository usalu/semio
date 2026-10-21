#!/usr/bin/env python3
"""🪞 mirror.py LIVE SCRATCH OVERLAY_DIR...: builds SCRATCH as real directories only along the paths of the overlay files and symlinks to LIVE everywhere else, then copies the overlay files in.
Nothing is ever written through a symlink: every directory on the way to an overlay file is real."""
import os, shutil, sys
live, scratch = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
overlays = [os.path.abspath(p) for p in sys.argv[3:]]
files = []
for base in overlays:
    for directory, _, names in os.walk(base):
        for name in names:
            full = os.path.join(directory, name)
            files.append((os.path.relpath(full, base), full))
real = set()
for rel, _ in files:
    parts = rel.split(os.sep)
    for i in range(len(parts)):
        real.add(os.sep.join(parts[:i]))
def build(rel):
    source, target = os.path.join(live, rel), os.path.join(scratch, rel)
    os.makedirs(target, exist_ok=True)
    if not os.path.isdir(source):
        return
    for name in os.listdir(source):
        child = os.path.join(rel, name) if rel else name
        if child in real:
            build(child)
        elif not any(r == child for r, _ in files):
            link = os.path.join(target, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(source, name), link)
shutil.rmtree(scratch, ignore_errors=True)
build("")
for rel, full in files:
    target = os.path.join(scratch, rel)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copy2(full, target)
print(len(files), "overlay files into", scratch)
