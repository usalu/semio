"""🐾 Preserve feline bone ends, shaft tapers and the fox's distal articulation at 1:200."""
import json
import runpy
from pathlib import Path

t=Path(__file__).parent
b=runpy.run_path(str(t/'living-botanical-shells.py'))
root=b['root']
for name in ('domestic-cat','red-fox'):
    for p in (next(root.glob('*/'+name+'/*.json')),t/'volumes/animals'/f'{name}.json'):
        raw=p.read_text();doc=json.loads(raw);a=doc.get('asset',doc);vol=a['volumes']['200']['structure']
        for s in vol:
            namepart=s['id'].removeprefix('near-').removeprefix('far-')
            if name=='domestic-cat' and len(s.get('sections',[]))==10:
                if namepart.startswith('caudal'):ids=(0,2,5,9)
                elif 'spine' in namepart:ids=(0,2,4,7,9)
                elif namepart in ('scapula','humerus','femur','tibia','radius-ulna'):ids=(0,1,4,7,9)
                else:ids=(0,2,5,8,9)
                s['sections']=[s['sections'][i] for i in ids]
            elif name=='domestic-cat' and namepart.startswith('caudal') and len(s.get('sections',[]))==5:
                s['sections']=[s['sections'][i] for i in (0,1,2,4)]
            elif name=='red-fox' and s.get('kind')=='ellipsoid' and any(x in s['id'] for x in ('joint-paw','joint-tail','joint-metacarpus','joint-metatarsus','joint-ear')):
                b['oval'](s,s['radii'],8,(-46,0,46))
        start=raw.index('[',raw.index('"structure":',raw.index('"200":',raw.index('"volumes":'))))
        original,size=json.JSONDecoder().raw_decode(raw[start:])
        if original==vol:continue
        value=json.dumps(vol,ensure_ascii=False,indent=2).replace('\n','\n'+' '*8)
        if p.read_text()!=raw:raise RuntimeError(p)
        p.write_text(raw[:start]+value+raw[start+size:])

