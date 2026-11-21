"""🎋 Transport the sections of each sunlit and shaded bamboo tip along its authored centreline."""
import json
import math
import runpy
import sys
from pathlib import Path

ticket=Path(__file__).parent
saved=sys.argv;sys.argv=[sys.argv[0]]
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));sys.argv=saved
add,sub,mul,dot,cross,unit,mesh=(b[k] for k in ('add','sub','mul','dot','cross','unit','mesh'))
root=b['root']
for path in (next(root.glob('*/bamboo-clump/*.json')),ticket/'volumes/vegetation/bamboo-clump.json'):
    raw=path.read_text();doc=json.loads(raw);a=doc.get('asset',doc)
    for k,n in [('100',8),('200',8),('500',6)]:
        for s in a['volumes'][k]['thermal']:
            if not s['id'].startswith(('th-tp-sun-','th-tp-shade-')) or s.get('kind')!='polyhedron':continue
            v=s['vertices'];sections=[v[j:j+n] for j in range(0,len(v)-2,n)]
            centers=[[sum(p[i] for p in sec)/len(sec) for i in range(3)] for sec in sections]
            rings=[];axis=None
            for i,(ring,c) in enumerate(zip(sections,centers)):
                tangent=unit(sub(centers[min(len(centers)-1,i+1)],centers[max(0,i-1)]))
                old=sub(ring[0],c) if axis is None else axis
                axis=unit(sub(old,mul(tangent,dot(old,tangent))))
                perp=cross(tangent,axis)
                rx=math.dist(ring[0],c)
                cosine=math.cos(2*math.pi/n);sine=math.sin(2*math.pi/n)
                ry=math.sqrt(max(0,(math.dist(ring[1],c)**2-(rx*cosine)**2)/sine**2))
                rings.append([add(c,add(mul(axis,rx*math.cos(j*math.tau/n)),mul(perp,ry*math.sin(j*math.tau/n)))) for j in range(n)])
            mesh(s,rings)
    # Preserve all other authored data and the compact topology layout.
    dec=json.JSONDecoder();cursor=0;parts=[]
    import re
    models=a['volumes'];stack=[s for _,v in models.items() for s in v['thermal'] if s['id'].startswith(('th-tp-sun-','th-tp-shade-')) and s.get('kind')=='polyhedron']
    byid={}
    for s in stack:byid.setdefault(s['id'],[]).append(s)
    for match in re.finditer(r'"id"\s*:\s*"(th-tp-(?:sun|shade)-\d+)"',raw):
        start=raw.rfind('{',cursor,match.start())
        obj,end=dec.raw_decode(raw,start)
        if obj.get('kind')!='polyhedron':continue
        replacement=byid[obj['id']].pop(0)
        if obj==replacement:continue
        parts.extend((raw[cursor:start],json.dumps(replacement,ensure_ascii=False,separators=(',',':'))))
        cursor=end
    parts.append(raw[cursor:]);out=''.join(parts)
    if out!=raw:
        if path.read_text()!=raw:raise RuntimeError(path)
        path.write_text(out)

