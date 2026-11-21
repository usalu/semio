"""🎍 Reduce the intermediate sprays while keeping their stems, widest leaf group and drooping tips."""
import itertools
import json
import math
from pathlib import Path

catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*vegetation/bamboo-clump/*.json'))
source=Path(__file__).parent/'volumes/vegetation/bamboo-clump.json'
for p in (catalog,source):
    raw=p.read_text()
    obj=json.loads(raw)
    v=obj.get('asset',obj)['volumes']['100']
    errors=[]
    for s in v['body']:
        if s.get('material')!='bamboo-leaf' or s.get('role')!='body' or len(s.get('sections',[]))!=9:
            continue
        rings=s['sections']
        widest=max(range(9),key=lambda i:rings[i]['rx']*rings[i]['ry'])
        def residual(chosen):
            errors=[]
            for i,j in zip(chosen,chosen[1:]):
                a,b=rings[i],rings[j]
                for q in range(i+1,j):
                    t=(q-i)/(j-i)
                    mid=[(1-t)*x+t*y for x,y in zip(a['at'],b['at'])]
                    dr=max(abs(rings[q][key]-((1-t)*a[key]+t*b[key])) for key in ('rx','ry'))
                    errors.append(math.dist(mid,rings[q]['at'])+dr)
            return max(errors)
        options=[(0,*middle,8) for middle in itertools.combinations(range(1,8),4) if widest in (0,*middle,8)]
        best=min(options,key=residual)
        errors.append(residual(best))
        s['sections']=[rings[i] for i in best]
    if errors:
        d=raw.index('"volumes":')
        start=raw.index('{',raw.index('"100":',d))
        old,n=json.JSONDecoder().raw_decode(raw[start:])
        depth='      ' if 'asset' in obj else '    '
        patch=json.dumps(v,ensure_ascii=False,indent=2).replace('\n','\n'+depth)
        if raw!=p.read_text(): raise SystemExit('Bamboo changed concurrently')
        p.write_text(raw[:start]+patch+raw[start+n:])
        print('[DEBUG]',len(errors),'sprays, largest interpolation error',round(max(errors)*1000,2),'mm')

