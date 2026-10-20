"""🔀 Best case for the facade carves: dead (none) + test-only edges removed, prod edges rewired (opt), user-of-own-defs dependents keep the hub edge. Reports hub reach and W. `python3 rewire_best.py`."""
from rewire import *
from classify import classify
base,rv=R0.all_reach();B=W(R0,base)
byn={c['name']:d for d,c in R0.C.items()}
F={'os-kernel':byn['semio-framework-os-kernel'],'umbrella':byn['semio-framework'],'plugin':byn['semio-framework-plugin']}
def run(hubs,dead=True,rewire=True,jobdead=False,mode='opt'):
    R=mk()
    for h in hubs:
        HD=F[h] if h in F else byn['semio-framework-'+h]
        cl=classify(h)
        if dead:
            for dd,c in cl.items():
                if c in('none','test'):
                    m=R.M[dd];m['deps']=[x for x in m['deps'] if not (x['kind'] in('normal','build') and R.tgt.get((dd,x['kind'],x['key']))==HD and not x['features'])]
        if rewire and h in F: apply(R,HD,plan(h,mode=mode))
    R.cache={}
    return R,R.all_reach()[0]
for label,hubs,dead,rw in [('IDEAL',('os-kernel','umbrella','plugin'),True,'ideal'),('dead only 4 hubs',('os-kernel','umbrella','plugin','job'),True,False),('dead+rewire 3 facades',('os-kernel','umbrella','plugin'),True,True),('dead+rewire 3 facades + job dead',('os-kernel','umbrella','plugin','job'),True,True)]:
    R,reach=run(hubs,dead,bool(rw),mode=('ideal' if rw=='ideal' else 'opt'))
    print(label,'W %d -> %d (%.2f%%)'%(B,W(R,reach),(W(R,reach)-B)*100/B),' '.join('%s %d->%d'%(x,base[byn['semio-framework-'+x]],reach[byn['semio-framework-'+x]]) for x in ('plugin','os-kernel','kernel','job')),'umb',base[byn['semio-framework']],'->',reach[byn['semio-framework']])
