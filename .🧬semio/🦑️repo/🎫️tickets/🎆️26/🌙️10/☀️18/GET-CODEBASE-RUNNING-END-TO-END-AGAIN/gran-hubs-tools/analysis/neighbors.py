"""🧲 Direct dependents (name, own reach, kind) and the direct-dependent count by workspace prefix for a hub. `python3 neighbors.py <hub> [N]`."""
from hubenv import *
h=sys.argv[1];n=int(sys.argv[2]) if len(sys.argv)>2 else 25
d=hubdir(h)
deps=sorted(rev[d],key=lambda x:-reach.get(C[x]['name'],0))
print(h,'direct dependents',len(deps))
for x in deps[:n]: print(' ',reach.get(C[x]['name']),C[x]['name'],x[:90])
pre=collections.Counter(x.split('/')[0]+'/'+(x.split('/')[1] if x.startswith('🧰') else '') for x in deps)
print(pre.most_common(8))
