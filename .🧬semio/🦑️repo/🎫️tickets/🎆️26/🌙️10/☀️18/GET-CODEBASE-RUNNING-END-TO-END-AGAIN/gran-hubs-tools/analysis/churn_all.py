"""📈 Commit count per crate for every crate in the manifest graph: module-tree files (prod only: files under 🧪️ excluded because cfg(test) modules are not part of a dependent's build) mapped to commits from one `git log --name-only` (read-only). Writes 🗑️generated/gran-hubs/churn-all.json {name: commits} and file-commit map file-commits.json."""
from hubenv import *
import time
t0=time.time()
out=subprocess.run(['git','log','--name-only','--format=@@%h'],cwd=ROOT,capture_output=True,text=True).stdout
fc=collections.defaultdict(set);cur=None
for l in out.splitlines():
    if l.startswith('@@'): cur=l[2:]
    elif l.strip(): fc[l.strip()].add(cur)
print('git log lines',len(out.splitlines()),'files',len(fc),round(time.time()-t0))
res={};resall={}
for d,c in C.items():
    try: fs=prod_tree(libfile(d,c))
    except Exception: fs=[]
    com=set();coma=set()
    for f in fs:
        r=os.path.relpath(f,ROOT);s=fc.get(r,set())
        coma|=s
        if isprod(f): com|=s
    mf=os.path.relpath(os.path.join(ROOT,d,'Cargo.toml'),ROOT)
    res[c['name']]=len(com);resall[c['name']]=len(com|fc.get(mf,set()))
json.dump({'prod':res,'prod_manifest':resall},open(OUT+'/churn-all.json','w'))
json.dump({k:sorted(v) for k,v in fc.items() if any(k.endswith(e) for e in ('.rs',))},open(OUT+'/file-commits.json','w'),ensure_ascii=False)
print('done',round(time.time()-t0))
tot=sum(res.values());print('crates with churn',sum(1 for v in res.values() if v),'sum',tot)
