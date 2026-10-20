"""🔍 After the best-case facade rewire: who still depends directly on a facade, by census class and cone. `python3 diag_residual.py <hub>`"""
import sys
hub=sys.argv[1]
src=open('rewire_best.py').read().split("for label,hubs")[0]
exec(src)
from classify import classify
R,reach=run(('os-kernel','umbrella','plugin'),True,True)
HD=F[hub] if hub in F else byn['semio-framework-'+hub]
rem=[d for d in R.M if any(R.tgt.get((d,x['kind'],x['key']))==HD and x['kind'] in('normal','build') for x in R.M[d]['deps'])]
_,rv=R.all_reach()
cl=classify(hub);cs=json.load(open(OUT+'/census-%s.json'%hub))['deps']
print(hub,'remaining direct dependents',len(rem),'reach',len(rv.get(HD,())))
print(collections.Counter(cl.get(d,'new-edge') for d in rem))
for d in sorted(rem,key=lambda d:-len(rv.get(d,())))[:12]:
    v=cs.get(d,{});print('  %-52s cone %5d class %-12s prod names %s'%(R.C[d]['name'][:52],len(rv.get(d,())),cl.get(d,'new-edge'),list(v.get('prod',{}))[:3]))
