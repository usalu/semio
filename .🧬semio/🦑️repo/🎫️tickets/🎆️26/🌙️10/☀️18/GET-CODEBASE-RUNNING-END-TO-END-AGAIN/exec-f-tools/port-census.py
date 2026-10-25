"""python3 port-census.py <snapshot.json>... : keyed->scheduling import edges of captured snapshots (their own import maps)"""
import sys,json,collections,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from domains import domain
allports=collections.defaultdict(set)
for snap in sys.argv[1:]:
    d=json.load(open(snap))
    n=len(d['files']);o=sum(domain(f['path']) is not None for f in d['files'])
    print(os.path.basename(snap),"files",n,"scheduling",o,"keyed",n-o)
    for f in d['files']:
        a=f['path']
        if domain(a) is not None: continue
        for spec,t in f['imports'].items():
            if t.startswith('installed:'):continue
            if domain(t) is not None: allports[t].add(a)
print(len(allports),"scheduling files imported from keyed files")
for t,imps in sorted(allports.items()): print(f" {domain(t):20s} {t.replace('🧰️framework/','')[-120:]}  <- {len(imps)}")
