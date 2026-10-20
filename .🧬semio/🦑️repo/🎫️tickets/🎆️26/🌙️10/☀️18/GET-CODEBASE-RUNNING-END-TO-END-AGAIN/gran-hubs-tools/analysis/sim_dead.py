"""🧪 What-if: remove hub edges classified none (+ test-only demoted to dev) per hub, feature-aware reach before/after, churn-weighted over the 366 hubs. `python3 sim_dead.py`."""
from hubenv import *
import copy
from graphfeat import Resolver
R0=Resolver()
ch=json.load(open(GEN+'/hubchurn.json'))
base_reach,_=R0.all_reach()
nm2d={c['name']:d for d,c in R0.C.items()}
def weighted(reach): return sum(ch[n]*reach.get(nm2d[n],0) for n in ch)
B=weighted(base_reach)
print('baseline churn*reach (feature-aware)',B)
def drops(h,classes):
    d=json.load(open(OUT+'/census-%s.json'%h));HD=d['dir'];out=set()
    for dd,v in d['deps'].items():
        cl='prod' if v['prod'] else 'test' if v['test'] else 'macro' if v['macros'] else 'none'
        if cl in classes: out.add((dd,HD))
    return out
def run(drop):
    M=copy.deepcopy(R0.M) if False else R0.M
    R=Resolver(M={d:dict(m) for d,m in R0.M.items()},C=R0.C)
    for d,HD in drop:
        m=R.M[d];keep=[]
        for x in m['deps']:
            t=R.tgt.get((d,x['kind'],x['key']))
            if t==HD and x['kind'] in('normal','build') and not x['features']: continue
            keep.append(x)
        m['deps']=keep
    return R.all_reach()[0]
rows=[]
for h,key in [('plugin','plugin'),('os-kernel','os-kernel'),('job','job'),('umbrella','semio-framework')]:
    for cl in (('none',),('none','test')):
        r=run(drops(h,cl));n='semio-framework-'+key if key!='semio-framework' else 'semio-framework'
        print(h,cl,'edges',len(drops(h,cl)),'reach',base_reach[nm2d[n]],'->',r[nm2d[n]],'weighted',B,'->',weighted(r))
allnone=set()
for h in ['plugin','os-kernel','job','umbrella']: allnone|=drops(h,('none','test'))
r=run(allnone);print('ALL four hubs none+test',len(allnone),'weighted',weighted(r))
for key in ['plugin','os-kernel','kernel','job']:
    n='semio-framework-'+key;print(' ',key,base_reach[nm2d[n]],'->',r[nm2d[n]])
print('  umbrella',base_reach[nm2d['semio-framework']],'->',r[nm2d['semio-framework']])
