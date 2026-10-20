"""🎠 Users of the `semio-framework-kernel` crate's own definitions via the umbrella census (owner = kernel): per defining file/item the user count and union cone, plus item churn. `python3 kernel_users.py`"""
from hubenv import *
from graphfeat import Resolver
R=Resolver();reach_,rv=R.all_reach()
cs=json.load(open(OUT+'/census-umbrella.json'));R_=cs['resolved']
KID='semio_framework_kernel'
byitem=collections.defaultdict(set)
for dd,v in cs['deps'].items():
    for p in v['prod']:
        r=R_[p]
        if r.get('owner')==KID and r['kind']=='def': byitem[(r['file'][-40:],r['name'])].add(dd)
KD=hubdir('kernel')
print('kernel reach',len(rv[KD]),'distinct own items used',len(byitem))
users=set()
for s in byitem.values(): users|=s
cone=set(users)
for u in users: cone|=rv.get(u,set())
print('users',len(users),'union cone',len(cone))
# cone of users sorted
us=sorted(users,key=lambda u:-len(rv.get(u,())))
for u in us[:12]: print('  ',R.C[u]['name'].replace('semio-framework-',''),len(rv.get(u,())))
rows=sorted(byitem.items(),key=lambda x:-len(x[1]))
for (f,n),s in rows[:25]:
    c=set(s)
    for u in s: c|=rv.get(u,set())
    print('%-34s users=%3d cone=%4d  %s'%(n,len(s),len(c),f))
