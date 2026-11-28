#!/usr/bin/env python3
"""H83: copies the fleet-status member to a scratch directory and makes its imports that leave the member absolute, so the copy runs outside the worktree. Usage: prep.py <dest>"""
import os, re, shutil, sys
SRC = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status"
dest = sys.argv[1]
shutil.rmtree(dest, ignore_errors=True)
shutil.copytree(SRC, dest, ignore=shutil.ignore_patterns("node_modules", "🗑️generated"))
pattern = re.compile(r'''(from\s*|import\(\s*|import\s*)(["'])(\.{1,2}/[^"']*)\2''')
for root, _, files in os.walk(dest):
    for name in files:
        if not name.endswith(".ts"):
            continue
        file = os.path.join(root, name)
        original = os.path.join(SRC, os.path.relpath(file, dest))
        text = open(file, encoding="utf-8").read()
        def fix(match):
            target = os.path.normpath(os.path.join(os.path.dirname(original), match.group(3)))
            if target.startswith(SRC + os.sep) or target == SRC:
                return match.group(0)
            return match.group(1) + match.group(2) + target + match.group(2)
        text = pattern.sub(fix, text).replace('from "ajv"', 'from "/Users/ueli/Documents/semio/node_modules/ajv"')
        open(file, "w", encoding="utf-8").write(text)
for name in ("🧪️tests/🐍️.py",):
    file = os.path.join(dest, name)
    text = open(file, encoding="utf-8").read().replace('os.path.join(HERE,*([".."]*7),', 'os.path.join("/Users/ueli/Documents/semio/🧰️framework",')
    open(file, "w", encoding="utf-8").write(text)
print("prepared", dest)
