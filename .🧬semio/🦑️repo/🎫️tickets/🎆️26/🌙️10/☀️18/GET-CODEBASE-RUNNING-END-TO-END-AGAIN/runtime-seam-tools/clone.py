#!/usr/bin/env python3
"""Copies the git-visible source below 🧰️framework into a scratch tree so touched suites run beside the overlay. Usage: clone.py <repo> <dest>"""
import os, shutil, subprocess, sys
repo, dest = sys.argv[1], sys.argv[2]
listing = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", "🧰️framework", "package.json", "bunfig.toml", "tsconfig.json"], cwd=repo, capture_output=True).stdout.decode("utf8").split("\0")
count = 0
for rel in filter(None, listing):
    src = os.path.join(repo, rel)
    if not os.path.isfile(src) or os.path.islink(src): continue
    out = os.path.join(dest, rel); os.makedirs(os.path.dirname(out), exist_ok=True)
    shutil.copy2(src, out); count += 1
link = os.path.join(dest, "node_modules")
if not os.path.lexists(link): os.symlink(os.path.join(repo, "node_modules"), link)
print(count, "files into", dest)
