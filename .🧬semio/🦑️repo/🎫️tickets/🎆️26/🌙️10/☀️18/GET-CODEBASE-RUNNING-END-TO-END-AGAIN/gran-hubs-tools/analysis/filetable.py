"""📋 Per-file peel table of a hub: lines, commits, distinct production users (dependents naming an item defined in the file), union cone of those users (feature-aware), potential benefit commits x (reach(hub) - cone). `python3 filetable.py <hub>`"""
from hubenv import *
from graphfeat import Resolver
hub=sys.argv[1]
R=Resolver();reach_,rv=R.all_reach()
HD=hubdir(hub) if hub!='umbrella' else byname['semio-framework']
FC=json.load(open(OUT+'/file-commits.json'))
cs=json.load(open(OUT+'/census-%s.json'%hub));R_=cs['resolved'];ID=cs['ident']
users=collections.defaultdict(set);names=collections.defaultdict(collections.Counter)
for dd,v in cs['deps'].items():
    for p in v['prod']:
        r=R_[p]
        if r['kind'] in('def','mod') and r['owner']==ID:
            users[r['file']].add(dd);names[r['file']][r['name']]+=1
fs=[os.path.relpath(f,ROOT) for f in tree(hub) if isprod(f)]
HR=len(rev_:=rv.get(HD,()))
rows=[]
for f in fs:
    com=len(FC.get(f,[]));u=users.get(f,set())
    cone=set(u)
    for x in u: cone|=rv.get(x,set())
    rows.append((com*(HR-len(cone)),com,len(u),len(cone),sum(1 for _ in open(os.path.join(ROOT,f),encoding='utf-8',errors='ignore')),f,names[f].most_common(3)))
rows.sort(reverse=True)
print(hub,'reach',HR,'total prod commits(file-sum)',sum(r[1] for r in rows))
for r in rows[:18]: print('%8d  c=%3d users=%4d cone=%4d lines=%4d  %s  %s'%(r[0],r[1],r[2],r[3],r[4],r[5][-62:],[a for a,_ in r[6]]))
