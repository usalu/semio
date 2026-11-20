"""🤲 Fits four individually tapered glove fingers around each courier's grip and routes the cyclist's four coarse spokes from the two hub flanges."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
def replace(text,source,solids,offset):
    i=text.index('{\n            "id": "'+source['id']+'"',offset)
    j=sp['member_end'](text,i)
    while json.loads(text[i:j])!=source:
        i=text.index('{\n            "id": "'+source['id']+'"',j)
        j=sp['member_end'](text,i)
    new=',\n          '.join(sp['dump'](p,5,'compact') for p in solids)
    return text[:i]+new+text[j:]
digits={
 'index':[(.09,-.005,-.032,.0105),(.105,-.037,-.033,.0100),(.082,-.055,-.034,.0086),(.063,-.046,-.034,.0072)],
 'middle':[(.092,-.006,-.011,.011),(.108,-.040,-.011,.0104),(.081,-.059,-.011,.0092),(.058,-.044,-.012,.0078)],
 'ring':[(.089,-.008,.011,.0102),(.104,-.04,.0105,.0097),(.08,-.057,.0115,.0084),(.061,-.045,.012,.0074)],
 'little':[(.08,-.009,.031,.0092),(.096,-.034,.032,.0085),(.08,-.05,.033,.0074),(.064,-.044,.033,.0066)]}
for slug in ('courier-with-hand-truck','cyclist'):
    path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=path.read_text()
    text=old
    a=json.loads(old)['asset']
    offset=text.index('"volumes": ')
    if slug=='courier-with-hand-truck':
        if any(p['id']=='glove-little-distal-r' for p in a['volumes']['50']['body']):continue
        v=a['volumes']['50']
        for side,sign in [('l',1),('r',-1)]:
            for i,part in enumerate(('proximal','middle','distal')):
                id=f'glove-finger-{i}-{side}'
                body=next(p for p in v['body'] if p['id']==id)
                heat=next(p for p in v['thermal'] if p.get('of')==id)
                fingers=[]
                regions=[]
                for digit,rows in digits.items():
                    p0,p1=rows[i:i+2]
                    b={**body,'id':f'glove-{digit}-{part}-{side}','p0':[p0[0],p0[1],sign*p0[2]],'p1':[p1[0],p1[1],sign*p1[2]],'r0':p0[3],'r1':p1[3]}
                    del b['repeat']
                    fingers.append(b)
                    regions.append({**heat,'id':'th-'+b['id'],'of':b['id']})
                text=replace(text,body,fingers,offset)
                text=replace(text,heat,regions,offset)
    else:
        for wheel in ('front','rear'):
            id='st-'+wheel+'-spokes'
            p=next(p for p in a['volumes']['500']['structure'] if p['id']==id)
            spans=[]
            for flange,start,end in [('near',[.026,.004,-.018],[.31,0,0]),('far',[-.004,.026,.018],[0,.31,0])]:
                rep={**p['repeat'],'count':2,'rotationRad':3.14159265}
                spans.append({**p,'id':id+'-'+flange,'p0':start,'p1':end,'repeat':rep})
            text=replace(text,p,spans,offset)
    assert path.read_text()==old
    assert json.loads(text)
    path.write_text(text)
