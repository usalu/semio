"""🔀 Rewire several facades at once and report W and the five hubs' reach. `python3 rewire_all.py`."""
from rewire import *
base,_=R0.all_reach();B=W(R0,base)
byn={c['name']:d for d,c in R0.C.items()}
F={'os-kernel':byn['semio-framework-os-kernel'],'umbrella':byn['semio-framework'],'plugin':byn['semio-framework-plugin']}
keyn=['semio-framework-plugin','semio-framework-os-kernel','semio-framework-kernel','semio-framework-job','semio-framework','semio-framework-os-store-engine','semio-framework-os-spr','semio-framework-plugin-reactor-turn','semio-framework-plugin-abi']
for mode in ('full','opt'):
    for combo in (('os-kernel','umbrella','plugin'),):
        R=mk()
        for h in combo: apply(R,F[h],plan(h,mode=mode))
        reach,_=R.all_reach()
        print(mode,combo,'W',B,'->',W(R,reach),'(%.1f%%)'%((W(R,reach)-B)*100/B),' '.join('%s %d->%d'%(n.replace('semio-framework-',''),base[byn[n]],reach.get(byn[n],0)) for n in keyn))
        print('  mean reach',sum(reach.values())/len(R.M),'vs',sum(base.values())/len(R.M))
