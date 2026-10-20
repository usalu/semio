#!/usr/bin/env python3
"""Gives an overlay a private node_modules when the dependency partition is unmounted (after a reboot).

usage: mknodemodules.py <checkout> <overlay> <closure-packages.json> <name>...
Copies the named third-party packages from the global bun cache (highest version; copies, because node resolves siblings from the real path) and every workspace package of
the closure file from the live checkout. Nothing in the checkout is written.
"""
import json, os, re, shutil, sys

checkout, overlay, closure = (os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3])
names = sys.argv[4:]
cache = os.path.expanduser("~/.bun/install/cache")
target = os.path.join(overlay, "node_modules")
if os.path.islink(target):
    os.unlink(target)
os.makedirs(target, exist_ok=True)


def version_key(entry):
    match = re.search(r"@(\d+)\.(\d+)\.(\d+)", entry)
    return tuple(int(part) for part in match.groups()) if match else (0, 0, 0)


def link(name, source):
    destination = os.path.join(target, name)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if not os.path.lexists(destination):
        os.symlink(source, destination)


def copy(name, source):
    destination = os.path.join(target, name)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if not os.path.lexists(destination):
        shutil.copytree(source, destination, symlinks=True)


for spec in names:
    name, _, pin = spec.partition("=")
    scope, _, leaf = name.rpartition("/")
    folder = os.path.join(cache, scope) if scope else cache
    pattern = re.escape(leaf) + (r"@" + re.escape(pin) + r"@@@1" if pin else r"@\d+\.\d+\.\d+.*@@@1")
    candidates = sorted([entry for entry in os.listdir(folder) if re.fullmatch(pattern, entry)], key=version_key) if os.path.isdir(folder) else []
    if not candidates:
        print("missing in cache:", spec)
        continue
    copy(name, os.path.join(folder, candidates[-1]))
for package, directory in json.load(open(closure)).items():
    link(package, os.path.join(checkout, directory))
print("node_modules ready", len(os.listdir(target)))
