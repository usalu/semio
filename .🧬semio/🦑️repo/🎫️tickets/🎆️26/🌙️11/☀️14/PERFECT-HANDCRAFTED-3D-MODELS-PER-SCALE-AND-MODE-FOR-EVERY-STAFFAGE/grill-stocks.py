"""🔥 Models the suspended rolled ash pan and the supported coal and cooking grates in their measured positions."""
from pathlib import Path
import json,copy
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*/charcoal-grill/*'));out=t/'🗑️generated'/'rotor'/'charcoal-grill.json'
doc=json.loads(r.read_bytes());a=doc['asset'];fine=copy.deepcopy(a['volumes']['50']);fb={q['id']:q for q in fine['body']}
keep=lambda sid:sid.startswith(('pan-hanger-','fuel-','grate-','briquette-','side-handle-','side-bracket-','lid-bracket-','damper-','vent-')) or sid in ['charcoal-bed','ash-pan','lid-handle']
for scale,v in a['volumes'].items():
    d=int(scale);body={q['id']:q for q in v['body']};fineTh={q['id']:q for q in fine['thermal']};fineSt={q['id']:q for q in fine['structure']}
    bd=[q for q in v['body'] if not keep(q['id'])]
    for sid,q in fb.items():
        if not keep(sid):continue
        q=copy.deepcopy(q)
        if sid=='ash-pan':q['profile']=[q['profile'][i] for i in {50:range(13),100:[0,1,2,3,4,6,7,8,9,11,12],200:[0,1,3,4,6,7,8,9,11,12],500:[0,1,3,6,7,9,11,12]}[d]]
        if sid=='charcoal-bed':q['profile']=[[round(y-.0005,6),rad]for i,(y,rad)in enumerate(q['profile'])if i in {50:range(12),100:[0,1,2,3,5,6,8,9,11],200:[0,1,3,5,7,9,11],500:range(12)}[d]]
        if sid.startswith('briquette-'):q['transform']['translation'][1]=round(q['transform']['translation'][1]-.0005,6)
        if d>50 and q['kind']=='loft' and len(q['sections'])>=6:
            secs=q['sections'];step=2 if d<500 else 3;q['sections']=[x for i,x in enumerate(secs) if i in [0,len(secs)-1] or i%step==0]
        bd.append(q)
    v['body']=bd;bb={q['id']:q for q in bd}
    for field,fs in [('thermal',fineTh),('structure',fineSt)]:
        current={q['id']:q for q in v[field] if not q['id'].startswith('th-grate-rod-')}
        for sid,q in fs.items():
            bodyID=q.get('of',sid.removeprefix('th-').removeprefix('member-'))
            if keep(bodyID) and bodyID in bb:
                c=copy.deepcopy(q)
                if sid=='th-charcoal-bed':c['profile']=bb['charcoal-bed']['profile']
                current[sid]=c
        v[field]=list(current.values())
    v['decisions'].append({'en':'A rolled ash pan 33 mm deep hangs from three arms below the bowl. The 390 mm coal bed rests on nine 6 mm fuel rods. Its bulk profile rises 40 mm beneath fourteen individual surface briquettes. The 27 cooking-grate rods are 6 mm in diameter.','de':'Die 33 mm tiefe, gebördelte Ascheschale hängt an drei Armen unter dem Kessel. Das 390 mm breite Kohlebett ruht auf neun 6-mm-Stäben. Das 40 mm hohe Grundprofil trägt vierzehn einzelne Briketts. Die 27 Grillroststäbe haben 6 mm Durchmesser.'})
    if d==50:continue
    rep=a['representations'][scale];fg=a['representations']['50']['geometry']
    for view in ['front','elevation']:
        pan=next(q for q in rep['geometry'][view] if q['id']=='ash-catcher');fp=next(q for q in fg[view]if q['id']=='ash-catcher');pan['points']=copy.deepcopy(fp['points']);pan['penMm']=.20
        for witness in [q for q in fg[view] if q['id'].startswith('pan-hanger')]:
            w=copy.deepcopy(witness);w['penMm']=.2
            if not any(q['id']==w['id']for q in rep['geometry'][view]):rep['geometry'][view].append(w)
regions={q['id']:q for q in a['thermal']['regions']}
regions['charcoal-bed']['operating']['heatW']=3000
regions['bowl-shell']['operating']['heatW']=1745
for key,w in [('ash-catcher',55),('cooking-grate',600)]:
    regions[key].update(model='operating',operating={'key':'charcoal','heatW':w})
a['thermal']['assumptions'].append({'en':'Of the existing surface heat allocations at full fire, 600 W from the fuel-bed allocation warm the rod grate and 55 W from the bowl allocation warm the suspended ash pan. These are estimated radiation and conduction shares. The combined surface input and the 8 kW fuel input stay the same.','de':'Bei vollem Feuer erwärmen 600 W aus dem bisherigen Wärmeanteil des Kohlebetts den Stabrost und 55 W aus dem Kesselanteil die Ascheschale. Es handelt sich um geschätzte Strahlungs- und Leitungsanteile. Die gesamte Oberflächenleistung und die Brennstoffleistung von 8 kW bleiben gleich.'})
out.write_text(json.dumps(doc,ensure_ascii=False))
