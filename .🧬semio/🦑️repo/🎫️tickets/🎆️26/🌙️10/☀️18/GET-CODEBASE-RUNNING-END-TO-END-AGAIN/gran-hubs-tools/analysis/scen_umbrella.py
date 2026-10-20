"""🧪 Scenario T1: dissolve the umbrella `semio-framework` (pure facade): dead and test-only edges dropped, production importers repointed to the first non-facade crate on the re-export chain, the one own const relocated to {time-travel-actions, tool-run-actions, manifest-app-surface}; reach and W after. `python3 scen_umbrella.py`"""
from rewire import *
from classify import classify
base,rv0=R0.all_reach();B=W(R0,base)
byn={c['name']:d for d,c in R0.C.items()}
U=byn['semio-framework']
cs=json.load(open(OUT+'/census-umbrella.json'));R_=cs['resolved'];ID=cs['ident']
cl=classify('umbrella')
extra={byn['semio-framework-time-travel-actions'],byn['semio-framework-tool-run-actions'],byn['semio-framework-manifest-app-surface']}
R=mk()
plan_=plan('umbrella',mode='ideal')
for dd,v in cs['deps'].items():
    c=cl[dd]
    if c in('none','test'): plan_[dd]=set();continue
    if not v['prod']: continue
    if dd not in plan_: plan_[dd]=set()
    if any(R_[p]['kind']=='def' and R_[p]['owner']==ID and R_[p]['name']=='FRAMEWORK_RESERVED_CONTRIBUTIONS' for p in v['prod']): plan_[dd]|=extra
apply(R,U,plan_)
reach,_=R.all_reach()
print('umbrella reach',base[U],'->',reach.get(U,0),' W',B,'->',W(R,reach),'(%.2f%%)'%((W(R,reach)-B)*100/B))
changed=[(base[d]-reach.get(d,0),R.C[d]['name'].replace('semio-framework-','')) for d in base if base[d]!=reach.get(d,0)]
changed.sort(reverse=True)
print('crates with reduced reach',len([1 for a,_ in changed if a>0]),'increased',len([1 for a,_ in changed if a<0]))
CHn=json.load(open(OUT+'/churn-all.json'))['prod']
print('top reductions by churn*delta:')
rows=sorted(((CHn.get(R.C[d]['name'],0)*(base[d]-reach.get(d,0)),R.C[d]['name'].replace('semio-framework-',''),base[d],reach.get(d,0),CHn.get(R.C[d]['name'],0)) for d in base if base[d]!=reach.get(d,0)),reverse=True)
for r in rows[:25]: print('  %7d %-40s %5d -> %5d churn %d'%r)
