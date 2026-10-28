#!/usr/bin/env python3
"""`overlay.py <root>`: copies every file below `work/` over the same relative path under <root> (a mirror), replacing clones by real copies so the live tree is never written through a clone."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
WORK = os.path.join(HERE, "work")
root = sys.argv[1]
count = 0
for base, _dirs, files in os.walk(WORK):
    for name in files:
        source = os.path.join(base, name)
        rel = os.path.relpath(source, WORK)
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if os.path.lexists(target):
            os.remove(target)
        shutil.copyfile(source, target)
        count += 1
print("overlay: %d file(s) under %s" % (count, root))
