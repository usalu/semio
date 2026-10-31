import re, sys
from lib import *
slug=re.compile(r'^[a-z0-9]+(?:-[a-z0-9]+)*$')
V1={'lod','coreTempC','riseK','surfaceTempC','insulationM2KW','metabolicShare','periodS','duty'}
bad=0
def err(*m):
    global bad; bad+=1; print('ERR',*m)
for aid in ['walking-commuter','seated-reader','running-child','wheelchair-user']:
    a=load(aid)['asset']
    bones={b['id'] for b in a['rig']['bones']}
    regs={r['id']:r for r in a['thermal']['regions']}
    ems={e['id'] for e in a['thermal']['emitters']}
    anch={x['id'] for x in a['anchors']}
    for e in a['thermal']['emitters']:
        if e['anchor'] not in anch: err(aid,'emitter anchor',e['id'])
        if e.get('region') and e['region'] not in regs: err(aid,'emitter region')
    allreg_at50=set()
    for k,r in a['representations'].items():
        ids=set(); used=set()
        for s in r['geometry']['elevation']:
            if s['id'] in ids: err(aid,k,'dup id',s['id'])
            ids.add(s['id'])
            if not slug.match(s['id']): err(aid,k,'slug',s['id'])
            if s['bone'] not in bones: err(aid,k,'bone',s['id'],s['bone'])
            if 'lod' in s: err(aid,'lod')
            rr=[s['region']] if s.get('region') else s.get('regions',[])
            if s['role']!='ground' and not rr: err(aid,k,'no region',s['id'])
            for x in rr:
                if x not in regs: err(aid,k,'unknown region',s['id'],x)
                used.add(x)
            if s.get('hatch') and s['hatch'] not in {h['id'] for h in r['hatches']}: err(aid,k,'hatch',s['id'])
            if s['kind']=='path' and s['d'].count('M')==0: err(aid,k,'path',s['id'])
        if k=='50': allreg_at50=used
        print(aid,k,'regions unused:',set(regs)-used)
    for p in a['poses']:
        t=p['thermal']
        if set(t['regions'])!=set(regs): err(aid,p['id'],'region binding')
        if set(t['emitters'])!={e for e in ems if next(x for x in a['thermal']['emitters'] if x['id']==e)['kind']!='breath'}: err(aid,p['id'],'emitters')
        if t['breathing'] not in {q['id'] for q in a['thermal']['respiration']['patterns']}: err(aid,p['id'],'pattern')
        for b in p['bones']:
            if b not in bones: err(aid,p['id'],'bone',b)
    for c in a['clips']:
        for t in c['tracks']:
            if t['target'].startswith('emitter:') and t['target'][8:] not in ems: err(aid,c['id'],t['target'])
        for e in c['events']:
            if e['kind'] not in ["contact-on","contact-off","ignition","shutdown","load-on","load-off","thermal","motion","operating"]: err(aid,c['id'],e['kind'])
        if len(c['keys'])==0: err(aid,'keys')
    for r in a['thermal']['regions']:
        bad_k=V1&set(r)
        if bad_k: err(aid,r['id'],bad_k)
    for p in a['thermal']['respiration']['patterns']:
        s=p['inhaleFraction']+p['pauseAfterInhaleFraction']+p['exhaleFraction']+p['pauseAfterExhaleFraction']
        if abs(s-1)>1e-6: err(aid,p['id'],'phase sum',s)
    # materials referenced
    mats={m['id'] for m in a['materials']}
    for r in a['thermal']['regions']:
        if r['material'] not in mats: err(aid,r['id'],'material',r['material'])
        for l in r['layers']:
            if l['material'] not in mats: err(aid,r['id'],l['id'],'layer material',l['material'])
    refs={r['id'] for r in a['provenance']['references']}
    import json as j
    txt=j.dumps(a)
    for m in re.findall(r'"reference": "([a-z0-9-]+)"',txt):
        if m not in refs: err(aid,'reference',m); refs.add(m)
    for m in a['materials']:
        pass
print('errors',bad)
