#!/usr/bin/env python3
"""🧭️ Run-time `Path::new(env!("CARGO_MANIFEST_DIR")).join("../..…")` reads of sources a companion test bin owns: the bin builds two directories deeper than the facade, so its plugin-relative prefix gains two more `..`.
usage: fixmanifest.py [--apply]"""
import glob, os, re, sys
from lib import *
import tree as T
import ed

RX = re.compile(r'(Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\()"(\.\./\.\.)')
seen, edits = set(), {}
for d in sorted(glob.glob(PLUG + "/🧷️companions/*tests-*")):
    root = d + "/🦀️.rs"
    if not os.path.exists(root):
        continue
    for f, _, _ in T.tree(root):
        if f in seen:
            continue
        seen.add(f)
        t = read(f)
        n = RX.sub(r'\1"../../../..', t)
        if n != t:
            edits[f] = n
for f, n in edits.items():
    print(rel(f)[-70:], len(RX.findall(read(f))))
if "--apply" in sys.argv:
    for f, n in edits.items():
        ed.put(f, n)
    print("applied", len(edits))
