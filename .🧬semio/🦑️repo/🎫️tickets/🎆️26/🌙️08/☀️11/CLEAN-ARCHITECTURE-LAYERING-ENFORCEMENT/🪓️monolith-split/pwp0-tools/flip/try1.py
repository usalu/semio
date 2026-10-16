import sys, os, difflib, collections
from common import *
from resolve import World
from layout import Layout
from engine2 import Engine
import model
REAL = ['abi','actor-host','identity','interaction','operation','window-config','window-transient','window-kits','window-kits-rich','decl-views','decl-emission','decl-app','app-builder','decl-surface','decl-registry','plugin-program']
keys = sys.argv[1:]
w = World()
mm = w.MM
w.build_parents([m["file"] for m in mm.values() if m["kind"] == "file"])
# parents for all files
parents = collections.defaultdict(list)
for f in w.files:
    for name, child, m in w.T.children(f):
        parents[child].append((f, name))
for f, ps in parents.items():
    w.parent.setdefault(f, ps[0][0])
L = Layout(w, set(REAL) | set(keys))
E = Engine(w, L, set(REAL) | set(keys), keys)
for k in keys:
    for f in sorted(L.by[k]):
        if "/🧪️tests/" in f: continue
        t = read(PLUG + "/" + f)
        n = E.rewrite(t, f, k)
        if n != t:
            print("=====", k, f)
            for l in difflib.unified_diff(t.split("\n"), n.split("\n"), lineterm="", n=0):
                if l.startswith(("+", "-")) and not l.startswith(("+++", "---")): print(l[:200])
print("UNRESOLVED", len(E.unresolved))
for u in E.unresolved[:60]: print(u)
