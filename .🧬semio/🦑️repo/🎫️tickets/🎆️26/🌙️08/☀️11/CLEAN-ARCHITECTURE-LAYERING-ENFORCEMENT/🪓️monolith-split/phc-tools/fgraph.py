import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, cfgeval
from itemgraph import build
REPO="/Users/ueli/Documents/semio/"
H=REPO+"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
ROOT=H+"📦️packages/🦀️rust/🦀️.rs"
model,nodes,edges=build(ROOT)
def testfile(fm):
    x=fm
    while x is not None:
        for c in x.cfg:
            if not cfgeval.holds(c,test=False): return True
        x=x.parent
    return False
byfile={}
for fm in model.by_file.values():
    byfile[fm.file]=fm
def short(p): return p.replace(H,'')
def fileof(k): return k[0]
agg=collections.defaultdict(lambda: collections.defaultdict(set))
for a,bs in edges.items():
    for b in bs:
        if fileof(a)!=fileof(b):
            agg[fileof(a)][fileof(b)].add(b[2])
mode=sys.argv[1] if len(sys.argv)>1 else 'lib'
for f in sorted(byfile, key=lambda f: short(f)):
    fm=byfile[f]
    t=testfile(fm)
    if mode=='lib' and t: continue
    print(('T ' if t else 'L ')+short(f), os.path.getsize(f))
    for g,names in sorted(agg[f].items(), key=lambda x: short(x[0])):
        gt=testfile(byfile[g]) if g in byfile else None
        if mode=='lib' and gt: continue
        print('     ->', short(g), len(names), ','.join(sorted(names))[:150])
