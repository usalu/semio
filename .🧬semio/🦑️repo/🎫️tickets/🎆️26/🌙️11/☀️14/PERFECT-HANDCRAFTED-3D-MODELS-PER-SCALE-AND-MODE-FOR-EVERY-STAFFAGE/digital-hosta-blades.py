"""🍃 Fit paper leaf halves and raised petioles to the hosta's cordate blades and wind joints."""
from pathlib import Path
import re,json
t=Path(__file__).resolve().parent;cat=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/hosta-clump/🔣️.json'));doc=json.loads(cat.read_text());a=doc['asset'];bones={q['id']:q for q in a['rig']['bones']};views={'50':['front'],'200':['elevation','front']}
for d,targets in views.items():
 v=a['volumes'][d];r=a['representations'][d]
 for view in targets:
    shapes=r['geometry'][view];retained=[q for q in shapes if not(q.get('filled') and q['bone'].startswith(('blade-','petiole-')))]
    for part in v['body']:
        name=part['id']
        if not re.fullmatch(r'blade-\d\d(-[lr])?|petiole-\d\d',name) or not part.get('vertices'):continue
        old=[q for q in shapes if q['bone']==part['bone']and q.get('filled')]
        if not old:continue
        n=part['triangles'][0][2];points=part['vertices'];count=(len(points)-2)//n
        if (len(points)-2)%n:raise ValueError(name)
        take=sorted({round(i*(count-1)/min(count-1,8 if d=='50' else 7))for i in range(min(count-1,8 if d=='50' else 7)+1)})
        rings=[points[i*n:i*n+n]for i in take];length=bones[part['bone']]['lengthM']
        for j,(first,last) in enumerate(zip(rings,rings[1:])):
            def frame(ring):
                x=sum(p[0] for p in ring)/len(ring);ys=[p[1]for p in ring];zs=[p[2]for p in ring]
                ym=(min(ys)+max(ys))/2
                radius=(max(ys)-min(ys))*(.385 if d=='200' and name.startswith('blade-') else .5)
                return x,min(zs),max(zs),ym-radius,ym+radius
            x0,l0,r0,b0,t0=frame(first);x1,l1,r1,b1,t1=frame(last)
            u0=max(0,min(x0/length,1));u1=max(0,min(x1/length,1))
            def span(p,q):
                m=(q-p)/(u1-u0)if abs(u1-u0)>1e-6 else 0
                return round(p-m*u0,7),round(p+m*(1-u0),7)
            y0,y0tip=span(b0,b1);y1,y1tip=span(t0,t1)
            if y0>=y1 or y0tip>=y1tip:
                c0,c1=span((b0+t0)/2,(b1+t1)/2);half=(t0-b0+t1-b1)/4
                y0,y1=round(c0-half,7),round(c0+half,7)
                y0tip,y1tip=round(c1-half,7),round(c1+half,7)
            mid=(x0+x1)/2
            original=min(old,key=lambda q:abs(mid-sum(p[0]for p in q['points'])/len(q['points'])))
            retained.append(dict(id=name+'-rib-'+str(j+1),bone=part['bone'],role=original['role'],z=original['z'],filled=True,tone=original['tone'],region=original['region'],material=part['material'],penMm=.20,kind='poly',closed=True,points=[[round(x,7),round(z,7)]for x,z in [(x0,l0),(x1,l1),(x1,r1),(x0,r0)]],solid=dict(kind='extruded',sweep='v',z0=y0,z1=y1,zTip=[y0tip,y1tip])))
    r['geometry'][view]=sorted(retained,key=lambda q:q['z'])
(t/'🗑️generated/digital-hosta-lamina.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
