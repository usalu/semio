"""🧪 Facade scenarios with macro pins: dead/test-only edges dropped, importers repointed (own-definition users, i.e. macro users, keep the hub edge), each hub pruned to the crates its remaining own content needs. Prints hub reach and W per scenario. `python3 scen_all.py`"""
from rewire2 import *
from macro_needs import macro_needs
from classify import classify
base,rv0=R0.all_reach();B=W(R0,base)
byn={c['name']:d for d,c in R0.C.items()}
F={'os-kernel':byn['semio-framework-os-kernel'],'umbrella':byn['semio-framework'],'plugin':byn['semio-framework-plugin']}
def scenario(hubs,mode='ideal',prune=True):
    R=mk()
    for h in hubs:
        cl=classify(h);HD=F[h]
        dead=[d for d,c in cl.items() if c in('none','test')]
        pl=plan2(h,mode)
        keep=None
        if prune:
            mc,_=macro_needs(h) if h!='umbrella' else (set(),{})
            own={R0.C[x]['name'] for x in ()}
            keep=set(mc)|({byn['semio-framework-time-travel-actions'],byn['semio-framework-tool-run-actions'],byn['semio-framework-manifest-app-surface']} if h=='umbrella' else set())
        apply2(R,HD,pl,dead=dead,keep=keep)
    reach,_=R.all_reach()
    return R,reach
def show(label,R,reach):
    print('%-34s W %d -> %d (%.2f%%)  '%(label,B,W(R,reach),(W(R,reach)-B)*100/B),' '.join('%s %d->%d'%(x.replace('semio-framework-',''),base[byn[x]],reach[byn[x]]) for x in ('semio-framework-plugin','semio-framework-os-kernel','semio-framework-kernel','semio-framework-job','semio-framework')))
if __name__=='__main__':
    for hubs in (('umbrella',),('plugin',),('os-kernel',),('umbrella','plugin','os-kernel')):
        for mode in ('full','ideal'):
            R,reach=scenario(hubs,mode)
            show('%s/%s'%('+'.join(hubs),mode),R,reach)
