"""🌀 Authors the separate fixed housings, mount struts, pitched impellers and open core for the two fan families."""
import math,json,copy
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/🔌️appliances'))
out=t/'🗑️generated'/'rotor';out.mkdir(parents=True,exist_ok=True)
def clipped(q,field,stations):return {**q,field:[q[field][i] for i in stations]}
def ring(r,n,reverse=False):
    return [[round(r*math.cos(k*math.tau/n),6),round(r*math.sin(k*math.tau/n),6)] for k in (range(n-1,-1,-1) if reverse else range(n))]
def rounded(half,rad,k):
    return [[round(cx+rad*math.cos(math.radians(start+90*i/k)),6),round(cy+rad*math.sin(math.radians(start+90*i/k)),6)] for cx,cy,start in [(half-rad,-half+rad,-90),(half-rad,half-rad,0),(-half+rad,half-rad,90),(-half+rad,-half+rad,180)] for i in range(k+1)]
def author(a):
    fine=a['volumes']['50'];b={q['id']:q for q in fine['body']};fth={q['of']:q for q in fine['thermal'] if 'of'in q};fst={q['id']:q for q in fine['structure']}
    hrv=a['id'].startswith('decentralised')
    for d,arc,n in [(100,4,48),(200,2,32),(500,1,24)]:
        v=a['volumes'][str(d)];body=[];th=[];st=[]
        def add(q,reg=None):
            body.append(q)
            ref=fth.get(q['id'])
            if ref or reg:th.append({**(ref or {}),'id':'th-'+q['id'],'of':q['id'],**({'region':reg} if reg else {})})
        plate=b['plate'];add({**plate,'profile':rounded(.13 if hrv else .11,.02 if hrv else .014,arc),'holes':[ring(.0945 if hrv else .1,n,True)]})
        if hrv:
            rim=b['plate-rim'];add({**rim,'profile':rounded(.13,.02,arc),'holes':[rounded(.1275,.0175,arc)[::-1]]})
            add(clipped(b['ring'],'profile',{100:[0,1,2,3,4,5,7,9,11,12],200:[0,1,2,3,4,5,8,11,12],500:[0,1,2,3,4,5,11,12]}[d]))
            sleeve=b['sleeve'];add({**sleeve,'profile':ring(.098,n),'holes':[ring(.095,n,True)]})
            if d==100:add({**b['rim'],'profile':ring(.104,n),'holes':[ring(.098,n,True)]})
            add(b['jacket'])
            add({**b['core'],'profile':ring(.08,n)})
        else:
            add(clipped(b['ring'],'profile',{100:[0,1,2,4,6,8,10,11],200:[0,1,2,5,8,10,11],500:[0,1,2,6,10,11]}[d]))
            q=b['spigot'];add({**q,'profile':q['profile'] if d==100 else [[-.015,.1],[.0375,.1],[.0375,.104],[.045,.104],[.045,.0975],[.0135,.0975],[.004,.088],[-.015,.088],[-.015,.1]]})
            if d==100:add(b['lip'])
        strut={**b['strut']}
        if d==100:strut['cornerRadius']=.0015
        elif d==200:strut['cornerRadius']=.00075
        else:strut.pop('cornerRadius',None)
        add(strut);add(b['stator' if hrv else 'flange'])
        stations=({100:[0,1,3,5,7,9,10],200:[0,2,5,8,10],500:[0,5,8,10]} if hrv else {100:[0,1,3,5,7,8],200:[0,2,4,6,8],500:[0,3,6,8]})[d]
        if d==100:
            for i in range(1,6):add(clipped(b['blade-'+str(i)],'sections',stations))
        else:
            blade=clipped(b['blade-1'],'sections',stations);blade['repeat']={'count':5,'rotationAxis':[0,0,1],'rotationRad':round(math.tau/5,8)}
            add(blade)
        hub=clipped(b['hub'],'profile',({100:[0,1,2,3,4,5,6],200:[0,1,3,4,5,6],500:[0,1,4,5,6]} if hrv else {100:[0,2,4,5,6,7,8],200:[0,3,5,6,7,8],500:[0,4,6,7,8]})[d]);add(hub)
        th.extend(q for q in v['thermal'] if q.get('tags'))
        for q in v['structure']:
            sid=q['id']
            if sid=='member-spider':st.append({**q,'id':'member-strut','of':'strut'})
            elif sid in ('member-shaft','joint-bearing'):st.append(fst[sid])
            elif 'of' not in q or any(bd['id']==q['of'] for bd in body):st.append(q)
        for sid in (['member-sleeve','member-stator','member-shaft','joint-bearing'] if hrv else ['member-spigot','member-flange','member-shaft','joint-bearing']):
            if not any(q['id']==sid for q in st):st.append(fst[sid])
        v.update(body=body,thermal=th,structure=st)
        v['decisions']=[{'en':f'At 1:{d} the digital model retains the five pitched blades and the physical transitions from inlet to outlet. The paper symbol abstracts the internal mechanism.', 'de':f'Im digitalen Modell 1:{d} bleiben die fünf angestellten Schaufeln und die Übergänge zwischen Einlass und Auslass erhalten. Das Drucksymbol vereinfacht den Innenaufbau.'},{'en':'The fixed spider carries the shaft bearing. The motor hub and impeller rotate together. Thermal surfaces follow the physical housing and exposed impeller.', 'de':'Die festen Stege tragen das Wellenlager. Motornabe und Laufrad drehen gemeinsam. Die Wärmeflächen folgen dem Gehäuse und dem freiliegenden Laufrad.'}]
for slug in ['duct-fan','decentralised-hrv-fan']:
    raw=next((r/slug).iterdir()).read_bytes();doc=json.loads(raw);author(doc['asset']);(out/(slug+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
