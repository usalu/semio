import sys, collections
sys.path.insert(0, ".")
from plancheck2 import *
planmod = sys.argv[1]
plan, w, edges, flags, cr, ch = run(planmod, quiet=True, show=0)
tw, tedges, tflags = build_graph(w, test=True)
byid = {id(a): a for a in w.atoms}
names = [n for n, _ in plan.CRATES]
TESTPIN = getattr(plan, "TEST_PIN", {})
byfile = collections.defaultdict(list)
for a in w.atoms:
    if id(a) in w.mount_atoms: continue
    if flags_for(w, a): byfile[a.file.path].append(a)
prodl = collections.Counter(); prodb = collections.Counter()
for a in w.atoms:
    if id(a) in cr and cr[id(a)] is not None:
        prodl[cr[id(a)]] += a.text.count("\n")+1; prodb[cr[id(a)]] += len(a.text.encode())
testl = collections.Counter(); testb = collections.Counter(); who = collections.defaultdict(list)
for path, atoms in byfile.items():
    rel = os.path.relpath(path, MCP)
    need = set()
    for a in atoms:
        for t in tedges.get(id(a), ()):
            b = byid[t]
            if not flags_for(w, b):
                ci = cr.get(id(b))
                if ci is not None: need.add(ci)
    if rel in TESTPIN: ci = names.index(TESTPIN[rel])
    elif need: ci = max(need)
    else: continue
    f = w.files[path]
    testl[ci] += f.src.count("\n")+1; testb[ci] += len(f.src.encode()); who[ci].append((rel, f.src.count("\n")+1))
for i, n in enumerate(names):
    print(f"{i:2d} {n:16s} prod~{int(prodl[i]*1.12):5d}L {prodb[i]//1000:4d}K   tests+{testl[i]:5d}L {testb[i]//1000:4d}K  total~{int(prodl[i]*1.12)+testl[i]}")
    for r in who[i]: print("        ", r)
