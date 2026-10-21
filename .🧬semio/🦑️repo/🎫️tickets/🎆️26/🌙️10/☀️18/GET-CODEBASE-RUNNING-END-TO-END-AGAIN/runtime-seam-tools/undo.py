#!/usr/bin/env python3
"""Restores the files apply.ts backed up and removes the runtime module it created. Dry run unless --write."""
import os, shutil, sys
here = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(os.path.join(here, *[".."] * 8))
backup = os.path.join(here, "..", "🗑️generated", "runtime-seam", "backup")
write = "--write" in sys.argv
restored = 0
for dp, dn, fn in os.walk(backup):
    for f in fn:
        src = os.path.join(dp, f); rel = os.path.relpath(src, backup); dst = os.path.join(root, rel)
        restored += 1
        if write: shutil.copy2(src, dst)
module = os.path.join(root, "🧰️framework", "🔨️modules", "⚙️runtime")
print({"restore": restored, "removeModule": os.path.isdir(module), "mode": "write" if write else "dry-run"})
if write and os.path.isdir(module): shutil.rmtree(module)
