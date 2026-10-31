import json,sys,re,math
from lib import *
SCH=json.load(open(f"{S}/🧬️schema/🔣️.json"))
def resolve(ref):
    n=SCH
    for p in ref.lstrip('#/').split('/'): n=n[p]
    return n
def val(x,sc,path,errs):
    if '$ref' in sc: return val(x,resolve(sc['$ref']),path,errs)
    if 'oneOf' in sc:
        ok=0; last=[]
        for o in sc['oneOf']:
            e=[]; val(x,o,path,e)
            if not e: ok+=1
            else: last.append(e[:3])
        if ok!=1: errs.append(f"{path}: oneOf matched {ok} {last if ok==0 else ''}")
        return
    if 'const' in sc and x!=sc['const']: errs.append(f"{path}: const {sc['const']}")
    if 'enum' in sc and x not in sc['enum']: errs.append(f"{path}: enum {x}")
    t=sc.get('type')
    if t=='null':
        if x is not None: errs.append(f"{path}: not null")
        return
    tm={'object':dict,'array':list,'string':str,'boolean':bool}
    if t=='number':
        if isinstance(x,bool) or not isinstance(x,(int,float)): errs.append(f"{path}: not number"); return
        if 'minimum' in sc and x<sc['minimum']: errs.append(f"{path}: <min {x}")
        if 'maximum' in sc and x>sc['maximum']: errs.append(f"{path}: >max {x}")
        if 'exclusiveMinimum' in sc and x<=sc['exclusiveMinimum']: errs.append(f"{path}: <=excl {x}")
        return
    if t=='integer':
        if not isinstance(x,int): errs.append(f"{path}: not integer"); return
    if t in tm and not isinstance(x,tm[t]): errs.append(f"{path}: not {t}"); return
    if t=='string':
        if 'pattern' in sc and not re.search(sc['pattern'],x): errs.append(f"{path}: pattern {x}")
        if 'minLength' in sc and len(x)<sc['minLength']: errs.append(f"{path}: short")
    if t=='array':
        if 'minItems' in sc and len(x)<sc['minItems']: errs.append(f"{path}: minItems")
        if 'maxItems' in sc and len(x)>sc['maxItems']: errs.append(f"{path}: maxItems")
        if 'items' in sc:
            for i,v in enumerate(x): val(v,sc['items'],f"{path}[{i}]",errs)
    if t=='object':
        for r in sc.get('required',[]):
            if r not in x: errs.append(f"{path}: missing {r}")
        props=sc.get('properties',{})
        for k,v in x.items():
            if k in props: val(v,props[k],f"{path}.{k}",errs)
            elif 'additionalProperties' in sc:
                ap=sc['additionalProperties']
                if ap is False: errs.append(f"{path}: extra {k}")
                elif isinstance(ap,dict):
                    if 'propertyNames' in sc and not re.search(sc['propertyNames'].get('pattern','.'),k): errs.append(f"{path}: key {k}")
                    val(v,ap,f"{path}.{k}",errs)
def semantic(a,errs):
    bones={b['id'] for b in a['rig']['bones']}; anchors={x['id']:x for x in a['anchors']}
    for x in a['anchors']:
        if x['bone'] not in bones: errs.append(f"anchor bone {x['id']}")
    regs={r['id']:r for r in a['thermal']['regions']}
    for r in regs.values():
        if r['model']=='body':
            pass
    body=[r for r in regs.values() if 'body' in r]
    if body:
        ms=sum(r['body']['metabolicShare'] for r in body); sf=sum(r['body']['skinFraction'] for r in body)
        if abs(ms-1)>1e-6: errs.append(f"metabolic shares {ms}")
        if abs(sf-1)>1e-6: errs.append(f"skin fractions {sf}")
        sa=sum(r['areaM2'] for r in body)
        print("  body area sum",round(sa,3),"DuBois",a['thermal']['body']['skinAreaM2'])
    # emitters
    em={e['id']:e for e in a['thermal']['emitters']}
    for e in em.values():
        if e['anchor'] not in anchors: errs.append(f"emitter anchor {e['id']}")
        if 'region' in e and e['region'] not in regs: errs.append(f"emitter region {e['id']}")
    rs=a['thermal'].get('respiration')
    if rs:
        for k in('noseEmitter','mouthEmitter'):
            if rs.get(k) and rs[k] not in em: errs.append(f"resp {k}")
        for m in rs['motion']:
            if m['bone'] not in bones: errs.append("motion bone")
    pats={p['id'] for p in (rs or {}).get('patterns',[])}
    nonbreath=[e for e in em if em[e]['kind']!='breath']
    for p in a['poses']:
        th=p['thermal']
        for r in regs:
            if r not in th['regions']: errs.append(f"pose {p['id']} lacks region {r}")
        for r in th['regions']:
            if r not in regs: errs.append(f"pose {p['id']} unknown region {r}")
        for e in nonbreath:
            if e not in th['emitters']: errs.append(f"pose {p['id']} lacks emitter {e}")
        if rs and th.get('breathing') not in pats: errs.append(f"pose {p['id']} breathing")
        for c in p['contacts']:
            if c not in anchors: errs.append(f"pose contact {c}")
        for b in p['bones']:
            if b not in bones: errs.append(f"pose bone {b}")
    pi={p['id'] for p in a['poses']}
    for c in a['clips']:
        for k in c['keys']:
            if k['pose'] not in pi: errs.append(f"clip pose {k['pose']}")
            for b in k.get('bones',{}):
                if b not in bones: errs.append(f"clip bone {b}")
        for sp in c['contacts']:
            if sp['anchor'] not in anchors: errs.append("clip contact anchor")
        for t in c['tracks']:
            tg=t['target']
            if tg.startswith('emitter:') and tg[8:] not in em: errs.append(f"track {tg}")
            if tg=='breathing' and any(v['v']>=len(pats) for v in t['keys']): errs.append("breathing idx")
    # representations
    for k,r in a['representations'].items():
        if r['status']!='authored': continue
        ids=set(); used=set()
        for el in ('elevation','plan'):
            for sh in r['geometry'].get(el,[]):
                if sh['id'] in ids: errs.append(f"{k} dup shape {sh['id']}")
                ids.add(sh['id'])
                if sh['bone'] not in bones: errs.append(f"{k} {sh['id']} bone {sh['bone']}")
                for rg in ([sh['region']] if 'region' in sh else [])+sh.get('regions',[]):
                    used.add(rg)
                    if rg not in regs: errs.append(f"{k} {sh['id']} region {rg}")
                if sh.get('hatch') and sh['hatch'] not in {h['id'] for h in r['hatches']}: errs.append("hatch")
                mat=sh.get('material')
                if mat and mat not in {m['id'] for m in a['materials']}: errs.append(f"{k} {sh['id']} material {mat}")
        miss=set(regs)-used
        if miss: errs.append(f"{k} regions not drawn: {sorted(miss)}")
        vr=r['validRange']
        if not vr['minDenominator']<=int(k)<=vr['maxDenominator']: errs.append(f"{k} validRange")
    mats={m['id'] for m in a['materials']}
    for r in regs.values():
        if r['material'] not in mats: errs.append(f"region material {r['id']}")
        for l in r['layers']:
            if l['material'] not in mats: errs.append(f"layer material {l['id']}")
    refs={r['id'] for r in a['provenance']['references']}
    def chk(o,p=""):
        if isinstance(o,dict):
            if 'reference' in o and o['reference'] not in refs: errs.append(f"unknown reference {o['reference']} at {p}")
            for k,v in o.items(): chk(v,p+"/"+k)
        elif isinstance(o,list):
            for i,v in enumerate(o): chk(v,p+f"[{i}]")
    chk(a)
if __name__=="__main__":
    for aid in sys.argv[1:]:
        doc=json.load(open(out_path(aid))); errs=[]
        val(doc,SCH['$defs']['AssetDocument'],"",errs)
        semantic(doc['asset'],errs)
        print(aid,"ERRORS" if errs else "OK",len(errs))
        for e in errs[:40]: print("  ",e)
