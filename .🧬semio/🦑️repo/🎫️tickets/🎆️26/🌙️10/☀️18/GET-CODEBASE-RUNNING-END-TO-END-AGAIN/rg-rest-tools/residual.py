"""List crate::/super::super refs in files mounted by a crate (not within own moved module path). usage: residual.py <manifest-rel>"""
import sys, os, re
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0, RT + "/rgplug-tools"); sys.path.insert(0, RT + "/rg-rest-tools")
from rgp import *
from modtree import tree
root, nodes = tree(sys.argv[1])
files = {root} | {n.file for n in nodes if n.file}
mods = {}
for n in nodes:
    if n.file: mods.setdefault(n.file, []).append("::".join(n.modpath))
for f in sorted(files):
    if not f.endswith(".rs") or not os.path.exists(f): continue
    t = open(f, encoding="utf8").read()
    refs = sorted(set(re.findall(r'\bcrate::[A-Za-z0-9_{}:, ]+', t)))
    own = mods.get(f, [""])
    bad = [r for r in refs if not any(r.startswith("crate::" + o) for o in own if o)]
    if bad: print(os.path.relpath(f, ROOT)[-70:], bad[:6])
print("files", len(files))
