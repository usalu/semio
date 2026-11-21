"""🔧 Seats the rotating shafts inside the fixed, bored fan supports and measures impeller rake against pump rotation."""
from pathlib import Path
from runpy import run_path
import json
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*appliances'));out=t/'🗑️generated'/'rotor'
for slug in ['duct-fan','decentralised-hrv-fan','circulation-pump']:
    src=next((r/slug).iterdir());doc=json.loads(src.read_bytes());asset=doc['asset']
    for d,v in asset['volumes'].items():
        b={q['id']:q for q in v['body']}
        if slug=='circulation-pump':
            q=b['impeller-vane']
            if q['profile'][len(q['profile'])//2-1][1]>0:q['profile']=[[x,-y]for x,y in q['profile'][::-1]]
            continue
        if 'shaft'in b:continue
        hrv=slug.startswith('decent');hub=b['hub'];stator=b['stator' if hrv else 'flange'];profile=hub['profile'][:-1]
        if hrv:profile.extend([[-.073,.0215],[-.0845,.0215],[-.0845,.004],[-.099,.004],[-.099,0]])
        else:profile[-1][0]=.009;profile.extend([[.009,.004],[-.030,.004],[-.030,0]])
        hub['profile']=profile
        lo,hi=(-.070,-.058) if hrv else (.011,.023)
        axleLo,axleHi=(-.099,-.054) if hrv else (-.030,.026)
        def turned(parent,id,bone,profile):return {'id':id,'kind':'revolve','bone':bone,'material':'motor-housing','role':'detail','tone':.45,'profile':profile,'axis':[0,0,1],'origin':[0,0,0]}
        shaft=turned(stator,'shaft','rotor',[[axleLo,0],[axleLo,.004],[axleHi,.004],[axleHi,0]])
        bush=turned(stator,'bearing','ring',[[lo,.00405],[lo,.009],[hi,.009],[hi,.00405],[lo,.00405]])
        p=stator['profile'];p[0][1]=.009;p[-1][1]=.009;p.append(p[0])
        v['body'].extend([shaft,bush])
        for q in v['structure']:
            if q['id']in ['member-shaft','joint-bearing']:
                joint=q['id']=='joint-bearing';q.clear();q.update({'id':'joint-bearing'if joint else 'member-shaft','of':'bearing' if joint else 'shaft','member':'rotor','part':'joint'if joint else 'member'})
        for i,q in enumerate(v['thermal']):
            if q.get('of') in ['hub',stator['id']]:
                base=b[q['of']]
                if q['of']=='hub':surface=base['profile'][:next(k for k,x in enumerate(base['profile']) if x[0]==(-.073 if hrv else .009) and x[1]>.025)+1];surface=surface+[[surface[-1][0],0]]
                else:surface=[[p[0][0],0],p[1],p[2],[p[2][0],0]]
                v['thermal'][i]={**base,'profile':surface,**{k:x for k,x in q.items()if k!='of'}}
    if slug=='duct-fan':
        c=next(q for q in asset['clips']if q['id']=='speed-steps')
        for k,angle in zip(c['keys'],[0,0,10800,54000,54000]):k.update(ease='linear',bones={'rotor':{'angleDeg':angle}})
    (out/(slug+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
