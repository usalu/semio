"""🔀 Rewire only heavy dependents (cone >= T) of the three facades; report W. `python3 rewire_heavy.py`."""
from rewire import *
base,rv=R0.all_reach();B=W(R0,base)
byn={c['name']:d for d,c in R0.C.items()}
F={'os-kernel':byn['semio-framework-os-kernel'],'umbrella':byn['semio-framework'],'plugin':byn['semio-framework-plugin']}
cone={d:len(rv.get(d,()))+1 for d in R0.M}
for T in (2000,1000,300,100,30):
    R=mk();n=0
    for h in F:
        rew=plan(h,select=lambda dd,v:cone.get(dd,0)>=T,mode='opt');n+=len(rew);apply(R,F[h],rew)
    reach,_=R.all_reach()
    print('T',T,'rewired',n,'W',B,'->',W(R,reach),'(%.2f%%)'%((W(R,reach)-B)*100/B),' '.join('%s %d->%d'%(x,base[byn['semio-framework-'+x]],reach[byn['semio-framework-'+x]]) for x in ('plugin','os-kernel','kernel','job')),'umb',base[byn['semio-framework']],'->',reach[byn['semio-framework']])
