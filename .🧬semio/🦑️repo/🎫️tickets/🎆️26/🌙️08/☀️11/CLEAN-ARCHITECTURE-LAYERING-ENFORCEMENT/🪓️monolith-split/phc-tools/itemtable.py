import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, cfgeval
from itemgraph import build
REPO="/Users/ueli/Documents/semio/"
H=REPO+"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
ROOT=H+"📦️packages/🦀️rust/🦀️.rs"
model,nodes,edges=build(ROOT)
def short(p): return p.replace(H,'')
def lab(p):
    parts=short(p).split('/')
    return parts[-2] if len(parts)>1 else 'ROOT'
want=sys.argv[1]
def itlines(nd):
    n=0
    for it in nd['items']:
        n+=it.toks_all[-1].el-it.toks_all[0].sl+1
    return n
rev=collections.defaultdict(set)
for a,bs in edges.items():
    for b in bs: rev[b].add(a)
rows=[]
for k,nd in nodes.items():
    if want not in k[0]: continue
    cfg=[c for it in nd['items'] for c in it.cfg]
    first=min(it.toks_all[0].sl for it in nd['items'])
    rows.append((first,k,nd,cfg))
rows.sort(key=lambda r:r[0])
for first,k,nd,cfg in rows:
    outs=sorted({(lab(b[0]) if b[0]!=k[0] else '')+':'+b[2] for b in edges.get(k,()) if True})
    ins=sorted({lab(a[0])+':'+a[2] for a in rev.get(k,()) if a[0]!=k[0]})
    print(f"{first+1:5d} {itlines(nd):5d} {nd['kind']:7s} {'::'.join(k[1]+(k[2],)):40s} cfg={cfg if cfg else ''}")
    if '-v' in sys.argv:
        print('        out:',', '.join(outs)[:300]); print('        in :',', '.join(ins)[:300])
