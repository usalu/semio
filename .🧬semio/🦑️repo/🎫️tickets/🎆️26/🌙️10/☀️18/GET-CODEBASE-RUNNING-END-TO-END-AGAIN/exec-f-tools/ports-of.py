"""python3 ports-of.py <out ports.json> <snapshot.json>... : ports (scheduling files imported from keyed files) per domain"""
import sys,json,collections,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from domains import domain
ports=collections.defaultdict(set)
for snap in sys.argv[2:]:
    d=json.load(open(snap))
    for f in d['files']:
        if domain(f['path']) is not None: continue
        for spec,t in f['imports'].items():
            if not t.startswith('installed:') and domain(t) is not None: ports[domain(t)].add(t)
json.dump({k:sorted(v) for k,v in ports.items()},open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)
