"""🔍 Residual direct dependents of a hub after replaying plans: names, cone and the blocker reason from the planner stats. `python3 residual.py <hub> <tag,tag>`"""
import sys
sys.argv=[sys.argv[0],sys.argv[2]]+sys.argv[1:2]
from rewire import *
hub=sys.argv[2];tags=sys.argv[1].split(',')
byn={c['name']:d for d,c in R0.C.items()}
HD=byn['semio-framework'] if hub=='umbrella' else byn['semio-framework-'+hub]
planned=set()
for t in tags:
    p=json.load(open(os.path.normpath(T+'/../staged/plans/%s.json'%t)))
    planned|=set(m['crate'] for m in p['manifests'])
stats=json.load(open(OUT+'/plan-%s.stats.json'%hub))['reasons']
base,rv=R0.all_reach()
rem=[d for d in rev[HD] if R0.C[d]['name'] not in planned]
rem.sort(key=lambda d:-len(rv.get(d,())))
print(hub,'direct dependents',len(rev[HD]),'not in plans',len(rem))
for d in rem[:25]:
    n=R0.C[d]['name'];print('  %-48s cone %5d  %s'%(n[:48],len(rv.get(d,()))+1,(stats.get(n) or ['(not census-reachable/dev only)'])[0][-110:]))
