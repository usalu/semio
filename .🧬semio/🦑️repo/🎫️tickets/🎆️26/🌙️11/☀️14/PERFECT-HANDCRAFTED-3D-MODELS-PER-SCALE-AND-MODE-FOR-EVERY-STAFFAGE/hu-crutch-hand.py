"""🖐️ Builds the grip channel through the palm and curled fingers and makes the two coarse ground supports visible."""
from math import sin,cos,atan2,radians
from pathlib import Path
from runpy import run_path
import json
t=Path(__file__).resolve().parent
sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
p=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/person-on-crutches/*'))
old=p.read_text();text=old;asset=json.loads(old)['asset']
def splice(k,q,items):
    global text
    idx=text.index('"'+k+'": {',text.index('"volumes": {'))
    start=text.index('{\n            "id": "'+q['id']+'"',idx);end=sp['member_end'](text,start)
    text=text[:start]+',\n          '.join(sp['dump'](item,5,'compact') for item in items)+text[end:]
fine=[(-4,-34),(11,-34),(24,-27),(33,-14),(37,2),(29,23),(15,35),(-6,38),(-23,30),(-35,16),(-39,0),(-35,-16),(-22,-29),(-12,-31),(-7,-22),(-14,-13),(-18,-2),(-16,9),(-8,16),(4,18),(14,12),(18,1),(15,-10),(7,-16),(-3,-18)]
coarse=[(-4,-37),(16,-34),(32,-22),(39,0),(31,24),(14,38),(-7,39),(-28,29),(-39,7),(-36,-14),(-20,-31),(-10,-32),(-6,-21),(-17,-10),(-19,4),(-10,16),(6,18),(17,8),(16,-8),(4,-18),(-3,-18)]
for k in ['200','500']:
    v=asset['volumes'][k];b={q['id']:q for q in v['body']};th={q['id']:q for q in v['thermal']};st={q['id']:q for q in v['structure']}
    if b['hand-l']['kind']=='extrude':continue
    for side,sign in [('l',1),('r',-1)]:
        id='hand-'+side;oldhand=b[id]
        crutch='crutch-'+side;grip='crutch-grip-'+side;tip='crutch-tip-'+side;shaft='crutch-tube-'+side
        precedent={q['id']:q for q in asset['volumes']['200']['body']}
        axis=precedent[grip]['axis'];rig=next(r for r in asset['rig']['bones'] if r['id']==crutch)
        theta=radians(rig['angleDeg'])+atan2(axis[1],axis[0]);co=cos(theta/2);si=sin(theta/2)
        quat=[(co-si)/2,(co+si)/2,(co+si)/2,(co-si)/2]
        pattern=fine if k=='200' else coarse
        profile=[[x/1000,sign*y/1000] for x,y in pattern]
        if sign<0:profile.reverse()
        hand=dict(id=id,kind='extrude',bone=id,material=oldhand['material'],role='body',tone=oldhand['tone'],profile=profile,plane='xy',**{'from':.01 if k=='200' else .011},to=.092 if k=='200' else .087,transform={'translation':[rig['x'],rig['y'],rig['z']],'rotation':[round(n,8) for n in quat]})
        splice(k,oldhand,[hand])
        g={**precedent[grip]}
        if k=='200':
            g['profile']=[[0,0],[0,.011],[.023,.012],[.03,.0165],[.095,.0185],[.114,.0205],[.118,.0185],[.118,0]]
            splice(k,b[grip],[g])
        else:
            g['profile']=[[0,0],[0,.012],[.021,.013],[.032,.018],[.095,.018],[.113,.022],[.117,.0375],[.122,.0375],[.125,.033],[.125,0]]
            ferrule={**precedent[tip],'profile':[[.823,0],[.823,.014],[.841,.018],[.855,.0375],[.862,.0375],[.880,.025],[.897,.0205],[.90335,.020],[.90335,0]]}
            tube={**b[shaft],'sections':[*b[shaft]['sections'][:-1],{**b[shaft]['sections'][-1],'at':[round(.850*x,5) for x in ferrule['axis']]}]}
            splice(k,b[shaft],[tube,g,ferrule])
            for id in [grip,tip]:
                q=th['th-'+id];splice(k,q,[{'id':q['id'],'of':id,'region':q['region']}])
        support=st['member-'+shaft]
        splice(k,support,[support,{'id':'member-'+grip,'of':grip,'member':crutch,'part':'member'},{'id':'member-'+tip,'of':tip,'member':crutch,'part':'member'}])
    if k=='500':
        for d in v['decisions']:
            for lang,s in d.items():
                fresh=s.replace('each hand is one mass, the crutch is one slender rod from cuff to ground with its cuff, because handle and ferrule fall below the feature','the palm and curled fingers wrap the rubber grip, and the shaft seats in a ground ferrule').replace('jede Hand ist eine Masse, die Krücke eine schlanke Stange von der Manschette bis zum Boden mit ihrer Manschette, weil Griff und Puffer unter die Einzelheit fallen','Handfläche und gekrümmte Finger umschließen den Gummigriff, und der Schaft sitzt im Gummipuffer am Boden').replace('grips and ferrules, which the body no longer shows, are surfaces at the size floor on the rod','grips and ferrules follow the visible rubber components').replace('Griffe und Puffer, die der Körper nicht mehr zeigt, sind Flächen in Mindestgröße an der Stange','Griffe und Puffer folgen den sichtbaren Gummibauteilen')
                if s!=fresh:text=text.replace(json.dumps(s,ensure_ascii=False),json.dumps(fresh,ensure_ascii=False),1)
if text!=old:
    doc=json.loads(text)['asset']
    assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/'person-on-crutches.json';s=snap.read_text();draft=json.loads(s)
    for k in ['200','500']:draft['volumes'][k]=doc['volumes'][k]
    assert snap.read_text()==s;snap.write_text(json.dumps(draft,ensure_ascii=False,separators=(',',':'))+'\n')
