import sys, collections
sys.path.insert(0, ".")
from graph2 import *
w, edges, flags = build_graph()
byid = {id(a): a for a in w.atoms}
sub = sys.argv[1]
rows = []
for a in w.atoms:
    if id(a) in w.mount_atoms: continue
    rel = os.path.relpath(a.file.path, MCP)
    if sub in rel and not flags[id(a)]:
        rows.append(a)
rows.sort(key=lambda a: a.item.line0)
for a in rows:
    print(f"L{a.item.line0:5d} {a.text.count(chr(10))+1:5d} {a.kind:6s} {a.item.vis:10s} {a.name}  impls={len(a.impls)} refs->{sorted(set(os.path.basename(os.path.dirname(byid[t].file.path)) for t in edges.get(id(a),()) if byid[t].file is not a.file))[:6]}")
