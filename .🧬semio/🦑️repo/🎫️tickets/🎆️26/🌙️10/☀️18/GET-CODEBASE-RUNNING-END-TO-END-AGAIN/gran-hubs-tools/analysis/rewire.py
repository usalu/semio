"""🔀 Rewire simulation: replace dependents' edges to a facade hub by edges to the producer crates their production paths resolve to (first re-export hop), then recompute feature-aware reach and W = sum churn(prod) x reach over all crates. Library + CLI: `python3 rewire.py` prints the scenarios."""
from hubenv import *
from graphfeat import Resolver
import copy
CH=json.load(open(OUT+'/churn-all.json'))['prod']
MAN=json.load(open(OUT+'/manifests.json'))
I2D={}
for dd,cc in C.items():
    I2D[(MAN.get(dd,{}).get('lib') or cc['name']).replace('-','_')]=dd
FAC={'semio_framework','semio_framework_os_kernel','semio_framework_plugin'}
def W(R,reach): return sum(CH.get(R.C[d]['name'],0)*n for d,n in reach.items())
def mk():
    return Resolver(M={d:{**m,'deps':list(m['deps'])} for d,m in R0.M.items()},C=R0.C)
R0=Resolver()
from cratemodel import Model,World
_I2L={}
for _dd,_cc in C.items(): _I2L[(MAN.get(_dd,{}).get('lib') or _cc['name']).replace('-','_')]=libfile(_dd,_cc)
WORLD=World(_I2L);Model.world=WORLD
def mod_crates(node,seen=None):
    """crates re-exported (transitively) by a pure re-export module; None when the module has own definitions."""
    seen=seen if seen is not None else set()
    if id(node) in seen: return set()
    seen.add(id(node))
    if node.defs: return None
    out=set()
    for vis,p,al,it in node.uses:
        if not vis.startswith('pub'): continue
        h=p[0]
        if h in('crate','self','super'):
            tgt=WORLD.models[list(WORLD.models)[0]] if False else None
            continue
        if h in node.children: 
            r=mod_crates(node.children[h],seen)
            if r is None: return None
            out|=r
        elif h in _I2L and h not in FAC: out.add(h)
    for nm,c in node.children.items():
        if c.vis.startswith('pub'):
            r=mod_crates(c,seen)
            if r is None: return None
            out|=r
    return out
def plan(hub,select=None,mode='full'):
    """returns {dependent_dir: set(producer_dir)} for dependents whose prod paths all resolve; mode 'opt' also admits unresolved."""
    d=json.load(open(OUT+'/census-%s.json'%hub));R_=d['resolved'];ID=d['ident'];out={}
    for dd,v in d['deps'].items():
        if select and not select(dd,v): continue
        if not v['prod']: continue
        ps=[p for p in v['prod'] if p!=''];prods=set();ok=True
        for p in ps:
            r=R_[p]
            if r['kind']=='unresolved' or r['kind']=='crate' and r['owner']==ID: 
                if mode=='full': ok=False
                continue
            if r['owner']==ID and r['kind']=='mod' and mode=='ideal':
                m=WORLD.model_for(ID);nd=[x for x in m.nodes if x.name==r['name'] and os.path.relpath(x.file,ROOT)==r['file']]
                mc=mod_crates(nd[0]) if nd else None
                if mc is not None: prods|=mc;continue
            if r['owner']==ID and r['kind'] in('def','mod'): ok=False;continue
            ch=[x for x in r.get('chain',[]) if x not in FAC and x!=ID] or [r['owner']]
            prods.add(ch[0])
        if v['macros'] and mode=='full': ok=False
        if mode=='ideal' and False: pass
        if not ok: continue
        tg={I2D[i] for i in prods if i in I2D}
        if len(tg)==len(prods): out[dd]=tg
        elif mode in('opt','ideal'): out[dd]=tg
    return out
def apply(R,HD,rew):
    for d,tg in rew.items():
        m=R.M[d];keep=[x for x in m['deps'] if not (x['kind'] in('normal','build') and R.tgt.get((d,x['kind'],x['key']))==HD)]
        have={R.tgt.get((d,x['kind'],x['key'])) for x in keep if x['kind'] in('normal','build')}
        for t in tg:
            if t in have or t==d: continue
            key='__rw_'+R.C[t]['name']
            keep.append({'key':key,'pkg':R.C[t]['name'],'kind':'normal','target':None,'optional':False,'features':[],'defaultFeatures':True,'ws':False,'path':None})
            R.tgt[(d,'normal',key)]=t
        m['deps']=keep
    R.cache={}
if __name__=='__main__':
    base,_=R0.all_reach();B=W(R0,base);print('W baseline',B)
    byn={c['name']:d for d,c in R0.C.items()}
    def show(R,reach,names):
        return ' '.join('%s %d->%d'%(n.replace('semio-framework-',''),base[byn[n]],reach.get(byn[n],0)) for n in names)
    keyn=['semio-framework-plugin','semio-framework-os-kernel','semio-framework-kernel','semio-framework-job','semio-framework','semio-framework-os-store-engine','semio-framework-os-spr']
    for hub,HD in [('os-kernel',byn['semio-framework-os-kernel']),('umbrella',byn['semio-framework']),('plugin',byn['semio-framework-plugin'])]:
        for mode in ('full','opt'):
            R=mk();rew=plan(hub,mode=mode);apply(R,HD,rew)
            reach,_=R.all_reach();print(hub,mode,'rewired',len(rew),'W',B,'->',W(R,reach),'(%.1f%%)'%((W(R,reach)-B)*100/B),show(R,reach,keyn))
