"""🔀 Rewire simulation v2 (library): importers of a facade are repointed to producer crates; importers that name the facade's own definitions (macros, consts, types) keep the hub edge; the hub's dependency list can be pruned to what its remaining own items need (`keep`). Feature-aware reach and W as in rewire.py."""
from rewire import *
def plan2(hub,mode='ideal'):
    """{dependent: (producer_dirs, keep_hub)} for production users; dead/test-only handled by the caller."""
    d=json.load(open(OUT+'/census-%s.json'%hub));R_=d['resolved'];ID=d['ident'];out={}
    for dd,v in d['deps'].items():
        if not v['prod']: continue
        prods=set();keep=False
        for p in [p for p in v['prod'] if p!=''] or ['']:
            r=R_[p]
            if p=='': continue
            if r['kind']=='unresolved': 
                if mode=='full': keep=True
                continue
            if r['kind']=='crate' and r['owner']==ID: 
                if mode=='full': keep=True
                continue
            if r['owner']==ID and r['kind']=='mod' and mode=='ideal':
                m=WORLD.model_for(ID);nd=[x for x in m.nodes if x.name==r['name'] and os.path.relpath(x.file,ROOT)==r['file']]
                mc=mod_crates(nd[0]) if nd else None
                if mc is not None: prods|=mc;continue
            if r['owner']==ID and r['kind'] in('def','mod'): keep=True;continue
            ch=[x for x in r.get('chain',[]) if x not in FAC and x!=ID] or [r['owner']]
            prods.add(ch[0])
        if v['macros']: keep=True
        tg={I2D[i] for i in prods if i in I2D}
        out[dd]=(tg,keep)
    return out
def apply2(R,HD,pl,dead=(),keep=None):
    for d in dead:
        m=R.M[d];m['deps']=[x for x in m['deps'] if not (x['kind'] in('normal','build') and R.tgt.get((d,x['kind'],x['key']))==HD and not x['features'])]
    for d,(tg,keep_hub) in pl.items():
        m=R.M[d]
        kept=[x for x in m['deps'] if keep_hub or not (x['kind'] in('normal','build') and R.tgt.get((d,x['kind'],x['key']))==HD)]
        have={R.tgt.get((d,x['kind'],x['key'])) for x in kept if x['kind'] in('normal','build')}
        for t in tg:
            if t in have or t==d: continue
            key='__rw_'+R.C[t]['name']
            kept.append({'key':key,'pkg':R.C[t]['name'],'kind':'normal','target':None,'optional':False,'features':[],'defaultFeatures':True,'ws':False,'path':None})
            R.tgt[(d,'normal',key)]=t
        m['deps']=kept
    if keep is not None:
        m=R.M[HD];m['deps']=[x for x in m['deps'] if x['kind'] not in('normal','build') or R.tgt.get((HD,x['kind'],x['key'])) in keep]
    R.cache={}
