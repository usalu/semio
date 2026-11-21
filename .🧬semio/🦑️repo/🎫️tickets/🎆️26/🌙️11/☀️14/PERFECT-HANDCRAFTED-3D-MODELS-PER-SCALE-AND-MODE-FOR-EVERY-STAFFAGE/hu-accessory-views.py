"""🔍 Binds the visible accessory profiles and coarse head or garment outlines to their physical thermal surfaces."""
from pathlib import Path
from runpy import run_path
import json
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
for slug in ['elderly-with-cane','person-with-rollator','tour-guide','jogger']:
    p=next((root/slug).iterdir());old=p.read_text();text=old;a=json.loads(text)['asset']
    name={'tour-guide':'badge','jogger':'headband'}.get(slug,'spectacle-frame')
    def change(start,id,items):
        global text
        i=text.rindex('{',0,text.index('"id": "'+id+'"',start));j=sp['member_end'](text,i);depth=(i-text.rindex('\n',0,i)-1)//2
        text=text[:i]+(',\n'+'  '*depth).join(sp['dump'](q,depth,'compact') for q in items)+text[j:]
    for d,r in a['representations'].items():
        for view,shapes in r['geometry'].items():
            marker=text.index('"'+view+'": [',text.index('"'+d+'": {',text.index('"representations": {')))
            for s in shapes:
                if name in s.get('regions',[]) and 'region' in s:
                    q={**s};q.pop('region');change(marker,s['id'],[q])
                if s['id']=='badge-edge' and 'solid' not in s:
                    b=next(q for q in a['volumes'][d]['body'] if q['id']=='badge')
                    x,y,_=b['transform']['translation'];rx,ry,rz=[v/2 for v in b['size']]
                    q={**s,'filled':True,'closed':True,'points':[[round(x-rx,5),round(y-ry,5)],[round(x+rx,5),round(y-ry,5)],[round(x+rx,5),round(y+ry,5)],[round(x-rx,5),round(y+ry,5)]],'solid':{'kind':'extruded','sweep':'w','z0':-rz,'z1':rz}}
                    change(marker,s['id'],[q])
            if any(s.get('region')==name or name in s.get('regions',[]) for s in shapes):continue
            parent={'tour-guide':'jacket','jogger':'hair'}.get(slug,'head')
            host=next((s for s in shapes if s['id']==parent),None)
            if not host:continue
            marker=text.index('"'+view+'": [',text.index('"'+d+'": {',text.index('"representations": {')))
            shapes50=a['representations']['50']['geometry'][view]
            items=[]
            if d!='500' and name=='spectacle-frame' and view=='elevation':
                arm=next(q for q in shapes50 if q['id']=='glasses-temple')
                real=next(q for q in a['volumes']['50']['body'] if q['id']=='glasses-temple-l')
                items=[{**arm,'points':[real['p0'][:2],real['p1'][:2]]}]
                if d=='100':items.append(next(q for q in shapes50 if q['id']=='glasses-rim'))
            elif name=='spectacle-frame' and d=='100' and view=='front':
                for q in shapes50:
                    if q['id'] in ('glasses-rim-l','glasses-rim-r'):items.append({**q,'points':q['points'][::2]})
                    elif q['id']=='glasses-bar':items.append(q)
            elif name=='badge' and d!='500' and view=='elevation':
                body=next(q for q in a['volumes'][d]['body'] if q['id']=='badge')
                x,y,_=body['transform']['translation'];edge=round(y-body['size'][1]/2,5)
                items=[{'id':'badge-edge','bone':'torso','role':'detail','z':host['z']+.1,'filled':False,'tone':.65,'region':'badge','material':'cotton-clothing','kind':'poly','points':[[round(x-.043,5),edge],[round(x+.043,5),edge]],'closed':False}]
            elif name=='headband' and d=='200' and view in ['elevation','front']:
                band=next(s for s in a['representations']['100']['geometry'][view] if s['id']=='headband')
                items=[{**band,'points':[[.172,-.103],[.199,-.1],[.199,.101],[.172,.103]] if view=='elevation' else [[.172,-.087],[.199,-.085],[.199,.085],[.172,.087]]}]
            if items:change(marker,parent,[host,*items])
            else:
                q={**host,'regions':list(dict.fromkeys([host['region'],*host.get('regions',[]),name]))}
                q.pop('region');change(marker,parent,[q])
    if name=='headband':
        d='500';vol=a['volumes'][d]
        for q in vol['thermal']:
            if q['id'] in ['th-hair-crown','th-hair-nape'] and 'layer' not in q:
                marker=text.index('"thermal": [',text.index('"500": {',text.index('"volumes": {')))
                change(marker,q['id'],[{**q,'layer':'hair-pack'}])
    assert json.loads(text) and p.read_text()==old
    if old!=text:p.write_text(text)
