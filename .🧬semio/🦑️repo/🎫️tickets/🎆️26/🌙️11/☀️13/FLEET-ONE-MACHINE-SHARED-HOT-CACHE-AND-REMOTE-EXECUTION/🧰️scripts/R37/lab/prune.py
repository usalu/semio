import re,os,json,collections,pickle
from path import deps,L,R,start
tbl=json.load(open(L+'🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json'))
nk=[k for d in tbl['domains'] for k in d.get('nativeKeyed',[])]
allp=[p for d in tbl['domains'] for p in d['prefixes']]
nat=[p for p in allp if not any(k.startswith(p) or p.startswith(k) for k in nk)]
def excluded(f,pref):
    r=f[len(R):]
    return any(r.startswith(p) for p in pref)
def bfs(prune,pref):
    seen={start};q=[start]
    while q:
        f=q.pop()
        if prune and f!=start and excluded(f,pref):continue
        for d in deps(f):
            if d not in seen:seen.add(d);q.append(d)
    return seen
full=bfs(False,nat)
cur={f for f in full if not excluded(f,nat)}
pruned={f for f in bfs(True,nat) if not excluded(f,nat)}
print('full',len(full),'current key (filter after walk)',len(cur),'pruned traversal',len(pruned),'removed by pruning',len(cur-pruned))
rows,=pickle.load(open('blast.pkl','rb'))
hot={f for f,n,c in rows if f.startswith(L) and n>=4000}
gone=[(f.replace(L,'LIB/'),) for f in cur-pruned if f in hot]
tot=sum(c for f,n,c in rows if f in (cur-pruned) and n>=4000)
print('hot (blast>=4000) LIB files removed by pruning',len(gone),'touches in 14d',tot)
for f,n,c in sorted(rows,key=lambda r:-r[2]*r[1]):
    if f in (cur-pruned):print(n,c,f.replace(L,'LIB/'))
