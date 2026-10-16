import sys, collections
sys.path.insert(0, ".")
from graph2 import *
w = world()
# atoms with impls from other files than their own, or impls spread
rows = []
for a in w.atoms:
    if id(a) in w.mount_atoms: continue
    if not a.impls: continue
    fl = collections.Counter()
    ln = collections.Counter()
    for it in a.impls:
        fl[os.path.relpath(it.file.path, MCP)] += 1
        ln[os.path.relpath(it.file.path, MCP)] += it.text.count("\n") + 1
    own = os.path.relpath(a.file.path, MCP)
    spread = {k: v for k, v in ln.items() if k != own}
    if spread:
        rows.append((a.name, own, dict(ln)))
for r in rows: print(r)
