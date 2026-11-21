"""👓 Seats the wire temples and bridge on their rims and separates the thermal field of each frame from facial tissue."""
import json,math,re,sys
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
for name,bridgeRise in [('elderly-with-cane',.0065),('person-with-rollator',.0055)]:
    p=next((root/name).iterdir());original=p.read_text();text=original;a=json.loads(text)['asset']
    v=a['volumes']['50'];by={s['id']:s for s in v['body']}
    left=by['glasses-rim-l'];right=by['glasses-rim-r'];cx,cy,cz=left['origin'];R=left['profile'][0][1];wire=left['profile'][0][0]
    bridge=by['glasses-bar'];temples=[by['glasses-temple-l'],by['glasses-temple-r']]
    if '--check' in sys.argv:
        assert any(r['id']=='spectacle-frame' and r['model']=='passive' for r in a['thermal']['regions']),name
        for s in [*temples,bridge]:
            ends=[s['p0']] if s in temples else [s['p0'],s['p1']]
            for x,y,z in ends:
                assert abs(y-cy)<1e-5 and abs(math.hypot(x-cx,abs(z)-cz)-R)<1e-5
        assert all(any(s['region']=='spectacle-frame' for s in v['thermal']) for v in a['volumes'].values())
        assert all('spectacle-frame' in p['thermal']['regions'] for p in a['poses'])
        print(name,'verified');continue
    if any(r['id']=='spectacle-frame' for r in a['thermal']['regions']):continue
    def replace(marker,id,items):
        global text
        j=text.index('"id": "'+id+'"',marker);i=text.rindex('{',0,j);j=sp['member_end'](text,i)
        depth=(i-text.rindex('\n',0,i)-1)//2
        text=text[:i]+(',\n'+'  '*depth).join(sp['dump'](q,depth,'compact') for q in items)+text[j:]
    def liststart(d,field):return text.index('"'+field+'": [',text.index('"'+d+'": {',text.index('"volumes": {')))
    bridgeHalf=round(cz-math.sqrt(R*R-bridgeRise*bridgeRise),6)
    bridge.update(p0=[round(cx+bridgeRise,6),cy,-bridgeHalf],p1=[round(cx+bridgeRise,6),cy,bridgeHalf],r0=.0005,r1=.0005)
    for side,arm in zip([1,-1],temples):
        dx=arm['p0'][0]-cx
        arm['p0']=[arm['p0'][0],cy,round(side*(cz+math.sqrt(R*R-dx*dx)),6)]
    for s in [bridge,*temples]:replace(liststart('50','body'),s['id'],[s])
    regionArea=2*4*math.pi**2*R*wire
    for s in [bridge,*temples]:
        radius=s['r0'];regionArea+=2*math.pi*radius*math.dist(s['p0'],s['p1'])+4*math.pi*radius**2
    region={'id':'spectacle-frame','title':{'en':'Wire spectacle rims, bridge and temples','de':'Brillenfassung mit Steg und Bügeln'},'kind':'surface','model':'passive','material':'spectacle-frame','areaM2':round(regionArea,5),'characteristicLengthM':round(2*wire,4),'geometry':'cylinder','skyViewFactor':.45,'layers':[],'heatCapacityJM2K':round(8000*470*wire/2),'source':'authored-estimate'}
    last=a['thermal']['regions'][-1];replace(text.index('"regions": [',text.index('"thermal": {\n      "environment"')),last['id'],[last,region])
    for entry in v['thermal']:
        if entry.get('of','').startswith('glasses-'):
            replace(liststart('50','thermal'),entry['id'],[{**entry,'region':region['id']}])
    for d in ['100','200','500']:
        current=a['volumes'][d]['thermal'];frame=[]
        for source in [left,right,bridge,*temples]:
            shape={**source,'id':'th-'+source['id'],'region':region['id']}
            if source in [left,right] and d!='100':
                if source is left:continue
                shape.update(id='th-spectacle-rims',repeat={'count':2,'translation':[0,0,round(2*cz,6)]})
            frame.append(shape)
        replace(liststart(d,'thermal'),current[-1]['id'],[current[-1],*frame])
    binding='''"face-skin": '''
    def bind(m):
        s=m[0];old=json.loads(m[2]);b={'exposedFraction':1,'sunlitFraction':old['sunlitFraction'],'contact':{'fraction':.05,'tempC':33}}
        return s+',\n'+m[1]+'"spectacle-frame": '+sp['dump'](b,len(m[1])//2,'compact')
    text,n=re.subn(r'(?m)^( +)"face-skin": (\{\n(?:(?!^\1\}).*\n)*?^\1\})',bind,text)
    assert n==len(a['poses']),(name,n,len(a['poses']))
    def shape(m):
        s=m[0]
        if '"region": "face-skin"' not in s:return s
        return re.sub(r'"material": "(skin|skin-aged)"','"material": "spectacle-frame"',s.replace('"region": "face-skin"','"region": "spectacle-frame"'))
    text=re.sub(r'\{\n              "id": "glasses-[^"]*".*?\n            \}',shape,text,flags=re.S)
    assert json.loads(text) and p.read_text()==original
    p.write_text(text);print(name,round(regionArea,5))
