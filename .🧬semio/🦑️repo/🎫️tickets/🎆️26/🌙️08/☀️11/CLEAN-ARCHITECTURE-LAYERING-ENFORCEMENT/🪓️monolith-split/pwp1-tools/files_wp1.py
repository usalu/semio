#!/usr/bin/env python3
# 📃️ Per-unit file lists (production vs test) of the WP1 trees, with line counts.
import os, sys
from lib import *
import modmap, wp1files, refs
from crates_wp1 import OWNER, NESTED

def unit_files(unit):
    mounts = modmap.snapshot_mounts()
    if unit in NESTED:
        f = PLUG + "/" + NESTED[unit][2]
    else:
        f = mounts[unit][0]
    return refs.module_paths(unit, f)

if __name__ == "__main__":
    for u in sorted(list(OWNER) + list(NESTED)):
        paths = unit_files(u)
        tot = 0
        print("==", u, OWNER.get(u) or NESTED[u][0])
        for p, mp in paths.items():
            n = read(p).count("\n")
            tot += n
            print(f"   {n:5d} {'::'.join(mp):40s} {p.replace(PLUG + '/', '')}")
