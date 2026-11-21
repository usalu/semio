"""🌴 Follow each date-palm frond’s ridged, bending blade and its actual lateral spread in wind."""
from pathlib import Path
import json
t=Path(__file__).resolve().parent
path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/date-palm/🔣️.json'))
doc=json.loads(path.read_text());a=doc['asset'];v=a['volumes']['500'];r=a['representations']['500']
bones={b['id']:b for b in a['rig']['bones']}
for view,shapes in r['geometry'].items():
    old={}
    for s in shapes:
        if s.get('region')=='fronds':old.setdefault(s['bone'],[]).append(s)
    drawn=[]
    for s in v['body']:
        if s['material']!='palm-frond':continue
        verts=s['vertices'];n=len(verts)-2
        assert n%4==0
        rings=[]
        for i in range(0,n,4):
            pts=verts[i:i+4];rings.append((sum(p[0]for p in pts)/4,min(p[1]for p in pts),max(p[1]for p in pts),min(p[2]for p in pts),max(p[2]for p in pts)))
        length=bones[s['bone']]['lengthM']
        for j,(p,q) in enumerate(zip(rings,rings[1:])):
            x,lo,hi,left,right=p;end,low,high,l,rgt=q
            u0=max(0,min(x/length,1));u1=max(0,min(end/length,1))
            for zone,(ta,tb,h) in enumerate([(0,1,.64)]):
                def station(yl,yh):return ((yl+yh)/2-(yh-yl)*h/2,(yl+yh)/2+(yh-yl)*h/2)
                p0,p1=station(lo,hi);q0,q1=station(low,high)
                profile=[[x,left+(right-left)*ta],[end,l+(rgt-l)*ta],[end,l+(rgt-l)*tb],[x,left+(right-left)*tb]]
                def ends(pa,qa):
                    slope=(qa-pa)/(u1-u0) if abs(u1-u0)>1e-7 else 0
                    return round(pa-u0*slope,6),round(pa+(1-u0)*slope,6)
                low0,low1=ends(p0,q0);high0,high1=ends(p1,q1)
                solid=dict(kind='extruded',sweep='v',z0=low0,z1=high0,zTip=[low1,high1])
                originals=old[s['bone']]
                original=originals[0]
                layers=sorted(((min(p[0] for p in q['points'])+max(p[0] for p in q['points']))/2,q['z'])for q in originals)
                middle=(x+end)/2
                depth=layers[0][1]if middle<=layers[0][0]else layers[-1][1]
                for (ax,az),(bx,bz) in zip(layers,layers[1:]):
                    if ax<=middle<=bx and bx>ax:depth=az+(bz-az)*(middle-ax)/(bx-ax);break
                drawn.append(dict(id=s['id']+'-'+str(j+1)+'-'+str(zone+1),bone=s['bone'],role='body',z=round(depth,6),penMm=.20,filled=True,tone=original['tone'],region='fronds',material='palm-frond',kind='poly',closed=True,points=[[round(x,6),round(y,6)]for x,y in profile],solid=solid))
    r['geometry'][view]=sorted([q for q in shapes if q.get('region')!='fronds']+drawn,key=lambda q:q['z'])
(t/'🗑️generated/digital-palm-rachis.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
