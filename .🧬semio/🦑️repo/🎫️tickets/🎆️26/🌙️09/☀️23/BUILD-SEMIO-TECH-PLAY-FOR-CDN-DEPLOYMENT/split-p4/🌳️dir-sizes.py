#!/usr/bin/env python3
"""🌳️ Prints a directory tree (to a depth) with recursive .rs file count and line count, tests split out.

usage: 🌳️dir-sizes.py <root> [depth]
"""
import os, sys
root, depth = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 3
def walk(path, level):
    files = lines = tfiles = tlines = 0
    rows = []
    for name in sorted(os.listdir(path)):
        full = os.path.join(path, name)
        if name in ("target", "node_modules") or name.startswith("."):
            continue
        if os.path.isdir(full):
            sub = walk(full, level + 1)
            files += sub[0]; lines += sub[1]; tfiles += sub[2]; tlines += sub[3]
            rows.append((name, sub))
        elif name.endswith(".rs"):
            n = sum(1 for _ in open(full, encoding="utf-8"))
            if "🧪️tests" in full or "/tests/" in full:
                tfiles += 1; tlines += n
            else:
                files += 1; lines += n
    return files, lines, tfiles, tlines, rows
def show(name, data, level):
    if level > depth:
        return
    print(f"{'  ' * level}{name}  src {data[0]}f/{data[1]}L  test {data[2]}f/{data[3]}L")
    for child, sub in data[4]:
        if sub[0] + sub[2]:
            show(child, sub, level + 1)
show(root, walk(root, 0), 0)
