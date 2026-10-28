"""🗂 [audit5] Lists the files of a mirror like `git ls-files` would for the harness (skips .git, .🧬semio, node_modules, .cache, target, dist): python3 -I files_list.py <root> <out>"""
import os, sys
root, out = sys.argv[1], sys.argv[2]
skip = {".git", ".🧬semio", "node_modules", ".cache", "target", "dist", ".nx", ".tmp", ".repo"}
with open(out, "w", encoding="utf8") as f:
    for d, dirs, files in os.walk(root):
        dirs[:] = sorted(x for x in dirs if x not in skip)
        for name in sorted(files): f.write(os.path.relpath(os.path.join(d, name), root) + "\n")
