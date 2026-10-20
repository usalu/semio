#!/usr/bin/env python3
# usage: overlay.py OV [COPY...]   scratch overlay of the repository: COPY paths (default: the repo library) are real copies, everything else is symlinked;
# node_modules is re-created as a real directory whose workspace links that point into a copied path are re-pointed into the overlay.
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CLONES = ["🧰️framework", "✏️s"]


def clone(src, dst):
    import subprocess
    subprocess.run(["cp", "-c", "-R", src, dst], check=True)


def build(ov, copies):
    if os.path.exists(ov):
        shutil.rmtree(ov)
    os.makedirs(ov)
    copies = [os.path.normpath(c) for c in copies]
    needed = {""}
    for copy in copies:
        parts = copy.split("/")
        for i in range(1, len(parts)):
            needed.add("/".join(parts[:i]))
    for rel in sorted(needed, key=lambda r: r.count("/")):
        src = os.path.join(ROOT, rel)
        dst = os.path.join(ov, rel)
        os.makedirs(dst, exist_ok=True)
        for name in os.listdir(src):
            child = (rel + "/" + name).lstrip("/")
            if child in needed or child in copies or name == "node_modules":
                continue
            link = os.path.join(src, name)
            if os.path.isfile(link) and not os.path.islink(link) and os.path.getsize(link) < (4 << 20):
                shutil.copy2(link, os.path.join(dst, name))
            else:
                os.symlink(link, os.path.join(dst, name))
    for copy in copies:
        src = os.path.join(ROOT, copy)
        dst = os.path.join(ov, copy)
        clone(src, dst)
    real = os.path.realpath(os.path.join(ROOT, "node_modules"))
    nm = os.path.join(ov, "node_modules")
    os.makedirs(nm)
    for name in os.listdir(real):
        src = os.path.join(real, name)
        if name != "@semio-tech":
            os.symlink(src, os.path.join(nm, name))
    scope = os.path.join(nm, "@semio-tech")
    os.makedirs(scope)
    for name in os.listdir(os.path.join(real, "@semio-tech")):
        target = os.readlink(os.path.join(real, "@semio-tech", name))
        for copy in copies:
            prefix = os.path.join(ROOT, copy)
            if target == prefix or target.startswith(prefix + "/"):
                target = os.path.join(ov, copy) + target[len(prefix):]
        os.symlink(target, os.path.join(scope, name))
    return ov


def add(ov, rel):
    rel = os.path.normpath(rel)
    parts = rel.split("/")
    for i in range(1, len(parts)):
        cur = os.path.join(ov, *parts[:i])
        if os.path.islink(cur):
            target = os.readlink(cur)
            os.unlink(cur)
            os.makedirs(cur)
            for name in os.listdir(target):
                link = os.path.join(target, name)
                if os.path.isfile(link) and not os.path.islink(link) and os.path.getsize(link) < (4 << 20):
                    shutil.copy2(link, os.path.join(cur, name))
                else:
                    os.symlink(link, os.path.join(cur, name))
    dst = os.path.join(ov, rel)
    if not os.path.islink(dst):
        return
    os.unlink(dst)
    src = os.path.join(ROOT, rel)
    clone(src, dst)
    scope = os.path.join(ov, "node_modules", "@semio-tech")
    prefix = os.path.join(ROOT, rel)
    for name in os.listdir(scope):
        link = os.path.join(scope, name)
        target = os.readlink(link)
        if target == prefix or target.startswith(prefix + "/"):
            os.unlink(link)
            os.symlink(os.path.join(ov, rel) + target[len(prefix):], link)


def remap(ov, target):
    for copy in CLONES:
        prefix = os.path.join(ROOT, copy)
        if target == prefix or target.startswith(prefix + "/"):
            return os.path.join(ov, copy) + target[len(prefix):]
    return target


def localize_node_modules(ov):
    count = 0
    for copy in CLONES:
        for base, dirs, _ in os.walk(os.path.join(ov, copy)):
            dirs[:] = [name for name in dirs if not os.path.islink(os.path.join(base, name))]
            link = os.path.join(base, "node_modules")
            if not os.path.islink(link):
                continue
            target = os.path.realpath(link)
            if not os.path.isdir(target):
                continue
            os.unlink(link)
            os.makedirs(link)
            for name in os.listdir(target):
                if name == "@semio-tech":
                    scope = os.path.join(link, name)
                    os.makedirs(scope)
                    for member in os.listdir(os.path.join(target, name)):
                        os.symlink(remap(ov, os.readlink(os.path.join(target, name, member))), os.path.join(scope, member))
                else:
                    os.symlink(os.path.join(target, name), os.path.join(link, name))
            count += 1
    return count


def closure(ov, entries):
    import json, subprocess
    out = subprocess.run(["bun", os.path.join(HERE, "closure.ts"), ROOT, *entries], capture_output=True, text=True, cwd=ROOT)
    data = json.loads(out.stdout)
    for rel in data["files"] + data["schemas"]:
        add(ov, rel)
    return len(data["files"]), len(data["schemas"])


if __name__ == "__main__":
    if sys.argv[1] == "--closure":
        print(closure(sys.argv[2], sys.argv[3:]))
    elif sys.argv[1] == "--add":
        for item in sys.argv[3:]:
            add(sys.argv[2], item)
    else:
        build(sys.argv[1], sys.argv[2:] or CLONES)
        print(sys.argv[1], localize_node_modules(sys.argv[1]))
