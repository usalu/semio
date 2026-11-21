"""📏 Draws the real terminal housing parts and the wet impeller back plate in the pump side views."""
from pathlib import Path
import json
t=Path(__file__).resolve().parent;p=t/'🗑️generated'/'rotor'/'circulation-pump.json';doc=json.loads(p.read_bytes());a=doc['asset']
paper=a['representations'];f=paper['50']['geometry']
for d in ['50','100','200']:
    g=paper[d]['geometry']
    for view in ['front','plan','elevation']:
        if d!='50':
            current=g[view];high=f[view];ids={q['id'] for q in high}
            if d=='200' and view=='elevation':
                allow=['stub-l','neck-l','union-l','stub-r','neck-r','union-r','casing','terminal-neck','terminal-box','terminal-lid','gland','motor-head','window']
                current=[{**q} for q in high if q['id'] in allow];g[view]=current
            else:
                current=[q for q in current if q['id'] not in ['terminal-neck','terminal-box','terminal-lid','flange','motor-flange']]
                g[view]=current
                current.extend({**q} for q in high if q['id'] in ['terminal-neck','terminal-box','terminal-lid','flange','motor-flange'])
            for q in current:
                if q['id'] in ['terminal-neck','terminal-box','terminal-lid','flange','motor-flange']:q['penMm']=.2
            if d=='200' and view!='elevation':
                rect={'id':'motor-cover-section','bone':'motor','role':'detail','z':4,'filled':True,'tone':.67,'kind':'rect','region':'motor','material':'aluminium','penMm':.2}
                current.append({**rect,**({'x':0,'y':.009,'w':.008,'h':.084} if view=='front' else {'x':-.042,'y':0,'w':.084,'h':.008})})
        if view=='front':
            g[view].append({'id':'impeller-back-section','bone':'rotor','role':'detail','z':4.6,'filled':True,'tone':.52,'kind':'rect','x':.020,'y':.020,'w':.0025,'h':.062,'material':'polyamide','penMm':.2})
            g[view].append({'id':'casing-wall-section','bone':'housing','role':'detail','z':4.5,'filled':True,'tone':.34,'kind':'rect','x':.025,'y':.091,'w':.0225,'h':.005,'region':'casing','material':'cast-iron','penMm':.2})
        if view=='plan' and d!='50':
            g[view].append({'id':'motor-can-section','bone':'motor','role':'detail','z':4.1,'filled':True,'tone':.57,'kind':'rect','x':.033,'y':-.06,'w':.005,'h':.054,'region':'motor','material':'aluminium','penMm':.2})
for d,ids in [('50',['flange','motor-flange']),('200',['union-l','union-r','neck-l','neck-r','body','gland'])]:
    for items in paper[d]['geometry'].values():
        if not isinstance(items,list):continue
        for q in items:
            if isinstance(q,dict) and q.get('id') in ids:q['penMm']=.2
coarse=a['volumes']['500']
for q in coarse['thermal']:
    if q['id'] in ['th-neck-l','th-neck-r']:q['profile']=[q['profile'][i] for i in [0,1,2,5,7,8]]
    if q['id']=='th-casing':
        for k in ['sections','capStart','capEnd']:q.pop(k,None)
        q.update(kind='revolve',profile=[[.012,0],[.012,.0462],[.014,.048],[.057,.048],[.064,.044],[.067,.033],[.067,0]],axis=[0,0,1],origin=[.1055,-.003,0])
p.write_text(json.dumps(doc,ensure_ascii=False))
