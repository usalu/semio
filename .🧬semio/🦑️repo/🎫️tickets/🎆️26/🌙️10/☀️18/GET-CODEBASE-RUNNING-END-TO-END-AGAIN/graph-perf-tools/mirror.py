#!/usr/bin/env python3
"""usage: mirror.py DIR [--control]   Realizes a scratch repository root at DIR: the library is a REAL COPY (so staged edits and tests never touch the checkout),
every other entry on the path to it is a symlink to the live tree (read-only use), node_modules/bunfig/package.json are symlinks. Run tests with
../inputs-closure-tools/run-tests.sh DIR <path relative to the library>. Apply the staged edits into it with: GP_ROOT=DIR python3 staged/apply.py apply --ignore-running."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
path = HERE
while path != os.path.dirname(path):
    if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
        break
    path = os.path.dirname(path)
ROOT = path
E = lambda code: chr(code) + "️"
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
directory = os.path.abspath(sys.argv[1])
if os.path.exists(directory):
    shutil.rmtree(directory)
os.makedirs(directory)
base = os.path.realpath(directory)


def safe(target):
    real = os.path.realpath(target)
    if real != base and not real.startswith(base + os.sep):
        raise SystemExit("refusing to write outside the mirror: " + target)
    return target


needed = {""}
parts = LIBRARY.split(os.sep)
for depth in range(len(parts)):
    needed.add(os.sep.join(parts[:depth]))
for relative in sorted(needed, key=len):
    real, mirror = os.path.join(ROOT, relative), os.path.join(directory, relative)
    os.makedirs(safe(mirror), exist_ok=True)
    for entry in os.listdir(real):
        child = os.path.join(relative, entry) if relative else entry
        if child in needed or child == LIBRARY:
            continue
        destination = os.path.join(mirror, entry)
        if not os.path.lexists(destination):
            os.symlink(os.path.join(real, entry), safe(destination))
shutil.copytree(os.path.join(ROOT, LIBRARY), safe(os.path.join(directory, LIBRARY)), symlinks=True)
for name in ("node_modules", "bunfig.toml", "package.json", "tsconfig.json", "bun.lock"):
    if os.path.lexists(os.path.join(ROOT, name)) and not os.path.lexists(os.path.join(directory, name)):
        os.symlink(os.path.join(ROOT, name), safe(os.path.join(directory, name)))
print("mirror written to " + directory)
