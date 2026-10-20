"""📈 Commit counts per value-family crate from the git history of the files its module tree mounts. Run with `cwd` = the generated folder."""
from graph import *
from tree import *
import subprocess,collections,os
fmap=collections.defaultdict(set)
for d in fam:
    for f in module_tree(libfile(d,C[d])): fmap[f].add(d)
out=subprocess.run(['git','log','--name-only','--format=@@%h','--','🧰️framework/🔨️modules/🌱️value'],cwd=ROOT,capture_output=True,text=True).stdout
commits=collections.defaultdict(set); cur=None
for l in out.splitlines():
    if l.startswith('@@'): cur=l[2:]
    elif l.strip():
        p=os.path.normpath(os.path.join(ROOT,l.strip()))
        for d in fmap.get(p,()): commits[d].add(cur)
import time
rows=[]
for d in fam:
    files=[f for f,ds in fmap.items() if d in ds]
    mt=max((os.path.getmtime(f) for f in files if os.path.exists(f)),default=0)
    rows.append((len(commits[d]),time.strftime('%m-%d %H:%M',time.localtime(mt)),len(trans(d)),C[d]['name'].replace('semio-framework-','')))
rows.sort(reverse=True)
for r in rows: print(r)
json.dump({C[d]['name']:len(commits[d]) for d in fam},open('churn.json','w'))
