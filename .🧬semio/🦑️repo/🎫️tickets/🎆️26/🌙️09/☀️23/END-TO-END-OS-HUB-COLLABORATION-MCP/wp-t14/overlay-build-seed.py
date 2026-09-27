#!/usr/bin/env python3
"""🌱️ Seeds the overlay's PRIVATE build-dir with copy-on-write clones of registry/git dependency units only (their dep-info
points into `~/.cargo/registry`, never into a working tree). Every path-package unit is left out, so no overlay unit can
be judged Fresh against the live tree's sources (memory "Scratch Clone Poisons Shared Build-Dir", reversed direction).
usage: overlay-build-seed.py <source build-dir>/<target triple dir…>/debug <overlay build-dir>/<same suffix>"""
import ctypes
import os
import re
import shutil
import sys

libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]
lock = open("/Users/ueli/Documents/semio/Cargo.lock", encoding="utf-8").read()
registry = set()
for block in lock.split("[[package]]"):
    name = re.search(r'^name = "([^"]+)"', block, re.M)
    if name and re.search(r'^source = "(registry|git|sparse)\+', block, re.M):
        registry.add(name.group(1))
source, destination = sys.argv[1], sys.argv[2]
os.makedirs(os.path.join(destination, "build"), exist_ok=True)
kept = dropped = 0
for unit in sorted(os.listdir(os.path.join(source, "build"))):
    target = os.path.join(destination, "build", unit)
    if unit not in registry:
        dropped += 1
        continue
    if os.path.lexists(target):
        shutil.rmtree(target)
    if libc.clonefile(os.path.join(source, "build", unit).encode(), target.encode(), 1) != 0:
        raise OSError(ctypes.get_errno(), unit)
    kept += 1
print(f"registry names {len(registry)}; units kept {kept}; path/unknown units left out {dropped}")
