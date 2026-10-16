import sys, collections, importlib
sys.path.insert(0, ".")
from plancheck2 import *
planmod = sys.argv[1]
plan, w, edges, flags, cr, ch = run(planmod, quiet=True, show=0)
tw, tedges, tflags = build_graph(w, test=True)
byid = {id(a): a for a in w.atoms}
names = [n for n, _ in plan.CRATES]
# which crate does a prod atom belong to
def crate_idx(a):
    return cr.get(id(a))
# group test atoms by file
byfile = collections.defaultdict(list)
for a in w.atoms:
    if id(a) in w.mount_atoms: continue
    if flags_for(w, a): byfile[a.file.path].append(a)
rows = []
for path, atoms in byfile.items():
    f = w.files[path]
    need = set(); foreign_tests = set()
    for a in atoms:
        for t in tedges.get(id(a), ()):
            b = byid[t]
            if flags_for(w, b):
                if b.file.path != path: foreign_tests.add(os.path.relpath(b.file.path, MCP))
            else:
                ci = cr.get(id(b))
                if ci is not None: need.add(ci)
    mx = max(need) if need else -1
    # home: the facet's chunk assigned crate for its parent file's top-level
    par = f.parent
    home = None
    if par is not None:
        pa = [x for x in w.atoms if x.file is par and id(x) not in w.mount_atoms and not flags_for(w, x)]
        cs = collections.Counter(cr.get(id(x)) for x in pa)
        home = cs.most_common(1)[0][0] if cs else None
    lines = sum(a.text.count("\n")+1 for a in atoms)
    rows.append((os.path.relpath(path, MCP), names[home] if home is not None else None, names[mx] if mx >= 0 else None, sorted(names[i] for i in need), sorted(foreign_tests), lines))
for r in sorted(rows):
    flag = "" if r[1] == r[2] else "  <-- MISMATCH"
    print(f"{r[0]}  home={r[1]} need={r[2]} lines={r[5]}{flag}")
    if r[1] != r[2]: print("      needs:", r[3], "foreign tests:", r[4])
