"""🚚 Which direct dependents carry a hub's reach: greedy cover of the transitive-dependent set by direct dependents (feature-aware closures), with each carrier's prod usage class. `python3 carriers.py <hub> [N]`."""
from hubenv import *
from graphfeat import Resolver
R=Resolver()
hub=sys.argv[1];N=int(sys.argv[2]) if len(sys.argv)>2 else 15
HD=hubdir(hub) if hub!='umbrella' else byname['semio-framework']
reach,rev_=R.all_reach()
users=rev_[HD]            # all roots whose closure contains HD
census=json.load(open(OUT+'/census-%s.json'%hub))['deps']
# direct dependents = roots whose closure contains HD AND have a direct (active) edge: approximate by census deps ∩ users
direct=[d for d in census if d in users]
# the transitive dependents of each direct dependent, restricted to users
cover={d:(rev_[d]&users)|{d} for d in direct}
left=set(users);order=[]
while left and len(order)<N:
    best=max(cover,key=lambda d:len(cover[d]&left))
    gain=len(cover[best]&left)
    if gain==0: break
    v=census[best];cl='prod' if v['prod'] else 'test' if v['test'] else 'macro' if v['macros'] else 'none'
    order.append((gain,len(cover[best]),R.C[best]['name'].replace('semio-framework-',''),cl,len(v['prod'])))
    left-=cover[best]
print(hub,'reach',len(users),'direct (active)',len(direct))
cum=0
for g,tot,n,cl,np_ in order: cum+=g;print('  +%5d (own cone %5d) cum %5d  %-45s %-6s names %d'%(g,tot,cum,n,cl,np_))
print('  uncovered after',len(order),'carriers:',len(left))
