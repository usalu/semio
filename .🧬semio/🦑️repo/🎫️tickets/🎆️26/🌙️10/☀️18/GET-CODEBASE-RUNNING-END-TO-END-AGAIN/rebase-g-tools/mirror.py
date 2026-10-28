#!/usr/bin/env python3
"""rev3fix-g overlay: builds a scratch tree whose ancestors of the given paths are real directories of symlinks and whose given paths are clones of the live ones (regular files beside a real directory are cloned too: the planner refuses symlinked or multiply linked sources), so a staged batch can be applied and tested without touching the live tree.
Usage: overlay.py <overlay-root> <path>...   (paths relative to the repo root; a path that does not exist yet only gets its ancestors)"""
import os, shutil, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
def main():
    import json
    out, paths = sys.argv[1], []
    for arg in sys.argv[2:]:
        if arg.startswith("@"):
            spec = json.load(open(arg[1:], encoding="utf-8"))
            paths += sorted({e.get("file") or spec.get("file") for e in spec["edits"]}) + [n["path"] for n in spec.get("newFiles",[])]
        else: paths.append(arg)
    os.makedirs(out, exist_ok=True)
    real = set()
    for p in paths:
        parts = p.split("/")
        for i in range(0, len(parts)): real.add("/".join(parts[:i]))
    targets = {p for p in paths if os.path.exists(os.path.join(ROOT, p))}
    def fill(rel):
        live = os.path.join(ROOT, rel) if rel else ROOT
        dest = os.path.join(out, rel) if rel else out
        os.makedirs(dest, exist_ok=True)
        if not os.path.isdir(live): return
        for name in os.listdir(live):
            child = (rel + "/" + name) if rel else name
            target = os.path.join(dest, name)
            if os.path.lexists(target): 
                if child in real and child not in targets: fill(child)
                continue
            if child in targets or (os.path.isfile(os.path.join(live, name)) and not os.path.islink(os.path.join(live, name))):
                subprocess.run(["cp", "-cR", os.path.join(live, name), target], check=True)
            elif child in real:
                fill(child)
            else:
                os.symlink(os.path.join(live, name), target)
    fill("")
main()
