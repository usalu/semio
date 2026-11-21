"""🐐 Draw the goat's middle and back costal arches and the pigeon's frontal lower limbs."""
from pathlib import Path
import json
import math

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
for slug,field in (('goat','volumes'),('pigeon','representations')):
    path=next(root.glob(f'*/{slug}/*.json'))
    raw=path.read_text()
    asset=json.loads(raw)['asset']
    value=asset[field]
    if slug=='goat':
        full={s['id']:s for s in value['50']['structure']}
        coarse=value['500']['structure']
        if not any(s['id']=='rib-girth' for s in coarse):
            continue
        pos=next(i for i,s in enumerate(coarse) if s['id']=='rib-girth')
        arches=[]
        for level in (7,12):
            for side in ('n','f'):
                pieces=[full[f'rib-{level}-{side}-{i}'] for i in range(1,5)]
                centres=[pieces[0]['p0'],*[p['p1'] for p in pieces]]
                sections=[]
                for i,at in enumerate(centres):
                    tangent=[b-a for a,b in zip(centres[max(0,i-1)],centres[min(4,i+1)])]
                    x,y,z=[v/math.hypot(*tangent) for v in tangent]
                    rot=[-y,x,0,1+z]
                    q=[round(v/math.hypot(*rot),6) for v in rot]
                    radius=pieces[min(i,3)]['r0']
                    sections.append(dict(at=at,rx=radius,ry=round(radius*0.81,6),rotation=q))
                arches.append(dict(id=f'rib-{level}-{side}',kind='loft',bone='thorax',material='bone',role='body',tone=0.9,sections=sections,capStart=True,capEnd=True,slender=True,member='thorax',part='member'))
        coarse[pos:pos+1]=arches
    else:
        front=value['200']['geometry']['front']
        for s in front:
            if s['id'] not in ('near-leg','near-foot','far-leg','far-foot') or s['closed']:
                continue
            foot='foot' in s['id']
            x,top=s['points'][0]
            y=s['points'][-1][1]
            widths=(0.0015,0.0021,0.0025) if foot else (0.0015,0.0020,0.00155)
            h=0.77 if foot else 0.47
            a,b,c=widths
            m=round(top+(y-top)*h,5)
            s['points']=[[round(u,5),v] for u,v in ((x-a,top),(x+a,top),(x+b,m),(x+c,y),(x-c,y),(x-b,m))]
            s.update(closed=True,filled=True)
            s.pop('penMm',None)
    start=raw.index('{',raw.index('"'+field+'":'))
    old,n=json.JSONDecoder().raw_decode(raw[start:])
    if old!=value:
        if path.read_text()!=raw: raise SystemExit(f'{slug} changed concurrently')
        path.write_text(raw[:start]+json.dumps(value,indent=2,ensure_ascii=False).replace('\n','\n    ')+raw[start+n:])
        print('[DEBUG]',slug)

