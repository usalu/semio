"""🔬 Profile of the five hub crates: dir, lib path, module-tree size, direct dependents, own dependencies, churn. Run: `nice -n 10 python3 profile.py`; reuses gran-value graph (GRAN_GEN)."""
import os,sys,json
T=os.path.dirname(os.path.abspath(__file__))
os.environ.setdefault("GRAN_GEN",os.path.normpath(T+"/../../🗑️generated/gran-value"))
sys.path.insert(0,os.path.normpath(T+"/../../gran-value-tools/analysis"))
from graph import *
from tree import *
reach=json.load(open(GRAN+"/reach.json")) if False else json.load(open(os.environ["GRAN_GEN"]+"/reach.json"))
churn=json.load(open(os.environ["GRAN_GEN"]+"/hubchurn.json"))
HUBS=['plugin','os-kernel','kernel','job','os-store-sync']
for h in HUBS:
    n='semio-framework-'+h
    d=byname.get(n)
    if not d: print(h,'MISSING');continue
    c=C[d]
    fs=module_tree(libfile(d,c))
    lines=sum(open(f,encoding='utf-8',errors='ignore').read().count('\n') for f in fs)
    print('==',n,'\n dir',d,'\n lib',c['libpath'],'\n files',len(fs),'lines',lines,'\n reach',reach[n],'direct',len(rev[d]),'churn',churn.get(n),'\n deps',sorted(C[t]['name'].replace('semio-framework-','') for k in('normal','build') for _,t in c['deps'][k] if t in C))
