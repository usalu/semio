#!/usr/bin/env python3
"""Copies (dereferenced, never linked) the named third-party packages and their transitive dependencies from the installed tree into <overlay>/node_modules for the test-only imports of the overlay.

usage: nodemods.py <overlay> <package>..."""
import json, os, shutil, sys

REAL = "/Users/ueli/Documents/semio/node_modules"


def locate(name, base):
    for directory in (base, REAL):
        path = os.path.join(directory, name)
        if os.path.exists(path):
            return os.path.realpath(path)
    return None


def main():
    overlay, names = sys.argv[1], sys.argv[2:]
    target = os.path.join(overlay, "node_modules")
    done, queue = set(), [(name, REAL) for name in names]
    while queue:
        name, base = queue.pop()
        if name in done:
            continue
        source = locate(name, base)
        if source is None:
            print("unresolved", name)
            continue
        done.add(name)
        destination = os.path.join(target, name)
        if not os.path.isdir(destination) or os.path.islink(destination):
            if os.path.islink(destination):
                os.unlink(destination)
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            shutil.copytree(source, destination, symlinks=False, ignore_dangling_symlinks=True)
        manifest = json.load(open(os.path.join(source, "package.json")))
        parent = os.path.dirname(source) if not name.startswith("@") else os.path.dirname(os.path.dirname(source))
        for dep in manifest.get("dependencies", {}):
            queue.append((dep, parent))
    print(len(done), "packages:", " ".join(sorted(done)))


main()
