#!/usr/bin/env python3
"""F129: builds a scratch mirror of the graph module (real copy) inside a tree of symlinks to the live checkout, then overlays the staged files. Never writes to the live tree. Usage: mirror.py <scratch-root>"""
import os, shutil, sys
LIVE = "/Users/ueli/Documents/semio"
T = LIVE + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
STAGE = T + "/🧰️scripts/F129/stage"
GRAPH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"
COPY = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
scratch = sys.argv[1]
root = os.path.join(scratch, "repo")
if os.path.exists(root):
    shutil.rmtree(root)
parts = COPY.split("/")
live = LIVE
dst = root
os.makedirs(dst)
for i, part in enumerate(parts):
    for name in os.listdir(live):
        if name == part or name.startswith(".git") and i == 0:
            continue
        os.symlink(os.path.join(live, name), os.path.join(dst, name))
    live = os.path.join(live, part)
    dst = os.path.join(dst, part)
    os.makedirs(dst, exist_ok=True)
shutil.rmtree(dst)
shutil.copytree(live, dst, symlinks=True, ignore=shutil.ignore_patterns("node_modules"))
for base, _dirs, files in os.walk(STAGE):
    for f in files:
        src = os.path.join(base, f)
        rel = os.path.relpath(src, STAGE)
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(src, target)
print(root)
