#!/usr/bin/env python3
"""H99: makes one file of the repo editable inside a scratch overlay without touching the repo: every directory on its path becomes a real directory of symlinks to its siblings, the file a real copy.
Usage: overlay-file.py <overlay root> <file path relative to the repo root>; prints the copied file."""
import os, shutil, sys
ov, rel = sys.argv[1], sys.argv[2]; R = "/Users/ueli/Documents/semio"
parts = rel.split("/"); cur, dst = R, ov
for i, part in enumerate(parts[:-1]):
    if os.path.islink(dst):
        target = os.path.realpath(dst); os.unlink(dst); os.makedirs(dst)
        for n in os.listdir(target): os.symlink(os.path.join(target, n), os.path.join(dst, n))
    os.makedirs(dst, exist_ok=True)
    nxt = os.path.join(dst, part)
    if os.path.islink(nxt):
        target = os.path.realpath(nxt); os.unlink(nxt); os.makedirs(nxt)
        for n in os.listdir(target): os.symlink(os.path.join(target, n), os.path.join(nxt, n))
    dst = nxt
f = os.path.join(dst, parts[-1])
if os.path.islink(f): os.unlink(f)
shutil.copy2(os.path.join(R, rel), f)
print(f)
