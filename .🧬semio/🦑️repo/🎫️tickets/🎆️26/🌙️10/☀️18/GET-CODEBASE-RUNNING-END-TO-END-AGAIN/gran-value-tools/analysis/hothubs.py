"""🔥 Churn times reach over all crates with reach of at least 1,500. Run with `cwd` = the generated folder."""
from sim import *
from tree import *
import subprocess,collections,os,json
reach=json.load(open('reach.json'))
hubs=[d for d in C if reach[C[d]['name']]>=1500]
print('hubs>=1500',len(hubs))
fmap=collections.defaultdict(set)
for d in hubs:
    try:
        for f in module_tree(libfile(d,C[d])): fmap[f].add(d)
    except Exception as e: pass
out=subprocess.run(['git','log','--name-only','--format=@@%h'],cwd=ROOT,capture_output=True,text=True).stdout
print('log lines',len(out.splitlines()))
commits=collections.defaultdict(set);cur=None
for l in out.splitlines():
    if l.startswith('@@'): cur=l[2:];continue
    if not l.strip(): continue
    p=os.path.normpath(os.path.join(ROOT,l.strip()))
    for d in fmap.get(p,()): commits[d].add(cur)
    # manifest
rows=sorted(((len(commits[d])*reach[C[d]['name']],len(commits[d]),reach[C[d]['name']],'FAM' if d in set(fam) else '',C[d]['name'].replace('semio-framework-','')) for d in hubs),reverse=True)
for r in rows[:30]: print(r)
tot=sum(r[0] for r in rows);fam_tot=sum(r[0] for r in rows if r[3]=='FAM')
print('hub churn*reach total',tot,'value family share',round(fam_tot/tot*100,1),'%')
json.dump({C[d]['name']:len(commits[d]) for d in hubs},open('hubchurn.json','w'))
