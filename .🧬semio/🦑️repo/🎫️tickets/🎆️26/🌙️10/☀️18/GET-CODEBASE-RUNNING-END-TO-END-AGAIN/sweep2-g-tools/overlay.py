#!/usr/bin/env python3
"""sweep2-g overlay: a scratch checkout that symlinks every entry of the real repository except along the paths of replaced files, which are real directories holding the staged copy. usage: overlay.py <scratch-dir> <map.json> where map = {"workspace/relative/path": "/abs/staged/file"}. Never writes into the real tree."""
import json, os, sys
root = "/Users/ueli/Documents/semio"
scratch, mapping = sys.argv[1], json.load(open(sys.argv[2]))
def build(rel, target):
    os.makedirs(target, exist_ok=True)
    wanted = {}
    for path, staged in mapping.items():
        if rel and not (path + "/").startswith(rel + "/"): continue
        rest = path[len(rel) + 1:] if rel else path
        head, _, tail = rest.partition("/")
        wanted.setdefault(head, []).append((tail, staged))
    for name in os.listdir(os.path.join(root, rel) if rel else root):
        if name in (".git", ".cache", "bazel-out", ".🧬semio"): continue
        src = os.path.join(root, rel, name) if rel else os.path.join(root, name)
        dst = os.path.join(target, name)
        if name in wanted:
            if any(tail == "" for tail, _ in wanted[name]):
                staged = [s for t, s in wanted[name] if t == ""][0]
                with open(staged, "rb") as a, open(dst, "wb") as b: b.write(a.read())
            else:
                build((rel + "/" if rel else "") + name, dst)
        else:
            os.symlink(src, dst)
build("", scratch)
