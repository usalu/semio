#!/usr/bin/env python3
# 🧱️ Facade coverage: names a flipped crate defines but does not export (private or in a module without a glob re-export) that the remaining monolith files of the Shell mention.
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import flip as F
from model import *
import items as itm
st = F.load_state()
defined, exported = collections.defaultdict(set), set()
for c in st["flipped"]:
    ex = F.exports_of(c)
    exported |= set(ex)
    for m in st["modules"][c]:
        for it in itm.table(read(m["file"])):
            if it["kind"] in ("impl", "use"):
                continue
            defined[it["name"]].add((c, os.path.basename(os.path.dirname(m["file"])), it["vis"] or "private"))
flipped_files = {m["file"] for c in st["flipped"] for m in st["modules"][c]}
remaining = []
for m in original_mounts():
    if m["kind"] in ("staging", "chunk") and m["file"] not in flipped_files:
        remaining.append(m["file"])
tests = []
for root in (SH + "/🧪️tests", E + "/🧪️tests", E + "/🧱️elements/🛰️Dock/🧪️tests", E + "/🧱️elements/🖼️IconRenderHost/🧪️tests"):
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d != "node_modules"]
        tests += [os.path.join(dp, f) for f in fn if f.endswith(".rs")]
local = set()
for f in remaining:
    for it in itm.table(read(f)):
        if it["kind"] not in ("impl", "use"):
            local.add(it["name"])
hits = collections.defaultdict(lambda: collections.defaultdict(set))
for group, files in (("remaining", remaining), ("tests", tests)):
    for f in files:
        toks = tokens(read(f))
        for t in toks:
            if t in defined and t not in exported and t not in local:
                hits[t][group].add(os.path.basename(os.path.dirname(f)))
print("unexported-but-used names:", len(hits))
for t, g in sorted(hits.items()):
    print(" ", t, sorted(defined[t]), {k: sorted(v)[:3] for k, v in g.items()})
