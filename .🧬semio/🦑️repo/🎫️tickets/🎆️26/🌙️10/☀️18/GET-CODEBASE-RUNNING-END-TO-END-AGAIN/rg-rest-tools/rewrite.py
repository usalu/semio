"""Apply spec rewrites to every .rs file mounted by the NEW crate of a carve spec, then re-prune its manifest. usage: rewrite.py <spec.json> [extra regex rewrites json]"""
import sys, os, re, json
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0, RT + "/rgplug-tools"); sys.path.insert(0, RT + "/rg-rest-tools")
import rgp
from rgp import *
rgp.BK = RT + "/🗑️generated/rg-rest/backup"
from modtree import tree
spec = json.load(open(sys.argv[1]))
rws = spec.get("rewrites", [])
npkg = spec["new"]["dir"] + "/📦️packages/🦀️rust"
root, nodes = tree(npkg + "/Cargo.toml")
files = {root} | {n.file for n in nodes if n.file}
n = 0
for f in sorted(files):
    if not f.endswith(".rs") or not os.path.exists(f):
        continue
    t0 = open(f, encoding="utf8").read(); t1 = t0
    for rw in rws:
        t1 = re.sub(rw["re"], rw["to"], t1)
    if t1 != t0:
        wr(os.path.relpath(f, ROOT), t1); n += 1
print("rewrote", n, "of", len(files))
