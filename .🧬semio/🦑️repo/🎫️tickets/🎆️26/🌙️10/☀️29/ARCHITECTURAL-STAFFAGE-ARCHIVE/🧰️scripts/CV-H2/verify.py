import sys, json
H="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
bad=0
for aid in ['elderly-with-cane','cyclist','worker-carrying-box','parent-with-stroller']:
    a=json.load(open(H+aid+"/🔣️.json"))['asset']; t=a['thermal']
    def err(*m):
        global bad; bad+=1; print('ERR',aid,*m)
    bones={b['id'] for b in a['rig']['bones']}; anchors={x['id'] for x in a['anchors']}
    regs=[r['id'] for r in t['regions']]; mats={m['id'] for m in a['materials']}
    em={e['id']:e for e in t['emitters']}; pats=[p['id'] for p in t['respiration']['patterns']]
    for r in t['regions']:
        if r['material'] not in mats: err('mat',r['id'])
        for l in r['layers']:
            if l['material'] not in mats: err('layer mat',l['material'])
    for e in t['emitters']:
        if e['anchor'] not in anchors: err('emitter anchor',e['id'])
        if e.get('region') and e['region'] not in regs: err('emitter region')
        if e['kind']=='vent' and e['operatingKey'] not in [o['id'] for o in t['operating']]: err('opkey')
    for m in t['respiration']['motion']:
        if m['bone'] not in bones: err('motion bone')
    if t['respiration']['noseEmitter'] not in em or t['respiration']['mouthEmitter'] not in em: err('resp emitters')
    nb=[e for e in em if em[e]['kind']!='breath']
    for p in a['poses']:
        th=p['thermal']
        if set(th['regions'])!=set(regs): err('pose regions',p['id'],set(regs)^set(th['regions']))
        if set(th['emitters'])!=set(nb): err('pose emitters',p['id'])
        if th['breathing'] not in pats: err('breathing',p['id'])
        for k,v in th['regions'].items():
            if not(0<=v['exposedFraction']<=1 and 0<=v['sunlitFraction']<=1): err('frac',p['id'],k)
    for c in a['clips']:
        for tr in c['tracks']:
            tg=tr['target']
            if tg.startswith('emitter:') and tg[8:] not in nb: err('track emitter',c['id'],tg)
            if tg=='breathing' and max(k['v'] for k in tr['keys'])>=len(pats): err('breathing idx')
        for ev in c['events']:
            if ev['kind'] in('inhale','exhale'): err('event')
    for k,r in a['representations'].items():
        ids=set(); hs={h['id'] for h in r['hatches']}
        for s in r['geometry']['elevation']:
            if s['id'] in ids: err('dup',k,s['id'])
            ids.add(s['id'])
            if s['bone'] not in bones: err('bone',s['id'])
            if s.get('hatch') and s['hatch'] not in hs: err('hatch',k,s['id'])
            for x in ([s['region']] if 'region' in s else [])+s.get('regions',[]):
                if x not in regs: err('region',k,s['id'],x)
            if 'region' not in s and 'regions' not in s and s['role']!='ground': err('unbound',k,s['id'])
            if s.get('material') and s['material'] not in mats: err('shape mat',s['id'])
            for fld in ('lod',):
                if fld in s: err('lod')
    for k in ('geometry',):
        if k in a: err('geometry left')
    print(aid,'regions',len(regs),'poses',len(a['poses']),'ok' )
print('errors',bad)
