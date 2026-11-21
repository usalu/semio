"""🦴 Retains readable vertebrosternal load paths through the coarsest models of seven previously articulated thoraces."""
from pathlib import Path
from runpy import run_path
import json,re
t=Path(__file__).resolve().parent
tools=run_path(str(t/'hu-open-ribs.py'));sp=tools['sp'];tube=tools['tube'];stations=tools['stations'];centre=tools['centre'];hum=tools['hum']
choices={
'child-on-kick-scooter':('costal-{n}-{s}',[200,500],2.5,1.8),
'cyclist':('costal-{n}-{s}',[200,500],4.4,2.9),
'skateboarder':('costal-{n}-{s}',[200,500],4.5,3.0),
'jogger':('sk-rib-{n}-{s}',[500],4.1,2.7),
'running-child':('sk-rib-{n}-{s}',[500],2.2,1.25),
'standing-phone-user':('rib-{s}-{n}',[500],4.0,2.8),
'worker-carrying-box':('rib-{s}-{n}',[500],4.4,3.0)
}
for slug,(key,scales,radius,sr) in choices.items():
    p=next((hum/slug).glob('*'));old=p.read_text();text=old
    asset=json.loads(old)['asset'];fine={q['id']:q for q in asset['volumes']['50']['structure']}
    fs=next(q for q in fine.values() if q.get('bone')=='torso' and 'sternum' in q['id'])
    sternum=stations(fs)
    for sc in scales:
        k=str(sc);v=asset['volumes'][k]
        if any(q['id']=='open-rib-2-l' for q in v['structure']):continue
        spine=next(q for q in v['structure'] if q.get('bone')=='torso' and q['id'] in ('spine','spine-bar','sk-spine'))
        vert=stations(spine);new=[]
        stops=[]
        for n in ([2,7,10] if sc==200 else [2,7]):
            for side in ['l','r']:
                points=[[c*1000 for c in q['at']] for q in fine[key.format(n=n,s=side)]['sections']]
                sign=1 if side=='l' else -1
                path=[points[round(i*(len(points)-1))] for i in ([0,.24,.48,.70,.84,1] if sc==200 else [0,.24,.48,.75,1])]
                path[0]=[points[0][0],centre(vert,points[0][0]),sign*2]
                if n<=7:
                    h=max(sternum[0][0],min(sternum[-1][0],points[-1][0]))
                    path[-1]=[h,centre(sternum,h),sign*2]
                    if side=='l':stops.append(h)
                elif n==10:
                    h=stops[-1];path[-1]=[h-1,centre(sternum,h)+1.5,sign*6]
                new.append(tube('open-rib-'+str(n)+'-'+side,path,radius,True))
        top,bot=max(stops),min(stops)
        mid=(top+bot)/2
        new.append(tube('open-sternum',[(h,centre(sternum,h),0) for h in (top,mid,bot)],sr,True))
        idx=text.index('"'+k+'": {',text.index('"volumes": {'));i,j=sp['member_range'](text[idx:],'        ','structure');i+=idx;j+=idx
        source=text[i:j];chunks=re.split(r',\n(?=          \{)',source);fresh=[]
        for q,chunk in zip(v['structure'],chunks):
            if not(q.get('bone')=='torso' and 'ribcage' in q['id']):fresh.append(chunk)
            if q is spine:fresh.extend('          '+sp['dump'](r,5,'compact') for r in new)
        assert len(chunks)==len(v['structure'])
        text=text[:i]+',\n'.join(fresh)+text[j:]
        note=v['decisions'][-1]
        for lang,extra in [('en',' Open ribs retain the vertebral and sternal attachments of this figure.'),('de',' Offene Rippenzüge erhalten die Ansätze an Wirbelsäule und Brustbein dieser Figur.')]:
            text=text.replace(json.dumps(note[lang],ensure_ascii=False),json.dumps(note[lang]+extra,ensure_ascii=False),1)
    doc=json.loads(text)['asset']
    if text==old:continue
    assert p.read_text()==old;p.write_text(text)
    snap=t/'volumes'/'humans'/(slug+'.json');orig=snap.read_text();saved=json.loads(orig)
    for sc in scales:
        k=str(sc);saved['volumes'][k]['structure']=doc['volumes'][k]['structure'];saved['volumes'][k]['decisions']=doc['volumes'][k]['decisions']
    assert snap.read_text()==orig;snap.write_text(json.dumps(saved,ensure_ascii=False,separators=(',',':'))+'\n')
    print(slug)
