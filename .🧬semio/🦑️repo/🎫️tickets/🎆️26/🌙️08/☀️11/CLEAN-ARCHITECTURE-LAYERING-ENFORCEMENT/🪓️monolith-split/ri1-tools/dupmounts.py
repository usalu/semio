#!/usr/bin/env python3
# 🧬 Files mounted by more than one compilation unit (crate roots and the monolith root): the double mounts the flip must not leave.
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
table = crate_table()
owners = collections.defaultdict(list)
for suffix, folder in table.items():
    root = T + "/" + folder + "/🦀️.rs"
    for f in modtree.tree(root):
        if f == root:
            continue
        owners[f].append(suffix)
lib = LIB_REL
for f in modtree.tree(R + "/🦀️.rs"):
    if f == R + "/🦀️.rs":
        continue
    owners[f].append("<monolith>")
rows = [(f, o) for f, o in owners.items() if len(o) > 1]
print("files with several owners:", len(rows))
for f, o in sorted(rows):
    print("  ", f.replace(E + "/", "E/").replace("🎯️targets/🧊️wgpu/", "T/").replace("🧱️elements/", "el/"), o)
