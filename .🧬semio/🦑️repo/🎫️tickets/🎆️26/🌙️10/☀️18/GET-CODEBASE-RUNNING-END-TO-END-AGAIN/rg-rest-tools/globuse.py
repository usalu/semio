"""For child files with `use super::*`, list which parent-namespace names they use, tagged by origin (parent def, parent import, sibling child). usage: globuse.py <parent.rs> <child.rs>..."""
import sys, re, os
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0, RT + "/rgplug-tools"); sys.path.insert(0, RT + "/rg-rest-tools")
from rgp import *
from nsmap import use_statements, free_idents
from modtree import walk_src
parent_path = sys.argv[1] if sys.argv[1].startswith("/") else ROOT + "/" + sys.argv[1]
parent = open(parent_path, encoding="utf8").read()
defs = defined_names(parent)
imported = {}
for u in use_statements(parent):
    for n in use_names(u):
        imported[n] = u.strip().split("\n")[0][:110]
nodes = []
walk_src(parent, parent_path, os.path.dirname(parent_path), (), nodes, ())
sib = {}
for n in nodes:
    if n.file and len(n.modpath) == 1 and os.path.exists(n.file):
        t = open(n.file, encoding="utf8").read()
        for name, k in defined_names(t).items():
            if k != "mod":
                sib.setdefault(name, set()).add(n.modpath[0])
        for u in use_statements(t):
            if u.lstrip().startswith("pub"):
                for nm in use_names(u):
                    sib.setdefault(nm, set()).add(n.modpath[0])
for cf in sys.argv[2:]:
    cp = cf if cf.startswith("/") else ROOT + "/" + cf
    ct = open(cp, encoding="utf8").read()
    ids = free_idents(ct)
    own = defined_names(ct)
    cuse = set()
    for u in use_statements(ct):
        cuse.update(use_names(u))
    skip = lambda n: n in own or n in cuse
    d = sorted(n for n in ids if n in defs and not skip(n))
    s = sorted((n, sorted(sib[n])) for n in ids if n in sib and n not in defs and not skip(n) and os.path.basename(os.path.dirname(cp)) not in [x for x in sib[n]])
    i = sorted(n for n in ids if n in imported and n not in defs and not skip(n))
    print("==", os.path.relpath(cp, ROOT)[-60:])
    print(" parent defs:", d)
    print(" sibling:", s)
    print(" parent imports:", i)
