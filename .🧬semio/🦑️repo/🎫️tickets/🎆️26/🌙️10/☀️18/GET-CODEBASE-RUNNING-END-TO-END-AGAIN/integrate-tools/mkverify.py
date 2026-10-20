#!/usr/bin/env python3
# 🧪 Builds a verification root (COPIES only) holding the relative-import closure of the given entry files, resolved first in the merged tree, then in the live tree.
# usage: mkverify.py <out-root> <entry-rel> [<entry-rel> ...] [--merged DIR] [--live DIR] [--extra FILE-REL ...]
# Bare specifiers are not followed (they resolve through node_modules; copy the few packages a test needs with copy-package.sh).
import os, re, shutil, subprocess, sys

args = sys.argv[1:]
out = args[0]
flag = lambda name, default: args[args.index(name) + 1] if name in args else default
merged, live = flag("--merged", "/private/tmp/ig/merged"), flag("--live", "/Users/ueli/Documents/semio")
entries = [a for a in args[1:] if not a.startswith("--") and a not in (merged, live)]
IMPORT = re.compile(r"""(?:\bfrom\s*|\bimport\s*\(\s*(?:/\*.*?\*/\s*)?|\bimport\s+|\brequire\s*\(\s*|\bexport\s+\*\s+from\s*)(["'`])(\.{1,2}/[^"'`$]+)\1""")
EXT = ["", ".ts", ".tsx", ".mts", ".mjs", ".cjs", ".js", ".json", ".d.mts", ".d.ts"]
seen, missing = set(), []


def find(rel):
    for root in (merged, live):
        path = os.path.join(root, rel)
        if os.path.isfile(path):
            return root, path
    return None, None


def walk(rel):
    if rel in seen:
        return
    seen.add(rel)
    root, path = find(rel)
    if path is None:
        missing.append(rel)
        return
    dest = os.path.join(out, rel)
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    if not os.path.exists(dest) or root == merged:
        shutil.copyfile(path, dest)
    for plain, declaration in ((".mjs", ".d.mts"), (".cjs", ".d.cts"), (".js", ".d.ts")):
        if rel.endswith(plain):
            walk(rel[: -len(plain)] + declaration) if find(rel[: -len(plain)] + declaration)[1] else None
    if not rel.endswith((".ts", ".tsx", ".mts", ".mjs", ".cjs", ".js")):
        return
    text = open(path, encoding="utf8", errors="replace").read()
    for match in IMPORT.finditer(text):
        spec = match.group(2)
        base = os.path.normpath(os.path.join(os.path.dirname(rel), spec))
        if base.startswith(".."):
            missing.append(f"{rel} -> {spec} (outside the root)")
            continue
        for ext in EXT:
            candidate = base + ext
            if find(candidate)[1]:
                walk(candidate)
                break
            index = os.path.join(base, "index" + ext) if ext else None
        else:
            missing.append(f"{rel} -> {spec}")


os.makedirs(out, exist_ok=True)
for entry in entries:
    walk(entry)
extras = [args[i + 1] for i, a in enumerate(args) if a == "--extra"]
for extra in extras:
    walk(extra)
print(f"copied {len(seen) - len([m for m in missing if ' -> ' not in m])} files into {out}")
for line in missing:
    print("MISSING", line)
