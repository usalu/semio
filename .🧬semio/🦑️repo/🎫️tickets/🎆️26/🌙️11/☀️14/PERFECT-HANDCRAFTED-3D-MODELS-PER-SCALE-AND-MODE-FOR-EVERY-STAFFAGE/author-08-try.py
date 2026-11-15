import sys, json
from author_08_lib import *
slug="person-with-white-cane"; cat="humans"
scale="50"
P=Prof(slug,scale)
a=load_asset(cat,slug)
sol=[]
for b in a['rig']['bones']:
    rows=bone_profile(P,b['id'])
    if not rows: print('no',b['id']); continue
    st=simplify(rows,FEATURE[scale]*0.5)
    print(b['id'],len(rows),len(st))
    sol.append(loft(b['id'],b['id'],'skin',0.5,'body',st))
d=load_src(cat,slug)
d['volumes'][scale]['body']=sol
json.dump(d,open('🗑️generated/author-08/try.json','w'))
